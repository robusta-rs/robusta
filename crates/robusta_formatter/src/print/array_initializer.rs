// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/print/array_expression.rs

use robusta_allocator::Vec;
use robusta_ast::ArrayInitializer;
use robusta_formatter_core::write;

use crate::ast_nodes::{AstNode, AstNodeListElement};
use crate::formatter::prelude::*;
use crate::formatter::trivia::format_dangling_comments;
use crate::print::FormatWrite;
use crate::print::array_element_list::{ArrayElementList, can_concisely_print_array_list};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayInitializer<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let elements = self.elements();
        let fill = can_concisely_print_array_list(
            self.span,
            elements.iter().map(|element| element.as_expression()),
            f,
        );
        FormatArrayInitializer::new(&elements, fill).fmt(f);
    }
}

pub struct FormatArrayInitializer<'a, 'b, T> {
    elements: &'b AstNode<'b, 'a, Vec<'a, T>>,
    fill: bool,
}

impl<'a, 'b, T> FormatArrayInitializer<'a, 'b, T> {
    pub fn new(elements: &'b AstNode<'b, 'a, Vec<'a, T>>, fill: bool) -> Self {
        Self { elements, fill }
    }
}

impl<'a, 'b, T: AstNodeListElement> Format<'a, JavaFormatContext<'a>>
    for FormatArrayInitializer<'a, 'b, T>
where
    AstNode<'b, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.elements.is_empty() {
            write!(
                f,
                [
                    token("{"),
                    format_dangling_comments(self.elements.parent().span())
                        .with_soft_block_indent(),
                    token("}")
                ]
            );
            return;
        }
        token("{").fmt(f);
        let group_id = f.state().group_id("array");
        let elements = ArrayElementList::new(self.elements, group_id, self.fill);
        group(&format_once(|f| {
            indent(&elements).fmt(f);
            soft_line_break().fmt(f);
        }))
        .with_group_id(Some(group_id))
        .fmt(f);
        token("}").fmt(f);
    }
}
