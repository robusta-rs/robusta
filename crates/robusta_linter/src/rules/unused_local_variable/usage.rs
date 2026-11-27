use std::cell::Cell;

use robusta_ast::{AssignmentTarget, AstKind, Expression, IdentifierReference};
use robusta_semantic::{NodeId, Reference, Semantic, SymbolId};
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::AssignOp;

use super::value::Value;

pub(super) struct Local<'a, 's> {
    pub semantic: &'s Semantic<'a>,
    pub symbol_id: SymbolId,
    pub effect_cache: Cell<Option<CachedValue>>,
}

#[derive(Clone, Copy)]
pub(super) struct CachedValue {
    owner: NodeId,
    span: Span,
    value: Value,
}

impl Local<'_, '_> {
    pub fn is_read(&self) -> bool {
        self.semantic
            .scoping()
            .references_for_symbol(self.symbol_id)
            .any(|reference| reference.kind().is_read() && self.update(reference).is_none())
    }

    pub fn update(&self, reference: &Reference<'_>) -> Option<NodeId> {
        if let Some(cached) = self.effect_cache.get()
            && !cached.value.effect
            && !cached.value.target_dependent
            && cached.span.contains_inclusive(reference.span())
        {
            return Some(cached.owner);
        }
        let nodes = self.semantic.nodes();
        for (id, node) in nodes.ancestors_enumerated(reference.node_id()) {
            match node.kind() {
                AstKind::AssignmentExpression(assignment) => {
                    if self.is_target(&assignment.target) {
                        if assignment.target.span().contains_inclusive(reference.span()) {
                            return (assignment.op == AssignOp::Assign || self.is_discarded(id))
                                .then_some(id);
                        }
                        if self.is_discarded(id)
                            && !self.value_has_effect(id, assignment.value.span())
                        {
                            return Some(id);
                        }
                    }
                    return None;
                }
                AstKind::UpdateExpression(update) if self.is_target(&update.target) => {
                    return self.is_discarded(id).then_some(id);
                }
                AstKind::LambdaExpression(_)
                | AstKind::MethodDeclaration(_)
                | AstKind::ExpressionStatement(_) => return None,
                _ => {}
            }
        }
        None
    }

    pub fn is_target(&self, target: &AssignmentTarget<'_>) -> bool {
        target.get_identifier_reference().is_some_and(|name| self.is_reference(name))
    }

    fn is_reference(&self, name: &IdentifierReference<'_>) -> bool {
        self.semantic.scoping().reference(name.reference_id()).symbol_id() == Some(self.symbol_id)
    }

    fn value_has_effect(&self, id: NodeId, span: Span) -> bool {
        if let Some(cached) = self.effect_cache.get()
            && cached.owner == id
        {
            return cached.value.effect;
        }
        let value = Value::inspect(self.semantic.nodes(), id, span);
        self.effect_cache.set(Some(CachedValue { owner: id, span, value }));
        value.effect
    }

    pub fn retained_value<'e, 'ast>(
        &self,
        mut expression: &'e Expression<'ast>,
    ) -> &'e Expression<'ast> {
        while let Expression::Assignment(assignment) = expression {
            if assignment.op != AssignOp::Assign || !self.is_target(&assignment.target) {
                break;
            }
            expression = &assignment.value;
        }
        expression
    }

    pub fn is_discarded(&self, mut id: NodeId) -> bool {
        let nodes = self.semantic.nodes();
        while id != NodeId::ROOT {
            let parent = nodes.parent_id(id);
            match nodes.kind(parent) {
                AstKind::ParenthesizedExpression(_) => id = parent,
                AstKind::ExpressionStatement(_) | AstKind::StatementExpressionList(_) => {
                    return true;
                }
                AstKind::SwitchRule(_) => {
                    return nodes
                        .ancestor_kinds(parent)
                        .find_map(|kind| match kind {
                            AstKind::SwitchStatement(_) => Some(true),
                            AstKind::SwitchExpression(_) => Some(false),
                            _ => None,
                        })
                        .unwrap_or(false);
                }
                _ => return false,
            }
        }
        false
    }

    pub fn declaration_id(&self) -> NodeId {
        self.semantic.scoping().symbol(self.symbol_id).declaration_node_id()
    }
}
