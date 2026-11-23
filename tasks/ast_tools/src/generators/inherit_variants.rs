// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/inherit_variants.rs

//! Generator for conversion methods between enums which inherit variants from other enums.
//!
//! Where enum `Parent` inherits all the variants of enum `Child`, the 2 enums are laid out in memory
//! such that the shared variants have identical discriminants and field types. This allows zero-cost
//! conversion between the 2 types.
//!
//! This generator produces, for each such `Parent` / `Child` pair:
//!
//! * Methods on `Parent`: `is_child`, `into_child`, `as_child`, `as_child_mut`, `to_child`, `to_child_mut`
//! * Methods on `Child`: `as_parent`
//! * `impl TryFrom<Parent> for Child`
//! * `impl From<Child> for Parent`
//!
//! It also generates a `match_child!` macro for each enum which is inherited by another enum, and
//! the table of each enum's variants which `#[ast]` macro reads to insert the inherited variants.

use std::io;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustc_hash::FxHashMap;

use crate::{
    AST_CRATE_PATH, AST_MACROS_CRATE_PATH, invalid,
    output::{Output, output_path},
    schema::{
        Def, EnumDef, Schema, TypeDef, TypeId, VariantDef, extensions::layout::PlatformLayout,
    },
    utils::{article_for, generate_phf_map, number_lit},
};

use super::{Generator, define_generator};

/// Generator for conversion methods between enums which inherit variants from other enums.
pub struct InheritVariantsGenerator;

define_generator!(InheritVariantsGenerator);

impl Generator for InheritVariantsGenerator {
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        check_layouts(schema)?;
        Ok(vec![generate_inherit_variants(schema), generate_enum_details(schema)])
    }
}

/// Converting a reference between 2 enums relies on them having the same size and alignment.
fn check_layouts(schema: &Schema) -> io::Result<()> {
    for enum_def in schema.enums() {
        for child in enum_def.all_inherits(schema) {
            let (parent_layout, child_layout) = (&enum_def.layout, &child.layout);
            let same = |parent: &PlatformLayout, child: &PlatformLayout| {
                parent.size == child.size && parent.align == child.align
            };
            if !same(&parent_layout.layout_64, &child_layout.layout_64)
                || !same(&parent_layout.layout_32, &child_layout.layout_32)
            {
                return Err(invalid(format!(
                    "{} and {} must have the same layout, as it inherits its variants",
                    enum_def.name, child.name
                )));
            }
        }
    }
    Ok(())
}

/// Generate `inherit_variants.rs` in `robusta_ast` - conversion methods, trait impls and `match_*!` macros.
fn generate_inherit_variants(schema: &Schema) -> Output {
    let impls = generate_impls(schema);
    let match_macros = generate_match_macros(schema);

    let output = quote! {
        //!@ Some `TryFrom` impls have a single non-shared variant left for the catch-all arm
        #![expect(clippy::match_wildcard_for_single_variants)]

        ///@@line_break
        use std::ptr::NonNull;

        ///@@line_break
        use crate::ast::*;

        ///@@line_break
        #impls

        #match_macros
    };

    Output::from_tokens(output_path(AST_CRATE_PATH, "inherit_variants.rs"), &output)
}

/// Generate `enums.rs` in `robusta_ast_macros`.
///
/// Contains 2 statics the `#[ast]` macro reads, so it doesn't have to derive this info from the
/// enum on every compilation:
///
/// * `ENUMS` - a `phf::Map` of `EnumDetails` for each enum, keyed by name.
/// * `INHERITED_ENUMS` - the (flattened) variants of each enum which is inherited by another enum.
///   `EnumDetails::inherits` contains indexes into this list. `#[ast]` macro inserts these variants
///   into enums which inherit them (replacing the `INHERIT` marker variants).
fn generate_enum_details(schema: &Schema) -> Output {
    // Assign an index to each enum which is inherited by another enum.
    // This is the index into `INHERITED_ENUMS`.
    let mut indexes = FxHashMap::<TypeId, u32>::default();
    let mut inheritable_enums = vec![];
    for enum_def in schema.enums() {
        if !enum_def.inherited_by.is_empty() {
            indexes.insert(enum_def.id, u32::try_from(inheritable_enums.len()).unwrap());
            inheritable_enums.push(enum_def);
        }
    }

    // Generate `ENUMS` `phf::Map`
    let map = generate_phf_map(schema.enums().map(|enum_def| {
        let is_fieldless = enum_def.is_fieldless();
        let inherit_indexes =
            enum_def.inherits.iter().map(|inherited_id| number_lit(indexes[inherited_id]));
        let details = quote! {
            EnumDetails { is_fieldless: #is_fieldless, inherits: &[#(#inherit_indexes),*] }
        };
        (enum_def.name.as_str(), details)
    }));

    // Generate `INHERITED_ENUMS` array elements - one `InheritedEnum` per inheritable enum, in index order
    let inherited_enums =
        inheritable_enums.iter().map(|enum_def| generate_inherited_enum(enum_def, schema));
    let count = number_lit(u64::try_from(inheritable_enums.len()).unwrap());

    let code = quote! {
        use crate::ast::{EnumDetails, EnumVariant, InheritedEnum};

        ///@@line_break
        /// Details of how `#[ast]` macro should modify enums.
        #[expect(clippy::unreadable_literal)]
        pub static ENUMS: phf::Map<&'static str, EnumDetails> = #map;

        ///@@line_break
        /// Each enum which is inherited by another enum, indexed by [`EnumDetails::inherits`].
        /// `#[ast]` macro inserts these enums' variants into enums which inherit them.
        ///
        /// [`EnumDetails::inherits`]: crate::ast::EnumDetails::inherits
        pub static INHERITED_ENUMS: [InheritedEnum; #count] = [ #(#inherited_enums),* ];
    };

    Output::from_tokens(output_path(AST_MACROS_CRATE_PATH, "enums.rs"), &code)
}

/// Generate an `InheritedEnum` describing an inheritable enum's (flattened) variants. e.g.:
///
/// ```ignore
/// InheritedEnum {
///     doc: " Inherited from [`Expression`]",
///     variants: &[
///         EnumVariant {
///             name: "BooleanLiteral",
///             inner_name: "BooleanLiteral",
///             inner_has_lifetime: false,
///             is_boxed: true,
///             discriminant: 0,
///         },
///         // ...
///     ],
/// }
/// ```
fn generate_inherited_enum(enum_def: &EnumDef, schema: &Schema) -> TokenStream {
    let enum_name = enum_def.name();

    let variants = enum_def.all_variants(schema).map(|variant| {
        let name = &variant.name;
        let discriminant = number_lit(variant.discriminant);

        let field_type =
            variant.field_type(schema).expect("Inheritable enum variants must have a field");
        // Variants are either `Box<'a, Inner>` or a named type (`Inner` / `Inner<'a>`) directly
        let (inner_type, is_boxed) = match field_type.as_box() {
            Some(box_def) => (box_def.inner_type(schema), true),
            None => (field_type, false),
        };
        assert!(
            matches!(inner_type, TypeDef::Struct(_) | TypeDef::Enum(_)),
            "Field type of inheritable enum variant `{enum_name}::{name}` is not a struct/enum",
        );
        let inner_name = inner_type.name();
        let inner_has_lifetime = inner_type.has_lifetime(schema);

        quote! {
            EnumVariant {
                name: #name,
                inner_name: #inner_name,
                inner_has_lifetime: #inner_has_lifetime,
                is_boxed: #is_boxed,
                discriminant: #discriminant,
            }
        }
    });

    let doc = format!(" Inherited from [`{enum_name}`]");
    let label = format!("@ `{enum_name}`");
    quote! {
        #[doc = #label]
        InheritedEnum {
            doc: #doc,
            variants: &[#(#variants),*],
        }
    }
}

/// Generate conversion methods and trait impls for all enums which inherit from other enums.
fn generate_impls(schema: &Schema) -> TokenStream {
    let impls = schema.enums().flat_map(|enum_def| {
        enum_def.all_inherits(schema).map(|child| generate_conversions(enum_def, child, schema))
    });

    quote! ( #(#impls)* )
}

/// Generate a `match_child!` macro for every enum which is inherited by another enum.
fn generate_match_macros(schema: &Schema) -> TokenStream {
    let mut output = TokenStream::new();

    for enum_def in schema.enums() {
        if !enum_def.inherited_by.is_empty() {
            output.extend(generate_match_macro(enum_def, schema));
        }
    }

    output
}

/// Generate a `match_child!` macro for an enum.
///
/// e.g. for `Expression`:
///
/// ```ignore
/// macro_rules! match_expression {
///     ($ty:ident) => {
///         $ty::BooleanLiteral(_) | $ty::NullLiteral(_) | /* ...all other variants... */
///     };
/// }
/// ```
fn generate_match_macro(enum_def: &EnumDef, schema: &Schema) -> TokenStream {
    let macro_ident = format_ident!("match_{}", enum_def.snake_name());

    let patterns = enum_def.all_variants(schema).map(|variant| {
        let variant_ident = variant.ident();
        quote!( $ty::#variant_ident(_) )
    });

    let doc = format!(" Macro for matching [`{}`]'s variants.", enum_def.name());
    let inherited =
        enum_def.all_inherits(schema).map(|item| item.name.as_str()).collect::<Vec<_>>();
    let inherited_doc = (!inherited.is_empty()).then(|| {
        let line = format!(" Includes variants inherited from [`{}`].", inherited.join("`], [`"));
        quote! {
            ///
            #[doc = #line]
        }
    });

    // `#[macro_export]` exports the macro at the crate root.
    quote! {
        ///@@line_break
        #[doc = #doc]
        #inherited_doc
        #[macro_export]
        macro_rules! #macro_ident {
            ($ty:ident) => {
                #(#patterns)|*
            };
        }
    }
}

/// Generate conversion methods and trait impls for converting between `parent` and `child`,
/// where `parent` inherits all of `child`'s variants.
fn generate_conversions(parent: &EnumDef, child: &EnumDef, schema: &Schema) -> TokenStream {
    let parent_ident = parent.ident();
    let parent_snake = parent.snake_name();
    let child_ident = child.ident();
    let child_snake = child.snake_name();

    // The shared variants are all of `child`'s variants (including those `child` itself inherits)
    let variant_idents = child.all_variants(schema).map(VariantDef::ident).collect::<Vec<_>>();

    let is_fn = format_ident!("is_{child_snake}");
    let into_fn = format_ident!("into_{child_snake}");
    let as_fn = format_ident!("as_{child_snake}");
    let as_mut_fn = format_ident!("as_{child_snake}_mut");
    let to_fn = format_ident!("to_{child_snake}");
    let to_mut_fn = format_ident!("to_{child_snake}_mut");
    let as_reverse_fn = format_ident!("as_{parent_snake}");

    let parent_name = parent.name();
    let child_name = child.name();
    let parent_article = article_for(parent_name);
    let child_article = article_for(child_name);

    let is_doc = format!(
        " Return if {parent_article} [`{parent_name}`] is {child_article} [`{child_name}`].",
    );
    let into_doc =
        format!(" Convert {parent_article} [`{parent_name}`] to {child_article} [`{child_name}`].");
    let from_doc =
        format!(" Convert {child_article} [`{child_name}`] to {parent_article} [`{parent_name}`].");

    let as_doc1 = format!(
        " Convert {parent_article} [`&{parent_name}`] to {child_article} [`&{child_name}`]."
    );
    let as_doc2 = format!(" [`&{parent_name}`]: {parent_name}");
    let as_doc3 = format!(" [`&{child_name}`]: {child_name}");

    let as_reverse_doc = format!(
        " Convert {child_article} [`&{child_name}`] to {parent_article} [`&{parent_name}`]."
    );

    let as_mut_doc1 = format!(
        " Convert {parent_article} [`&mut {parent_name}`] to {child_article} [`&mut {child_name}`]."
    );
    let as_mut_doc2 = format!(" [`&mut {parent_name}`]: {parent_name}");
    let as_mut_doc3 = format!(" [`&mut {child_name}`]: {child_name}");

    quote! {
        ///@@line_break
        impl<'a> #parent_ident<'a> {
            #[doc = #is_doc]
            #[inline]
            pub fn #is_fn(&self) -> bool {
                matches!(self, #(Self::#variant_idents(_))|*)
            }

            ///@@line_break
            #[doc = #into_doc]
            ///
            /// # Panics
            /// Panics if not convertible.
            #[inline]
            pub fn #into_fn(self) -> #child_ident<'a> {
                #child_ident::try_from(self).unwrap()
            }

            ///@@line_break
            #[doc = #as_doc1]
            ///
            #[doc = #as_doc2]
            #[doc = #as_doc3]
            #[inline]
            pub fn #as_fn(&self) -> Option<&#child_ident<'a>> {
                if self.#is_fn() {
                    ///@ SAFETY: Transmute is safe because discriminants + types are identical between
                    ///@ `parent` and `child` for the shared variants
                    Some(unsafe { NonNull::from_ref(self).cast::<#child_ident>().as_ref() })
                } else {
                    None
                }
            }

            ///@@line_break
            #[doc = #as_mut_doc1]
            ///
            #[doc = #as_mut_doc2]
            #[doc = #as_mut_doc3]
            #[inline]
            pub fn #as_mut_fn(&mut self) -> Option<&mut #child_ident<'a>> {
                if self.#is_fn() {
                    ///@ SAFETY: Transmute is safe because discriminants + types are identical between
                    ///@ `parent` and `child` for the shared variants
                    Some(unsafe { NonNull::from_mut(self).cast::<#child_ident>().as_mut() })
                } else {
                    None
                }
            }

            ///@@line_break
            #[doc = #as_doc1]
            ///
            /// # Panics
            /// Panics if not convertible.
            ///
            #[doc = #as_doc2]
            #[doc = #as_doc3]
            #[inline]
            pub fn #to_fn(&self) -> &#child_ident<'a> {
                self.#as_fn().unwrap()
            }

            ///@@line_break
            #[doc = #as_mut_doc1]
            ///
            /// # Panics
            /// Panics if not convertible.
            ///
            #[doc = #as_mut_doc2]
            #[doc = #as_mut_doc3]
            #[inline]
            pub fn #to_mut_fn(&mut self) -> &mut #child_ident<'a> {
                self.#as_mut_fn().unwrap()
            }
        }

        ///@@line_break
        impl<'a> #child_ident<'a> {
            #[doc = #as_reverse_doc]
            ///
            #[doc = #as_doc3]
            #[doc = #as_doc2]
            #[inline]
            pub fn #as_reverse_fn(&self) -> &#parent_ident<'a> {
                ///@ SAFETY: Transmute is safe because discriminants + types are identical between
                ///@ `parent` and `child` for the shared variants
                unsafe { NonNull::from_ref(self).cast::<#parent_ident>().as_ref() }
            }
        }

        ///@@line_break
        impl<'a> TryFrom<#parent_ident<'a>> for #child_ident<'a> {
            type Error = ();

            ///@@line_break
            #[doc = #into_doc]
            ///
            /// # Errors
            /// Returns `Err` if not convertible.
            #[inline]
            fn try_from(value: #parent_ident<'a>) -> Result<Self, Self::Error> {
                ///@ Compiler should implement this as a check of discriminant and then zero-cost transmute,
                ///@ as discriminants for `parent` and `child` are aligned
                match value {
                    #(#parent_ident::#variant_idents(o) => Ok(#child_ident::#variant_idents(o)),)*
                    _ => Err(()),
                }
            }
        }

        ///@@line_break
        impl<'a> From<#child_ident<'a>> for #parent_ident<'a> {
            #[doc = #from_doc]
            #[inline]
            fn from(value: #child_ident<'a>) -> Self {
                ///@ Compiler should implement this as zero-cost transmute as discriminants
                ///@ for `child` and `parent` are aligned
                match value {
                    #(#child_ident::#variant_idents(o) => #parent_ident::#variant_idents(o),)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        generators::{AssertLayoutsGenerator, Generator, InheritVariantsGenerator},
        parse::parse_sources,
        schema::Schema,
    };

    fn schema(source: &str) -> Schema {
        let file = syn::parse_file(source).expect("a source file");
        let mut schema = parse_sources(vec![file]).expect("a schema");
        AssertLayoutsGenerator.prepare(&mut schema);
        schema
    }

    const NODES: &str = "
        #[ast(visit)] pub struct A { pub flag: u8 }
        #[ast(visit)] pub struct B { pub flag: u8 }
        #[ast(visit)] pub struct C { pub flag: u8 }
        #[ast(visit)] pub enum Leaf<'a> { A(Box<'a, A>) = 8 }
        #[ast(visit)] pub enum Middle<'a> { INHERIT(Leaf<'a>), B(Box<'a, B>) = 1 }
        #[ast(visit)] pub enum Top<'a> { C(Box<'a, C>) = 0, INHERIT(Middle<'a>) }
        #[ast] pub enum Plain { D = 0, E = 1 }
    ";

    #[test]
    fn an_enum_reads_its_inherited_variants_through_every_level() {
        let schema = schema(NODES);
        let top = schema.type_by_name("Top").as_enum().expect("an enum");
        let variants =
            top.all_variants(&schema).map(|variant| variant.name.as_str()).collect::<Vec<_>>();
        assert_eq!(variants, ["C", "B", "A"]);
        let inherits = top.all_inherits(&schema).map(|item| item.name.as_str()).collect::<Vec<_>>();
        assert_eq!(inherits, ["Middle", "Leaf"]);
    }

    #[test]
    fn conversions_cover_every_inherited_choice_and_the_macro_matches_every_variant() {
        let outputs = InheritVariantsGenerator.generate(&schema(NODES)).expect("outputs");
        let conversions = &outputs[0];
        assert_eq!(conversions.path, "crates/robusta_ast/src/generated/inherit_variants.rs");
        for expected in [
            "impl < 'a > TryFrom < Top < 'a >> for Middle < 'a >",
            "impl < 'a > From < Leaf < 'a >> for Top < 'a >",
            "pub fn as_leaf (& self) -> Option < & Leaf < 'a >>",
            "macro_rules ! match_middle",
            "macro_rules ! match_leaf",
            "$ ty :: B (_) | $ ty :: A (_)",
        ] {
            assert!(conversions.code.contains(expected), "{expected}\n{}", conversions.code);
        }
        assert!(!conversions.code.contains("match_top"), "{}", conversions.code);

        let enums = &outputs[1];
        assert_eq!(enums.path, "crates/robusta_ast_macros/src/generated/enums.rs");
        for expected in [
            "name : \"A\" , inner_name : \"A\" , inner_has_lifetime : false , is_boxed : true , discriminant : 8",
            "is_fieldless : true , inherits : & []",
        ] {
            assert!(enums.code.contains(expected), "{expected}\n{}", enums.code);
        }
    }

    #[test]
    fn a_choice_with_a_different_layout_from_what_it_inherits_is_an_error() {
        let schema = schema(
            "
            #[ast(visit)] pub struct Wide { pub first: u32, pub second: u32, pub third: u32 }
            #[ast(visit)] pub struct A { pub flag: u8 }
            #[ast(visit)] pub enum Leaf<'a> { A(Box<'a, A>) = 8 }
            #[ast(visit)] pub enum Top<'a> { INHERIT(Leaf<'a>), Wide(Wide) = 0 }
            ",
        );
        let error = InheritVariantsGenerator.generate(&schema).err().expect("an error");
        assert_eq!(
            error.to_string(),
            "Top and Leaf must have the same layout, as it inherits its variants"
        );
    }
}
