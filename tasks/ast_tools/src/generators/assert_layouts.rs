// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/assert_layouts.rs

//! Calculate memory layout of all types.
//! Generate const assertions for the correctness of those calculations.
//!
//! Memory layouts are different on 64-bit and 32-bit platforms.
//! Calculate each separately, and generate assertions for each.

use std::{
    cmp::{max, min},
    io,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    AST_CRATE_PATH, AST_MACROS_CRATE_PATH,
    output::{Output, output_path},
    schema::{
        Def, Discriminant, EnumDef, FieldDef, PrimitiveDef, Schema, StructDef, StructOrEnum,
        TypeDef, TypeId, Visibility,
        extensions::layout::{Layout, Niche, Offset, PlatformLayout},
    },
    utils::{generate_phf_map, number_lit},
};

use super::{Generator, define_generator};

/// Generator for memory layout assertions.
pub struct AssertLayoutsGenerator;

define_generator!(AssertLayoutsGenerator);

impl Generator for AssertLayoutsGenerator {
    /// Calculate layouts of all types.
    fn prepare(&self, schema: &mut Schema) {
        let mut calculator = LayoutCalculator { schema };
        for type_id in calculator.schema.types.indices() {
            calculator.calculate_type(type_id);
        }
    }

    /// Generate assertions that calculated layouts are correct,
    /// and struct layout data for `robusta_ast_macros` crate.
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        Ok(vec![generate_assertions(schema), generate_struct_details(schema)])
    }
}

/// Layout calculator.
struct LayoutCalculator<'s> {
    schema: &'s mut Schema,
}

impl LayoutCalculator<'_> {
    /// Calculate layout for a type.
    ///
    /// If layout was calculated already, just return the existing `Layout`.
    fn calculate_type(&mut self, type_id: TypeId) -> Layout {
        // `align` field is set to 0 initially, but that's an illegal value
        if self.schema.types[type_id].layout().layout_64.align == 0 {
            let layout = match &self.schema.types[type_id] {
                TypeDef::Struct(_) => self.calculate_struct(type_id),
                TypeDef::Enum(_) => self.calculate_enum(type_id),
                TypeDef::Primitive(primitive_def) => Self::calculate_primitive(primitive_def),
                TypeDef::Option(option_def) => self.calculate_option(option_def.inner_type_id),
                TypeDef::Box(_) => Self::calculate_box(),
                TypeDef::Vec(_) => Self::calculate_vec(),
                TypeDef::Cell(cell_def) => self.calculate_cell(cell_def.inner_type_id),
            };
            *self.schema.types[type_id].layout_mut() = layout;
        }
        self.schema.types[type_id].layout().clone()
    }
}

/// What the variants of an enum, including inherited, add up to.
struct EnumState {
    min_discriminant: Discriminant,
    max_discriminant: Discriminant,
    layout_64: PlatformLayout,
    layout_32: PlatformLayout,
}

struct StructState<'s> {
    layout_64: PlatformLayout,
    layout_32: PlatformLayout,
    struct_def: &'s mut StructDef,
    field_count: u32,
    current_align_64: u32,
    current_align_32: u32,
}

struct FieldData {
    index: usize,
    layout: Layout,
    priority: u64,
}

const MAX_ALIGN: u32 = 1 << 31;

impl<'s> StructState<'s> {
    fn new(struct_def: &'s mut StructDef) -> Self {
        Self {
            layout_64: PlatformLayout::from_size_align(0, 1),
            layout_32: PlatformLayout::from_size_align(0, 1),
            struct_def,
            field_count: 0,
            current_align_64: MAX_ALIGN,
            current_align_32: MAX_ALIGN,
        }
    }

    fn add_field(&mut self, field_data: &FieldData) {
        let offset_64 = Self::update(
            &mut self.layout_64,
            &field_data.layout.layout_64,
            &mut self.current_align_64,
            &self.struct_def.name,
        );
        let offset_32 = Self::update(
            &mut self.layout_32,
            &field_data.layout.layout_32,
            &mut self.current_align_32,
            &self.struct_def.name,
        );

        // Store offset on `field`
        let field = &mut self.struct_def.fields[field_data.index];
        field.offset = Offset { offset_64, offset_32, layout_index: self.field_count };

        self.field_count += 1;
    }

    fn update(
        layout: &mut PlatformLayout,
        field_layout: &PlatformLayout,
        current_align: &mut u32,
        struct_name: &str,
    ) -> u32 {
        // Check alignment is correct for this field
        let offset = layout.size;
        assert!(
            offset.is_multiple_of(field_layout.align),
            "Incorrect alignment for struct fields in `{struct_name}`"
        );

        // Update alignment
        layout.align = max(layout.align, field_layout.align);

        // Update niche.
        // Take the largest niche. Preference for (in order):
        // * Largest single range of niche values.
        // * Largest number of niche values at start of range.
        // * Earlier field (earlier after re-ordering).
        if let Some(field_niche) = &field_layout.niche
            && layout.niche.as_ref().is_none_or(|niche| {
                field_niche.count_max() > niche.count_max()
                    || (field_niche.count_max() == niche.count_max()
                        && field_niche.count_start > niche.count_start)
            })
        {
            layout.niche = Some(field_niche.clone());
        }

        // Next field starts after this one
        if field_layout.size > 0 {
            let next_offset = offset + field_layout.size;
            layout.size = next_offset;

            // Get highest alignment next field can have
            *current_align = next_offset.isolate_lowest_one();
        }

        // Return offset of this field
        offset
    }
}

impl LayoutCalculator<'_> {
    /// Calculate layout for a struct.
    ///
    /// All structs in AST are `#[repr(C)]`. In a `#[repr(C)]` struct, compiler does not re-order the fields,
    /// so they are stored in memory in same order as they're defined.
    ///
    /// So we determine a field order here which avoids excess padding. [`generate_struct_details`] generates
    /// code describing this field order, and `#[ast]` macro re-orders the fields.
    ///
    /// This gives us the stability guarantees of `#[repr(C)]` without the downside of structs which are
    /// larger than they need to be due to excess padding.
    ///
    /// Alignment of the struct is the highest alignment of its fields (or 1 if no fields).
    /// Size of struct is a multiple of its alignment.
    ///
    /// A struct has a niche if any of its fields has a niche. The niche will be the largest niche
    /// in any of its fields. Padding bytes are not used as niches.
    ///
    /// # Field order
    ///
    /// Fields are ordered according to the following rules, in order:
    ///
    /// 1. `span: Span` goes first.
    /// 2. `node_id: Cell<_>` goes next.
    /// 3. Fields with higher alignment on 64-bit systems go first.
    /// 4. Fields with higher alignment on 32-bit systems go first.
    /// 5. If the highest-aligned field cannot go next, take the next field which has suitable alignment.
    /// 6. Otherwise, retain original field order.
    ///
    /// The AST is generally visited in source order, so keeping fields in original order as much as
    /// possible is preferable for CPU caching.
    ///
    /// `span` and `node_id` have the same position in every node that has them, so a choice or an
    /// `AstKind` can read either without a branch.
    ///
    /// Ordering by 64-bit alignment first, then 32-alignment creates a layout that is optimally packed
    /// on both platforms. A pointer-aligned field has the same alignment as the 8-byte fields before it
    /// on 64-bit systems, and as the 4-byte fields after it on 32-bit systems, so it never results in
    /// padding on either platform.
    fn calculate_struct(&mut self, type_id: TypeId) -> Layout {
        // Get layout of fields' types and calculate optimal field order
        let mut fields = (0..self.schema.struct_def(type_id).fields.len())
            .map(|index| {
                let field_type_id = self.schema.struct_def(type_id).fields[index].type_id;
                let layout = self.calculate_type(field_type_id);

                let field = &self.schema.struct_def(type_id).fields[index];
                let field_type = field.type_def(self.schema);
                let priority = if field.name == "span" && field_type.name() == "Span" {
                    // Highest priority
                    u64::MAX
                } else if field.name == "node_id" && field_type.is_cell() {
                    // 2nd highest priority
                    u64::MAX - 1
                } else {
                    // Prioritize fields with higher alignment on 64-bit systems, then 32-bit systems
                    (u64::from(layout.layout_64.align) << 32) | u64::from(layout.layout_32.align)
                };

                FieldData { index, layout, priority }
            })
            .collect::<Vec<_>>();

        fields.sort_by(|f1, f2| f1.priority.cmp(&f2.priority).reverse());

        // Add fields
        let mut state = StructState::new(self.schema.struct_def_mut(type_id));

        while !fields.is_empty() {
            // Find next field with alignment <= current alignment for both 32 bit and 64 bit.
            // `fields` is sorted by alignment in descending order,
            // so this will pick the next field with largest alignment which can be placed at current offset.
            let index = fields.iter().position(|f| {
                f.layout.layout_64.align <= state.current_align_64
                    && f.layout.layout_32.align <= state.current_align_32
            });

            if let Some(index) = index {
                // Found field with suitable alignment - add it to struct
                let field_data = fields.remove(index);
                state.add_field(&field_data);
            } else {
                // No field with suitable alignment found.
                // Double desired alignment, and add padding so it's aligned.
                // Go round loop again to find next field which satisfies new looser alignment requirements.
                let smallest_align_64 = fields.last().expect("not empty").layout.layout_64.align;
                if state.current_align_64 < smallest_align_64 {
                    // No field meets alignment requirement for 64 bit
                    state.current_align_64 *= 2;
                    state.layout_64.size =
                        state.layout_64.size.next_multiple_of(state.current_align_64);
                } else {
                    // No field meets alignment requirement for 32 bit
                    state.current_align_32 *= 2;
                    state.layout_32.size =
                        state.layout_32.size.next_multiple_of(state.current_align_32);
                }
            }
        }

        // Round up size to alignment
        let StructState { mut layout_64, mut layout_32, .. } = state;
        layout_64.size = layout_64.size.next_multiple_of(layout_64.align);
        layout_32.size = layout_32.size.next_multiple_of(layout_32.align);

        Layout { layout_64, layout_32 }
    }

    /// Calculate layout for an enum.
    ///
    /// All enums in AST are `#[repr(C, u8)]` (if has fields) or `#[repr(u8)]` if fieldless.
    ///
    /// `#[repr(C, u8)]` enums have alignment of highest-aligned variant.
    /// Size is size of the union of the variants + alignment of highest-aligned variant.
    /// The union's size is the size of the largest variant, rounded up to that alignment.
    ///
    /// Fieldless `#[repr(u8)]` enums obey the same rules. Fieldless variants act as size 0, align 1.
    ///
    /// Any unused discriminant values at start of end of the range form a niche.
    fn calculate_enum(&mut self, type_id: TypeId) -> Layout {
        let mut state = EnumState {
            min_discriminant: Discriminant::MAX,
            max_discriminant: 0,
            layout_64: PlatformLayout::from_size_align(0, 1),
            layout_32: PlatformLayout::from_size_align(0, 1),
        };
        self.process_enum_variants(type_id, &mut state);
        let EnumState { min_discriminant, max_discriminant, mut layout_64, mut layout_32 } = state;

        layout_64.size = layout_64.size.next_multiple_of(layout_64.align) + layout_64.align;
        layout_32.size = layout_32.size.next_multiple_of(layout_32.align) + layout_32.align;

        // Any unused discriminant values at start of end of the range form a niche.
        // Note: The unused discriminants must be at start or end of range, *not* in the middle.
        // `#[repr(u8)] enum Foo { A = 0, B = 255 }` has no niche.
        // The largest available range (from start or from end) is used for the niche.
        let niches_start = min_discriminant;
        let niches_end = Discriminant::MAX - max_discriminant;

        if niches_start != 0 || niches_end != 0 {
            let niche = Niche::new(u32::from(niches_start), u32::from(niches_end));
            layout_64.niche = Some(niche.clone());
            layout_32.niche = Some(niche);
        }

        Layout { layout_64, layout_32 }
    }

    /// Add an enum's variants to `state`, and those of the enums it inherits.
    fn process_enum_variants(&mut self, type_id: TypeId, state: &mut EnumState) {
        for index in 0..self.schema.enum_def(type_id).variants.len() {
            let variant = &self.schema.enum_def(type_id).variants[index];

            state.min_discriminant = min(state.min_discriminant, variant.discriminant);
            state.max_discriminant = max(state.max_discriminant, variant.discriminant);

            if let Some(field_type_id) = variant.field_type_id {
                let variant_layout = self.calculate_type(field_type_id);

                state.layout_64.size = max(state.layout_64.size, variant_layout.layout_64.size);
                state.layout_64.align = max(state.layout_64.align, variant_layout.layout_64.align);
                state.layout_32.size = max(state.layout_32.size, variant_layout.layout_32.size);
                state.layout_32.align = max(state.layout_32.align, variant_layout.layout_32.align);
            }
        }

        for index in 0..self.schema.enum_def(type_id).inherits.len() {
            let inherited_type_id = self.schema.enum_def(type_id).inherits[index];
            self.process_enum_variants(inherited_type_id, state);
        }
    }

    /// Calculate layout for an `Option`.
    ///
    /// * If inner type has a niche:
    ///   `Option` uses that niche to represent `None`.
    ///   The `Option` is same size and alignment as the inner type.
    /// * If inner type has no niche:
    ///   The `Option`'s size = inner type size + inner type alignment.
    ///   `Some` / `None` discriminant is stored as a `bool` in first byte.
    ///   This introduces a new niche, identical to a struct with `bool` as first field.
    fn calculate_option(&mut self, inner_type_id: TypeId) -> Layout {
        fn consume_niche(layout: &mut PlatformLayout) {
            if let Some(niche) = &mut layout.niche {
                if niche.count_start == 0 {
                    niche.count_end -= 1;
                } else {
                    niche.count_start -= 1;
                }

                if niche.count_start == 0 && niche.count_end == 0 {
                    layout.niche = None;
                }
            } else {
                layout.size += layout.align;
                layout.niche = Some(Niche::new(0, 254));
            }
        }

        let mut layout = self.calculate_type(inner_type_id);
        consume_niche(&mut layout.layout_64);
        consume_niche(&mut layout.layout_32);
        layout
    }

    /// Calculate layout for a `Box`.
    ///
    /// All `Box`es have same layout, regardless of the inner type.
    /// `Box`es are pointer-sized, with a single niche (like `NonNull`).
    fn calculate_box() -> Layout {
        Layout {
            layout_64: PlatformLayout::from_size_align_niche(8, 8, Niche::new(1, 0)),
            layout_32: PlatformLayout::from_size_align_niche(4, 4, Niche::new(1, 0)),
        }
    }

    /// Calculate layout for a `Vec`.
    ///
    /// All `Vec`s have same layout, regardless of the inner type: a pointer, two `u32`s, and a reference.
    /// They have a single niche on the pointer, which is `NonNull`.
    fn calculate_vec() -> Layout {
        Layout {
            layout_64: PlatformLayout::from_size_align_niche(24, 8, Niche::new(1, 0)),
            layout_32: PlatformLayout::from_size_align_niche(16, 4, Niche::new(1, 0)),
        }
    }

    /// Calculate layout for a `Cell`.
    ///
    /// `Cell`s have same layout as their inner type, but with no niche.
    fn calculate_cell(&mut self, inner_type_id: TypeId) -> Layout {
        let mut layout = self.calculate_type(inner_type_id);
        layout.layout_64.niche = None;
        layout.layout_32.niche = None;
        layout
    }

    /// Calculate layout for a primitive.
    fn calculate_primitive(primitive_def: &PrimitiveDef) -> Layout {
        match primitive_def.name {
            "bool" => Layout::from_size_align_niche(1, 1, &Niche::new(0, 254)),
            "u8" => Layout::from_size_align(1, 1),
            "u32" => Layout::from_size_align(4, 4),
            "NonZeroU32" => Layout::from_size_align_niche(4, 4, &Niche::new(1, 0)),
            // `&str` is a `NonNull` pointer + `usize` pair. Niche for 0 on the pointer field
            "&str" => Layout {
                layout_64: PlatformLayout::from_size_align_niche(16, 8, Niche::new(1, 0)),
                layout_32: PlatformLayout::from_size_align_niche(8, 4, Niche::new(1, 0)),
            },
            name => panic!("Unknown primitive type: {name}"),
        }
    }
}

/// Generate layout assertions for all types.
fn generate_assertions(schema: &Schema) -> Output {
    let mut assertions_64 = TokenStream::new();
    let mut assertions_32 = TokenStream::new();

    for type_def in schema.structs_and_enums() {
        match type_def {
            StructOrEnum::Struct(struct_def) => {
                assertions_64.extend(generate_struct_assertions(struct_def, true, schema));
                assertions_32.extend(generate_struct_assertions(struct_def, false, schema));
            }
            StructOrEnum::Enum(enum_def) => {
                assertions_64.extend(generate_enum_assertions(enum_def, true));
                assertions_32.extend(generate_enum_assertions(enum_def, false));
            }
        }
    }

    let tokens = quote! {
        use std::mem::offset_of;

        ///@@line_break
        use oxc_index::nonmax::NonMaxU32;
        use robusta_span::Span;
        use robusta_syntax::{
            node::NodeId,
            operator::{AssignOp, BinaryOp, PostfixOp, UnaryOp},
            reference::ReferenceId,
            scope::ScopeId,
            symbol::SymbolId,
        };

        ///@@line_break
        use crate::ast::*;

        ///@@line_break
        #[cfg(target_pointer_width = "64")]
        const _: () = { #assertions_64 };

        ///@@line_break
        #[cfg(target_pointer_width = "32")]
        const _: () = { #assertions_32 };

        ///@@line_break
        #[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
        const _: () = panic!("Platforms with pointer width other than 64 or 32 bit are not supported");
    };
    Output::from_tokens(output_path(AST_CRATE_PATH, "assert_layouts.rs"), &tokens)
}

/// Get the layout of a type on a platform.
fn platform_layout(layout: &Layout, is_64: bool) -> &PlatformLayout {
    if is_64 { &layout.layout_64 } else { &layout.layout_32 }
}

/// Generate layout assertions for a struct.
/// This includes size and alignment assertions, plus assertions about offset of fields.
fn generate_struct_assertions(struct_def: &StructDef, is_64: bool, schema: &Schema) -> TokenStream {
    let layout = platform_layout(&struct_def.layout, is_64);

    let fields_total_bytes: u32 = struct_def
        .fields
        .iter()
        .map(|field| platform_layout(field.type_def(schema).layout(), is_64).size)
        .sum();
    let padding_bytes = layout.size - fields_total_bytes;
    let padding_comment = format!("@ Padding: {padding_bytes} bytes");

    let struct_ident = struct_def.ident();
    let size_align_assertions = generate_size_align_assertions(layout, &struct_ident);

    // Cannot create assertions for private fields (can't access them)
    // or foreign types (we don't know what fields they have)
    let mut fields = struct_def
        .fields
        .iter()
        .filter(|field| !struct_def.is_foreign && field.visibility != Visibility::Private)
        .collect::<Vec<_>>();
    fields.sort_by_key(|field| field.offset.layout_index);
    let offset_assertions = fields.into_iter().map(|field| {
        let field_ident = field.ident();
        let offset = number_lit(field_offset(field, is_64));
        quote! {
            assert!(offset_of!(#struct_ident, #field_ident) == #offset);
        }
    });

    quote! {
        ///@@line_break
        #[doc = #padding_comment]
        #size_align_assertions
        #(#offset_assertions)*
    }
}

/// Get the offset of a field on a platform.
fn field_offset(field: &FieldDef, is_64: bool) -> u32 {
    if is_64 { field.offset.offset_64 } else { field.offset.offset_32 }
}

/// Generate layout assertions for an enum.
/// This is just size and alignment assertions.
fn generate_enum_assertions(enum_def: &EnumDef, is_64: bool) -> TokenStream {
    let size_align_assertions =
        generate_size_align_assertions(platform_layout(&enum_def.layout, is_64), &enum_def.ident());
    quote! {
        ///@@line_break
        #size_align_assertions
    }
}

/// Generate size and alignment assertions for a type.
fn generate_size_align_assertions(layout: &PlatformLayout, ident: &Ident) -> TokenStream {
    let size = number_lit(layout.size);
    let align = number_lit(layout.align);
    quote! {
        assert!(size_of::<#ident>() == #size);
        assert!(align_of::<#ident>() == #align);
    }
}

/// Generate struct field orders for `robusta_ast_macros` crate.
///
/// `#[ast]` macro will re-order struct fields in order we provide here.
fn generate_struct_details(schema: &Schema) -> Output {
    let map = generate_phf_map(schema.structs().map(|struct_def| {
        // Get layout indexes of fields in source order.
        // If struct as written already has fields in layout order, then no-reordering is required,
        // in which case output `None`.
        let new_indexes =
            struct_def.fields.iter().map(|field| field.offset.layout_index).collect::<Vec<_>>();
        let reordering_needed =
            new_indexes.iter().enumerate().any(|(index, &new)| index != new as usize);

        let field_order = if reordering_needed {
            let new_indexes = new_indexes
                .into_iter()
                .map(|index| number_lit(u8::try_from(index).expect("a few fields")));
            quote!(Some(&[#(#new_indexes),*]))
        } else {
            // Field order stays as it is in source
            quote!(None)
        };

        let is_node = struct_def.kind.has_kind;

        (
            struct_def.name.as_str(),
            quote!( StructDetails { field_order: #field_order, is_node: #is_node } ),
        )
    }));

    let tokens = quote! {
        use crate::ast::StructDetails;

        ///@@line_break
        /// Details of how `#[ast]` macro should modify structs.
        #[expect(clippy::unreadable_literal)]
        pub static STRUCTS: phf::Map<&'static str, StructDetails> = #map;
    };

    Output::from_tokens(output_path(AST_MACROS_CRATE_PATH, "structs.rs"), &tokens)
}

#[cfg(test)]
mod tests {
    use crate::{
        generators::{AssertLayoutsGenerator, AstKindGenerator, Generator},
        parse::parse_sources,
        schema::{Schema, StructDef},
    };

    fn schema(source: &str) -> Schema {
        let file = syn::parse_file(source).expect("a source file");
        let mut schema = parse_sources(vec![file]).expect("a schema");
        AssertLayoutsGenerator.prepare(&mut schema);
        schema
    }

    fn struct_def<'s>(schema: &'s Schema, name: &str) -> &'s StructDef {
        schema.type_by_name(name).as_struct().expect("a struct")
    }

    /// Field names in memory order.
    fn memory_order(struct_def: &StructDef) -> Vec<&str> {
        let mut fields = struct_def.fields.iter().collect::<Vec<_>>();
        fields.sort_by_key(|field| field.offset.layout_index);
        fields.into_iter().map(|field| field.name.as_str()).collect()
    }

    const NODE: &str = "
        #[ast] pub struct Span { pub start: u32, pub end: u32 }
        #[ast(visit)]
        pub struct Node<'a> {
            pub flag: bool,
            pub name: &'a str,
            pub node_id: Cell<u32>,
            pub span: Span,
            pub children: Vec<'a, Node<'a>>,
        }
    ";

    #[test]
    fn only_a_struct_with_a_kind_is_marked_a_node() {
        let mut schema = schema(
            "define_nonmax_u32_index_type! { #[ast] pub struct NodeId; }
            #[ast(foreign = NonMaxU32)] struct NonMaxU32Alias(NonZeroU32);
            #[ast(visit)] pub struct Node { pub node_id: Cell<NodeId> }
            #[ast(visit)] pub struct Modifier { pub flag: bool }",
        );
        AstKindGenerator.prepare(&mut schema);
        let outputs = AssertLayoutsGenerator.generate(&schema).unwrap();
        let details = outputs.iter().find(|output| output.path.ends_with("structs.rs")).unwrap();
        assert_eq!(details.code.matches("is_node : true").count(), 1, "{}", details.code);
    }

    #[test]
    fn span_and_node_id_lead_memory_and_source_order_is_kept() {
        let schema = schema(NODE);
        let node = struct_def(&schema, "Node");
        assert_eq!(memory_order(node), ["span", "node_id", "flag", "name", "children"]);
        let source_order = node.fields.iter().map(|field| field.name.as_str()).collect::<Vec<_>>();
        assert_eq!(source_order, ["flag", "name", "node_id", "span", "children"]);

        let offset = |name: &str| {
            let field = node.fields.iter().find(|field| field.name == name).expect("a field");
            (field.offset.offset_64, field.offset.offset_32)
        };
        assert_eq!(offset("span"), (0, 0));
        assert_eq!(offset("node_id"), (8, 8));
        assert_eq!(offset("flag"), (12, 12));
        assert_eq!(offset("name"), (16, 16));
        assert_eq!(offset("children"), (32, 24));
        assert_eq!((node.layout.layout_64.size, node.layout.layout_64.align), (56, 8));
        assert_eq!((node.layout.layout_32.size, node.layout.layout_32.align), (40, 4));
    }

    #[test]
    fn a_choice_rounds_its_largest_variant_up_to_its_alignment() {
        let schema = schema(
            "
            #[ast] pub struct Span { pub start: u32, pub end: u32 }
            #[ast] pub struct Keyword { pub span: Span, pub kind: u8 }
            #[ast] pub struct Name<'a> { pub span: Span, pub name: &'a str }
            #[ast] pub enum Modifier<'a> { Keyword(Keyword) = 0, Name(Box<'a, Name<'a>>) = 1 }
            #[ast] pub struct Holder<'a> { pub modifier: Option<Modifier<'a>>, pub span: Option<Span> }
            ",
        );
        let modifier = schema.type_by_name("Modifier").layout();
        assert_eq!((modifier.layout_64.size, modifier.layout_64.align), (24, 8));
        assert_eq!((modifier.layout_32.size, modifier.layout_32.align), (16, 4));

        // `Option<Modifier>` takes the tag's niche, `Option<Span>` has none to take
        let holder = &struct_def(&schema, "Holder").layout;
        assert_eq!((holder.layout_64.size, holder.layout_64.align), (40, 8));
        assert_eq!((holder.layout_32.size, holder.layout_32.align), (28, 4));
    }

    #[test]
    fn a_choice_lays_out_its_inherited_variants_as_its_own() {
        let schema = schema(
            "
            #[ast] pub struct Span { pub start: u32, pub end: u32 }
            #[ast] pub enum Leaf { A(Span) = 8 }
            #[ast] pub enum Top<'a> { B(Box<'a, Span>) = 0, INHERIT(Leaf) }
            #[ast] pub struct Holder<'a> { pub top: Option<Top<'a>> }
            ",
        );
        // `Leaf`'s `Span` sets the size, and `Box` the alignment, of `Top`.
        let top = schema.type_by_name("Top").layout();
        assert_eq!((top.layout_64.size, top.layout_64.align), (16, 8));
        assert_eq!((top.layout_32.size, top.layout_32.align), (12, 4));

        // The discriminants run from 0 to `Leaf`'s 8, which leaves the niche for `None`.
        let niche = top.layout_64.niche.as_ref().expect("a niche");
        assert_eq!((niche.count_start, niche.count_end), (0, 247));
        let holder = &struct_def(&schema, "Holder").layout;
        assert_eq!(holder.layout_64.size, 16);
    }

    #[test]
    fn assertions_check_every_field_and_foreign_types_only_size_and_alignment() {
        let schema = schema(
            "
            #[ast] pub struct Span { pub start: u32, pub end: u32 }
            #[ast(foreign = Flags)] struct FlagsAlias(u8);
            #[ast] pub struct Comment { pub span: Span, pub flags: Flags }
            ",
        );
        let outputs = AssertLayoutsGenerator.generate(&schema).expect("outputs");
        let assertions = &outputs[0];
        assert_eq!(assertions.path, "crates/robusta_ast/src/generated/assert_layouts.rs");
        for expected in [
            "offset_of ! (Comment , span) == 0",
            "offset_of ! (Comment , flags) == 8",
            "size_of :: < Flags > () == 1",
        ] {
            assert!(assertions.code.contains(expected), "{expected}\n{}", assertions.code);
        }
        assert!(!assertions.code.contains("offset_of ! (Flags"), "{}", assertions.code);
    }
}
