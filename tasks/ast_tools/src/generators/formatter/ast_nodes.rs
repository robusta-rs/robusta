// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/ast_tools/src/generators/formatter/ast_nodes.rs

use std::{fmt::Write as _, io};

use crate::{
    generators::{Generator, define_generator},
    output::Output,
    schema::{Def, EnumDef, FieldDef, Schema, StructDef, StructOrEnum, TypeDef},
    utils::snake_case,
};

use super::{
    FORMATTER_GENERATED_PATH, check_choice_payloads, enum_pattern, node, payload, source, unboxed,
};

/// Nodes whose last child has `following_span_start = 0`, so its trailing comments stay with it
/// rather than leading a following sibling outside the node. A switch rule's body is its last
/// statement, as a switch case's last statement is.
const AST_NODE_WITHOUT_FOLLOWING_NODE_LIST: &[&str] =
    &["FormalParameters", "RecordHeader", "SwitchRule"];

/// Variants of `Statement` whose last child still leaves comments to their next sibling, because
/// each is also a member of a type body.
const AST_NODE_WITH_FOLLOWING_NODE_LIST: &[&str] =
    &["ClassDeclaration", "InterfaceDeclaration", "EnumDeclaration", "RecordDeclaration"];

/// Generator for the formatter's `AstNode` views.
pub struct FormatterAstNodesGenerator;

define_generator!(FormatterAstNodesGenerator);

impl Generator for FormatterAstNodesGenerator {
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        check_choice_payloads(schema)?;
        let path = format!("{FORMATTER_GENERATED_PATH}/ast_nodes.rs");
        Ok(vec![Output { path, code: generate(schema) }])
    }
}

pub(super) fn generate(schema: &Schema) -> String {
    let nodes = schema.structs().filter(|item| item.kind.has_kind).collect::<Vec<_>>();
    let mut output = String::from(
        "use robusta_allocator::Vec; use robusta_ast::*; use robusta_span::{GetSpan, Span};
         use crate::ast_nodes::AstNode;
         ///@@line_break
         #[derive(Clone, Copy)] pub enum AstNodes<'parent, 'a> { Dummy,",
    );
    for item in &nodes {
        let _ = writeln!(
            output,
            "{}(&'parent AstNode<'parent, 'a, {}>),",
            item.name,
            source(*item, schema)
        );
    }
    output += "}\nimpl<'parent, 'a> AstNodes<'parent, 'a> { pub fn span(&self) -> Span { match self { Self::Dummy => panic!(\"root has no parent\"),";
    for item in &nodes {
        let _ = writeln!(output, "Self::{}(node) => node.span(),", item.name);
    }
    output += "}} pub fn parent(&self) -> &AstNodes<'parent, 'a> { match self { Self::Dummy => panic!(\"root has no parent\"),";
    for item in &nodes {
        let _ = writeln!(output, "Self::{}(node) => node.parent(),", item.name);
    }
    output +=
        "}} pub fn debug_name(&self) -> &'static str { match self { Self::Dummy => \"Dummy\",";
    for item in &nodes {
        let _ = writeln!(output, "Self::{}(_) => {:?},", item.name, item.name);
    }
    output += "}}}\n";
    output += "#[derive(Clone, Copy, Debug)] pub enum AstNodeKind<'parent, 'a> {";
    for item in &nodes {
        let _ = writeln!(output, "{}(AstNode<'parent, 'a, {}>),", item.name, source(*item, schema));
    }
    output += "}\nimpl<'a> AstNodeKind<'_, 'a> { pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> { match self {";
    for item in &nodes {
        let _ = writeln!(output, "Self::{}(node) => AstNodes::{}(node),", item.name, item.name);
    }
    output += "}}}\nimpl GetSpan for AstNodeKind<'_, '_> { fn span(&self) -> Span { match self {";
    for item in &nodes {
        let _ = writeln!(output, "Self::{}(node) => node.span(),", item.name);
    }
    output += "}}}\n";
    for item in schema.visited() {
        generate_impl(&mut output, item, schema);
    }
    output
}

fn generate_impl(output: &mut String, item: StructOrEnum<'_>, schema: &Schema) {
    let is_choice = matches!(item, StructOrEnum::Enum(_));
    let ast = is_choice
        || item.has_lifetime(schema)
        || has_kind(item)
        || matches!(item, StructOrEnum::Struct(item) if item.visited_fields(schema).next().is_some());
    let generics = if is_choice {
        "<'parent, 'a>"
    } else if ast {
        "<'a>"
    } else {
        ""
    };
    let parent_lifetime = if is_choice { "'parent" } else { "'_" };
    let ast_lifetime = if ast { "'a" } else { "'_" };
    let _ = writeln!(
        output,
        "impl{generics} AstNode<{parent_lifetime}, {ast_lifetime}, {}> {{",
        source(&item, schema)
    );
    if has_kind(item) {
        let _ = writeln!(
            output,
            "pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {{ AstNodes::{}(self) }}",
            item.name()
        );
    }
    match item {
        StructOrEnum::Struct(item) => {
            for (index, field) in item.fields.iter().enumerate() {
                if field.name == "span" || !field.type_def(schema).contains_visited(schema) {
                    continue;
                }
                field_accessor(output, item, field, &item.fields[index + 1..], schema);
            }
        }
        StructOrEnum::Enum(item) => {
            if resolves_to_kind(item, schema) {
                *output += "pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> { match self.inner {";
                for variant in item.all_variants(schema) {
                    let (ty, inner) = payload(variant, schema);
                    let child = node(ty).expect("kind payload");
                    let pattern = enum_pattern(item, variant);
                    let _ = writeln!(
                        output,
                        "{pattern} => AstNodeKind::{}(AstNode {{ inner: {inner}, parent: self.parent, following_span_start: self.following_span_start }}),",
                        child.name()
                    );
                }
                *output += "}}\n";
            }
            let variants = item.all_variants(schema).collect::<Vec<_>>();
            for variant in &variants {
                let fallback = if variants.len() == 2 {
                    let other = variants
                        .iter()
                        .find(|other| other.name != variant.name)
                        .expect("two variants");
                    format!("{}::{}(..)", item.name, other.name)
                } else {
                    "_".to_string()
                };
                let (ty, value) = payload(variant, schema);
                let _ = writeln!(
                    output,
                    "pub fn as_{}(&self) -> Option<AstNode<'parent, 'a, {}>> {{ match self.inner {{ {} => Some(AstNode {{ inner: {value}, parent: self.parent, following_span_start: self.following_span_start }}), {fallback} => None }} }}",
                    snake_case(&variant.name),
                    source(ty, schema),
                    enum_pattern(item, variant),
                );
            }
            for inherited in item.all_inherits(schema) {
                let _ = writeln!(
                    output,
                    "pub fn as_{snake}(&self) -> Option<AstNode<'parent, 'a, {ty}>> {{ self.inner.as_{snake}().map(|inner| AstNode {{ inner, parent: self.parent, following_span_start: self.following_span_start }}) }}",
                    snake = inherited.snake_name(),
                    ty = source(inherited, schema),
                );
            }
        }
    }
    *output += "}\n";
}

fn has_kind(item: StructOrEnum<'_>) -> bool {
    matches!(item, StructOrEnum::Struct(item) if item.kind.has_kind)
}

/// Whether every variant holds a node with a kind.
fn resolves_to_kind(item: &EnumDef, schema: &Schema) -> bool {
    item.all_variants(schema).all(|variant| node(payload(variant, schema).0).is_some_and(has_kind))
}

fn field_accessor(
    output: &mut String,
    item: &StructDef,
    field: &FieldDef,
    following_fields: &[FieldDef],
    schema: &Schema,
) {
    let name = &field.name;
    let parent = if item.kind.has_kind { "self.as_ast_nodes()" } else { "self.parent" };
    let default_following =
        if has_no_following_node(item, schema) { "0" } else { "self.following_span_start" };
    let mut optional = Vec::new();
    let mut fallback = default_following.to_string();
    for next in following_fields {
        let value = format!("self.inner.{}", next.name);
        if let Some(start) = following_start(next.type_def(schema), &value, schema) {
            match start {
                FollowingStart::Direct(start) => {
                    fallback = start;
                    break;
                }
                start @ FollowingStart::Optional { .. } => optional.push(start),
            }
        }
    }
    let following = match optional.as_slice() {
        [] => fallback,
        [start] => start.with_fallback(&fallback),
        starts => {
            let mut starts = starts.iter().map(FollowingStart::as_option);
            let mut expression = starts.next().unwrap();
            for start in starts {
                expression = format!("{expression}.or_else(|| {start})");
            }
            format!("{expression}.unwrap_or({fallback})")
        }
    };
    let mut ty = field.type_def(schema);
    let optional = ty.is_option();
    if let TypeDef::Option(option_def) = ty {
        ty = option_def.inner_type(schema);
    }
    let boxed = ty.is_box();
    ty = unboxed(ty, schema);
    let value = if optional { "inner".to_string() } else { format!("&self.inner.{name}") };
    let value = if boxed { format!("({value}).as_ref()") } else { value };
    let child = format!(
        "AstNode {{ inner: {value}, parent: {parent}, following_span_start: {following} }}"
    );
    if optional {
        let _ = writeln!(
            output,
            "pub fn {name}(&self) -> Option<AstNode<'_, 'a, {}>> {{ self.inner.{name}.as_ref().map(|inner| {child}) }}",
            source(ty, schema)
        );
    } else {
        let _ = writeln!(
            output,
            "pub fn {name}(&self) -> AstNode<'_, 'a, {}> {{ {child} }}",
            source(ty, schema)
        );
    }
}

enum FollowingStart {
    Direct(String),
    Optional { value: String, inner: Box<Self> },
}

impl FollowingStart {
    fn as_option(&self) -> String {
        let Self::Optional { value, inner } = self else { unreachable!() };
        match inner.as_ref() {
            Self::Direct(expression) => format!("{value}.map(|it| {expression})"),
            inner @ Self::Optional { .. } => {
                format!("{value}.and_then(|it| {})", inner.as_option())
            }
        }
    }

    fn with_fallback(&self, fallback: &str) -> String {
        let Self::Optional { value, inner } = self else { unreachable!() };
        match inner.as_ref() {
            Self::Direct(expression) => format!("{value}.map_or({fallback}, |it| {expression})"),
            Self::Optional { .. } => format!("{}.unwrap_or({fallback})", self.as_option()),
        }
    }
}

fn following_start(ty: &TypeDef, value: &str, schema: &Schema) -> Option<FollowingStart> {
    match ty {
        TypeDef::Option(option_def) => {
            optional_start(option_def.inner_type(schema), value, "as_ref", schema)
        }
        TypeDef::Vec(vec_def) => optional_start(vec_def.inner_type(schema), value, "first", schema),
        TypeDef::Box(box_def) => following_start(box_def.inner_type(schema), value, schema),
        ty => node(ty)
            .filter(|item| has_span(*item))
            .map(|_| FollowingStart::Direct(format!("{value}.span().start"))),
    }
}

fn optional_start(
    inner: &TypeDef,
    value: &str,
    accessor: &str,
    schema: &Schema,
) -> Option<FollowingStart> {
    following_start(inner, "it", schema).map(|inner| FollowingStart::Optional {
        value: format!("{value}.{accessor}()"),
        inner: Box::new(inner),
    })
}

pub(super) fn has_span(item: StructOrEnum<'_>) -> bool {
    item.generates_derive("GetSpan")
}

/// Whether `item` is a statement or listed in `AST_NODE_WITHOUT_FOLLOWING_NODE_LIST`, whose last
/// child keeps the comments up to the node's end instead of leaving them to its next sibling.
fn has_no_following_node(item: &StructDef, schema: &Schema) -> bool {
    if AST_NODE_WITHOUT_FOLLOWING_NODE_LIST.contains(&item.name.as_str()) {
        return true;
    }
    if AST_NODE_WITH_FOLLOWING_NODE_LIST.contains(&item.name.as_str()) {
        return false;
    }
    let statements = schema.type_by_name("Statement").as_enum().expect("a choice");
    statements.all_variants(schema).any(|variant| {
        node(payload(variant, schema).0).is_some_and(|node| node.name() == item.name)
    })
}
