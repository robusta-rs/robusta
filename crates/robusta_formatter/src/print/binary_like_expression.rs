// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/print/binary_like_expression.rs

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::trivia::format_hoisted_leading_comments;
use crate::utils::is_header_condition;
use robusta_allocator::Allocator;
use robusta_ast::{BinaryExpression, Comment, Expression, InstanceofExpression};
use robusta_formatter_core::{format_args, write};
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::BinaryOp;
use robusta_syntax::precedence::{GetPrecedence, Precedence};

/// A binary or `instanceof` expression.
#[derive(Clone, Copy)]
pub enum BinaryLikeExpression<'a, 'b> {
    Binary(&'b AstNode<'b, 'a, BinaryExpression<'a>>),
    Instanceof(&'b AstNode<'b, 'a, InstanceofExpression<'a>>),
}

impl<'a, 'b> BinaryLikeExpression<'a, 'b> {
    fn left(self) -> AstNode<'b, 'a, Expression<'a>> {
        match self {
            Self::Binary(binary) => binary.lhs(),
            Self::Instanceof(instanceof) => instanceof.operand(),
        }
    }

    fn right_span(self) -> Span {
        match self {
            Self::Binary(binary) => binary.rhs.span(),
            Self::Instanceof(instanceof) => instanceof.rhs.span(),
        }
    }

    fn parent(self) -> &'b AstNodes<'b, 'a> {
        match self {
            Self::Binary(binary) => binary.parent(),
            Self::Instanceof(instanceof) => instanceof.parent(),
        }
    }

    /// Whether this is a `&&` or `||` expression. These break apart from the
    /// other operators, `instanceof` among them, in a chain that mixes both.
    fn is_logical(self) -> bool {
        matches!(self, Self::Binary(binary) if is_logical(binary.op))
    }

    /// Whether `expression`, inside any parentheses, is a binary expression of this kind.
    fn is_same_kind(self, mut expression: &Expression<'_>) -> bool {
        while let Expression::Parenthesized(parenthesized) = expression {
            expression = &parenthesized.expression;
        }
        match expression {
            Expression::Binary(binary) => is_logical(binary.op) == self.is_logical(),
            Expression::Instanceof(_) => !self.is_logical(),
            _ => false,
        }
    }

    /// Whether the parent, outside any parentheses, is a binary expression of this kind.
    fn has_same_kind_parent(self) -> bool {
        let mut parent = self.parent();
        while let AstNodes::ParenthesizedExpression(parenthesized) = parent {
            parent = parenthesized.parent();
        }
        match parent {
            AstNodes::BinaryExpression(binary) => is_logical(binary.op) == self.is_logical(),
            AstNodes::InstanceofExpression(_) => !self.is_logical(),
            _ => false,
        }
    }

    /// The left side when the chain continues through it, a binary expression
    /// that [`should_flatten`] accepts. It is in the arena, as it is the parent
    /// of the sides formatted after it, so a chain is walked without recursion.
    fn flattened_left(self, allocator: &'b Allocator) -> Option<Self> {
        let Self::Binary(binary) = self else { return None };
        let left = binary.lhs().as_binary()?;
        should_flatten(binary.op, left.op).then(|| Self::Binary(left.in_arena(allocator)))
    }
}

impl GetSpan for BinaryLikeExpression<'_, '_> {
    fn span(&self) -> Span {
        match self {
            Self::Binary(binary) => binary.span,
            Self::Instanceof(instanceof) => instanceof.span,
        }
    }
}

const fn is_logical(op: BinaryOp) -> bool {
    matches!(op, BinaryOp::And | BinaryOp::Or)
}

/// Whether a left operand with `operator` joins the chain of `parent_operator`
/// rather than keeping a group of its own.
fn should_flatten(parent_operator: BinaryOp, operator: BinaryOp) -> bool {
    let precedence = operator.precedence();
    if parent_operator.precedence() != precedence {
        return false;
    }

    match precedence {
        // `a == b == c` reads as `(a == b) == c`
        // `a << 3 << 4` reads as `(a << 3) << 4`
        Precedence::Equals | Precedence::Shift => false,
        Precedence::Multiply => {
            // `a * 3 % 5` reads as `(a * 3) % 5`
            if parent_operator == BinaryOp::Rem || operator == BinaryOp::Rem {
                return false;
            }

            // `a * 3 / 5` reads as `(a * 3) / 5`
            parent_operator == operator
        }
        _ => true,
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for BinaryLikeExpression<'a, '_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let parent = self.parent();
        let is_header_condition = is_header_condition(parent, self.span());

        let mut innermost = *self;
        let mut depth = 0;
        while let Some(left) = innermost.flattened_left(f.allocator()) {
            innermost = left;
            depth += 1;
        }
        let left = innermost.left();
        let left = group(&left);
        let right_sides =
            format_once(|f| format_right_sides(innermost, depth, is_header_condition, f));

        // The parentheses of a member's object group and indent it: `(a + b).c`.
        if parent.is_parenthesized_member_object() && !is_header_condition {
            write!(f, [left, right_sides]);
        } else if should_not_indent_if_parent_indents(parent)
            || should_indent_if_parent_inlines(parent)
        {
            write!(f, [group(&format_args!(left, right_sides))]);
        } else if is_header_condition || takes_whole_indent(parent) {
            write!(
                f,
                [group(&wrapped_indent(is_header_condition, &format_args!(left, right_sides)))]
            );
        } else {
            write!(f, [group(&format_args!(left, indent(&right_sides)))]);
        }
    }
}

/// Whether the parent indents the expression already, so its operators do not indent again.
fn should_not_indent_if_parent_indents(parent: &AstNodes<'_, '_>) -> bool {
    match parent {
        AstNodes::ForStatement(_) | AstNodes::LambdaExpression(_) => true,
        // The parentheses of a conditional's test break inside, and the
        // conditional decides as if they were absent: `(a || b) ? c : d`.
        AstNodes::ParenthesizedExpression(parenthesized) => {
            parent.is_parenthesized_conditional_test()
                && should_not_indent_if_parent_indents(parenthesized.parent())
        }
        AstNodes::ConditionalExpression(conditional) => {
            !matches!(
                conditional.parent(),
                AstNodes::ReturnStatement(_)
                    | AstNodes::ThrowStatement(_)
                    | AstNodes::YieldStatement(_)
                    | AstNodes::MethodCall(_)
                    | AstNodes::ClassInstanceCreation(_)
                    | AstNodes::QualifiedClassInstanceCreation(_)
                    | AstNodes::ConstructorInvocation(_)
                    | AstNodes::EnumConstant(_)
            ) && !is_header_condition(conditional.parent(), conditional.span)
        }
        _ => false,
    }
}

/// Whether the expression indents as a whole, so the operators of a chain
/// inside it indent past those that join it:
///
/// ```java
/// return firstOperand
///     + secondOperand
///   > limit;
/// ```
///
/// `return`, `throw`, and `yield` take the expression without parentheses of
/// their own, and a header condition's conditional indents its test with its
/// branches.
fn takes_whole_indent(parent: &AstNodes<'_, '_>) -> bool {
    match parent {
        AstNodes::ReturnStatement(_)
        | AstNodes::ThrowStatement(_)
        | AstNodes::YieldStatement(_) => true,
        AstNodes::ConditionalExpression(conditional) => {
            is_header_condition(conditional.parent(), conditional.span)
        }
        _ => false,
    }
}

/// There are cases where the parent decides to inline the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(parent: &AstNodes<'_, '_>) -> bool {
    matches!(
        parent,
        AstNodes::AssignmentExpression(_)
            | AstNodes::ElementValuePair(_)
            | AstNodes::ResourceDeclaration(_)
            | AstNodes::VariableDeclarator(_)
    )
}

/// Writes the right sides from the innermost one back up `depth` levels to the chain's root.
fn format_right_sides<'a>(
    mut current: BinaryLikeExpression<'a, '_>,
    depth: usize,
    inside_condition: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    format_right_side(current, inside_condition, f);
    for _ in 0..depth {
        let BinaryLikeExpression::Binary(binary) = current else { unreachable!() };
        // A flattened left side prints without its wrapper; the root's wrapper prints its own.
        binary.format_trailing_comments(f);
        current = match binary.parent() {
            AstNodes::BinaryExpression(parent) => BinaryLikeExpression::Binary(parent),
            _ => unreachable!(),
        };
        format_right_side(current, inside_condition, f);
    }
}

/// Writes the operator and right side of `binary`, grouped on their own unless
/// a neighbor is the same kind of binary expression: `a + b + c` breaks at
/// every operator together, while the operands of `a + b` stay together.
fn format_right_side<'a>(
    binary: BinaryLikeExpression<'a, '_>,
    inside_condition: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let operator_and_right = format_once(|f| match binary {
        BinaryLikeExpression::Binary(binary) => {
            format_operator_and_break(token(binary.op.as_str()), binary.rhs.span(), f);
            binary.rhs().fmt(f);
        }
        // The keyword is grouped with the type it tests rather than with the
        // operand: the pair moves below the operand together, and the second
        // break is spent only when the pair does not fit a line of its own.
        BinaryLikeExpression::Instanceof(instanceof) => write!(
            f,
            [
                soft_line_break_or_space(),
                format_hoisted_leading_comments(instanceof.rhs.span()),
                group(&format_args!(
                    token("instanceof"),
                    soft_line_break_or_space(),
                    instanceof.rhs()
                ))
            ]
        ),
    });

    let is_right_same_kind = match binary {
        BinaryLikeExpression::Binary(node) => binary.is_same_kind(&node.rhs),
        BinaryLikeExpression::Instanceof(_) => false,
    };
    let should_group = !(binary.has_same_kind_parent()
        || binary.is_same_kind(&binary.left())
        || is_right_same_kind
        || (inside_condition && binary.is_logical()));

    if should_group {
        // A line comment printed between the sides breaks the line before the operator.
        let left_end = binary.left().span().end;
        let right_start = binary.right_span().start;
        let should_break = f
            .comments()
            .printed_comments()
            .iter()
            .rev()
            .take_while(|comment| left_end < comment.span.start && right_start > comment.span.end)
            .any(Comment::is_line);
        write!(f, [group(&operator_and_right).should_expand(should_break)]);
    } else {
        write!(f, [operator_and_right]);
    }
}

/// `<soft break><operator><space>`, with the operand's leading comments
/// hoisted above the operator when one of them ends its line.
pub fn format_operator_and_break<'a>(
    operator: impl Format<'a, JavaFormatContext<'a>>,
    operand_span: Span,
    f: &mut JavaFormatter<'_, 'a>,
) {
    write!(
        f,
        [
            soft_line_break_or_space(),
            format_hoisted_leading_comments(operand_span),
            operator,
            space()
        ]
    );
}
