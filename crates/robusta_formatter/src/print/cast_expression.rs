// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/as_or_satisfies_expression.rs
// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/print/mod.rs

use robusta_ast::{CastExpression, Comment};
use robusta_formatter_core::{format_args, write};
use robusta_span::GetSpan;

use crate::ast_nodes::AstNode;
use crate::formatter::comments::stays_on_operator_line;
use crate::formatter::prelude::*;
use crate::formatter::trivia::{FormatTrailingComments, format_hoisted_leading_comments};
use crate::print::FormatWrite;
use crate::print::intersection_type::format_type_bounds;
use crate::utils::assignment_like::left_trailing_comments;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CastExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let types_end = self.additional_bounds.last().map_or(self.ty.span(), GetSpan::span).end;
        let format_types = format_once(|f| {
            format_content_without_comments_after(
                &format_once(|f| {
                    format_hoisted_leading_comments(self.ty.span()).fmt(f);
                    format_type_bounds(
                        f,
                        std::iter::once(self.ty()).chain(self.additional_bounds().iter()),
                    );
                }),
                types_end,
                f,
            );
            FormatTrailingComments::Comments(left_trailing_comments(types_end, b')', f)).fmt(f);
        });
        write!(f, [token("("), group(&soft_block_indent(&format_types))]);
        let (glued, layout) = comments_after_cast(types_end, self.operand.span().start, f);
        token(")").fmt(f);
        FormatTrailingComments::Comments(glued).fmt(f);
        match layout {
            OperandLayout::Inline => write!(f, [space(), self.operand()]),
            OperandLayout::SoftBreak => group(&soft_line_indent_or_space(&self.operand())).fmt(f),
            OperandLayout::HardBreak => {
                write!(f, [indent(&format_args!(hard_line_break(), self.operand()))]);
            }
        }
    }
}

enum OperandLayout {
    Inline,
    SoftBreak,
    HardBreak,
}

/// Splits the comments after the cast's `)` and chooses the operand's layout.
fn comments_after_cast<'a>(
    types_end: u32,
    operand_start: u32,
    f: &JavaFormatter<'_, 'a>,
) -> (&'a [Comment], OperandLayout) {
    let comments = f.comments();
    let has_deferred_comment = !comments.comments_before_character(types_end, b')').is_empty();
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
