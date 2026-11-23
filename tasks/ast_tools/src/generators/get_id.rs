// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/get_id.rs

//! Generator for ID getter/setter methods on all structs with a semantic ID field
//! (`node_id`, `scope_id`, `symbol_id`, `reference_id`), and on choices whose variants all hold
//! a struct with a `node_id`.

use std::io;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{Output, output_path},
    schema::{Def, EnumDef, Schema, StructDef},
};

use super::{Generator, define_generator};

/// Semantic ID types.
const SEMANTIC_ID_TYPES: [&str; 4] = ["NodeId", "ScopeId", "SymbolId", "ReferenceId"];

/// Generator for methods to get/set IDs on the types which have them.
pub struct GetIdGenerator;

define_generator!(GetIdGenerator);

impl Generator for GetIdGenerator {
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        let struct_impls =
            schema.structs().filter_map(|struct_def| generate_for_struct(struct_def, schema));
        let enum_impls = schema.enums().filter_map(|enum_def| generate_for_enum(enum_def, schema));

        let tokens = quote! {
            #![expect(clippy::inline_always)]

            ///@@line_break
            use robusta_syntax::{node::NodeId, reference::ReferenceId, scope::ScopeId, symbol::SymbolId};

            ///@@line_break
            use crate::ast::*;

            #(#struct_impls)*

            #(#enum_impls)*
        };

        Ok(vec![Output::from_tokens(output_path(AST_CRATE_PATH, "get_id.rs"), &tokens)])
    }
}

fn generate_for_struct(struct_def: &StructDef, schema: &Schema) -> Option<TokenStream> {
    let struct_name = struct_def.name();

    let methods = struct_def
        .fields
        .iter()
        .filter_map(|field| {
            let mut inner_type = field.type_def(schema).as_cell()?.inner_type(schema);
            let mut is_option = false;
            if let Some(option_def) = inner_type.as_option() {
                inner_type = option_def.inner_type(schema);
                is_option = true;
            }
            if !SEMANTIC_ID_TYPES.contains(&inner_type.name()) {
                return None;
            }

            let field_ident = field.ident();
            let inner_type_ident = inner_type.ident();
            let set_method_ident = format_ident!("set_{field_ident}");
            let inner_type_name = inner_type.name();
            let get_doc1 = format!(" Get [`{inner_type_name}`] of [`{struct_name}`].");
            let mut get_doc = quote!( #[doc = #get_doc1] );
            let set_doc = format!(" Set [`{inner_type_name}`] of [`{struct_name}`].");

            let (get_body, set_body) = if is_option {
                let get_doc2 = format!(" Panics if `{field_ident}` is [`None`].");
                get_doc.extend(quote! {
                    ///
                    /// Only use this method after semantic analysis.
                    ///
                    /// # Panics
                    #[doc = #get_doc2]
                });
                (
                    quote!( self.#field_ident.get().unwrap() ),
                    quote!( self.#field_ident.set(Some(#field_ident)); ),
                )
            } else {
                (quote!( self.#field_ident.get() ), quote!( self.#field_ident.set(#field_ident); ))
            };

            Some(quote! {
                ///@@line_break
                #get_doc
                #[inline]
                pub fn #field_ident(&self) -> #inner_type_ident {
                    #get_body
                }

                ///@@line_break
                #[doc = #set_doc]
                #[inline]
                pub fn #set_method_ident(&self, #field_ident: #inner_type_ident) {
                    #set_body
                }
            })
        })
        .collect::<TokenStream>();

    if methods.is_empty() {
        return None;
    }

    let struct_ty = struct_def.ty_anon(schema);
    Some(quote! {
        ///@@line_break
        impl #struct_ty {
            #methods
        }
    })
}

fn generate_for_enum(enum_def: &EnumDef, schema: &Schema) -> Option<TokenStream> {
    // Check all variants are structs with a `NodeId` field (`has_kind` is only `true` for structs that do).
    // Also check if all variants are consistently boxed or consistently unboxed.
    let mut all_variants_boxed = true;
    let mut all_variants_unboxed = true;

    for variant in enum_def.all_variants(schema) {
        let mut field_type = variant.field_type(schema)?;
        if let Some(box_def) = field_type.as_box() {
            field_type = box_def.inner_type(schema);
            all_variants_unboxed = false;
        } else {
            all_variants_boxed = false;
        }

        if !field_type.as_struct()?.kind.has_kind {
            return None;
        }
    }

    // If all variants are consistently boxed or consistently unboxed, add `#[inline(always)]` to the method.
    // `NodeId` field is in a consistent position in all AST structs, so if all variants have the same shape,
    // the method should boil down to a single instruction.
    let maybe_inline = if all_variants_boxed || all_variants_unboxed {
        quote! {
            ///@ `#[inline(always)]` because this should boil down to a single instruction.
            #[inline(always)]
        }
    } else {
        quote!()
    };

    let matches = enum_def.all_variants(schema).map(|variant| {
        let variant_ident = variant.ident();
        quote!( Self::#variant_ident(it) => it.node_id() )
    });
    let setters = enum_def.all_variants(schema).map(|variant| {
        let variant_ident = variant.ident();
        quote!( Self::#variant_ident(it) => it.set_node_id(node_id) )
    });

    let enum_ty = enum_def.ty_anon(schema);
    let get_doc = format!(" Get [`NodeId`] of [`{}`].", enum_def.name());
    let set_doc = format!(" Set [`NodeId`] of [`{}`].", enum_def.name());

    Some(quote! {
        ///@@line_break
        impl #enum_ty {
            #[doc = #get_doc]
            #maybe_inline
            pub fn node_id(&self) -> NodeId {
                match self {
                    #(#matches),*
                }
            }

            ///@@line_break
            #[doc = #set_doc]
            #maybe_inline
            pub fn set_node_id(&self, node_id: NodeId) {
                match self {
                    #(#setters),*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        generators::{AstKindGenerator, Generator, GetIdGenerator},
        parse::parse_sources,
    };

    #[test]
    fn a_node_id_field_and_an_all_node_choice_get_the_pair() {
        let source = "
            define_nonmax_u32_index_type! { #[ast] pub struct NodeId; }
            #[ast(foreign = NonMaxU32)] struct NonMaxU32Alias(NonZeroU32);
            #[ast(visit)] pub struct Node { pub node_id: Cell<NodeId> }
            #[ast(visit)] pub struct Modifier { pub flag: bool }
            #[ast(visit)] pub enum Boxed<'a> { A(Box<'a, Node>) = 0 }
            #[ast(visit)] pub enum Inline { B(Node) = 0 }
            #[ast(visit)] pub enum Both<'a> { INHERIT(Boxed<'a>), C(Node) = 1 }
            #[ast(visit)] pub enum Inheriting<'a> { INHERIT(Boxed<'a>), D(Box<'a, Node>) = 1 }
            #[ast(visit)] pub enum Mixed<'a> { E(Node) = 0, F(Modifier) = 1 }
            #[ast(visit)] pub enum InheritsMixed<'a> { INHERIT(Mixed<'a>), G(Node) = 2 }
            #[ast] pub enum Plain { G = 0, H = 1 }
        ";
        let mut schema = parse_sources(vec![syn::parse_file(source).unwrap()]).unwrap();
        AstKindGenerator.prepare(&mut schema);
        let outputs = GetIdGenerator.generate(&schema).unwrap();
        let code = &outputs[0].code;
        let impls = |name: &str| code.matches(&format!("impl {name}")).count();
        assert_eq!(impls("Node"), 1, "{code}");
        assert_eq!(impls("Boxed"), 1, "{code}");
        assert_eq!(impls("Inline"), 1, "{code}");
        assert_eq!(impls("Both"), 1, "{code}");
        assert_eq!(impls("Inheriting"), 1, "{code}");
        assert_eq!(impls("Modifier"), 0, "{code}");
        assert_eq!(impls("Mixed"), 0, "{code}");
        assert_eq!(impls("InheritsMixed"), 0, "{code}");
        assert_eq!(impls("Plain"), 0, "{code}");

        // A choice of one shape reads the id with a single instruction, inherited variants included
        let inline_always = |name: &str| {
            let body = &code[code.find(&format!("impl {name}")).expect("an impl")..];
            let body = &body[..body.find("set_node_id").expect("a setter")];
            body.contains("inline(always)")
        };
        assert!(inline_always("Boxed"), "{code}");
        assert!(inline_always("Inline"), "{code}");
        assert!(inline_always("Inheriting"), "{code}");
        assert!(!inline_always("Both"), "{code}");
    }
}
