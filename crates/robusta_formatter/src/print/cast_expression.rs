// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/as_or_satisfies_expression.rs
// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/print/mod.rs

use robusta_ast::CastExpression;
use robusta_formatter_core::{format_args, write};
use robusta_span::GetSpan;

use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::{FormatTrailingComments, format_hoisted_leading_comments};
use crate::print::intersection_type::format_type_bounds;
use crate::print::{FormatWrite, OperandLayout, comments_after_prefix_operator};
use crate::utils::assignment_like::left_trailing_comments;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::is_header_condition;

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
        write!(f, ["(", group(&soft_block_indent(&format_types))]);
        let has_deferred_comment =
            !f.comments().comments_before_character(types_end, b')').is_empty();
        let (glued, layout) =
            comments_after_prefix_operator(self.operand.span().start, has_deferred_comment, f);
        ")".fmt(f);
        FormatTrailingComments::Comments(glued).fmt(f);
        match layout {
            OperandLayout::Inline => write!(f, [space(), self.operand()]),
            OperandLayout::SoftBreak => group(&soft_line_indent_or_space(&self.operand())).fmt(f),
            OperandLayout::HardBreak => {
                let in_header = is_header_condition(self.parent(), self.span);
                wrapped_indent(in_header, &format_args!(hard_line_break(), self.operand())).fmt(f);
            }
        }
    }
}
