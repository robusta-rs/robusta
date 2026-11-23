use std::{iter::FusedIterator, slice};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::utils::{pluralize, snake_case};

use super::{
    super::{
        FileId,
        extensions::{content_eq::ContentEqType, layout::Layout, visit::VisitEnum},
    },
    Def, Schema, TypeDef, TypeId,
};

pub type Discriminant = u8;

/// Type definition for an enum.
pub struct EnumDef {
    pub id: TypeId,
    pub name: String,
    /// The name from `#[plural(...)]`, where adding `s` would clash with another type's name.
    pub plural_name: Option<String>,
    pub file_id: FileId,
    pub has_lifetime: bool,
    pub visit: VisitEnum,
    /// The traits named by `#[generate_derive(...)]`.
    pub generated_derives: Vec<String>,
    pub variants: Vec<VariantDef>,
    /// Enums whose variants this enum inherits, through `INHERIT(EnumName<'a>)` marker variants.
    pub inherits: Vec<TypeId>,
    /// Enums which inherit this enum's variants. This is the reverse of [`inherits`].
    ///
    /// [`inherits`]: EnumDef::inherits
    pub inherited_by: Vec<TypeId>,
    pub layout: Layout,
    pub content_eq: ContentEqType,
}

impl EnumDef {
    /// Get plural type name in snake case.
    pub fn plural_snake_name(&self) -> String {
        snake_case(&self.plural_name.clone().unwrap_or_else(|| pluralize(&self.name)))
    }

    /// Get iterator over all this enum's variants, including inherited.
    #[must_use]
    pub fn all_variants<'s>(&'s self, schema: &'s Schema) -> AllVariantsIter<'s> {
        AllVariantsIter {
            variants_iter: self.variants.iter(),
            inherits_iter: self.all_inherits(schema),
        }
    }

    /// Get iterator over all enums this enum inherits variants from, transitively,
    /// in depth-first pre-order.
    #[must_use]
    pub fn all_inherits<'s>(&'s self, schema: &'s Schema) -> AllInheritsIter<'s> {
        AllInheritsIter { schema, stack: vec![self.inherits.iter()] }
    }

    /// Get whether all variants are fieldless.
    #[must_use]
    pub fn is_fieldless(&self) -> bool {
        // All AST enums are `#[repr(C, u8)]` or `#[repr(u8)]`.
        // Such enums must have at least 1 variant, so only way can have size 1
        // is if all variants are fieldless.
        self.layout.layout_64.size == 1
    }
}

impl Def for EnumDef {
    fn id(&self) -> TypeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn has_lifetime(&self, _schema: &Schema) -> bool {
        self.has_lifetime
    }

    fn ty_with_lifetime(&self, schema: &Schema, anon: bool) -> TokenStream {
        let ident = self.ident();
        let lifetime = self.lifetime_maybe_anon(schema, anon);
        quote!( #ident #lifetime )
    }

    fn maybe_inner_type<'s>(&self, _schema: &'s Schema) -> Option<&'s TypeDef> {
        None
    }
}

/// An enum variant.
pub struct VariantDef {
    pub name: String,
    pub field_type_id: Option<TypeId>,
    pub discriminant: Discriminant,
}

impl VariantDef {
    /// Get variant name as an [`Ident`].
    ///
    /// [`Ident`]: struct@Ident
    #[must_use]
    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    /// Get whether this variant has no field.
    #[must_use]
    pub fn is_fieldless(&self) -> bool {
        self.field_type_id.is_none()
    }

    /// Get the type of the variant's field, if it has one.
    #[must_use]
    pub fn field_type<'s>(&self, schema: &'s Schema) -> Option<&'s TypeDef> {
        self.field_type_id.map(|type_id| &schema.types[type_id])
    }
}

/// Iterator over all an enum's variants, including inherited.
///
/// Created by [`EnumDef::all_variants`].
pub struct AllVariantsIter<'s> {
    variants_iter: slice::Iter<'s, VariantDef>,
    inherits_iter: AllInheritsIter<'s>,
}

impl<'s> Iterator for AllVariantsIter<'s> {
    type Item = &'s VariantDef;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(variant) = self.variants_iter.next() {
                return Some(variant);
            }
            self.variants_iter = self.inherits_iter.next()?.variants.iter();
        }
    }
}

impl FusedIterator for AllVariantsIter<'_> {}

/// Iterator over all enums an enum inherits variants from, transitively.
///
/// Yields in depth-first pre-order. The inheritance graph is a tree, so no enum is yielded twice.
///
/// Created by [`EnumDef::all_inherits`].
pub struct AllInheritsIter<'s> {
    schema: &'s Schema,
    /// Stack of iterators over `inherits` lists, one per level of the depth-first traversal.
    stack: Vec<slice::Iter<'s, TypeId>>,
}

impl<'s> Iterator for AllInheritsIter<'s> {
    type Item = &'s EnumDef;

    fn next(&mut self) -> Option<Self::Item> {
        let type_id = loop {
            let iter = self.stack.last_mut()?;
            if let Some(&type_id) = iter.next() {
                break type_id;
            }
            self.stack.pop();
        };

        let enum_def = self.schema.enum_def(type_id);
        self.stack.push(enum_def.inherits.iter());
        Some(enum_def)
    }
}

impl FusedIterator for AllInheritsIter<'_> {}
