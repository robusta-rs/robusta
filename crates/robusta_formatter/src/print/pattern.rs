use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::separated::{format_empty_parentheses, format_parenthesized_frame};
use crate::print::annotations::format_modifiers_inline;
use crate::print::{FormatWrite, format_name_after_type};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::write_comments_before_opener;
use robusta_ast::{MatchAllPattern, RecordPattern, TypePattern};
use robusta_formatter_core::format_args;
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
        let Some(first) = self.components.first() else {
            let open_end = f.comments().position_after_character(ty_end, b'(');
            format_empty_parentheses(f, Span::new(open_end, self.span.end));
            return;
        };
        format_parenthesized_frame(f, first.span().start, None, |f| {
            f.join_with(&format_args!(",", soft_line_break_or_space()))
                .entries(self.components().iter());
        });
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MatchAllPattern> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "_".fmt(f);
    }
}
