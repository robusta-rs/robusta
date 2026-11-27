use robusta_allocator::GetAddress;
use robusta_ast::{AstKind, Expression, LocalVariableDeclaration};
use robusta_semantic::{AstNodes, NodeId};
use robusta_span::{GetSpan, Span};
use robusta_syntax::precedence::{GetPrecedence, Precedence};
use rustc_hash::FxHashSet;

use super::{usage::Local, value::Value};
use crate::utils::{
    follows_documentation, normalize_deletions, pad_fix_with_token_boundary, removal_span,
};
use crate::{Fix, FixKind};

impl Local<'_, '_> {
    pub fn fix(&self, kind: AstKind<'_>, source: &str) -> (FixKind, Vec<Fix>) {
        let mut edits = Vec::new();
        let mut fix_kind = FixKind::SAFE_FIX;
        let AstKind::VariableDeclarator(declarator) = kind else { unreachable!() };
        let nodes = self.semantic.nodes();
        let declaration_id = nodes.parent_id(self.declaration_id());
        let AstKind::LocalVariableDeclaration(declaration) = nodes.kind(declaration_id) else {
            unreachable!()
        };
        if matches!(nodes.parent_kind(declaration_id), AstKind::TypePattern(_)) {
            // The pattern keeps its type, and the value is still tested against it.
            edits.push(Fix::new(
                declaration.ty.span().source_text(source).to_owned(),
                declaration.span,
            ));
        } else {
            let (kind, keep) =
                declarator.init.as_ref().map_or((FixKind::SAFE_FIX, false), |value| {
                    let expression =
                        value.as_expression().map(|expression| self.retained_value(expression));
                    Value::inspect(nodes, self.declaration_id(), value.span()).classify(expression)
                });
            fix_kind = kind;
            let initializer = if keep { declarator.init.as_ref().map(GetSpan::span) } else { None };
            self.fix_declaration(source, declaration, initializer, &mut fix_kind, &mut edits);
        }
        let mut seen = FxHashSet::default();
        for reference in self.semantic.scoping().references_for_symbol(self.symbol_id) {
            if let Some(id) = self.update(reference)
                && seen.insert(id)
            {
                let (kind, edit) = self.fix_update(id);
                fix_kind = cautious(fix_kind, kind);
                edits.push(edit);
            }
        }
        self.fix_for_lists(&seen, &mut edits);
        // An enclosing deletion subsumes writes inside its discarded value.
        edits.sort_by_key(|edit| (edit.span.start, std::cmp::Reverse(edit.span.end)));
        let mut combined: Vec<Fix> = Vec::new();
        for edit in edits {
            if combined.last().is_none_or(|previous| !previous.span.contains_inclusive(edit.span)) {
                combined.push(edit);
            }
        }
        normalize_deletions(source, &mut combined);
        (fix_kind, combined)
    }

    fn fix_declaration(
        &self,
        source: &str,
        decl: &LocalVariableDeclaration<'_>,
        keep: Option<Span>,
        fix_kind: &mut FixKind,
        edits: &mut Vec<Fix>,
    ) {
        let nodes = self.semantic.nodes();
        let declaration_id = nodes.parent_id(self.declaration_id());
        let is_for = matches!(nodes.parent_kind(declaration_id), AstKind::ForStatement(_));
        let index = decl
            .declarators
            .iter()
            .position(|declarator| declarator.span == nodes.kind(self.declaration_id()).span())
            .unwrap();
        let count = decl.declarators.len();
        if let Some(value) = keep
            && count == 1
        {
            edits.push(Fix::delete(Span::new(decl.span.start, value.start)));
            return;
        }
        if let Some(value) = keep
            && !is_for
        {
            let prefix =
                Span::new(decl.span.start, decl.declarators[0].span.start).source_text(source);
            let before = if index == 0 {
                Span::new(decl.span.start, value.start)
            } else {
                Span::new(decl.declarators[index - 1].span.end, value.start)
            };
            edits.push(Fix::new(if index == 0 { "" } else { "; " }, before));
            if index + 1 < count {
                edits.push(Fix::new(
                    format!("; {prefix}"),
                    Span::new(value.end, decl.declarators[index + 1].span.start),
                ));
            }
            return;
        }
        if keep.is_some() {
            *fix_kind = FixKind::UNSAFE_FIX;
        }
        if count == 1 && !is_for {
            edits.push(self.delete_statement(decl.span));
            return;
        }
        let span = if count == 1 {
            decl.span
        } else if index == 0 {
            Span::new(decl.declarators[0].span.start, decl.declarators[1].span.start)
        } else {
            Span::new(decl.declarators[index - 1].span.end, decl.declarators[index].span.end)
        };
        edits.push(Fix::delete(span));
    }

    fn fix_update(&self, id: NodeId) -> (FixKind, Fix) {
        let nodes = self.semantic.nodes();
        let (kind, value) = match nodes.kind(id) {
            AstKind::AssignmentExpression(assignment) => {
                if !self.is_discarded(id) {
                    return (FixKind::UNSAFE_FIX, self.replace_with_value(id, &assignment.value));
                }
                let (kind, keep) = Value::inspect(nodes, id, assignment.value.span())
                    .classify(Some(self.retained_value(&assignment.value)));
                let kind = if assignment.op == robusta_syntax::operator::AssignOp::Assign {
                    kind
                } else {
                    FixKind::UNSAFE_FIX
                };
                (kind, keep.then_some(&assignment.value))
            }
            _ => (FixKind::UNSAFE_FIX, None),
        };
        (kind, self.replace_statement(id, value))
    }

    /// Replaces an assignment inside a larger expression with its value, and
    /// drops the parentheses around it where the value keeps its place without
    /// them. They stay when another write of the variable encloses them, since
    /// its edit moves their neighbors, or lies in the value, whose edit this
    /// replacement would subsume.
    fn replace_with_value(&self, id: NodeId, value: &Expression<'_>) -> Fix {
        let nodes = self.semantic.nodes();
        let span = nodes.kind(id).span();
        let parent_id = nodes.parent_id(id);
        if let AstKind::ParenthesizedExpression(parenthesized) = nodes.kind(parent_id)
            && !self.semantic.has_comments_between(Span::new(parenthesized.span.start, span.start))
            && !self
                .semantic
                .has_comments_between(Span::new(value.span().end, parenthesized.span.end))
            && !needs_parentheses(nodes, parent_id, value)
            && !self.is_inside_write(parent_id)
            && !self.is_referenced_in(value.span())
        {
            let source = self.semantic.source_text();
            let mut replacement = value.span().source_text(source).to_owned();
            pad_fix_with_token_boundary(source, parenthesized.span, &mut replacement);
            return Fix::new(replacement, parenthesized.span);
        }
        Fix::delete(Span::new(span.start, value.span().start))
    }

    /// Whether the variable is referenced inside `span`.
    fn is_referenced_in(&self, span: Span) -> bool {
        self.semantic
            .scoping()
            .references_for_symbol(self.symbol_id)
            .any(|reference| span.contains_inclusive(reference.span()))
    }

    /// Whether `id` is inside the value of an assignment to the variable.
    fn is_inside_write(&self, id: NodeId) -> bool {
        self.semantic
            .nodes()
            .ancestor_kinds(id)
            .take_while(|kind| {
                !matches!(
                    kind,
                    AstKind::LambdaExpression(_)
                        | AstKind::MethodDeclaration(_)
                        | AstKind::ExpressionStatement(_)
                )
            })
            .any(|kind| {
                matches!(kind, AstKind::AssignmentExpression(outer) if self.is_target(&outer.target))
            })
    }

    fn replace_statement(&self, id: NodeId, value: Option<&Expression<'_>>) -> Fix {
        let nodes = self.semantic.nodes();
        let span = nodes.kind(id).span();
        let parent_id = nodes.parent_id(id);
        if let Some(value) = value {
            return Fix::delete(Span::new(span.start, value.span().start));
        }
        match nodes.kind(parent_id) {
            AstKind::ExpressionStatement(statement) => {
                if matches!(
                    nodes.parent_kind(parent_id),
                    AstKind::Block(_) | AstKind::SwitchCase(_)
                ) {
                    self.delete_statement(statement.span)
                } else {
                    Fix::new("{}", statement.span)
                }
            }
            AstKind::SwitchRule(rule) => Fix::new("{}", Span::new(span.start, rule.span.end)),
            AstKind::StatementExpressionList(_) => Fix::delete(span),
            _ => unreachable!("discarded update has a statement or for-list parent"),
        }
    }

    /// Deletes a statement from a statement list, with its lines when nothing
    /// else is on them. Directly after a documentation comment it leaves `;`,
    /// so the comment cannot attach to the next declaration.
    fn delete_statement(&self, span: Span) -> Fix {
        let semantic = self.semantic;
        if follows_documentation(semantic.comments(), span) {
            Fix::new(";", span)
        } else {
            Fix::delete(removal_span(semantic.source_text(), span))
        }
    }

    fn fix_for_lists(&self, updates: &FxHashSet<NodeId>, edits: &mut Vec<Fix>) {
        let nodes = self.semantic.nodes();
        let mut lists = updates
            .iter()
            .filter_map(|&id| {
                let parent = nodes.parent_id(id);
                match nodes.kind(parent) {
                    AstKind::StatementExpressionList(list) => Some((parent, list)),
                    _ => None,
                }
            })
            .peekable();
        if lists.peek().is_none() {
            return;
        }
        let deletions: FxHashSet<_> =
            edits.iter().filter_map(|edit| edit.content.is_empty().then_some(edit.span)).collect();
        let mut seen = FxHashSet::default();
        for (parent, list) in lists {
            if !seen.insert(parent) {
                continue;
            }
            let expressions = &list.expressions;
            let mut index = 0;
            while index < expressions.len() {
                if !deletions.contains(&expressions[index].span()) {
                    index += 1;
                    continue;
                }
                let first = index;
                while index < expressions.len() && deletions.contains(&expressions[index].span()) {
                    index += 1;
                }
                // Remove one separator for the whole consecutive group.
                let span = if first > 0 {
                    Span::new(expressions[first - 1].span().end, expressions[index - 1].span().end)
                } else if index < expressions.len() {
                    Span::new(expressions[0].span().start, expressions[index].span().start)
                } else {
                    expressions[0].span().cover(expressions[index - 1].span())
                };
                edits.push(Fix::delete(span));
            }
        }
    }
}

/// Whether `value` needs the parentheses `id` to keep its place in the
/// expression around them.
fn needs_parentheses(nodes: &AstNodes<'_>, id: NodeId, value: &Expression<'_>) -> bool {
    let node = nodes.kind(id);
    let precedence = value.precedence();
    let parent = nodes.parent_kind(id);
    match parent {
        AstKind::BinaryExpression(binary) => {
            let operator = binary.op.precedence();
            if binary.lhs.address() == node.address() {
                precedence < operator
            } else {
                precedence <= operator
            }
        }
        AstKind::InstanceofExpression(_) => precedence < Precedence::Compare,
        AstKind::UnaryExpression(_) => precedence < Precedence::Prefix,
        // A reference type cast reads a following `+` or `-` as a binary operator.
        AstKind::CastExpression(_) => precedence < Precedence::Postfix,
        AstKind::ConditionalExpression(conditional) => {
            if conditional.condition.address() == node.address() {
                precedence <= Precedence::Conditional
            } else {
                conditional.when_false.address() == node.address()
                    && precedence < Precedence::Conditional
            }
        }
        AstKind::ArrayAccess(access) => {
            access.array.address() == node.address() && precedence < Precedence::Member
        }
        AstKind::FieldAccess(_)
        | AstKind::MethodMember(_)
        | AstKind::MethodReference(_)
        | AstKind::QualifiedClassInstanceCreation(_) => {
            // A number keeps them: `1.` reads as a floating-point literal.
            parent.span().start == node.span().start
                && (precedence < Precedence::New
                    || matches!(value, Expression::IntLiteral(_) | Expression::FloatLiteral(_)))
        }
        _ => false,
    }
}

fn cautious(left: FixKind, right: FixKind) -> FixKind {
    if left.is_unsafe() || right.is_unsafe() { FixKind::UNSAFE_FIX } else { FixKind::SAFE_FIX }
}
