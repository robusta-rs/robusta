use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::separated::format_delimited_frame;
use crate::formatter::trivia::format_dangling_comments;
use crate::print::annotations::format_modifiers_inline;
use crate::print::{FormatWrite, format_name_after_type};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::write_comments_before_opener;
use robusta_ast::{MatchAllPattern, RecordPattern, TypePattern};
use robusta_formatter_core::{format_args, write};
use robusta_span::GetSpan;
use robusta_span::Span;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, TypePattern<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_modifiers_inline(f, &self.modifiers());
        format_content_without_comments_after(&self.ty(), self.ty.span().end, f);
        format_name_after_type(self.name.span.start, &self.name(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, RecordPattern<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_content_without_comments_after(&self.ty(), self.ty.span().end, f);
        let ty_end = self.ty.span().end;
        write_comments_before_opener(ty_end, b'(', f);
        if self.components.is_empty() {
            let open_end = f.comments().position_after_character(ty_end, b'(');
            write!(
                f,
                [
                    token("("),
                    format_dangling_comments(Span::new(open_end, self.span.end))
                        .with_soft_block_indent(),
                    token(")")
                ]
            );
            return;
        }
        format_delimited_frame(f, "(", ")", None, |f| {
            f.join_with(&format_args!(token(","), soft_line_break_or_space()))
                .entries(self.components().iter());
        });
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MatchAllPattern> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        token("_").fmt(f);
    }
}
