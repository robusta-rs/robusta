use proc_macro2::TokenStream;
use quote::quote;

use super::{super::extensions::layout::Layout, Def, Schema, TypeDef, TypeId};

/// Type definition for a `Box`.
pub struct BoxDef {
    pub id: TypeId,
    pub name: String,
    pub inner_type_id: TypeId,
    pub layout: Layout,
}

impl BoxDef {
    /// Get inner type.
    #[must_use]
    pub fn inner_type<'s>(&self, schema: &'s Schema) -> &'s TypeDef {
        &schema.types[self.inner_type_id]
    }
}

impl Def for BoxDef {
    fn id(&self) -> TypeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn has_lifetime(&self, _schema: &Schema) -> bool {
        true
    }

    fn ty_with_lifetime(&self, schema: &Schema, anon: bool) -> TokenStream {
        let inner_ty = self.inner_type(schema).ty_with_lifetime(schema, anon);
        let lifetime = if anon { quote!('_) } else { quote!('a) };
        quote!( Box<#lifetime, #inner_ty> )
    }

    fn maybe_inner_type<'s>(&self, schema: &'s Schema) -> Option<&'s TypeDef> {
        Some(self.inner_type(schema))
    }
}
