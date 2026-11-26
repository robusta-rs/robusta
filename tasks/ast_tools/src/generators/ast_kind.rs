// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/generators/ast_kind.rs

//! Generator for `AstKind` and `AstType`.

use std::{fmt::Write as _, io};

use crate::{
    AST_CRATE_PATH,
    output::{Output, output_path},
    schema::{Def, Schema},
};

use super::{Generator, define_generator};

/// Generator for `AstKind` and `AstType`.
pub struct AstKindGenerator;

define_generator!(AstKindGenerator);

impl Generator for AstKindGenerator {
    /// Set `has_kind` for visited structs with a `NodeId`.
    fn prepare(&self, schema: &mut Schema) {
        let node_id_cell_type_id = schema.type_names.get("Cell<NodeId>").copied();
        for struct_def in schema.structs_mut() {
            struct_def.kind.has_kind = struct_def.visit.has_visitor()
                && struct_def.fields.iter().any(|field| {
                    field.name == "node_id" && Some(field.type_id) == node_id_cell_type_id
                });
        }
    }

    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        Ok(vec![Output {
            path: output_path(AST_CRATE_PATH, "ast_kind.rs"),
            code: generate(schema),
        }])
    }
}

const HEADER: &str = "\
use robusta_allocator::{Address, GetAddress, UnstableAddress};
use robusta_span::{GetSpan, Span};
use robusta_syntax::node::NodeId;
///@@line_break
use crate::*;
///@@line_break
/// Typed view of concrete AST nodes.
///
/// Dispatch enums such as [`Expression`] have no variant, only their payload nodes.
#[derive(Debug, Clone, Copy)]
pub enum AstKind<'a> {
";

const AST_TYPE_HEADER: &str = "\
}
///@@line_break
/// The fieldless discriminant of [`AstKind`]: one dense `u8` value per node
/// kind, usable as an array index for kind-keyed dispatch tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AstType {
";

/// Render the node-kind source, in discriminant order.
fn generate(schema: &Schema) -> String {
    let nodes = schema.structs().filter(|item| item.kind.has_kind).collect::<Vec<_>>();
    let mut output = String::from(HEADER);

    for item in &nodes {
        let kind = &item.name;
        let lifetime = if item.has_lifetime(schema) { "<'a>" } else { "" };
        let _ = writeln!(output, "    {kind}(&'a {kind}{lifetime}),");
    }

    output += AST_TYPE_HEADER;
    for item in &nodes {
        let _ = writeln!(output, "    {},", item.name);
    }

    output += "}\n///@@line_break\nimpl AstType {\n";
    output += "    /// Number of node kinds; discriminants are dense in `0..COUNT`.\n";
    let _ = writeln!(output, "    pub const COUNT: usize = {};", nodes.len());
    output += "///@@line_break\n";
    output += "    /// Every node kind, in discriminant order.\n";
    output += "    pub const ALL: [Self; Self::COUNT] = [\n";
    for item in &nodes {
        let _ = writeln!(output, "        Self::{},", item.name);
    }
    output += "    ];\n}\n///@@line_break\n";

    output += "impl GetSpan for AstKind<'_> {\n";
    output += "    fn span(&self) -> Span {\n        match self {\n";
    for item in &nodes {
        let _ = writeln!(output, "            Self::{}(node) => node.span(),", item.name);
    }
    output += "        }\n    }\n}\n///@@line_break\n";

    output += "impl GetAddress for AstKind<'_> {\n";
    output += "    /// Get [`Address`] of an [`AstKind`].\n";
    output += "    ///@ This boils down to 1 instruction.\n";
    output += "    ///@ In all cases, it gets the pointer from the reference in the `AstKind`.\n";
    output += "    #[inline]\n    fn address(&self) -> Address {\n        match *self {\n";
    for item in &nodes {
        let _ = writeln!(output, "            Self::{}(it) => it.unstable_address(),", item.name);
    }
    output += "        }\n    }\n}\n///@@line_break\n";

    output += "impl AstKind<'_> {\n";
    output += "    /// Get [`NodeId`] of an [`AstKind`].\n";
    output += "    #[inline]\n    pub fn node_id(&self) -> NodeId {\n        match self {\n";
    for item in &nodes {
        let _ = writeln!(output, "            Self::{}(it) => it.node_id(),", item.name);
    }
    output += "        }\n    }\n///@@line_break\n";
    output += "    /// Set [`NodeId`] of an [`AstKind`].\n";
    output +=
        "    #[inline]\n    pub fn set_node_id(&self, node_id: NodeId) {\n        match self {\n";
    for item in &nodes {
        let _ = writeln!(output, "            Self::{}(it) => it.set_node_id(node_id),", item.name);
    }
    output += "        }\n    }\n///@@line_break\n";
    output += "    /// The fieldless discriminant of this node kind.\n";
    output += "    #[inline]\n    pub const fn ty(self) -> AstType {\n        match self {\n";
    for item in &nodes {
        let kind = &item.name;
        let _ = writeln!(output, "            Self::{kind}(_) => AstType::{kind},");
    }
    output += "        }\n    }\n///@@line_break\n";
    output += "    /// The concrete variant name, useful when inspecting a traversal.\n";
    output += "    #[inline]\n";
    output += "    pub const fn debug_name(self) -> &'static str {\n        match self {\n";
    for item in &nodes {
        let kind = &item.name;
        let _ = writeln!(output, "            Self::{kind}(_) => \"{kind}\",");
    }
    output += "        }\n    }\n}\n";

    output
}
