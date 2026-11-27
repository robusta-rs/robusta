pub mod assignment_like;
pub mod conditional;
pub mod expression;
pub mod format_node_without_trailing_comments;
pub mod member_chain;
pub mod statement_body;

use crate::ast_nodes::AstNodes;
use crate::formatter::prelude::*;
use robusta_span::{GetSpan, Span};

/// Where a keyword the AST keeps no span for ends, including Unicode escapes
/// in its spelling.
pub fn keyword_end(f: &JavaFormatter<'_, '_>, keyword_start: u32, keyword: &str) -> u32 {
    f.source_text().characters_end(keyword_start, keyword.len())
}

/// Whether the expression at `span` is what the parentheses of an `if`,
/// `while`, `do`, `switch`, or `synchronized` header hold, where wrapped lines
/// follow dart format.
///
/// Parentheses, prefix operators, casts, and the member accesses after an
/// object add no indent, so `!(a && b)` and the `a.b()` of `a.b().c` count.
pub fn is_header_condition(parent: &AstNodes<'_, '_>, span: Span) -> bool {
    match parent {
        AstNodes::IfStatement(node) => node.condition.span() == span,
        AstNodes::WhileStatement(node) => node.condition.span() == span,
        AstNodes::DoStatement(node) => node.condition.span() == span,
        AstNodes::SwitchStatement(node) => node.selector.span() == span,
        AstNodes::SwitchExpression(node) => node.selector.span() == span,
        AstNodes::SynchronizedStatement(node) => node.lock.span() == span,
        AstNodes::FieldAccess(node) => {
            node.target.span() == span && is_header_condition(parent.parent(), parent.span())
        }
        AstNodes::ArrayAccess(node) => {
            node.array.span() == span && is_header_condition(parent.parent(), parent.span())
        }
        AstNodes::ParenthesizedExpression(_)
        | AstNodes::UnaryExpression(_)
        | AstNodes::CastExpression(_) => is_header_condition(parent.parent(), parent.span()),
        AstNodes::UpdateExpression(update) if update.prefix => {
            is_header_condition(parent.parent(), parent.span())
        }
        _ => false,
    }
}
