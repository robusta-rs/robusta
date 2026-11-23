use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::print::FormatWrite;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::utils::keyword_end;
use robusta_ast::{Expression, ReturnStatement, ThrowStatement};
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ReturnStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_keyword_expression_statement(f, "return", self, self.value().as_ref());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ThrowStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_keyword_expression_statement(f, "throw", self, Some(&self.value()));
    }
}

/// Formats a `return`, `throw`, or `yield` statement. The same-line comments
/// before its `;` print after it as the statement's dangling comments, so a
/// line comment among them does not break the group around the statement.
pub fn format_keyword_expression_statement<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    keyword: &'static str,
    statement: &AstNode<'_, 'ast, impl GetSpan>,
    value: Option<&AstNode<'_, 'ast, Expression<'ast>>>,
) {
    let span = statement.span();
    let content = format_with(|f| {
        token(keyword).fmt(f);
        if let Some(value) = value {
            space().fmt(f);
            value.fmt(f);
        }
    });
    let content_end =
        value.map_or_else(|| keyword_end(f, span.start, keyword), |value| value.span().end);
    let format =
        FormatContentWithSemicolon::new(&content, content_end, span.end).with_dangling_comments();
    // The `;` of a switch rule's `throw` ends the rule (JLS 14.11.1).
    if matches!(statement.parent(), AstNodes::SwitchRule(_)) {
        format.moving_documentation_comments().fmt(f);
    } else {
        format.fmt(f);
    }
}
