use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::utils::snake_case;

use super::{FileId, Schema, TypeId};

mod r#box;
mod cell;
mod r#enum;
mod option;
mod primitive;
mod r#struct;
mod r#type;
mod vec;
pub use r#box::BoxDef;
pub use cell::CellDef;
pub use r#enum::{Discriminant, EnumDef, VariantDef};
pub use option::OptionDef;
pub use primitive::PrimitiveDef;
pub use r#struct::{FieldDef, StructDef};
pub use r#type::TypeDef;
pub use vec::VecDef;

/// Trait for type defs.
pub trait Def {
    /// Get [`TypeId`] for type.
    fn id(&self) -> TypeId;

    /// Get type name.
    fn name(&self) -> &str;

    /// Get if type has a lifetime.
    fn has_lifetime(&self, schema: &Schema) -> bool;

    /// Get type name as an [`Ident`].
    ///
    /// [`Ident`]: struct@Ident
    fn ident(&self) -> Ident {
        format_ident!("{}", self.name())
    }

    /// Get type name in snake case.
    fn snake_name(&self) -> String {
        snake_case(self.name())
    }

    /// Get type signature (including lifetimes).
    fn ty(&self, schema: &Schema) -> TokenStream {
        self.ty_with_lifetime(schema, false)
    }

    /// Get type signature (including anonymous lifetimes).
    fn ty_anon(&self, schema: &Schema) -> TokenStream {
        self.ty_with_lifetime(schema, true)
    }

    /// Get type signature (including lifetimes).
    /// Lifetimes are anonymous (`'_`) if `anon` is true.
    fn ty_with_lifetime(&self, schema: &Schema, anon: bool) -> TokenStream;

    /// Get lifetime (if type has one).
    /// Lifetime is anonymous (`'_`) if `anon` is true.
    fn lifetime_maybe_anon(&self, schema: &Schema, anon: bool) -> TokenStream {
        if anon { self.lifetime_anon(schema) } else { self.lifetime(schema) }
    }

    /// Get lifetime (if type has one).
    fn lifetime(&self, schema: &Schema) -> TokenStream {
        if self.has_lifetime(schema) { quote!( <'a> ) } else { quote!() }
    }

    /// Get anonymous lifetime (if type has one).
    fn lifetime_anon(&self, schema: &Schema) -> TokenStream {
        if self.has_lifetime(schema) { quote!( <'_> ) } else { quote!() }
    }

    /// Get inner type, if type has one.
    ///
    /// This is the direct inner type e.g. `Option<Vec<Expression>>` -> `Vec<Expression>`.
    /// Use [`innermost_type`] method if you want `Expression` in this example.
    ///
    /// Returns `None` for types which don't have a single inner type (structs, enums, and primitives).
    ///
    /// [`innermost_type`]: Def::innermost_type
    fn maybe_inner_type<'s>(&self, schema: &'s Schema) -> Option<&'s TypeDef>;

    /// Get innermost type.
    ///
    /// e.g. `Expression` in `Option<Vec<Expression>>`.
    fn innermost_type<'s>(&self, schema: &'s Schema) -> &'s TypeDef {
        if let Some(inner_type) = self.maybe_inner_type(schema) {
            inner_type.innermost_type(schema)
        } else {
            &schema.types[self.id()]
        }
    }
}

/// Visibility of a struct field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    /// `pub(crate)` or `pub(super)`
    Restricted,
    Private,
}

/// Reference to a [`StructDef`] or [`EnumDef`].
#[derive(Clone, Copy)]
pub enum StructOrEnum<'s> {
    Struct(&'s StructDef),
    Enum(&'s EnumDef),
}

impl StructOrEnum<'_> {
    /// Get whether the walk visits this type.
    #[must_use]
    pub fn is_visited(self) -> bool {
        match self {
            Self::Struct(struct_def) => struct_def.visit.has_visitor(),
            Self::Enum(enum_def) => enum_def.visit.has_visitor(),
        }
    }

    /// Get ID of the file this type is defined in.
    #[must_use]
    pub fn file_id(self) -> FileId {
        match self {
            Self::Struct(struct_def) => struct_def.file_id,
            Self::Enum(enum_def) => enum_def.file_id,
        }
    }

    /// Get whether a trait is generated for this type.
    #[must_use]
    pub fn generates_derive(self, trait_name: &str) -> bool {
        let generated_derives = match self {
            Self::Struct(struct_def) => &struct_def.generated_derives,
            Self::Enum(enum_def) => &enum_def.generated_derives,
        };
        generated_derives.iter().any(|name| name == trait_name)
    }
}

impl Def for StructOrEnum<'_> {
    fn id(&self) -> TypeId {
        match self {
            Self::Struct(struct_def) => struct_def.id(),
            Self::Enum(enum_def) => enum_def.id(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Struct(struct_def) => struct_def.name(),
            Self::Enum(enum_def) => enum_def.name(),
        }
    }

    fn has_lifetime(&self, schema: &Schema) -> bool {
        match self {
            Self::Struct(struct_def) => struct_def.has_lifetime(schema),
            Self::Enum(enum_def) => enum_def.has_lifetime(schema),
        }
    }

    fn ty_with_lifetime(&self, schema: &Schema, anon: bool) -> TokenStream {
        match self {
            Self::Struct(struct_def) => struct_def.ty_with_lifetime(schema, anon),
            Self::Enum(enum_def) => enum_def.ty_with_lifetime(schema, anon),
        }
    }

    fn maybe_inner_type<'s>(&self, _schema: &'s Schema) -> Option<&'s TypeDef> {
        None
    }
}
