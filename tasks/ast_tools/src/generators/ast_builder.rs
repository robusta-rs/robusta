// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/ast_builder.rs

//! Generator for AST builder methods defined directly on AST types.
//!
//! A node is built with `BreakStatement::new(span, label, builder)`, where `builder` is
//! anything which implements `GetAstBuilder` (e.g. an `AstBuilder`, or the parser).

use std::io;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    AST_CRATE_PATH, invalid,
    output::{Output, output_path},
    schema::{Def, EnumDef, FieldDef, Schema, StructDef, StructOrEnum, TypeDef, VariantDef},
    utils::article_for,
};

use super::{Generator, define_generator};

/// Generator for builder methods defined directly on AST types.
pub struct AstBuilderGenerator;

define_generator!(AstBuilderGenerator);

impl Generator for AstBuilderGenerator {
    /// Generate builder methods on AST types.
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        let impls = schema
            .visited()
            .map(|type_def| generate_builder_impl(type_def, schema))
            .collect::<io::Result<TokenStream>>()?;

        let tokens = quote! {
            //! AST node builder methods.
            //!
            //! Each method is generic over [`GetAstBuilder`], so it can be called with a builder
            //! directly, or with a type which holds one (e.g. the parser).
            //!
            //! Before forwarding the builder to another build method, methods first call `builder.builder()`
            //! to obtain the concrete [`AstBuild`]er. This way, everything below the outermost call is
            //! monomorphized over the [`AstBuild`] type (of which there are few), rather than over every
            //! [`GetAstBuilder`] type the method is called with.

            //!@@line_break
            #![expect(clippy::default_trait_access)]

            ///@@line_break
            use std::cell::Cell;

            ///@@line_break
            use robusta_allocator::{Box, GetAllocator, IntoIn, Vec};
            use robusta_span::Span;
            use robusta_syntax::operator::{AssignOp, BinaryOp, PostfixOp, UnaryOp};

            ///@@line_break
            use crate::{ast::*, builder::{AstBuild, GetAstBuilder}};

            #impls
        };

        Ok(vec![Output::from_tokens(output_path(AST_CRATE_PATH, "ast_builder.rs"), &tokens)])
    }
}

/// Param for a builder method.
///
/// Contains reference to the struct field, and various other bits of data derived from it.
#[expect(clippy::struct_field_names)]
struct Param<'d> {
    /// Struct field which this param is for
    field: &'d FieldDef,
    /// Struct field name identifier
    ident: Ident,
    /// Function parameter e.g. `span: Span`
    fn_param: TokenStream,
    /// `true` if the field holds a semantic ID other than the node's own. Semantic analysis sets
    /// it, so it starts at its default and takes no param.
    is_default: bool,
    /// `true` if is `NodeId` field
    is_node_id: bool,
    /// `true` if the param is generic, e.g. `imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>`.
    is_into_in: bool,
}

impl Param<'_> {
    /// Get whether the builder method takes this param.
    fn is_param(&self) -> bool {
        !self.is_node_id && !self.is_default
    }
}

/// Generate `impl` block containing builder methods for a type.
fn generate_builder_impl(type_def: StructOrEnum<'_>, schema: &Schema) -> io::Result<TokenStream> {
    let methods = match type_def {
        StructOrEnum::Struct(struct_def) => generate_builder_methods_for_struct(struct_def, schema),
        StructOrEnum::Enum(enum_def) => generate_builder_methods_for_enum(enum_def, schema)?,
    };
    let ty = type_def.ty(schema);
    let lifetime = type_def.lifetime(schema);

    Ok(quote! {
        ///@@line_break
        impl #lifetime #ty {
            #methods
        }
    })
}

/// Generate builder methods for a struct.
///
/// Generates two builder methods:
/// 1. To build an owned type, named `new`.
/// 2. To build a boxed type, named `boxed`.
fn generate_builder_methods_for_struct(struct_def: &StructDef, schema: &Schema) -> TokenStream {
    let params = get_struct_params(struct_def, schema);
    let fn_params = params.iter().filter_map(|param| param.is_param().then_some(&param.fn_param));
    let fn_params = quote!( #(#fn_params),* );
    let fields = get_struct_fields(&params);
    let args = params.iter().filter(|param| param.is_param()).map(|param| &param.ident);

    // Function only needs lifetime param for `impl GetAstBuilder<'a>` if the struct doesn't have a lifetime itself
    let lifetime_param = if struct_def.has_lifetime { quote!() } else { quote!( <'a> ) };

    // Only generate a `boxed` method if `Box<T>` exists in AST
    let struct_name = struct_def.name();
    let is_boxed = schema.type_names.contains_key(&format!("Box<{struct_name}>"));

    // Generate main builder method
    let article = article_for(struct_name);
    let fn_doc1 = format!(" Build {article} [`{struct_name}`].");
    let mut fn_docs = quote!( #[doc = #fn_doc1] );
    if is_boxed {
        let fn_doc2 = format!(" use [`{struct_name}::boxed`] instead.");
        fn_docs.extend(quote! {
            ///
            /// If you want the built node to be allocated in the memory arena,
            #[doc = #fn_doc2]
        });
    }

    let params_docs = generate_doc_comment_for_params(&params);
    let unused_builder_attr =
        (!fields.to_string().contains("builder")).then(|| quote!(#[expect(unused_variables)]));
    let struct_ident = struct_def.ident();

    let new_method = quote! {
        ///@@line_break
        #fn_docs
        #params_docs
        #unused_builder_attr
        #[inline]
        pub fn new #lifetime_param (#fn_params, builder: &impl GetAstBuilder<'a>) -> Self {
            let builder = builder.builder();
            #struct_ident { #fields }
        }
    };

    if !is_boxed {
        return new_method;
    }

    // Generate `boxed` builder method
    let boxed_doc1 =
        format!(" Build {article} [`{struct_name}`], and store it in the memory arena.");
    let boxed_doc2 =
        format!(" If you want a stack-allocated node, use [`{struct_name}::new`] instead.");

    quote! {
        #new_method

        ///@@line_break
        #[doc = #boxed_doc1]
        ///
        /// Returns a [`Box`] containing the newly-allocated node.
        #[doc = #boxed_doc2]
        #params_docs
        #[inline]
        pub fn boxed #lifetime_param (
            #fn_params, builder: &impl GetAstBuilder<'a>
        ) -> Box<'a, Self> {
            let builder = builder.builder();
            // Allocate via `&Allocator` (not `builder`), so `Box::new_in` shares a
            // monomorphization with every other `&Allocator`-based allocation
            Box::new_in(Self::new(#(#args),*, builder), &builder.allocator())
        }
    }
}

/// Get params for builder methods for a struct.
fn get_struct_params<'s>(struct_def: &'s StructDef, schema: &'s Schema) -> Vec<Param<'s>> {
    let node_id_cell_type_id = schema.type_names.get("Cell<NodeId>").copied();

    struct_def
        .fields
        .iter()
        .map(|field| {
            let type_def = field.type_def(schema);
            let ty = type_def.ty(schema);
            let is_node_id = Some(field.type_id) == node_id_cell_type_id;
            let is_default = !is_node_id && type_def.is_cell();
            let is_into_in = matches!(type_def, TypeDef::Vec(_));

            let fn_param_ty = if is_into_in { quote!( impl IntoIn<'a, #ty> ) } else { ty };
            let ident = field.ident();
            let fn_param = quote!( #ident: #fn_param_ty );

            Param { field, ident, fn_param, is_default, is_node_id, is_into_in }
        })
        .collect()
}

/// Get fields for a struct builder method.
///
/// The generated field values reference a local `builder` binding (`let builder = builder.builder();`)
/// for the allocator and node ID.
///
/// ```ignore
/// pub fn new(span: Span, bar: Bar<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
///     let builder = builder.builder();
///     Foo { node_id: Cell::new(builder.node_id()), span, bar }
/// //        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ fields
/// }
/// ```
fn get_struct_fields(params: &[Param]) -> TokenStream {
    let fields = params.iter().map(|param| {
        let ident = &param.ident;
        if param.is_node_id {
            quote!( #ident: Cell::new(builder.node_id()) )
        } else if param.is_default {
            quote!( #ident: Default::default() )
        } else if param.is_into_in {
            quote!( #ident: #ident.into_in(builder.allocator()) )
        } else {
            quote!( #ident )
        }
    });
    quote!( #(#fields),* )
}

/// Generate builder methods for an enum.
///
/// Generates a builder method for every variant of the enum, including inherited variants.
/// Each method is named after the node the variant holds, with a `new_` prefix,
/// e.g. `Expression::new_binary_expression`, not `Expression::new_binary`.
fn generate_builder_methods_for_enum(
    enum_def: &EnumDef,
    schema: &Schema,
) -> io::Result<TokenStream> {
    enum_def
        .all_variants(schema)
        .map(|variant| generate_builder_method_for_enum_variant(enum_def, variant, schema))
        .collect()
}

/// Generate builder method for an enum variant.
fn generate_builder_method_for_enum_variant(
    enum_def: &EnumDef,
    variant: &VariantDef,
    schema: &Schema,
) -> io::Result<TokenStream> {
    let mut variant_type = variant.field_type(schema);
    let mut is_boxed = false;
    if let Some(TypeDef::Box(box_def)) = variant_type {
        variant_type = Some(box_def.inner_type(schema));
        is_boxed = true;
    }
    let Some(TypeDef::Struct(struct_def)) = variant_type else {
        return Err(invalid(format!(
            "{}::{}: a variant the walk visits holds a struct",
            enum_def.name, variant.name
        )));
    };

    let params = get_struct_params(struct_def, schema);
    let fn_name = format_ident!("new_{}", struct_def.snake_name());
    let fn_params = params.iter().filter_map(|param| param.is_param().then_some(&param.fn_param));
    let args = params.iter().filter(|param| param.is_param()).map(|param| &param.ident);

    let variant_ident = variant.ident();
    let struct_ident = struct_def.ident();
    let inner_fn_name = if is_boxed { quote!(boxed) } else { quote!(new) };
    // Function only needs lifetime param for `impl GetAstBuilder<'a>` if the enum doesn't have a lifetime itself
    let lifetime_param = if enum_def.has_lifetime { quote!() } else { quote!( <'a> ) };

    // Generate doc comments
    let enum_name = enum_def.name();
    let article_enum = article_for(enum_name);
    let fn_doc1 = format!(" Build {article_enum} [`{enum_name}::{variant_ident}`].");
    let mut fn_docs = quote!( #[doc = #fn_doc1] );
    if is_boxed {
        let variant_type_name = struct_def.name();
        let article_variant = article_for(variant_type_name);
        let fn_doc2 = format!(
            " This node contains {article_variant} [`{variant_type_name}`] that will be stored in the memory arena."
        );
        fn_docs.extend(quote! {
            ///
            #[doc = #fn_doc2]
        });
    }
    let params_docs = generate_doc_comment_for_params(&params);

    Ok(quote! {
        ///@@line_break
        #fn_docs
        #params_docs
        #[inline]
        pub fn #fn_name #lifetime_param (
            #(#fn_params),*, builder: &impl GetAstBuilder<'a>
        ) -> Self {
            Self::#variant_ident(#struct_ident::#inner_fn_name(#(#args),*, builder.builder()))
        }
    })
}

/// Generate doc comment for function params.
fn generate_doc_comment_for_params(params: &[Param]) -> TokenStream {
    let lines = params.iter().filter(|param| param.is_param()).map(|param| {
        let field = param.field;
        let field_name = &field.name;
        let field_comment = if let Some(field_comment) = field.doc_comment.as_deref() {
            format!(" * `{field_name}`: {field_comment}")
        } else if field_name == "span" {
            " * `span`: The [`Span`] covering this node".to_string()
        } else {
            format!(" * `{field_name}`")
        };
        quote!( #[doc = #field_comment] )
    });

    quote! {
        ///
        /// ## Parameters
        #(#lines)*
    }
}

#[cfg(test)]
mod tests {
    use cow_utils::CowUtils;

    use super::*;
    use crate::parse::parse_sources;

    /// Generate from `source`, after the types every node needs, without the token spacing.
    fn generate(source: &str) -> io::Result<String> {
        let source = format!(
            "
            define_nonmax_u32_index_type! {{ #[ast] pub struct NodeId; }}
            define_nonmax_u32_index_type! {{ #[ast] pub struct SymbolId; }}
            #[ast(foreign = NonMaxU32)] struct NonMaxU32Alias(NonZeroU32);
            #[ast] pub struct Span {{ pub start: u32, pub end: u32 }}
            {source}"
        );
        let schema = parse_sources(vec![syn::parse_file(&source).unwrap()]).unwrap();
        let output = AstBuilderGenerator.generate(&schema)?.remove(0);
        Ok(output.code.cow_replace(' ', "").into_owned())
    }

    #[test]
    fn a_node_takes_its_fields_and_leaves_its_ids_to_the_builder_and_semantic_analysis() {
        let code = generate(
            "#[ast(visit)]
            pub struct Binding<'a> {
                pub node_id: Cell<NodeId>,
                pub span: Span,
                /// The name.
                pub name: &'a str,
                pub symbol_id: Cell<Option<SymbolId>>,
            }",
        )
        .unwrap();
        assert!(
            code.contains("pubfnnew(span:Span,name:&'astr,builder:&implGetAstBuilder<'a>)->Self"),
            "{code}"
        );
        assert!(code.contains("node_id:Cell::new(builder.node_id())"), "{code}");
        assert!(code.contains("symbol_id:Default::default()"), "{code}");
        assert!(code.contains("`name`:Thename."), "{code}");
    }

    #[test]
    fn a_list_is_converted_into_the_arena() {
        let code = generate(
            "#[ast(visit)]
            pub struct List<'a> { pub span: Span, pub items: Vec<'a, Span> }",
        )
        .unwrap();
        assert!(code.contains("items:implIntoIn<'a,Vec<'a,Span>>"), "{code}");
        assert!(code.contains("items:items.into_in(builder.allocator())"), "{code}");
    }

    #[test]
    fn only_a_node_something_boxes_gets_a_boxed_method() {
        let code = generate(
            "#[ast(visit)] pub struct Held { pub node_id: Cell<NodeId>, pub span: Span }
            #[ast(visit)] pub struct Inline { pub node_id: Cell<NodeId>, pub span: Span }
            #[ast(visit)] pub struct Plain { pub span: Span }
            #[ast(visit)]
            pub enum Choice<'a> { A(Box<'a, Held>) = 0, B(Inline) = 1, C(Plain) = 2 }",
        )
        .unwrap();
        assert_eq!(code.matches("pubfnboxed").count(), 1, "{code}");
        assert!(code.contains("Self::A(Held::boxed(span,builder.builder()))"), "{code}");
        assert!(code.contains("Self::B(Inline::new(span,builder.builder()))"), "{code}");
        // A struct with no ID has no use for its builder
        assert_eq!(code.matches("expect(unused_variables)").count(), 1, "{code}");
    }

    #[test]
    fn a_choice_has_a_method_per_variant_named_for_its_node_including_inherited_variants() {
        let code = generate(
            "#[ast(visit)] pub struct LeafNode { pub node_id: Cell<NodeId>, pub span: Span }
            #[ast(visit)] pub struct OtherNode { pub node_id: Cell<NodeId>, pub span: Span }
            #[ast(visit)] pub enum Inner<'a> { Leaf(Box<'a, LeafNode>) = 0 }
            #[ast(visit)] pub enum Outer<'a> { INHERIT(Inner<'a>), Other(Box<'a, OtherNode>) = 1 }",
        )
        .unwrap();
        let outer = &code[code.find("impl<'a>Outer").expect("an impl")..];
        assert!(outer.contains("pubfnnew_leaf_node("), "{code}");
        assert!(outer.contains("pubfnnew_other_node("), "{code}");
        assert!(outer.contains("Self::Leaf(LeafNode::boxed("), "{code}");
    }

    #[test]
    fn a_visited_variant_without_a_struct_is_an_error() {
        let error = generate(
            "#[ast] pub enum Kind { A = 0 }
            #[ast(visit)] pub enum Choice { K(Kind) = 0 }",
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "Choice::K: a variant the walk visits holds a struct");

        let error = generate("#[ast(visit)] pub enum Choice { Unit = 0 }").unwrap_err();
        assert_eq!(error.to_string(), "Choice::Unit: a variant the walk visits holds a struct");
    }
}
