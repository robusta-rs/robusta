use proc_macro2::TokenStream;
use quote::quote;

use super::{super::extensions::layout::Layout, Def, Schema, TypeDef, TypeId};

/// Type definition for a primitive type.
pub struct PrimitiveDef {
    pub id: TypeId,
    pub name: &'static str,
    pub layout: Layout,
}

impl Def for PrimitiveDef {
    fn id(&self) -> TypeId {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn has_lifetime(&self, _schema: &Schema) -> bool {
        self.name == "&str"
    }

    fn ty_with_lifetime(&self, _schema: &Schema, anon: bool) -> TokenStream {
        if self.name == "&str" {
            if anon { quote!(&'_ str) } else { quote!(&'a str) }
        } else {
            let ident = self.ident();
            quote!(#ident)
        }
    }

    fn maybe_inner_type<'s>(&self, _schema: &'s Schema) -> Option<&'s TypeDef> {
        None
    }
}
