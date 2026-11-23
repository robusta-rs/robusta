//! Derive for `GetSpan` trait.

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::{Def, EnumDef, Schema, StructDef, StructOrEnum, TypeDef};

use super::{Derive, define_derive};

/// Derive for `GetSpan` trait.
pub struct DeriveGetSpan;

define_derive!(DeriveGetSpan);

impl Derive for DeriveGetSpan {
    fn trait_name(&self) -> &'static str {
        "GetSpan"
    }

    fn crate_name(&self) -> &'static str {
        "robusta_span"
    }

    fn prelude(&self) -> TokenStream {
        quote! {
            use robusta_span::{GetSpan, Span};
        }
    }

    fn derive(&self, type_def: StructOrEnum<'_>, schema: &Schema) -> TokenStream {
        match type_def {
            StructOrEnum::Struct(struct_def) => derive_struct(struct_def, schema),
            StructOrEnum::Enum(enum_def) => derive_enum(enum_def, schema),
        }
    }
}

/// Generate `GetSpan` trait implementation for a struct.
fn derive_struct(struct_def: &StructDef, schema: &Schema) -> TokenStream {
    let ty = struct_def.ty_anon(schema);

    quote! {
        impl GetSpan for #ty {
            #[inline]
            fn span(&self) -> Span {
                self.span
            }
        }
    }
}

/// Generate `GetSpan` trait implementation for an enum.
fn derive_enum(enum_def: &EnumDef, schema: &Schema) -> TokenStream {
    let ty = enum_def.ty_anon(schema);

    let matches = enum_def.all_variants(schema).map(|variant| {
        let variant_ident = variant.ident();
        if variant.field_type(schema).is_some_and(TypeDef::is_box) {
            quote!( Self::#variant_ident(it) => GetSpan::span(&**it) )
        } else {
            quote!( Self::#variant_ident(it) => GetSpan::span(it) )
        }
    });

    quote! {
        impl GetSpan for #ty {
            fn span(&self) -> Span {
                match self {
                    #(#matches),*
                }
            }
        }
    }
}
