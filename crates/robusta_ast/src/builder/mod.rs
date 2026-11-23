// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_ast/src/builder/mod.rs

//! AST builder.
//!
//! AST nodes are created by builder methods defined on the AST types themselves,
//! which are passed an `&impl GetAstBuilder` or an `&impl GetAllocator`:
//!
//! * `Statement::new_expression_statement(span, expression, &builder)`
//! * `Vec::new_in(&builder)`
//!
//! These methods are defined in `../generated/ast_builder.rs`.

use robusta_allocator::{Allocator, GetAllocator};
use robusta_syntax::node::NodeId;

/// Trait for types which can create AST nodes.
///
/// Provides the [`NodeId`]s used by the AST node builder methods defined on AST types
/// (e.g. `Statement::new_expression_statement`), and the memory arena through [`GetAllocator`].
///
/// [`AstBuild`] types must also implement [`GetAstBuilder`], referring to themselves.
pub trait AstBuild<'a>: GetAstBuilder<'a, Builder = Self> + GetAllocator<'a> {
    /// Get [`NodeId`] to assign to an AST node.
    fn node_id(&self) -> NodeId;
}

/// Trait for types which provide access to an [`AstBuild`]er.
///
/// Implemented by the [`AstBuild`]ers themselves (returning `self`), and by types which hold one
/// (e.g. the parser). AST node builder methods take any `&impl GetAstBuilder<'a>`,
/// so they can be called with a builder directly, or with a type which holds one.
pub trait GetAstBuilder<'a> {
    /// The [`AstBuild`]er type that this provides access to.
    type Builder: AstBuild<'a>;

    /// Get the [`AstBuild`]er.
    fn builder(&self) -> &Self::Builder;
}

/// AST builder which assigns dummy [`NodeId`]s to AST nodes.
///
/// For use where no `NodeId`s are required on AST nodes as they are built,
/// because semantic analysis assigns them.
pub struct AstBuilder<'a> {
    /// The memory allocator used to allocate AST types in the arena.
    allocator: &'a Allocator,
}

impl<'a> AstBuilder<'a> {
    /// Create a new [`AstBuilder`] that will allocate AST types in the provided [`Allocator`].
    #[inline]
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator }
    }
}

impl<'a> AstBuild<'a> for AstBuilder<'a> {
    /// Get [`NodeId`] to assign to an AST node.
    ///
    /// [`AstBuilder`] does not assign real `NodeId`s - it always returns [`NodeId::DUMMY`].
    #[inline]
    fn node_id(&self) -> NodeId {
        NodeId::DUMMY
    }
}

impl<'a> GetAstBuilder<'a> for AstBuilder<'a> {
    type Builder = Self;

    #[inline]
    fn builder(&self) -> &Self {
        self
    }
}

impl<'a> GetAllocator<'a> for AstBuilder<'a> {
    /// Get the memory [`Allocator`] to allocate AST types in.
    #[inline]
    fn allocator(&self) -> &'a Allocator {
        self.allocator
    }
}
