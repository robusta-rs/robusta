use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::utils::{pluralize, snake_case};

use super::{
    super::{
        FileId,
        extensions::{
            content_eq::{ContentEqStructField, ContentEqType},
            dummy::DummyStruct,
            kind::Kind,
            layout::{Layout, Offset},
            visit::VisitStruct,
        },
    },
    Def, Schema, TypeDef, TypeId, Visibility,
};

/// Type definition for a struct.
pub struct StructDef {
    pub id: TypeId,
    pub name: String,
    /// The name from `#[plural(...)]`, where adding `s` would clash with another type's name.
    pub plural_name: Option<String>,
    pub file_id: FileId,
    pub has_lifetime: bool,
    /// Whether this describes a type defined elsewhere, through `#[ast(foreign = Type)]`.
    pub is_foreign: bool,
    pub visit: VisitStruct,
    /// The traits named by `#[generate_derive(...)]`.
    pub generated_derives: Vec<String>,
    /// Fields in source order, whatever their order in memory.
    pub fields: Vec<FieldDef>,
    pub kind: Kind,
    pub layout: Layout,
    pub dummy: DummyStruct,
    pub content_eq: ContentEqType,
}

impl StructDef {
    /// Get plural type name in snake case.
    pub fn plural_snake_name(&self) -> String {
        snake_case(&self.plural_name.clone().unwrap_or_else(|| pluralize(&self.name)))
    }

    /// Get iterator over the fields the walk visits, in source order.
    pub fn visited_fields<'s>(&'s self, schema: &'s Schema) -> impl Iterator<Item = &'s FieldDef> {
        self.fields.iter().filter(|field| field.type_def(schema).contains_visited(schema))
    }
}

impl Def for StructDef {
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

/// A struct field.
pub struct FieldDef {
    pub name: String,
    pub type_id: TypeId,
    pub visibility: Visibility,
    /// The first line of the field's doc comment.
    pub doc_comment: Option<String>,
    pub offset: Offset,
    pub content_eq: ContentEqStructField,
}

impl FieldDef {
    /// Get field name as an [`Ident`].
    ///
    /// [`Ident`]: struct@Ident
    #[must_use]
    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    /// Get field type.
    #[must_use]
    pub fn type_def<'s>(&self, schema: &'s Schema) -> &'s TypeDef {
        &schema.types[self.type_id]
    }
}
