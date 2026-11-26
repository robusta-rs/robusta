//! File-local Java semantic analysis.

mod ast_types_bitset;
mod binder;
mod builder;
mod node;
mod pattern_scope;
mod scoping;

use std::cell::OnceCell;

pub use ast_types_bitset::AstTypesBitset;
pub use builder::SemanticBuilder;
pub use node::{AstNode, AstNodes};
pub use pattern_scope::{Unresolved, block_can_complete_normally};
pub use robusta_syntax::{node::NodeId, reference::ReferenceId, scope::ScopeId, symbol::SymbolId};
pub use scoping::{Redeclaration, Reference, ReferenceKind, Scope, Scoping, Symbol, SymbolKind};

use robusta_ast::Comment;
use robusta_javadoc::JavadocFinder;
use robusta_span::Span;

/// Shared file-local analysis for one compilation unit, built by [`SemanticBuilder`].
///
/// No classpath lookup, type inference, or overload resolution.
#[derive(Debug)]
pub struct Semantic<'a> {
    source_text: &'a str,
    comments: &'a [Comment],
    javadocs: OnceCell<JavadocFinder<'a>>,
    nodes: AstNodes<'a>,
    scoping: Scoping<'a>,
}

impl<'a> Semantic<'a> {
    /// The source text of the analyzed compilation unit.
    pub const fn source_text(&self) -> &'a str {
        self.source_text
    }

    /// The unit's comments in source order.
    pub const fn comments(&self) -> &'a [Comment] {
        self.comments
    }

    /// Whether any comment starts within `span`.
    pub fn has_comments_between(&self, span: Span) -> bool {
        robusta_ast::has_comments_between(self.comments, span)
    }

    /// The unit's Javadoc, the documentation comments that document a
    /// declaration, with lazily cached structure.
    pub fn javadocs(&self) -> &JavadocFinder<'a> {
        self.javadocs.get_or_init(|| {
            JavadocFinder::new(
                self.source_text,
                self.comments,
                self.nodes.iter().map(AstNode::kind),
            )
        })
    }

    pub const fn nodes(&self) -> &AstNodes<'a> {
        &self.nodes
    }

    pub const fn scoping(&self) -> &Scoping<'a> {
        &self.scoping
    }
}

#[cfg(test)]
mod tests;
