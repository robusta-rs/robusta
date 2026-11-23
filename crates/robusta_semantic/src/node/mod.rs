mod nodes;

pub use nodes::AstNodes;

use robusta_ast::AstKind;
use robusta_syntax::{node::NodeId, scope::ScopeId};

/// One concrete syntax node and the scope it sits in.
#[derive(Debug, Clone, Copy)]
pub struct AstNode<'a> {
    kind: AstKind<'a>,
    scope_id: ScopeId,
}

impl<'a> AstNode<'a> {
    pub(crate) const fn new(kind: AstKind<'a>, scope_id: ScopeId) -> Self {
        Self { kind, scope_id }
    }

    #[inline]
    pub const fn kind(&self) -> AstKind<'a> {
        self.kind
    }

    /// Node id assigned to this AST node.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.kind.node_id()
    }

    /// The scope this node is in, not the scope it creates.
    #[inline]
    pub const fn scope_id(&self) -> ScopeId {
        self.scope_id
    }
}
