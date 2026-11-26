use std::io;

use oxc_index::IndexVec;
use quote::ToTokens;
use rustc_hash::FxHashMap;
use syn::{
    Attribute, Expr, ExprLit, Field, GenericArgument, Generics, Ident, ItemStruct, Lit, Meta,
    PathArguments, Type, Variant, Visibility as SynVisibility, punctuated::Punctuated,
    token::Comma,
};

use crate::{
    DERIVES,
    codegen::{GeneratorOrDerive, get_runners},
    invalid,
    schema::{
        BoxDef, CellDef, Def, EnumDef, FieldDef, File, FileId, OptionDef, PrimitiveDef, Schema,
        StructDef, TypeDef, TypeId, VariantDef, VecDef, Visibility,
        extensions::{
            content_eq::{ContentEqStructField, ContentEqType},
            dummy::{DummyEnum, DummyStruct},
            kind::Kind,
            layout::{Layout, Offset},
            visit::{VisitEnum, VisitStruct, VisitVec},
        },
    },
};

use super::{
    attr::{AttrLocation, AttrPart},
    skeleton::{EnumSkeleton, Skeleton, StructSkeleton},
};

/// Parse [`Skeleton`]s into [`TypeDef`]s and return [`Schema`].
pub fn parse(skeletons: Vec<Skeleton>, files: IndexVec<FileId, File>) -> io::Result<Schema> {
    let mut parser = Parser {
        type_names: FxHashMap::default(),
        names: skeletons.iter().map(|skeleton| skeleton.name().to_string()).collect(),
        extra_types: Vec::new(),
    };
    for (type_id, name) in parser.names.iter_enumerated() {
        if parser.type_names.insert(name.clone(), type_id).is_some() {
            return Err(invalid(format!("2 types with same name: {name}")));
        }
    }

    let mut types = IndexVec::with_capacity(skeletons.len());
    for (index, skeleton) in skeletons.into_iter().enumerate() {
        let type_id = TypeId::from_usize(index);
        types.push(match skeleton {
            Skeleton::Struct(skeleton) => parser.parse_struct(type_id, skeleton)?,
            Skeleton::Enum(skeleton) => parser.parse_enum(type_id, skeleton)?,
        });
    }
    types.extend(parser.extra_types);
    set_inherited_by(&mut types)?;

    Ok(Schema { types, type_names: parser.type_names, files })
}

/// Populate `inherited_by` on enums, the reverse mapping of `inherits`.
fn set_inherited_by(types: &mut IndexVec<TypeId, TypeDef>) -> io::Result<()> {
    let mut pairs = Vec::new();
    for enum_def in types.iter().filter_map(TypeDef::as_enum) {
        for &inherited_id in &enum_def.inherits {
            pairs.push((inherited_id, enum_def.id));
        }
    }
    for (inherited_id, inheritor_id) in pairs {
        let inheritor = types[inheritor_id].name().to_string();
        let inherited = types[inherited_id]
            .as_enum_mut()
            .ok_or_else(|| invalid(format!("{inheritor}: `INHERIT` must name an enum")))?;
        inherited.inherited_by.push(inheritor_id);
    }
    Ok(())
}

/// Types parser.
struct Parser {
    /// Mapping from the name of every type found so far to its [`TypeId`].
    type_names: FxHashMap<String, TypeId>,
    /// Name of every type found so far.
    names: IndexVec<TypeId, String>,
    /// Types with no definition in the source files: primitives and containers.
    extra_types: Vec<TypeDef>,
}

impl Parser {
    /// Parse [`StructSkeleton`] to yield a [`TypeDef`].
    fn parse_struct(&mut self, type_id: TypeId, skeleton: StructSkeleton) -> io::Result<TypeDef> {
        let StructSkeleton { name, file_id, is_foreign, item } = skeleton;
        let fields = item
            .fields
            .iter()
            .map(|field| self.parse_field(field, &name, is_foreign))
            .collect::<io::Result<Vec<_>>>()?;
        let (generated_derives, plural_name) =
            parse_generated_derives_and_plural_name(&item.attrs, &name)?;
        let mut struct_def = StructDef {
            id: type_id,
            plural_name,
            has_lifetime: has_lifetime(&item.generics),
            is_foreign,
            visit: VisitStruct::default(),
            generated_derives,
            fields,
            kind: Kind::default(),
            layout: Layout::default(),
            dummy: DummyStruct::default(),
            content_eq: ContentEqType::default(),
            name,
            file_id,
        };
        parse_struct_attrs(&mut struct_def, &item)?;
        Ok(TypeDef::Struct(struct_def))
    }

    /// Parse struct field to [`FieldDef`].
    fn parse_field(
        &mut self,
        field: &Field,
        struct_name: &str,
        is_foreign: bool,
    ) -> io::Result<FieldDef> {
        let visibility = convert_visibility(&field.vis);
        let name = match &field.ident {
            Some(ident) => ident.to_string(),
            // Only layout reads a foreign type's field or a private field
            None if is_foreign || visibility == Visibility::Private => String::new(),
            None => return Err(invalid(format!("{struct_name}: a node has named fields"))),
        };
        let type_id = self.parse_type(&field.ty, struct_name)?;

        // Get doc comment
        let mut doc_comment = None;
        for attr in &field.attrs {
            if let Meta::NameValue(name_value) = &attr.meta
                && name_value.path.is_ident("doc")
                && let Expr::Lit(expr_lit) = &name_value.value
                && let Lit::Str(lit_str) = &expr_lit.lit
            {
                doc_comment = Some(lit_str.value().trim().to_string());
                break;
            }
        }

        Ok(FieldDef {
            name,
            type_id,
            visibility,
            doc_comment,
            offset: Offset::default(),
            content_eq: ContentEqStructField::default(),
        })
    }

    /// Parse [`EnumSkeleton`] to yield a [`TypeDef`].
    fn parse_enum(&mut self, type_id: TypeId, skeleton: EnumSkeleton) -> io::Result<TypeDef> {
        let EnumSkeleton { name, file_id, item } = skeleton;
        let mut variants = Vec::with_capacity(item.variants.len());
        let mut inherits = Vec::new();
        for variant in &item.variants {
            if variant.ident == "INHERIT" {
                inherits.push(self.parse_inherit_marker(variant, &name)?);
            } else {
                variants.push(self.parse_variant(variant, &name)?);
            }
        }
        let (generated_derives, plural_name) =
            parse_generated_derives_and_plural_name(&item.attrs, &name)?;
        let mut enum_def = EnumDef {
            id: type_id,
            plural_name,
            has_lifetime: has_lifetime(&item.generics),
            visit: VisitEnum::default(),
            generated_derives,
            variants,
            inherits,
            inherited_by: Vec::new(),
            layout: Layout::default(),
            dummy: DummyEnum::default(),
            content_eq: ContentEqType::default(),
            name,
            file_id,
        };
        for attr in &item.attrs {
            if attr.path().is_ident("ast") {
                parse_ast_parts(attr, AttrLocation::EnumAstAttr(&mut enum_def))?;
            } else {
                parse_attr(attr, AttrLocation::Enum(&mut enum_def))?;
            }
        }
        // `INHERIT` markers are not real variants, so are skipped here, to stay aligned with
        // `enum_def.variants`.
        let real_variants = item.variants.iter().filter(|variant| variant.ident != "INHERIT");
        for (variant_index, variant) in real_variants.enumerate() {
            for attr in &variant.attrs {
                parse_attr(attr, AttrLocation::EnumVariant(&mut enum_def, variant_index))?;
            }
        }
        Ok(TypeDef::Enum(enum_def))
    }

    /// Get the [`TypeId`] of the enum an `INHERIT(EnumName<'a>)` marker variant names.
    fn parse_inherit_marker(&self, variant: &Variant, enum_name: &str) -> io::Result<TypeId> {
        let bad_marker = || invalid(format!("{enum_name}: `INHERIT` must name an enum"));
        let [field] = variant.fields.iter().collect::<Vec<_>>()[..] else {
            return Err(bad_marker());
        };
        let Type::Path(path) = &field.ty else { return Err(bad_marker()) };
        let segment = path.path.segments.last().ok_or_else(bad_marker)?;
        self.type_names.get(&segment.ident.to_string()).copied().ok_or_else(bad_marker)
    }

    /// Parse enum variant to [`VariantDef`].
    fn parse_variant(&mut self, variant: &Variant, enum_name: &str) -> io::Result<VariantDef> {
        let name = variant.ident.to_string();
        let field_type_id = match variant.fields.iter().collect::<Vec<_>>().as_slice() {
            [] => None,
            [field] if field.ident.is_none() => Some(self.parse_type(&field.ty, enum_name)?),
            _ => {
                return Err(invalid(format!(
                    "{enum_name}::{name}: only enum variants with a single unnamed field are supported"
                )));
            }
        };
        let Some((_, discriminant)) = &variant.discriminant else {
            return Err(invalid(format!(
                "{enum_name}::{name}: all enum variants must have explicit discriminants"
            )));
        };
        let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = discriminant else {
            return Err(invalid(format!("{enum_name}::{name}: bad discriminant")));
        };
        let discriminant = lit
            .base10_parse()
            .map_err(|error| invalid(format!("{enum_name}::{name}: bad discriminant: {error}")))?;
        Ok(VariantDef { name, field_type_id, discriminant })
    }

    /// Resolve type to its [`TypeId`].
    fn parse_type(&mut self, ty: &Type, owner: &str) -> io::Result<TypeId> {
        let unknown = || invalid(format!("{owner}: unknown type `{}`", ty.to_token_stream()));
        match ty {
            Type::Reference(reference) => match reference.elem.as_ref() {
                Type::Path(path) if path.path.is_ident("str") => self.primitive("&str", owner),
                _ => Err(unknown()),
            },
            Type::Path(path) => {
                let segment = path.path.segments.last().ok_or_else(unknown)?;
                let name = segment.ident.to_string();
                let PathArguments::AngleBracketed(args) = &segment.arguments else {
                    return self.named_type(&name, owner);
                };
                let inner = args.args.iter().find_map(|arg| match arg {
                    GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                });
                match inner {
                    Some(inner) => {
                        let inner_type_id = self.parse_type(inner, owner)?;
                        self.container(&name, inner_type_id, owner)
                    }
                    None => self.named_type(&name, owner),
                }
            }
            _ => Err(unknown()),
        }
    }

    /// Get the [`TypeId`] of a type with no type arguments.
    fn named_type(&mut self, name: &str, owner: &str) -> io::Result<TypeId> {
        match self.type_names.get(name) {
            Some(&type_id) => Ok(type_id),
            None => self.primitive(name, owner),
        }
    }

    /// Get the [`TypeId`] of a primitive, creating its [`TypeDef`] on first use.
    fn primitive(&mut self, name: &str, owner: &str) -> io::Result<TypeId> {
        if let Some(&type_id) = self.type_names.get(name) {
            return Ok(type_id);
        }
        let name = match name {
            "bool" => "bool",
            "u8" => "u8",
            "u32" => "u32",
            "NonZeroU32" => "NonZeroU32",
            "&str" => "&str",
            _ => return Err(invalid(format!("{owner}: unknown type `{name}`"))),
        };
        Ok(self.create_type(name.to_string(), |id, _| {
            TypeDef::Primitive(PrimitiveDef { id, name, layout: Layout::default() })
        }))
    }

    /// Get the [`TypeId`] of a container, creating its [`TypeDef`] on first use.
    fn container(
        &mut self,
        wrapper: &str,
        inner_type_id: TypeId,
        owner: &str,
    ) -> io::Result<TypeId> {
        let name = format!("{wrapper}<{}>", self.names[inner_type_id]);
        if let Some(&type_id) = self.type_names.get(&name) {
            return Ok(type_id);
        }
        let layout = Layout::default();
        let build: fn(TypeId, String, TypeId, Layout) -> TypeDef = match wrapper {
            "Option" => |id, name, inner_type_id, layout| {
                TypeDef::Option(OptionDef { id, name, inner_type_id, layout })
            },
            "Box" => |id, name, inner_type_id, layout| {
                TypeDef::Box(BoxDef { id, name, inner_type_id, layout })
            },
            "Vec" => |id, name, inner_type_id, layout| {
                TypeDef::Vec(VecDef { id, name, inner_type_id, visit: VisitVec::default(), layout })
            },
            "Cell" => |id, name, inner_type_id, layout| {
                TypeDef::Cell(CellDef { id, name, inner_type_id, layout })
            },
            _ => return Err(invalid(format!("{owner}: unknown type `{name}`"))),
        };
        Ok(self.create_type(name, |id, name| build(id, name, inner_type_id, layout)))
    }

    /// Create a new type definition.
    fn create_type(
        &mut self,
        name: String,
        build: impl FnOnce(TypeId, String) -> TypeDef,
    ) -> TypeId {
        let type_id = self.names.push(name.clone());
        self.type_names.insert(name.clone(), type_id);
        self.extra_types.push(build(type_id, name));
        type_id
    }
}

fn has_lifetime(generics: &Generics) -> bool {
    generics.lifetimes().next().is_some()
}

/// Parse attributes on a struct and its fields with the generators and derives which own them.
fn parse_struct_attrs(struct_def: &mut StructDef, item: &ItemStruct) -> io::Result<()> {
    for attr in &item.attrs {
        if attr.path().is_ident("ast") {
            parse_ast_parts(attr, AttrLocation::StructAstAttr(struct_def))?;
        } else {
            parse_attr(attr, AttrLocation::Struct(struct_def))?;
        }
    }
    for (field_index, field) in item.fields.iter().enumerate() {
        for attr in &field.attrs {
            parse_attr(attr, AttrLocation::StructField(struct_def, field_index))?;
        }
    }
    Ok(())
}

/// Get the generator or derive which owns attribute `attr_name`.
fn attr_processor(attr_name: &str) -> Option<GeneratorOrDerive> {
    get_runners().into_iter().find(|processor| processor.attrs().contains(&attr_name))
}

/// Parse `#[ast]` attribute parts with the generators and derives which own them, e.g.
/// `#[ast(visit)]`.
fn parse_ast_parts(attr: &Attribute, mut location: AttrLocation<'_>) -> io::Result<()> {
    if matches!(attr.meta, Meta::Path(_)) {
        return Ok(());
    }
    let parts = attr
        .parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
        .map_err(|error| invalid(format!("{location}: bad #[ast] attribute: {error}")))?;
    for meta in &parts {
        let attr_name = meta.path().to_token_stream().to_string();
        if attr_name == "foreign" {
            continue;
        }
        let Some(processor) = attr_processor(&attr_name) else {
            return Err(invalid(format!("{location}: unknown #[ast] marker `{attr_name}`")));
        };
        processor.parse_attr(&attr_name, location.unpack(), AttrPart::None)?;
    }
    Ok(())
}

/// Parse an attribute, if a generator or derive owns it.
fn parse_attr(attr: &Attribute, mut location: AttrLocation<'_>) -> io::Result<()> {
    let attr_name = attr.path().to_token_stream().to_string();
    let Some(processor) = attr_processor(&attr_name) else { return Ok(()) };
    let bad_attr = |location: &AttrLocation<'_>| invalid(format!("{location}: bad #[{attr_name}]"));
    match &attr.meta {
        Meta::Path(_) => processor.parse_attr(&attr_name, location, AttrPart::None),
        Meta::List(list) => {
            let parts = list
                .parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                .map_err(|_| bad_attr(&location))?;
            for meta in &parts {
                let Meta::Path(path) = meta else { return Err(bad_attr(&location)) };
                let part_name = path.to_token_stream().to_string();
                processor.parse_attr(&attr_name, location.unpack(), AttrPart::Tag(&part_name))?;
            }
            Ok(())
        }
        Meta::NameValue(_) => Err(bad_attr(&location)),
    }
}

/// Get the traits named by `#[generate_derive(...)]` on a type, and its `#[plural(...)]` name.
fn parse_generated_derives_and_plural_name(
    attrs: &[Attribute],
    type_name: &str,
) -> io::Result<(Vec<String>, Option<String>)> {
    let mut generated_derives = Vec::new();
    let mut plural_name = None;
    for attr in attrs {
        if attr.path().is_ident("generate_derive") {
            let traits =
                attr.parse_args_with(Punctuated::<Ident, Comma>::parse_terminated).map_err(
                    |error| invalid(format!("{type_name}: bad #[generate_derive]: {error}")),
                )?;
            for trait_ident in traits {
                let trait_name = trait_ident.to_string();
                if !DERIVES.iter().any(|derive| derive.trait_name() == trait_name) {
                    return Err(invalid(format!(
                        "{type_name}: unknown derive trait `{trait_name}`"
                    )));
                }
                generated_derives.push(trait_name);
            }
        } else if attr.path().is_ident("plural") {
            let ident = attr
                .parse_args::<Ident>()
                .map_err(|error| invalid(format!("{type_name}: bad #[plural]: {error}")))?;
            if plural_name.is_some() {
                return Err(invalid(format!("{type_name}: multiple #[plural] attributes")));
            }
            plural_name = Some(ident.to_string());
        }
    }
    Ok((generated_derives, plural_name))
}

/// Convert `syn::Visibility` to our `Visibility` type.
fn convert_visibility(vis: &SynVisibility) -> Visibility {
    match vis {
        SynVisibility::Public(_) => Visibility::Public,
        SynVisibility::Restricted(_) => Visibility::Restricted,
        SynVisibility::Inherited => Visibility::Private,
    }
}
