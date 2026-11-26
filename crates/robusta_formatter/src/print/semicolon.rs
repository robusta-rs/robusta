// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_formatter/src/print/semicolon.rs

use crate::formatter::prelude::*;
use crate::formatter::trivia::{FormatTrailingComments, format_documentation_comments_before};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use robusta_ast::{Comment, Expression};
use robusta_span::{GetSpan, Span};

/// The same-line comments after `content_end` that print behind the `;`
/// ending the node at `node_end`: `int x /* c */;` becomes `int x; /* c */`.
/// Only a run that ends the line moves; the trailing pass prints the other
/// comments before the `;` after it, and own-line comments stay pending for
/// the next construct's leading pass.
pub fn trailing_comments_to_move_behind_semicolon<'a>(
    f: &JavaFormatter<'_, 'a>,
    content_end: u32,
    node_end: u32,
) -> &'a [Comment] {
    let comments = f.comments();
    if !comments.has_comment_in_span(Span::new(content_end, node_end)) {
        return &[];
    }
    comments.end_of_line_comments_after(content_end)
}

/// Formats `content` followed by a `;`, printing the content's same-line
/// trailing comments behind the `;`: `int x /* c */;` becomes `int x; /* c */`.
/// A line comment among them does not break the group around the statement:
/// `while (a) x = 1 // c` + `;` prints `while (a) x = 1; // c`.
///
/// Documentation comments stay before a `;` that could precede a declaration.
/// A trailing `///` can move when `else` or a `do`'s `while` follows instead.
pub struct FormatContentWithSemicolon<'b, T> {
    content: &'b T,
    /// End position of the content; comments between it and the semicolon
    /// move behind the semicolon.
    content_end: u32,
    /// End position of the whole node, after the semicolon.
    node_end: u32,
    /// Whether documentation comments before the `;` stay before it.
    keeps_documentation_comments: bool,
}

impl<'b, T> FormatContentWithSemicolon<'b, T> {
    pub fn new(content: &'b T, content_end: u32, node_end: u32) -> Self {
        Self { content, content_end, node_end, keeps_documentation_comments: true }
    }

    /// For a `;` no declaration can follow, as one ending a module directive
    /// (JLS §7.7) or a switch rule (JLS §14.11.1): documentation comments move
    /// behind it as well.
    pub fn moving_documentation_comments(mut self) -> Self {
        self.keeps_documentation_comments = false;
        self
    }

    /// Moves a line comment between a postfix operand and its operator behind
    /// the `;`: `k // c` + `++;` prints `k++; // c`.
    pub fn with_postfix_comments(
        mut self,
        expression: &Expression<'_>,
        f: &JavaFormatter<'_, '_>,
    ) -> Self {
        if let Expression::Postfix(postfix) = expression
            && let Some(comment) = f.comments().comments_before(self.content_end).last()
            && comment.is_line()
            && f.comments().end_of_line_comments_after(postfix.operand.span().end).last()
                == Some(comment)
        {
            self.content_end = comment.span.start;
        }
        self
    }
}

impl<'a, T> Format<'a, JavaFormatContext<'a>> for FormatContentWithSemicolon<'_, T>
where
    T: Format<'a, JavaFormatContext<'a>>,
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_content_without_comments_after(self.content, self.content_end, f);
        let before_continuation = f.context().continuation_body_end == Some(self.node_end);
        let mut comments =
            trailing_comments_to_move_behind_semicolon(f, self.content_end, self.node_end);
        // Keep own-line documentation and runs with multiline blocks in place.
        let moves_markdown = before_continuation
            && comments
                .last()
                .is_some_and(|comment| comment.is_line() && comment.is_documentation())
            && !comments.iter().any(Comment::is_multiline_block)
            && f.comments().documentation_comment_run(self.content_end, self.node_end).len()
                <= comments.len();
        if self.keeps_documentation_comments && !moves_markdown {
            let content_end =
                format_documentation_comments_before(self.content_end, self.node_end, f);
            if content_end != self.content_end {
                comments =
                    trailing_comments_to_move_behind_semicolon(f, content_end, self.node_end);
            }
        }
        ";".fmt(f);
        FormatTrailingComments::Dangling(comments).fmt(f);
        if let Some(comment) = comments.last().filter(|comment| comment.is_line()) {
            f.comments().mark_line_comment_moved_behind(self.node_end, comment);
        }
    }
}
