use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::format_hoisted_leading_comments;
use crate::print::FormatWrite;
use robusta_ast::UnionType;
use robusta_formatter_core::write;
use robusta_span::GetSpan;

/// The alternatives break where the catch parameter that holds them wraps.
impl<'a> FormatWrite<'a> for AstNode<'_, 'a, UnionType<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        for (index, ty) in self.types().iter().enumerate() {
            if index > 0 {
                write!(
                    f,
                    [
                        soft_line_break_or_space(),
                        format_hoisted_leading_comments(ty.span()),
                        "|",
                        space()
                    ]
                );
            }
            ty.fmt(f);
        }
    }
}
