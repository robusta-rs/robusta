//! Derive for `ContentEq` trait.

use std::io;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    invalid,
    parse::attr::{AttrLocation, AttrPart},
    schema::{Def, EnumDef, Schema, StructDef, StructOrEnum, TypeDef},
};

use super::{Derive, define_derive};

/// Derive for `ContentEq` trait.
pub struct DeriveContentEq;

define_derive!(DeriveContentEq);

impl Derive for DeriveContentEq {
    fn trait_name(&self) -> &'static str {
        "ContentEq"
    }

    fn crate_name(&self) -> &'static str {
        "robusta_span"
    }

    /// Own `#[content_eq]` attr on structs, enums, or struct fields.
    /// The attr is legal on structs and enums which don't derive this trait.
    fn attrs(&self) -> &[&'static str] {
        &["content_eq"]
    }

    /// Parse `#[content_eq(skip)]` attr.
    fn parse_attr(
        &self,
        attr_name: &str,
        location: AttrLocation<'_>,
        part: AttrPart<'_>,
    ) -> io::Result<()> {
        // No need to check attr name is `content_eq`, because that's the only attribute this derive handles.
        if !matches!(part, AttrPart::Tag("skip")) {
            return Err(invalid(format!("{location}: bad #[{attr_name}]")));
        }

        match location {
            AttrLocation::Struct(struct_def) => struct_def.content_eq.skip = true,
            AttrLocation::Enum(enum_def) => enum_def.content_eq.skip = true,
            AttrLocation::StructField(struct_def, field_index) => {
                struct_def.fields[field_index].content_eq.skip = true;
            }
            _ => return Err(invalid(format!("{location}: bad #[{attr_name}]"))),
        }

        Ok(())
    }

    fn prelude(&self) -> TokenStream {
        quote! {
            use robusta_span::ContentEq;
        }
    }

    fn derive(&self, type_def: StructOrEnum<'_>, schema: &Schema) -> TokenStream {
        match type_def {
            StructOrEnum::Struct(struct_def) => derive_struct(struct_def, schema),
            StructOrEnum::Enum(enum_def) => derive_enum(enum_def, schema),
        }
    }
}

fn derive_struct(struct_def: &StructDef, schema: &Schema) -> TokenStream {
    let mut uses_other = true;

    let body = if struct_def.content_eq.skip {
        // Struct has `#[content_eq(skip)]` attr. So `content_eq` always returns true.
        uses_other = false;
        quote!(true)
    } else {
        let fields = struct_def
            .fields
            .iter()
            .filter(|field| !field.content_eq.skip)
            .filter(|field| {
                let innermost_type = field.type_def(schema).innermost_type(schema);
                match innermost_type {
                    TypeDef::Struct(struct_def) => !struct_def.content_eq.skip,
                    TypeDef::Enum(enum_def) => !enum_def.content_eq.skip,
                    _ => true,
                }
            })
            .map(|field| {
                let ident = field.ident();
                quote!( ContentEq::content_eq(&self.#ident, &other.#ident) )
            });

        let mut body = quote!( #(#fields)&&* );
        if body.is_empty() {
            body = quote!(true);
            uses_other = false;
        }
        body
    };

    generate_impl(&struct_def.ty_anon(schema), &body, uses_other)
}

fn derive_enum(enum_def: &EnumDef, schema: &Schema) -> TokenStream {
    let mut uses_other = true;

    let body = if enum_def.content_eq.skip {
        // Enum has `#[content_eq(skip)]` attr. So `content_eq` always returns true.
        uses_other = false;
        quote!(true)
    } else if enum_def.is_fieldless() {
        // We assume fieldless enums implement `PartialEq`
        quote!(self == other)
    } else {
        let matches = enum_def.all_variants(schema).map(|variant| {
            let ident = variant.ident();
            if variant.is_fieldless() {
                quote!( (Self::#ident, Self::#ident) => true )
            } else {
                quote!( (Self::#ident(a), Self::#ident(b)) => a.content_eq(b) )
            }
        });

        quote! {
            match (self, other) {
                #(#matches,)*
                _ => false,
            }
        }
    };

    generate_impl(&enum_def.ty_anon(schema), &body, uses_other)
}

fn generate_impl(ty: &TokenStream, body: &TokenStream, uses_other: bool) -> TokenStream {
    let other_ident = format_ident!("{}", if uses_other { "other" } else { "_" });
    quote! {
        impl ContentEq for #ty {
            fn content_eq(&self, #other_ident: &Self) -> bool {
                #body
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        derives::Derive,
        generators::{AssertLayoutsGenerator, Generator},
        parse::parse_sources,
    };

    use super::DeriveContentEq;

    fn derived(source: &str) -> String {
        let source = format!(
            "#[ast] #[content_eq(skip)] pub struct Span {{ pub start: u32, pub end: u32 }}
             define_nonmax_u32_index_type! {{ #[ast] #[content_eq(skip)] pub struct NodeId; }}
             define_nonmax_u32_index_type! {{ #[ast] #[content_eq(skip)] pub struct ScopeId; }}
             #[ast(foreign = NonMaxU32)] struct NonMaxU32Alias(NonZeroU32);
             {source}"
        );
        let mut schema = parse_sources(vec![syn::parse_file(&source).unwrap()]).unwrap();
        AssertLayoutsGenerator.prepare(&mut schema);
        let outputs = DeriveContentEq.output(&schema);
        assert_eq!(outputs.len(), 1, "one crate's types make one output");
        outputs.into_iter().next().unwrap().code
    }

    #[test]
    fn a_struct_compares_every_field_but_those_a_skip_marker_covers() {
        let code = derived(
            "#[ast] #[generate_derive(ContentEq)]
             pub struct Node<'a> {
                 pub node_id: Cell<NodeId>,
                 pub span: Span,
                 #[content_eq(skip)] pub comments: Vec<'a, Span>,
                 pub name: &'a str,
                 pub child: Option<Box<'a, Node<'a>>>,
                 pub scope_id: Cell<Option<ScopeId>>,
             }",
        );
        assert!(
            code.contains("ContentEq :: content_eq (& self . name , & other . name)"),
            "{code}"
        );
        assert!(code.contains("ContentEq :: content_eq (& self . child , & other . child)"));
        for skipped in ["node_id", "span", "comments", "scope_id"] {
            assert!(!code.contains(&format!("self . {skipped}")), "{skipped}: {code}");
        }
    }

    #[test]
    fn a_struct_with_nothing_to_compare_is_always_equal() {
        let code = derived(
            "#[ast] #[generate_derive(ContentEq)]
             pub struct Marker { pub node_id: Cell<NodeId>, pub span: Span }",
        );
        assert!(code.contains("fn content_eq (& self , _ : & Self) -> bool { true }"), "{code}");
    }

    #[test]
    fn a_type_level_skip_makes_a_deriving_type_always_equal() {
        let code = derived(
            "#[ast] #[content_eq(skip)] #[generate_derive(ContentEq)]
             pub struct Ignored { pub value: u32 }
             #[ast] #[content_eq(skip)] #[generate_derive(ContentEq)]
             pub enum Other { A = 0, B = 1 }",
        );
        assert_eq!(code.matches("fn content_eq (& self , _ : & Self) -> bool { true }").count(), 2);
    }

    #[test]
    fn a_fieldless_enum_compares_with_partial_eq() {
        let code = derived("#[ast] #[generate_derive(ContentEq)] pub enum Kind { A = 0, B = 1 }");
        assert!(code.contains("self == other"), "{code}");
    }

    #[test]
    fn a_choice_matches_its_own_and_inherited_variants_and_nothing_else() {
        let code = derived(
            "#[ast] #[generate_derive(ContentEq)] pub struct A { pub value: u32 }
             #[ast] #[generate_derive(ContentEq)] pub struct B { pub value: u32 }
             #[ast] #[generate_derive(ContentEq)]
             pub enum Inner<'a> { A(Box<'a, A>) = 0 }
             #[ast] #[generate_derive(ContentEq)]
             pub enum Outer<'a> { B(Box<'a, B>) = 1, Empty = 2, INHERIT(Inner<'a>) }",
        );
        assert!(code.contains("(Self :: B (a) , Self :: B (b)) => a . content_eq (b)"), "{code}");
        assert!(code.contains("(Self :: A (a) , Self :: A (b)) => a . content_eq (b)"), "{code}");
        assert!(code.contains("(Self :: Empty , Self :: Empty) => true"), "{code}");
        assert!(code.contains("_ => false"), "{code}");
    }
}
