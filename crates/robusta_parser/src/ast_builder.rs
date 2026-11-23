// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/ast_builder.rs

use std::cell::Cell;

use robusta_allocator::{Allocator, GetAllocator};
use robusta_ast::builder::{AstBuild, GetAstBuilder};
use robusta_syntax::node::NodeId;

/// Assign IDs as nodes are constructed, reserving zero for the compilation unit.
///
/// IDs follow construction order, rather than visitor order, and may have gaps
/// from discarded nodes, such as those of an ordinary parse that a compact one replaces.
pub struct ParserAstBuilder<'a> {
    allocator: &'a Allocator,
    next_node_id: Cell<usize>,
}

impl<'a> ParserAstBuilder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator, next_node_id: Cell::new(1) }
    }
}

impl<'a> AstBuild<'a> for ParserAstBuilder<'a> {
    #[inline]
    fn node_id(&self) -> NodeId {
        let index = self.next_node_id.get();
        assert!(index <= NodeId::MAX_INDEX, "Too many AST nodes");
        self.next_node_id.set(index + 1);
        NodeId::new(index)
    }
}

impl<'a> GetAstBuilder<'a> for ParserAstBuilder<'a> {
    type Builder = Self;

    #[inline]
    fn builder(&self) -> &Self {
        self
    }
}

impl<'a> GetAllocator<'a> for ParserAstBuilder<'a> {
    #[inline]
    fn allocator(&self) -> &'a Allocator {
        self.allocator
    }
}
