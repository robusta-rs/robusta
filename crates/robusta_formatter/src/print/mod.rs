pub mod annotations;
pub mod array_dimension;
pub mod array_element_list;
pub mod array_initializer;
mod assert_statement;
pub mod binary_like_expression;
pub mod block_statement;
pub mod call_like_expression;
mod cast_expression;
pub mod class;
pub mod import_declaration;
pub mod intersection_type;
pub mod lambda_expression;
pub mod member_expression;
pub mod method;
mod method_reference;
pub mod module_declaration;
pub mod names;
pub mod parameters;
pub mod pattern;
pub mod program;
pub mod return_or_throw_statement;
pub mod semicolon;
mod statement_expression_list;
pub mod switch_statement;
pub mod text_block;
pub mod try_statement;
pub mod type_parameters;
pub mod variable_declaration;

use crate::formatter::comments::stays_on_operator_line;
use crate::formatter::prelude::*;
use crate::formatter::separated::{argument_list_end, format_after_opening_paren_comment};
use crate::formatter::trivia::{
    FormatLeadingComments, FormatTrailingComments, format_opening_paren_comment,
    format_trailing_comments,
};
use crate::print::annotations::{format_modifiers_inline, format_type_annotations};
use crate::print::array_dimension::{
    format_array_type_dimensions, format_dimension_after, format_type_dimension,
};
use crate::print::binary_like_expression::BinaryLikeExpression;
use crate::print::call_like_expression::arguments::format_arguments;
use crate::print::member_expression::{
    format_comments_around_member_separator, format_member_separator,
};
use crate::print::return_or_throw_statement::format_keyword_expression_statement;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::text_block::format_text_block;
use crate::print::type_parameters::format_type_arguments_before_name;

use crate::ast_nodes::impls::ast_nodes::format_node_leading_comments;
use crate::ast_nodes::{AstNode, AstNodes};
use crate::utils::assignment_like::AssignmentLike;
use crate::utils::conditional::should_extra_indent;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{
    FormatStatementBody, StatementContinuation, format_continuation_keyword, format_head_body,
    write_comments_after_unbraced_body, write_comments_before_opener,
    write_comments_between_blocks, write_head_body_separator, write_node_with_terminator,
    write_trailing_comments, write_trailing_comments_before,
};
use crate::utils::{is_header_condition, keyword_end};
use robusta_ast::*;
use robusta_formatter_core::{FormatElement, FormatElements, format_args, write};
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::UnaryOp;

pub trait FormatWrite<'a> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>);

    fn leading_comments_start(&self) -> u32
    where
        Self: GetSpan,
    {
        self.span().start
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, IntLiteral<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        text_without_whitespace(self.raw).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, FloatLiteral<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        text_without_whitespace(self.raw).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CharLiteral<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        text(self.raw).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, StringLiteral<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.is_text_block() {
            format_text_block(self.raw, f);
        } else {
            text(self.raw).fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, BooleanLiteral> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        // The source spelling keeps any Unicode escapes.
        text_without_whitespace(f.source_text().slice_range(self.span.start, self.span.end)).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, NullLiteral> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        // The source spelling keeps any Unicode escapes.
        text_without_whitespace(f.source_text().slice_range(self.span.start, self.span.end)).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ThisExpression> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "this".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, Super> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "super".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedThis<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_qualified_keyword(f, &self.qualifier(), "this");
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedSuper<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_qualified_keyword(f, &self.qualifier(), "super");
    }
}

/// `Outer.this` or `Outer.super`.
pub fn format_qualified_keyword<'p, 'a, T: GetSpan>(
    f: &mut JavaFormatter<'_, 'a>,
    qualifier: &AstNode<'p, 'a, T>,
    keyword: &'static str,
) where
    AstNode<'p, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    let span = qualifier.span();
    format_content_without_comments_after(qualifier, span.end, f);
    format_dot_before_nodeless_operand(span, &keyword, f);
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ClassLiteral<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let ty = self.ty.span();
        format_content_without_comments_after(&self.ty(), ty.end, f);
        format_dot_before_nodeless_operand(ty, &"class", f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, UnaryExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.op.as_str().fmt(f);
        let operand = self.operand();
        let (glued, layout) = comments_after_prefix_operator(operand.span().start, false, f);
        // A comment ending its line keeps it, and the operand is indented
        // below it as a wrapped line.
        if matches!(layout, OperandLayout::HardBreak) {
            FormatTrailingComments::Comments(glued).fmt(f);
            let in_header = is_header_condition(self.parent(), self.span);
            wrapped_indent(in_header, &format_args!(hard_line_break(), operand)).fmt(f);
            return;
        }
        // `- -x` must keep its space: fused to `--x` it would tokenize as
        // a decrement and change meaning (same for `+ +x`, `- --x`, and
        // `+ ++x`).
        let would_fuse = if let Expression::Unary(inner) = operand.inner() {
            match self.op {
                UnaryOp::Plus => matches!(inner.op, UnaryOp::Plus | UnaryOp::PreIncrement),
                UnaryOp::Minus => matches!(inner.op, UnaryOp::Minus | UnaryOp::PreDecrement),
                _ => false,
            }
        } else {
            false
        };
        if would_fuse {
            space().fmt(f);
        }
        operand.fmt(f);
    }
}

/// Where the operand of a prefix operator starts after the comments between
/// them.
pub enum OperandLayout {
    Inline,
    SoftBreak,
    HardBreak,
}

/// Splits the comments after a prefix operator, such as `!` or a cast's `)`,
/// into those that stay on its line and those that lead the operand, and
/// chooses the operand's layout. `has_deferred_comment` says a comment before
/// the cast's `)` prints after it.
pub fn comments_after_prefix_operator<'a>(
    operand_start: u32,
    has_deferred_comment: bool,
    f: &JavaFormatter<'_, 'a>,
) -> (&'a [Comment], OperandLayout) {
    let comments = f.comments();
    let pending = comments.comments_before(operand_start);
    let glued_count = if has_deferred_comment {
        0
    } else {
        let stays = pending.iter().take_while(|comment| stays_on_operator_line(comment)).count();
        if stays == pending.len() {
            stays
        } else {
            pending[..stays]
                .iter()
                .rposition(|comment| !comment.is_multiline_block())
                .map_or(0, |last| last + 1)
        }
    };
    let (glued, leading) = pending.split_at(glued_count);
    let starts_after_break = glued.iter().any(Comment::is_line)
        || leading.iter().any(|comment| {
            comment.is_line()
                || comment.followed_by_newline()
                    && (comment.preceded_by_newline() || comment.is_multiline_block())
        })
        || has_deferred_comment && leading.iter().skip(1).any(Comment::preceded_by_newline)
        || !has_deferred_comment
            && leading.first().is_some_and(Comment::preceded_by_newline)
            && leading.iter().any(Comment::followed_by_newline);
    let layout = if starts_after_break {
        OperandLayout::HardBreak
    } else if has_deferred_comment
        && leading.last().is_some_and(|comment| {
            f.source_text().lines_after_skipping_terminators(comment.span.end, Some(operand_start))
                > 0
        })
    {
        // Break before a moved block run whenever its end also breaks.
        OperandLayout::SoftBreak
    } else {
        OperandLayout::Inline
    };
    (glued, layout)
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, PostfixExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let operand = self.operand();
        let operand_end = operand.span().end;
        if matches!(self.parent(), AstNodes::ExpressionStatement(_) | AstNodes::SwitchRule(_))
            && f.comments().has_comment_in_span(Span::new(operand_end, self.span.end))
        {
            format_content_without_comments_after(&operand, operand_end, f);
            let comments = f.comments().comments_before(self.span.end);
            write_comments_between_blocks(comments, f);
        } else {
            operand.fmt(f);
        }
        self.op.as_str().fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, BinaryExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        BinaryLikeExpression::Binary(self).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, InstanceofExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        BinaryLikeExpression::Instanceof(self).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, AssignmentExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        AssignmentLike::AssignmentExpression(self).fmt(f);
    }
}

/// The `.` after `qualifier`, then `operand`, which starts with a token the
/// AST keeps no node for: `Outer.this`, `String.class`, `outer.new Inner()`.
pub fn format_dot_before_nodeless_operand<'a>(
    qualifier: Span,
    operand: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let start =
        f.comments().token_start_after(f.comments().position_after_character(qualifier.end, b'.'));
    format_comments_around_member_separator(qualifier, start, ".", f);
    format_member_separator(".", start, f);
    operand.fmt(f);
}

/// The suffix stays outside the separator's group so a hard break within
/// the suffix does not also force a break before it.
fn format_indented_continuation<'a>(
    separator: Line,
    suffix: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    format_indented_continuation_with_mode(separator, suffix, false, f);
}

fn format_indented_continuation_with_mode<'a>(
    separator: Line,
    suffix: &impl Format<'a, JavaFormatContext<'a>>,
    measure_flat: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    format_wrapped_continuation(separator, suffix, measure_flat, false, f);
}

/// [`format_indented_continuation_with_mode`], indenting a broken suffix twice
/// when `double` is set.
fn format_wrapped_continuation<'a>(
    separator: Line,
    suffix: &impl Format<'a, JavaFormatContext<'a>>,
    measure_flat: bool,
    double: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let id = f.state().group_id("indented_continuation");
    let indented_separator = wrapped_indent(double, &separator);
    let separator = group(&indented_separator).with_group_id(Some(id));
    let separator = if measure_flat { separator.measure_flat() } else { separator };
    let suffix = indent_if_group_breaks(suffix, id);
    if double {
        write!(f, [separator, indent_if_group_breaks(&suffix, id)]);
    } else {
        write!(f, [separator, suffix]);
    }
}

/// A declared name after its type, moved to an indented line only when it
/// does not fit after the type. `name` is the name and the rest of its line,
/// such as a declarator's `= value`. Comments before the name lead it.
fn format_name_after_type<'a>(
    name_start: u32,
    name: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    format_name_after_type_with_indent(name_start, name, false, false, f);
}

/// [`format_name_after_type`], indenting a moved name twice when `double` is
/// set, so a method name does not line up with the body below it.
/// `measure_flat` is as [`format_wrapped_continuation`] takes it.
fn format_name_after_type_with_indent<'a>(
    name_start: u32,
    name: &impl Format<'a, JavaFormatContext<'a>>,
    measure_flat: bool,
    double: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let comments = f.comments().comments_before(name_start);
    if comments.iter().any(|comment| {
        comment.preceded_by_newline()
            || comment.followed_by_newline()
            || comment.is_multiline_block()
    }) {
        // A comment that starts, ends or spans a line moves the name itself.
        let separated = format_once(|f| {
            write_type_name_separator(name_start, f);
            name.fmt(f);
        });
        wrapped_indent(double, &separated).fmt(f);
    } else {
        format_wrapped_continuation(soft_line_break_or_space(), name, measure_flat, double, f);
    }
}

/// The break between a type and the name after it, inside an indent the
/// caller provides. A comment that ends the type's line keeps its place, with
/// the line break after it.
fn write_type_name_separator(name_start: u32, f: &mut JavaFormatter<'_, '_>) {
    if f.comments().has_leading_own_line_comment(name_start) {
        space().fmt(f);
    } else {
        group(&soft_line_break_or_space()).fmt(f);
    }
}

// Adapted from https://github.com/oxc-project/oxc/blob/676b7e19e5216d51efe4b12a1bacf7a9aaf2164d/crates/oxc_formatter/src/print/mod.rs
/// An expression whose enclosing parentheses bound its leading and trailing comments.
///
/// Deliberately ungrouped: the expression owns its breaks, and the parentheses
/// stay attached to its first and last tokens. A group around the parentheses
/// would expand on multiline content and introduce an unwanted delimiter break.
/// Without a group's closing break to flush trailing line comments, the closing
/// delimiter needs a line-suffix boundary so those comments stay inside it.
///
/// A cast, binary, or `instanceof` expression that is a member's object
/// breaks inside the parentheses instead, with its comments, as does a binary
/// or `instanceof` expression that is a prefix operator's operand, a
/// conditional where [`should_extra_indent`] holds, and a binary,
/// `instanceof`, or conditional expression that is a conditional's test, and
/// any expression after a line comment on the `(`'s line. Apart from a cast,
/// these keep the layout of a header's condition.
impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ParenthesizedExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let parent = self.parent();
        let is_member_object = parent.is_member_object_span(self.span);
        let breaks_inside = match self.expression {
            Expression::Cast(_) => is_member_object,
            Expression::Binary(_) | Expression::Instanceof(_) => {
                ((is_member_object || self.as_ast_nodes().is_parenthesized_unary_operand())
                    && !is_header_condition(parent, self.span))
                    || self.as_ast_nodes().is_parenthesized_conditional_test()
            }
            Expression::Conditional(_) => {
                (is_member_object
                    && !is_header_condition(parent, self.span)
                    && should_extra_indent(parent, self.span))
                    || self.as_ast_nodes().is_parenthesized_conditional_test()
            }
            _ => false,
        };
        "(".fmt(f);
        let expression = self.expression();
        if format_after_opening_paren_comment(f, self.expression.span().start, &expression) {
            ")".fmt(f);
        } else if breaks_inside {
            write!(f, [group(&soft_block_indent(&expression)), ")"]);
        } else {
            write!(f, [expression, line_suffix_boundary(), ")"]);
        }
    }
}

/// A header's `(condition)` from `header_start`.
///
/// A condition that can break keeps the parentheses on its first and last
/// tokens. One that cannot breaks inside them instead, as does one after a
/// line comment on the `(`'s line.
pub fn format_header_condition<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    header_start: u32,
    condition: &AstNode<'_, 'ast, Expression<'ast>>,
) {
    let condition_span = condition.span();
    let close_end = f.comments().position_after_character(condition_span.end, b')');
    let trailing = format_trailing_comments(Span::new(header_start, close_end), condition_span, 0);
    "(".fmt(f);
    let after_comment = format_once(|f| {
        format_node_leading_comments(condition.parent(), condition_span.start, f);
        format_content_without_comments_after(condition, condition_span.end, f);
        trailing.fmt(f);
    });
    if format_after_opening_paren_comment(f, condition_span.start, &after_comment) {
        ")".fmt(f);
        return;
    }
    // The leading comments are formatted apart, so their breaks do not count
    // as the condition's.
    let leading = f.format_to_vec(|f| {
        format_node_leading_comments(condition.parent(), condition_span.start, f);
    });
    let formatted = f
        .format_to_vec(|f| format_content_without_comments_after(condition, condition_span.end, f));
    let can_break = may_break(&formatted);
    let content = format_once(move |f| {
        for element in leading.into_iter().chain(formatted) {
            f.write_element(element);
        }
        trailing.fmt(f);
    });
    if can_break {
        write!(f, [content, line_suffix_boundary(), ")"]);
    } else {
        write!(f, [group(&soft_block_indent(&content)), ")"]);
    }
}

/// Whether `elements` hold a line break or a choice of layouts, such as
/// grouped arguments, whose expanded variants break.
fn may_break(elements: &[FormatElement<'_>]) -> bool {
    elements.may_directly_break()
        || elements.iter().any(|element| match element {
            FormatElement::BestFitting(_) => true,
            FormatElement::Interned(interned) => may_break(interned),
            _ => false,
        })
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EmptyStatement> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        ";".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ConstructorInvocation<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let keyword = node.kind.as_str();
        // Only trivia separates the keyword from this position.
        let before_keyword = match (&node.type_args, &node.qualifier) {
            (Some(type_args), _) => type_args.span.end,
            (None, Some(qualifier)) => {
                f.comments().position_after_character(qualifier.span().end, b'.')
            }
            (None, None) => node.span.start,
        };
        // `this` and `super` each end with their only `s` or `r`.
        let keyword_end = f
            .comments()
            .position_after_character(before_keyword, *keyword.as_bytes().last().unwrap());
        let parent = self.as_ast_nodes();
        let close_end = argument_list_end(f, &node.args, keyword_end);
        let invocation = format_with(|f| {
            if let Some(type_args) = self.type_args() {
                format_type_arguments_before_name(f, &type_args);
                let keyword_start =
                    f.comments().position_after_character(before_keyword, keyword.as_bytes()[0])
                        - 1;
                format_node_leading_comments(&parent, keyword_start, f);
            }
            keyword.fmt(f);
        });
        let content = format_with(|f| {
            if let Some(qualifier) = &node.qualifier {
                let span = qualifier.span();
                format_content_without_comments_after(
                    &self.qualifier().expect("qualifier"),
                    span.end,
                    f,
                );
                format_dot_before_nodeless_operand(span, &invocation, f);
            } else {
                invocation.fmt(f);
            }
            write_comments_before_opener(keyword_end, b'(', f);
            format_arguments(f, &self.args(), close_end);
        });
        FormatContentWithSemicolon::new(&content, close_end, node.span.end).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ExpressionStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let expression = self.expression();
        let end = expression.span().end;
        FormatContentWithSemicolon::new(&expression, end, self.span.end)
            .with_postfix_comments(expression.inner(), f)
            .fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, WhileStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let content = format_once(|f| {
            "while".fmt(f);
            space().fmt(f);
            format_header_condition(f, node.span.start, &self.condition());
            FormatStatementBody::new(&self.body()).fmt(f);
        });
        group(&content).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, DoStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let body_end = node.body.span().end;
        // Lexical scan for the keyword's first byte: the gap holds only trivia and `while`
        let while_start = f.comments().position_after_character(body_end, b'w') - 1;
        let body = self.body();
        let format_body = FormatStatementBody::new(&body);
        if matches!(node.body, Statement::Block(_)) {
            // The block's trailing pass would claim a comment past the `while`
            // keyword, so the keyword site splits the comments instead.
            "do".fmt(f);
            format_content_without_comments_after(&format_body, body_end, f);
            let comments = f.comments().comments_before_character(body_end, b'w');
            if write_comments_between_blocks(comments, f) {
                space().fmt(f);
            }
        } else {
            let content = format_once(|f| {
                "do".fmt(f);
                format_body
                    .format_before_continuation(StatementContinuation::DoWhile(while_start), f);
            });
            group(&content).fmt(f);
            write_comments_after_unbraced_body(body_end, while_start, f);
        }
        let content = format_with(|f| {
            "while".fmt(f);
            space().fmt(f);
            format_header_condition(f, while_start, &self.condition());
        });
        let close_end = f.comments().position_after_character(node.condition.span().end, b')');
        FormatContentWithSemicolon::new(&content, close_end, node.span.end).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EnhancedForStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let dimensions = self.dims();
        // Own-line comments between the variable and the `:` lead the statement.
        // Those after the `:` stay with the iterable, which wraps after it.
        let variable_end =
            node.dims.last().map_or(node.name.span.end, |dimension| dimension.span.end);
        let colon_start = f.comments().position_after_character(variable_end, b':') - 1;
        let comments = f.comments().own_line_comments_before(colon_start);
        if comments.first().is_some_and(|comment| comment.span.start > variable_end) {
            FormatLeadingComments::Comments(comments).fmt(f);
        }
        let variable_start = node
            .modifiers
            .first()
            .map_or_else(|| node.ty.span().start, |modifier| modifier.span().start);
        let head = format_once(|f| {
            format_modifiers_inline(f, &self.modifiers());
            format_content_without_comments_after(&self.ty(), node.ty.span().end, f);
            wrapped_indent(
                true,
                &format_once(|f| {
                    write_type_name_separator(node.name.span.start, f);
                    let name = self.name();
                    if dimensions.is_empty() {
                        name.fmt(f);
                    } else {
                        format_content_without_comments_after(&name, name.span.end, f);
                    }
                }),
            )
            .fmt(f);
            group(&format_once(|f| {
                if !dimensions.is_empty() {
                    format_array_type_dimensions(f, node.name.span.end, &dimensions);
                    write_comments_before_opener(variable_end, b':', f);
                }
                write!(f, [space(), ":"]);
                // The iterable wraps inside the `for` parentheses, as dart format wraps an `if`.
                wrapped_indent(
                    true,
                    &format_once(|f| {
                        soft_line_break_or_space().fmt(f);
                        format_content_without_comments_after(
                            &self.iterable(),
                            node.iterable.span().end,
                            f,
                        );
                    }),
                )
                .fmt(f);
            }))
            .fmt(f);
            // The head is ungrouped, so a line suffix from these comments
            // flushes at the boundary before the `)`.
            let iterable_span = node.iterable.span();
            let close_end = f.comments().position_after_character(iterable_span.end, b')');
            format_trailing_comments(Span::new(node.span.start, close_end), iterable_span, 0)
                .fmt(f);
        });
        let content = format_once(|f| {
            write!(f, ["for", space(), "("]);
            if format_after_opening_paren_comment(f, variable_start, &head) {
                ")".fmt(f);
            } else {
                write!(f, [head, line_suffix_boundary(), ")"]);
            }
            FormatStatementBody::new(&self.body()).fmt(f);
        });
        group(&content).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, BreakStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if let Some(label) = self.label() {
            let content = format_with(|f| write!(f, ["break", space(), label]));
            FormatContentWithSemicolon::new(&content, label.span.end, self.span.end).fmt(f);
        } else {
            let end = keyword_end(f, self.span.start, "break");
            FormatContentWithSemicolon::new(&"break", end, self.span.end).fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ContinueStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if let Some(label) = self.label() {
            let content = format_with(|f| write!(f, ["continue", space(), label]));
            FormatContentWithSemicolon::new(&content, label.span.end, self.span.end).fmt(f);
        } else {
            let end = keyword_end(f, self.span.start, "continue");
            FormatContentWithSemicolon::new(&"continue", end, self.span.end).fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SynchronizedStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        "synchronized".fmt(f);
        space().fmt(f);
        format_header_condition(f, node.span.start, &self.lock());
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, YieldStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_keyword_expression_statement(f, "yield", self, Some(&self.value()));
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, LabeledStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        write_node_with_terminator(&self.label(), ":", f);
        write_head_body_separator(node.statement.span().start, f);
        hard_line_break().fmt(f);
        self.statement().fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, IfStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let then_end = node.then_branch.span().end;
        let content = format_once(|f| {
            "if".fmt(f);
            space().fmt(f);
            format_header_condition(f, node.span.start, &self.condition());
            let then_branch = self.then_branch();
            let consequent = FormatStatementBody::new(&then_branch);
            if node.else_branch.is_some() {
                consequent.format_before_continuation(StatementContinuation::Else, f);
            } else {
                consequent.fmt(f);
            }
        });
        group(&content).fmt(f);

        let Some(else_branch) = self.else_branch() else {
            return;
        };
        if matches!(node.then_branch, Statement::Block(_)) {
            format_continuation_keyword(f, "else", then_end);
        } else {
            // An unbraced then-branch already ended its line, so the keyword breaks
            // regardless of the configured placement.
            let else_start = f.comments().position_after_character(then_end, b'e') - 1;
            write_comments_after_unbraced_body(then_end, else_start, f);
            "else".fmt(f);
        }
        let is_else_if = matches!(else_branch.inner(), Statement::If(_));
        let alternate = FormatStatementBody::new(&else_branch).with_forced_space(is_else_if);
        group(&alternate).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ForStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let is_headless = node.init.is_none() && node.condition.is_none() && node.update.is_none();
        let content = format_once(|f| {
            "for".fmt(f);
            let for_end = keyword_end(f, node.span.start, "for");
            let open_end = f.comments().position_after_character(for_end, b'(');
            space().fmt(f);
            "(".fmt(f);
            let init_start = self.init().map_or_else(
                || f.comments().position_after_character(open_end, b';') - 1,
                |init| init.span().start,
            );
            let comments = format_with(|f| {
                format_opening_paren_comment(init_start, f);
            });
            let clauses = format_once(|f| {
                // Each clause keeps the comments before its closing `;` or `)`. Those
                // after the `)` lead the body.
                let init_end = if let Some(init) = self.init() {
                    let end = init.span().end;
                    format_content_without_comments_after(&init, end, f);
                    end
                } else {
                    // Nothing precedes these comments, so they print in place after the
                    // `(`, and a line comment among them ends its own line.
                    let comments = f.comments().comments_before_character(open_end, b';');
                    let _line_comment_riding = write_trailing_comments(comments, true, f);
                    open_end
                };
                let (mut cursor, line_comment_riding) = write_for_head_slot(init_end, f);
                if !is_headless {
                    // A second pass reads a comment trailing the `;` before an empty
                    // condition's `;`, where it prints against that `;`. After a riding
                    // line comment, such comments start the next line, and the `;`
                    // follows a block comment there directly.
                    let trailing = f
                        .comments()
                        .printed_comments()
                        .last()
                        .filter(|comment| comment.span.start >= cursor);
                    match trailing {
                        Some(comment) if node.condition.is_none() => {
                            if !(line_comment_riding && comment.is_block()) {
                                soft_line_break().fmt(f);
                            }
                        }
                        _ => soft_line_break_or_space().fmt(f),
                    }
                }
                let condition_end = self.condition().map_or(cursor, |condition| {
                    format_content_without_comments_after(&condition, condition.span().end, f);
                    condition.span().end
                });
                (cursor, _) = write_for_head_slot(condition_end, f);
                if let Some(update) = self.update() {
                    soft_line_break_or_space().fmt(f);
                    format_content_without_comments_after(&update, update.span.end, f);
                    let close_end = f.comments().position_after_character(update.span.end, b')');
                    let header = Span::new(node.span.start, close_end);
                    format_trailing_comments(header, update.span, 0).fmt(f);
                } else {
                    // A line comment flushes at the head's closing break, before the `)`.
                    let _ = write_trailing_comments_before(cursor, b')', f);
                }
            });
            write!(f, [group(&format_args!(comments, soft_block_indent(&clauses))), ")"]);
            FormatStatementBody::new(&self.body()).fmt(f);
        });
        group(&content).fmt(f);
    }
}

/// A basic `for` header clause closed by a `;`, after its content ends at
/// `anchor`: the comments before the `;`, the `;`, and the comments that end
/// its line. Returns where the `;` ends, and whether a line comment rides past
/// it, which puts the comments ending its line at the start of the next one.
fn write_for_head_slot(anchor: u32, f: &mut JavaFormatter<'_, '_>) -> (u32, bool) {
    // A line comment riding past the `;` breaks the head group, so a break
    // follows the `;`.
    let line_comment_riding = write_trailing_comments_before(anchor, b';', f);
    ";".fmt(f);
    let end = f.comments().position_after_character(anchor, b';');
    FormatTrailingComments::Comments(f.comments().end_of_line_comments_after(end)).fmt(f);
    (end, line_comment_riding)
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, PrimitiveType<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_annotations(f, &self.annotations());
        // The keyword prints without Unicode escapes, as other keywords do.
        self.kind.as_str().fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, VoidType> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "void".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, VarType> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "var".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayType<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let element = self.element();
        let element_end = element.span().end;
        format_content_without_comments_after(&element, element_end, f);
        let dimension = format_with(|f| {
            format_type_dimension(f, &self.annotations(), element_end, self.span.end);
        });
        format_dimension_after(f, element_end, !self.annotations.is_empty(), &dimension);
    }
}
