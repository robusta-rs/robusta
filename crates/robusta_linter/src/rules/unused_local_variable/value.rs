use robusta_ast::{AstKind, Expression, TypeArguments};
use robusta_semantic::{AstNodes, NodeId};
use robusta_span::{GetSpan, Span};

use crate::FixKind;

#[derive(Clone, Copy, Default)]
pub(super) struct Value {
    pub effect: bool,
    pub target_dependent: bool,
}

impl Value {
    pub fn inspect(nodes: &AstNodes<'_>, owner: NodeId, span: Span) -> Self {
        let mut value = Self::default();
        let mut skipped = None;
        for node in nodes
            .iter()
            .skip(owner.index() + 1)
            .take_while(|node| nodes.kind(owner).span().contains_inclusive(node.kind().span()))
        {
            let kind = node.kind();
            let child = kind.span();
            if !span.contains_inclusive(child)
                || skipped.is_some_and(|skip: Span| skip.contains_inclusive(child))
            {
                continue;
            }
            match kind {
                AstKind::LambdaExpression(_) => {
                    value.target_dependent = true;
                    skipped = Some(child);
                }
                AstKind::MethodReference(_) => value.target_dependent = true,
                AstKind::ClassInstanceCreation(new) => {
                    value.effect = true;
                    value.target_dependent |= new.body.is_some()
                        && new.ty.type_arguments().is_some_and(TypeArguments::is_diamond);
                }
                AstKind::QualifiedClassInstanceCreation(new) => {
                    value.effect = true;
                    value.target_dependent |= new.body.is_some()
                        && new.type_args.as_deref().is_some_and(TypeArguments::is_diamond);
                }
                AstKind::ClassBody(_) => skipped = Some(child),
                AstKind::MethodCall(_)
                | AstKind::AssignmentExpression(_)
                | AstKind::UpdateExpression(_)
                | AstKind::SwitchExpression(_) => value.effect = true,
                _ => {}
            }
        }
        value
    }

    pub fn classify(self, expression: Option<&Expression<'_>>) -> (FixKind, bool) {
        let standalone = expression.is_some_and(Expression::is_statement_expression);
        if self.effect && (!standalone || self.target_dependent) {
            (FixKind::UNSAFE_FIX, false)
        } else if standalone {
            (FixKind::UNSAFE_FIX, true)
        } else if expression.is_some_and(|expression| {
            matches!(
                expression,
                Expression::IntLiteral(_)
                    | Expression::FloatLiteral(_)
                    | Expression::CharLiteral(_)
                    | Expression::StringLiteral(_)
                    | Expression::BooleanLiteral(_)
                    | Expression::NullLiteral(_)
            )
        }) {
            (FixKind::SAFE_FIX, false)
        } else {
            (FixKind::UNSAFE_FIX, false)
        }
    }
}
