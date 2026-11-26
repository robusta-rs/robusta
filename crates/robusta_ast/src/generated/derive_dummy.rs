// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/dummy.rs`.

use robusta_allocator::{Allocator, Dummy};

use crate::ast::java::declarations::*;

impl<'a> Dummy<'a> for CompilationUnit<'a> {
    /// Create a dummy [`CompilationUnit`].
    ///
    /// Does not allocate any data into arena.
    fn dummy(allocator: &'a Allocator) -> Self {
        Self {
            node_id: Dummy::dummy(allocator),
            span: Dummy::dummy(allocator),
            comments: Dummy::dummy(allocator),
            content: Dummy::dummy(allocator),
            scope_id: Dummy::dummy(allocator),
        }
    }
}

impl<'a> Dummy<'a> for CompilationUnitContent<'a> {
    /// Create a dummy [`CompilationUnitContent`].
    ///
    /// Does not allocate any data into arena.
    fn dummy(allocator: &'a Allocator) -> Self {
        Self::Ordinary(Dummy::dummy(allocator))
    }
}

impl<'a> Dummy<'a> for OrdinaryCompilationUnit<'a> {
    /// Create a dummy [`OrdinaryCompilationUnit`].
    ///
    /// Does not allocate any data into arena.
    fn dummy(allocator: &'a Allocator) -> Self {
        Self {
            node_id: Dummy::dummy(allocator),
            span: Dummy::dummy(allocator),
            package: Dummy::dummy(allocator),
            imports: Dummy::dummy(allocator),
            types: Dummy::dummy(allocator),
        }
    }
}
