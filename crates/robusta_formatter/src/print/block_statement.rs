use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::print::FormatWrite;
use crate::utils::statement_body::format_braced_body;
use robusta_ast::Block;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, Block<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        format_braced_body(f, node.statements.is_empty(), node.span, |f| {
            f.join_nodes_with_hardline().entries(self.statements().iter());
        });
    }
}
