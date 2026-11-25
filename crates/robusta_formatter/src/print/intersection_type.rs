use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::format_hoisted_leading_comments;
use robusta_ast::Type;
use robusta_formatter_core::write;
use robusta_span::GetSpan;

/// Bounds shared by type parameters and intersection casts.
pub fn format_type_bounds<'parent, 'a: 'parent>(
    f: &mut JavaFormatter<'_, 'a>,
    bounds: impl IntoIterator<Item = AstNode<'parent, 'a, Type<'a>>>,
) {
    group(&format_once(|f| {
        for (index, bound) in bounds.into_iter().enumerate() {
            if index == 0 {
                bound.fmt(f);
            } else {
                indent(&format_once(|f| {
                    write!(
                        f,
                        [
                            soft_line_break_or_space(),
                            format_hoisted_leading_comments(bound.span()),
                            format_with(|f| format_additional_bound(f, &bound))
                        ]
                    );
                }))
                .fmt(f);
            }
        }
    }))
    .fmt(f);
}

pub fn format_additional_bound<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    bound: &AstNode<'_, 'a, Type<'a>>,
) {
    write!(f, ["&", space(), bound]);
}
