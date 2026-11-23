use oxc_index::IndexVec;
use robusta_ast::*;
use robusta_syntax::{node::NodeId, scope::ScopeId};

use super::AstNode;

/// Concrete AST nodes flattened in deterministic preorder.
///
/// Dispatch enums such as [`Expression`] and [`Statement`] are transparent:
/// a node's parent is the nearest concrete node that owns it. The compilation
/// unit is always [`NodeId::ROOT`] and is its own parent.
#[derive(Debug, Default)]
pub struct AstNodes<'a> {
    nodes: IndexVec<NodeId, AstNode<'a>>,
    /// `node` -> `parent`
    parent_ids: IndexVec<NodeId, NodeId>,
}

impl<'a> AstNodes<'a> {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AstNode<'a>> + '_ {
        self.nodes.iter()
    }

    pub fn iter_enumerated(&self) -> impl ExactSizeIterator<Item = (NodeId, &AstNode<'a>)> + '_ {
        self.nodes.iter_enumerated()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Walk up from a node's parent to the compilation unit.
    fn ancestor_ids(&self, mut id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::from_fn(move || {
            (id != NodeId::ROOT).then(|| {
                id = self.parent_id(id);
                id
            })
        })
    }

    /// Walk up from a node's parent to the compilation unit, yielding each kind.
    pub fn ancestor_kinds(&self, id: NodeId) -> impl Iterator<Item = AstKind<'a>> + '_ {
        self.ancestor_ids(id).map(|id| self.kind(id))
    }

    /// Walk up from a node's parent to the compilation unit, yielding each id and node.
    pub fn ancestors_enumerated(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (NodeId, &AstNode<'a>)> + '_ {
        self.ancestor_ids(id).map(|id| (id, self.get_node(id)))
    }

    #[inline]
    pub fn kind(&self, id: NodeId) -> AstKind<'a> {
        self.nodes[id].kind()
    }

    #[inline]
    pub fn parent_id(&self, id: NodeId) -> NodeId {
        self.parent_ids[id]
    }

    pub fn parent_kind(&self, id: NodeId) -> AstKind<'a> {
        self.kind(self.parent_id(id))
    }

    pub fn parent_node(&self, id: NodeId) -> &AstNode<'a> {
        self.get_node(self.parent_id(id))
    }

    #[inline]
    pub fn get_node(&self, id: NodeId) -> &AstNode<'a> {
        &self.nodes[id]
    }

    /// The [`CompilationUnit`] that is also the root of the AST.
    #[inline]
    pub fn compilation_unit(&self) -> &'a CompilationUnit<'a> {
        if let Some(node) = self.nodes.first()
            && let AstKind::CompilationUnit(unit) = node.kind()
        {
            return unit;
        }
        unreachable!("the compilation unit is always the root node");
    }

    /// Add a node below `parent_id` and get its id.
    pub(crate) fn add_node(
        &mut self,
        kind: AstKind<'a>,
        scope_id: ScopeId,
        parent_id: NodeId,
    ) -> NodeId {
        self.parent_ids.push(parent_id);
        self.nodes.push(AstNode::new(kind, scope_id))
    }

    /// Add the compilation unit as the root node, which is its own parent.
    pub(crate) fn add_compilation_unit_node(&mut self, kind: AstKind<'a>, scope_id: ScopeId) {
        debug_assert!(self.nodes.is_empty() && matches!(kind, AstKind::CompilationUnit(_)));
        self.parent_ids.push(NodeId::ROOT);
        self.nodes.push(AstNode::new(kind, scope_id));
    }
}
