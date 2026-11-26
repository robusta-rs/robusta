// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_syntax/src/node.rs

//! AST node ID.

use oxc_index::define_nonmax_u32_index_type;
use robusta_allocator::{Allocator, Dummy};
use robusta_ast_macros::ast;

define_nonmax_u32_index_type! {
    /// AST node ID.
    ///
    /// The parser assigns IDs that are unique within the tree but may have gaps
    /// and do not follow visitor order. Semantic analysis replaces them with
    /// dense IDs that index its node table.
    #[ast]
    #[content_eq(skip)]
    pub struct NodeId;
}

impl NodeId {
    /// ID of a node built outside the parser, until semantic analysis assigns one.
    pub const DUMMY: Self = Self::new(0);

    /// ID of the compilation unit.
    pub const ROOT: Self = Self::new(0);
}

impl Default for NodeId {
    #[inline]
    fn default() -> Self {
        Self::DUMMY
    }
}

impl<'a> Dummy<'a> for NodeId {
    #[inline]
    fn dummy(_: &'a Allocator) -> Self {
        Self::DUMMY
    }
}
