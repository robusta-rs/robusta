// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/print/sequence_expression.rs

use robusta_ast::StatementExpressionList;
use robusta_formatter_core::write;

use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::print::FormatWrite;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, StatementExpressionList<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let mut expressions = self.expressions().iter();
        if self.expressions.len() == 1 {
            write!(f, [expressions.next()]);
            return;
        }

        let format_inner = format_once(|f| {
            let separator = format_with(|f| {
                write!(f, [token(","), line_suffix_boundary(), soft_line_break_or_space()]);
            });
            write!(f, [expressions.next(), token(","), line_suffix_boundary()]);
            // The list only appears in a `for` header, where a broken list indents the
            // expressions after the first.
            let rest = format_once(|f| {
                write!(f, soft_line_break_or_space());
                f.join_with(separator).entries(expressions);
            });
            write!(f, [indent(&rest)]);
        });

        write!(f, group(&format_inner));
    }
}
