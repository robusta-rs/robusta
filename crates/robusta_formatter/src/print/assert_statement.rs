use robusta_ast::{AssertStatement, Comment, Expression};
use robusta_span::GetSpan;

use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::{FormatWrite, format_indented_continuation};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::write_trailing_comments_before;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, AssertStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let content = format_with(|f| {
            token("assert").fmt(f);
            space().fmt(f);
            let condition_end = node.condition.span().end;
            if let Some(message) = self.message() {
                format_content_without_comments_after(&self.condition(), condition_end, f);
                format_message(&message, condition_end, f);
            } else {
                self.condition().fmt(f);
            }
        });
        let content_end = node.message.as_ref().unwrap_or(&node.condition).span().end;
        FormatContentWithSemicolon::new(&content, content_end, node.span.end).fmt(f);
    }
}

/// Format the `:` and the message after the condition ending at `condition_end`.
fn format_message<'a>(
    message: &AstNode<'_, 'a, Expression<'a>>,
    condition_end: u32,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let needs_flush = write_trailing_comments_before(condition_end, b':', f);
    let colon_end = f.comments().position_after_character(condition_end, b':');
    // A line comment that rides past the colon keeps that layout on the next pass.
    // Comments the condition deferred are still pending: nothing rides, all lead
    // the message in order.
    let comments = f.comments();
    let colon_has_line_comment = !comments.has_comment_before(colon_end)
        && comments.end_of_line_comments_after(colon_end).last().is_some_and(Comment::is_line);
    let detail = format_once(|f| {
        token(":").fmt(f);
        if needs_flush {
            hard_line_break().fmt(f);
        }
        space().fmt(f);
        message.fmt(f);
    });
    if needs_flush || colon_has_line_comment {
        space().fmt(f);
        detail.fmt(f);
    } else {
        format_indented_continuation(soft_line_break_or_space(), &detail, f);
    }
}
