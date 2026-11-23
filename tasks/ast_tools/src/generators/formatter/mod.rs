mod ast_nodes;
mod format;

use std::io;

use crate::{
    invalid,
    schema::{Def, EnumDef, Schema, StructOrEnum, TypeDef, VariantDef},
};

pub use ast_nodes::FormatterAstNodesGenerator;
pub use format::FormatterFormatGenerator;

/// Path of the generated formatter sources, relative to the workspace root.
const FORMATTER_GENERATED_PATH: &str = "crates/robusta_formatter/src/ast_nodes/generated";

/// `fmt` and the `AstNode` views dispatch to what a choice variant holds, so every
/// variant holds a node. A choice that includes another choice inherits its variants.
fn check_choice_payloads(schema: &Schema) -> io::Result<()> {
    for item in schema.enums().filter(|item| item.visit.has_visitor()) {
        for variant in &item.variants {
            let holds_node = variant.field_type(schema).is_some_and(|ty| {
                matches!(node(unboxed(ty, schema)), Some(StructOrEnum::Struct(_)))
            });
            if !holds_node {
                return Err(invalid(format!("{}::{} must hold a node", item.name, variant.name)));
            }
        }
    }
    Ok(())
}

/// What a choice variant holds, and the expression that borrows it from `field_0`.
fn payload<'s>(variant: &VariantDef, schema: &'s Schema) -> (&'s TypeDef, &'static str) {
    let ty = variant.field_type(schema).expect("a choice variant holds a node");
    if ty.is_box() { (unboxed(ty, schema), "field_0.as_ref()") } else { (ty, "field_0") }
}

/// The type a `Box` holds, or the type itself.
fn unboxed<'s>(ty: &'s TypeDef, schema: &'s Schema) -> &'s TypeDef {
    match ty {
        TypeDef::Box(box_def) => box_def.inner_type(schema),
        ty => ty,
    }
}

fn enum_pattern(item: &EnumDef, variant: &VariantDef) -> String {
    format!("{}::{}(field_0)", item.name, variant.name)
}

fn source(ty: &impl Def, schema: &Schema) -> String {
    ty.ty(schema).to_string()
}

/// `ty` as the node or choice it is, without looking through containers.
fn node(ty: &TypeDef) -> Option<StructOrEnum<'_>> {
    ty.as_struct_or_enum().filter(|item| item.is_visited())
}

#[cfg(test)]
mod tests {
    use crate::{
        generators::{AstKindGenerator, Generator},
        parse::parse_sources,
        schema::Schema,
    };

    use super::{ast_nodes, check_choice_payloads, format};

    fn choice(payload: &str) -> Schema {
        let source = format!(
            "#[ast] pub struct Span {{ pub start: u32, pub end: u32 }}
             #[ast(visit)] pub struct Node {{ pub span: Span }}
             #[ast(visit)] pub enum Choice<'a> {{ Variant{payload} = 0 }}"
        );
        parse_sources(vec![syn::parse_file(&source).expect("a source file")]).expect("a schema")
    }

    /// The generated source, without the whitespace the formatter settles.
    fn generated(generate: fn(&Schema) -> String) -> String {
        let source = "
            #[ast] pub struct Span { pub start: u32, pub end: u32 }
            #[ast] pub struct NodeId { pub index: u32 }
            #[ast(visit)]
            #[generate_derive(GetSpan)]
            pub struct Child { pub node_id: Cell<NodeId>, pub span: Span }
            #[ast(visit)]
            pub struct Block { pub node_id: Cell<NodeId>, pub child: Child }
            #[ast(visit)]
            pub struct ClassDeclaration { pub node_id: Cell<NodeId>, pub child: Child }
            #[ast(visit)]
            pub struct Other { pub node_id: Cell<NodeId>, pub child: Child }
            #[ast(visit)]
            pub enum Local<'a> { Class(Box<'a, ClassDeclaration>) = 8, Other(Box<'a, Other>) = 9 }
            #[ast(visit)]
            pub enum Statement<'a> { Block(Box<'a, Block>) = 0, INHERIT(Local<'a>) }
        ";
        let mut schema =
            parse_sources(vec![syn::parse_file(source).expect("a source file")]).expect("a schema");
        AstKindGenerator.prepare(&mut schema);
        generate(&schema).split_whitespace().collect()
    }

    #[test]
    fn a_choice_dispatches_an_inherited_variant_to_the_node_it_holds() {
        let code = generated(format::generate);
        let statement = &code[code.find("AstNode<'_,'a,Statement<'a>>").expect("an impl")..];
        assert!(
            statement.contains("Statement::Class(field_0)=>{AstNode::<ClassDeclaration>"),
            "{code}"
        );
        assert!(!statement.contains("AstNode::<Local"), "{code}");

        let code = generated(ast_nodes::generate);
        let statement = &code[code.find("Statement<'a>>{").expect("an impl")..];
        assert!(
            statement.contains("Statement::Class(field_0)=>AstNodeKind::ClassDeclaration("),
            "{code}"
        );
        assert!(statement.contains("pubfnas_class(&self)"), "{code}");
        assert!(statement.contains("pubfnas_local(&self)"), "{code}");
    }

    #[test]
    fn a_statement_keeps_comments_from_its_next_sibling_unless_it_is_also_a_member() {
        let code = generated(ast_nodes::generate);
        let accessor = |owner: &str| {
            let body = &code[code.find(&format!("'a,{owner}>{{")).expect("an impl")..];
            let body = &body[body.find("pubfnchild").expect("an accessor")..];
            body[..=body.find('}').expect("a body")].to_string()
        };
        assert!(accessor("Block").contains("following_span_start:0"), "{code}");
        assert!(accessor("Other").contains("following_span_start:0"), "{code}");
        assert!(
            accessor("ClassDeclaration").contains("following_span_start:self.following_span_start"),
            "a class declaration is also a member, which its next sibling bounds\n{code}"
        );
    }

    #[test]
    fn a_choice_variant_holds_a_node() {
        for payload in ["(Node)", "(Box<'a, Node>)"] {
            assert!(check_choice_payloads(&choice(payload)).is_ok(), "{payload}");
        }
        for payload in [
            "(Vec<'a, Node>)",
            "(Option<Node>)",
            "(Span)",
            "",
            "(Box<'a, Choice<'a>>)",
            "(Choice<'a>)",
        ] {
            let error = check_choice_payloads(&choice(payload)).unwrap_err();
            assert_eq!(error.to_string(), "Choice::Variant must hold a node", "{payload}");
        }
    }
}
