// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/utils/conditional.rs

use robusta_ast::ConditionalExpression;
use robusta_formatter_core::write;
use robusta_span::{GetSpan, Span};

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::trivia::FormatTrailingComments;
use crate::print::FormatWrite;
use crate::print::binary_like_expression::format_operator_and_break;
use crate::source_text;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::is_header_condition;

/// Prints the comments from `start`, the end of an operand, that trail it
/// before the next operand at `end`: those up to the first that ends its line,
/// or else those before `operator`. A comment on a later line leads the next
/// operand.
fn format_trailing_comments(start: u32, end: u32, operator: u8, f: &mut JavaFormatter<'_, '_>) {
    let count = trailing_comment_count(start, end, operator, f);
    let comments = &f.comments().unprinted_comments()[..count];
    FormatTrailingComments::Comments(comments).fmt(f);
}

fn trailing_comment_count(
    mut start: u32,
    end: u32,
    operator: u8,
    f: &JavaFormatter<'_, '_>,
) -> usize {
    let comments = f.comments();
    let source = f.source_text();
    let pending = comments.unprinted_comments();
    let mut before_operator = None;
    for (index, comment) in pending.iter().enumerate() {
        if comment.span.end > end {
            return before_operator.unwrap_or(index);
        }
        if source.contains_newline_between(start, comment.span.start) {
            return index;
        }
        if comment.is_line() || comment.followed_by_newline() {
            return index + 1;
        }
        if before_operator.is_none()
            && source_text::character_end(source.slice_range(start, comment.span.start), operator)
                .is_some()
        {
            before_operator = Some(index);
        }
        start = comment.span.end;
    }
    before_operator.unwrap_or(pending.len())
}

/// Where a conditional sits, which decides how it indents. The test of a
/// conditional is never a conditional without parentheses (JLS §15.25).
#[derive(Clone, Copy)]
enum ConditionalLayout {
    /// This conditional isn't a branch of another conditional.
    ///
    /// ```java
    /// return a ? b : c;
    /// ```
    Root,
    /// Conditional that is the consequent of another conditional.
    ///
    /// ```java
    /// condition1
    ///   ? condition2
    ///     ? consequent2 // <-- consequent and alternate get indented
    ///     : alternate2
    ///   : alternate1;
    /// ```
    NestedConsequent,
    /// Conditional that is the alternate of another conditional.
    ///
    /// The test of a nested alternate is aligned with the parent's `:`.
    ///
    /// ```java
    /// outerCondition
    ///   ? consequent
    ///   : nestedAlternate
    ///       + binary // <- notice how the content is aligned to the `: `
    ///     ? consequentOfNestedAlternate
    ///     : alternateOfNestedAlternate;
    /// ```
    NestedAlternate,
}

impl<'a> AstNode<'_, 'a, ConditionalExpression<'a>> {
    fn layout(&self) -> ConditionalLayout {
        match self.parent() {
            AstNodes::ConditionalExpression(parent) if parent.when_true.span() == self.span => {
                ConditionalLayout::NestedConsequent
            }
            AstNodes::ConditionalExpression(_) => ConditionalLayout::NestedAlternate,
            _ => ConditionalLayout::Root,
        }
    }

    fn format_test(&self, f: &mut JavaFormatter<'_, 'a>) {
        let condition = self.condition();
        let condition_end = condition.span().end;
        format_content_without_comments_after(&condition, condition_end, f);
        format_trailing_comments(condition_end, self.when_true.span().start, b'?', f);
    }

    /// Writes `? consequent : alternate`, each branch aligned past its operator.
    fn format_consequent_and_alternate(&self, f: &mut JavaFormatter<'_, 'a>) {
        let when_true = self.when_true();
        let when_false = self.when_false();
        format_operator_and_break("?", when_true.span(), f);
        let when_true_end = when_true.span().end;
        format_branch(
            &format_once(|f| {
                format_content_without_comments_after(&when_true, when_true_end, f);
                format_trailing_comments(when_true_end, when_false.span().start, b':', f);
            }),
            f,
        );
        format_operator_and_break(":", when_false.span(), f);
        format_branch(&when_false, f);
    }

    fn write_root(
        &self,
        tail: impl Format<'a, JavaFormatContext<'a>>,
        f: &mut JavaFormatter<'_, 'a>,
    ) {
        let parent = self.parent();
        let is_header_condition = is_header_condition(parent, self.span);
        // As a member's object, `)` takes a line of its own when the conditional
        // breaks, after the comments that end its last operand's line:
        // ```java
        // (veryLongCondition
        //   ? a
        //   : b // comment
        // ).more
        // ```
        let closes_member_object = parent.is_parenthesized_member_object()
            && !is_header_condition
            && !should_extra_indent(parent.parent(), parent.span());
        write!(
            f,
            [group(&format_once(|f| {
                // A header condition indents its test with its branches.
                if is_header_condition {
                    wrapped_indent(
                        true,
                        &format_once(|f| {
                            self.format_test(f);
                            tail.fmt(f);
                        }),
                    )
                    .fmt(f);
                } else {
                    self.format_test(f);
                    indent(&tail).fmt(f);
                }
                if closes_member_object {
                    self.format_trailing_comments(f);
                    soft_line_break().fmt(f);
                }
            }))]
        );
    }
}

/// A branch aligns two columns past its `? ` or `: `, or indents with tabs.
fn format_branch<'a>(
    branch: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    if f.options().indent_style.is_space() {
        align(2, branch).fmt(f);
    } else {
        indent(branch).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ConditionalExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let tail = format_once(|f| self.format_consequent_and_alternate(f));
        match self.layout() {
            ConditionalLayout::Root => self.write_root(tail, f),
            // The `dedent` removes the parent's alignment of this consequent, so
            // the `indent` indents it one level past the parent's `?`.
            ConditionalLayout::NestedConsequent => {
                self.format_test(f);
                dedent(&indent(&tail)).fmt(f);
            }
            ConditionalLayout::NestedAlternate => {
                align(2, &format_once(|f| self.format_test(f))).fmt(f);
                tail.fmt(f);
            }
        }
    }
}

/// Whether the parentheses at `span` around a conditional break inside, as the
/// object of a member chain that a declaration, an assignment, `return`,
/// `throw`, `yield`, or a unary operator takes whole:
///
/// ```java
/// return (
///   a
///     ? b
///     : c
/// ).member;
/// ```
pub fn should_extra_indent(mut parent: &AstNodes<'_, '_>, mut span: Span) -> bool {
    let start = span;
    loop {
        match parent {
            AstNodes::MethodCall(call) if call.callee.span() == span => {}
            member if member.is_member_object_span(span) => {}
            _ => break,
        }
        span = parent.span();
        parent = parent.parent();
    }
    // Without a member chain there is nothing to indent.
    if span == start {
        return false;
    }
    match parent {
        AstNodes::VariableDeclarator(declarator) => {
            declarator.init.as_ref().is_some_and(|init| init.span() == span)
        }
        AstNodes::AssignmentExpression(assignment) => assignment.value.span() == span,
        AstNodes::ReturnStatement(statement) => {
            statement.value.as_ref().is_some_and(|value| value.span() == span)
        }
        AstNodes::ThrowStatement(statement) => statement.value.span() == span,
        AstNodes::YieldStatement(statement) => statement.value.span() == span,
        AstNodes::UnaryExpression(unary) => unary.operand.span() == span,
        AstNodes::UpdateExpression(update) => update.prefix && update.target.span() == span,
        _ => false,
    }
}
