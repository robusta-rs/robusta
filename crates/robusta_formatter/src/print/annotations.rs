use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::format_leading_comments;
use crate::print::FormatWrite;
use crate::print::array_element_list::can_concisely_print_array_list;
use crate::print::array_initializer::FormatArrayInitializer;
use crate::print::call_like_expression::arguments::{
    Argument, can_group_array_argument, can_group_expression_argument, format_arguments,
    has_comment_around_last_argument,
};
use crate::print::lambda_expression::GroupedCallArgumentLayout;
use crate::utils::assignment_like::AssignmentLike;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::format_after_comments_before_opener;
use robusta_allocator::Vec;
use robusta_ast::{
    Annotation, AnnotationArgument, ElementValueArrayInitializer, ElementValuePair,
    KeywordModifier, LambdaExpression, Modifier,
};
use robusta_formatter_core::write;
use robusta_span::{GetSpan, Span};

pub fn format_modifiers_inline<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    modifiers: &AstNode<'_, 'ast, Vec<'ast, Modifier<'ast>>>,
) {
    format_inline_modifiers(f, modifiers.iter());
}

fn format_inline_modifiers<'parent, 'ast: 'parent>(
    f: &mut JavaFormatter<'_, 'ast>,
    modifiers: impl IntoIterator<Item = AstNode<'parent, 'ast, Modifier<'ast>>>,
) {
    let mut modifiers = modifiers.into_iter().peekable();
    while let Some(modifier) = modifiers.next() {
        if modifiers.peek().is_none() && modifier.as_annotation().is_some() {
            format_annotation_prefix(f, [modifier], None);
        } else {
            format_inline_nodes(f, [modifier]);
        }
    }
}

/// Print `nodes` through their generated passes, each followed by a space, or
/// by a line break when its trailing pass printed a comment that rides a line
/// suffix, a line comment or one starting a later line, which would otherwise
/// print past the code that follows. A `line_suffix_boundary` would fail the
/// fit measurement of an annotation's arguments and expand them.
pub fn format_inline_nodes<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    nodes: impl IntoIterator<Item = impl Format<'ast, JavaFormatContext<'ast>> + GetSpan>,
) {
    for node in nodes {
        let printed = f.comments().printed_comments().len();
        node.fmt(f);
        let end = node.span().end;
        // A comment hoisted ahead of the statement was printed before the node.
        if f.comments().printed_comments()[printed..].iter().any(|comment| {
            comment.span.start >= end && (comment.is_line() || comment.preceded_by_newline())
        }) {
            hard_line_break().fmt(f);
        } else {
            space().fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, KeywordModifier> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        text_without_whitespace(self.kind.as_str()).fmt(f);
    }
}

/// Type-use annotations before what they annotate.
pub fn format_type_annotations<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    annotations: &AstNode<'_, 'a, Vec<'a, Annotation<'a>>>,
) {
    format_annotation_prefix(f, annotations.iter(), None);
}

pub fn format_modifiers_stacked<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    modifiers: &AstNode<'_, 'ast, Vec<'ast, Modifier<'ast>>>,
) {
    let lead = leading_annotation_count(modifiers);
    format_annotation_prefix(f, modifiers.iter().take(lead), Some(hard_line_break()));
    format_inline_modifiers(f, modifiers.iter().skip(lead));
}

pub fn format_annotations_stacked<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    annotations: &AstNode<'_, 'ast, Vec<'ast, Annotation<'ast>>>,
) {
    format_annotation_prefix(f, annotations.iter(), Some(hard_line_break()));
}

fn leading_annotation_count(modifiers: &[Modifier<'_>]) -> usize {
    modifiers.iter().take_while(|modifier| matches!(modifier, Modifier::Annotation(_))).count()
}

/// Format the modifiers of a field or local variable. Its leading annotations
/// form a group of their own, which puts each on its own line when one is
/// followed by a line break in the source.
pub fn format_modifiers_grouped<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    modifiers: &AstNode<'_, 'ast, Vec<'ast, Modifier<'ast>>>,
) {
    let lead = leading_annotation_count(modifiers);
    if lead > 0 {
        let source = f.source_text();
        let should_expand =
            modifiers.iter().take(lead).any(|annotation| source.ends_line(annotation.span().end));
        write!(
            f,
            [group(&format_with(|f| {
                format_annotation_prefix(
                    f,
                    modifiers.iter().take(lead),
                    Some(soft_line_break_or_space()),
                );
            }))
            .should_expand(should_expand)]
        );
    }
    format_inline_modifiers(f, modifiers.iter().skip(lead));
}

/// Annotations before the rest of a declaration, each printing its own
/// comments and followed by `separator`.
///
/// A comment that starts a line after the last annotation leads the code
/// after it, which stays on the comment's line, with no blank line before it:
///
/// ```java
/// @Deprecated
/// /* package */ void f() {}
/// ```
fn format_annotation_prefix<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    annotations: impl IntoIterator<Item = impl Format<'ast, JavaFormatContext<'ast>> + GetSpan>,
    separator: Option<Line>,
) {
    let mut annotations = annotations.into_iter().peekable();
    while let Some(annotation) = annotations.next() {
        if annotations.peek().is_none()
            && let Some((comment_start, code_start)) =
                comment_starting_line_after(annotation.span().end, f)
        {
            format_content_without_comments_after(&annotation, comment_start, f);
            hard_line_break().fmt(f);
            format_leading_comments(Span::empty(code_start)).fmt(f);
        } else if let Some(separator) = separator {
            write!(f, [annotation, separator]);
        } else {
            format_inline_nodes(f, [annotation]);
        }
    }
}

/// The start of the first pending comment that starts a line between `end`
/// and the next token, and that token's start.
fn comment_starting_line_after(end: u32, f: &JavaFormatter<'_, '_>) -> Option<(u32, u32)> {
    let code_start = f.comments().token_start_after(end);
    f.comments()
        .comments_before(code_start)
        .iter()
        .find(|comment| comment.span.start >= end && comment.preceded_by_newline())
        .map(|comment| (comment.span.start, code_start))
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, Annotation<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        token("@").fmt(f);
        let name = self.name();
        format_content_without_comments_after(&name, name.span().end, f);
        if let Some(args) = self.args() {
            let arguments = format_with(|f| format_arguments(f, &args, self.span.end));
            format_after_comments_before_opener(name.span().end, b'(', &arguments, f);
        }
    }
}

impl<'a> Argument<'a> for AnnotationArgument<'a> {
    /// An annotation's values are constants, never functions.
    fn is_function_composition(_arguments: &[Self]) -> bool {
        false
    }

    /// Only a single-element annotation's value can group, as a call's lone
    /// argument does: a pair groups as little as an assignment, and pairs are
    /// the only entries of a longer list. An array of values groups as an
    /// array creation's initializer does.
    fn grouped_layout(
        arguments: &AstNode<'_, 'a, Vec<'a, Self>>,
        f: &JavaFormatter<'_, 'a>,
    ) -> Option<GroupedCallArgumentLayout> {
        let [value] = &arguments[..] else {
            return None;
        };
        let can_group = match value {
            AnnotationArgument::Array(array) => {
                can_group_array_argument(array.span, array.values.is_empty(), f)
            }
            value => {
                value.as_expression().is_some_and(|value| can_group_expression_argument(value, f))
            }
        };
        (can_group && !has_comment_around_last_argument(None, value.span(), f))
            .then_some(GroupedCallArgumentLayout::GroupedLastArgument)
    }

    fn as_lambda<'p>(
        argument: &AstNode<'p, 'a, Self>,
    ) -> Option<AstNode<'p, 'a, LambdaExpression<'a>>> {
        argument.as_lambda()
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ElementValuePair<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        AssignmentLike::ElementValuePair(self).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ElementValueArrayInitializer<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let values = self.values();
        let fill = can_concisely_print_array_list(
            self.span,
            values.iter().map(|value| value.as_expression()),
            f,
        );
        FormatArrayInitializer::new(&values, fill).fmt(f);
    }
}
