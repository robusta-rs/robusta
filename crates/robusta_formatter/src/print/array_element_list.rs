// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/print/array_element_list.rs

use robusta_allocator::Vec;
use robusta_ast::Expression;
use robusta_formatter_core::GroupId;
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::UnaryOp;

use crate::ast_nodes::{AstNode, AstNodeListElement};
use crate::formatter::prelude::*;

pub struct ArrayElementList<'a, 'b, T> {
    elements: &'b AstNode<'b, 'a, Vec<'a, T>>,
    group_id: GroupId,
    fill: bool,
}

impl<'a, 'b, T> ArrayElementList<'a, 'b, T> {
    pub fn new(elements: &'b AstNode<'b, 'a, Vec<'a, T>>, group_id: GroupId, fill: bool) -> Self {
        Self { elements, group_id, fill }
    }
}

impl<'a, 'b, T: AstNodeListElement> Format<'a, JavaFormatContext<'a>>
    for ArrayElementList<'a, 'b, T>
where
    AstNode<'b, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let count = self.elements.len();
        let entry = |f: &mut JavaFormatter<'_, 'a>, index: usize, element: AstNode<'b, 'a, T>| {
            element.fmt(f);
            if index + 1 < count {
                token(",").fmt(f);
            } else {
                if_group_breaks(&token(",")).with_group_id(Some(self.group_id)).fmt(f);
            }
        };
        soft_line_break().fmt(f);
        if self.fill {
            let mut filler = f.fill();
            for (index, element) in self.elements.iter().enumerate() {
                filler.entry(
                    &format_with(|f| {
                        // A blank line between elements in the source stays, and breaks the list.
                        if f.lines_before(element.span()) > 1 {
                            empty_line().fmt(f);
                        } else {
                            soft_line_break_or_space().fmt(f);
                        }
                    }),
                    &format_once(|f| entry(f, index, element)),
                );
            }
            filler.finish();
        } else {
            let mut join = f.join_nodes_with_soft_line();
            for (index, element) in self.elements.iter().enumerate() {
                join.entry(element.span(), &format_once(|f| entry(f, index, element)));
            }
        }
    }
}

pub fn can_concisely_print_array_list<'p, 'a: 'p>(
    span: Span,
    elements: impl Iterator<Item = Option<AstNode<'p, 'a, Expression<'a>>>>,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    let comments = f.comments().unprinted_comments();
    let mut comments_iter = comments.iter().peekable();
    let mut elements = elements.peekable();
    let first_start =
        elements.peek().copied().flatten().map_or(span.end, |element| element.span().start);
    for element in elements {
        match element.as_deref() {
            Some(Expression::IntLiteral(_) | Expression::FloatLiteral(_)) => {}
            Some(Expression::Unary(unary)) => {
                while comments_iter
                    .peek()
                    .is_some_and(|comment| comment.span.start < unary.span.start)
                {
                    comments_iter.next();
                }
                if !matches!(unary.op, UnaryOp::Plus | UnaryOp::Minus)
                    || !matches!(
                        &unary.operand,
                        Expression::IntLiteral(_) | Expression::FloatLiteral(_)
                    )
                    || comments_iter
                        .peek()
                        .is_some_and(|comment| comment.span.start < unary.span.end)
                {
                    return false;
                }
            }
            _ => return false,
        }
    }
    // The first element's leading pass prints a line comment before it on
    // its own line.
    !comments
        .iter()
        .skip_while(|comment| comment.span.start < first_start)
        .take_while(|comment| comment.span.start < span.end)
        .any(|comment| comment.is_line() && !comment.preceded_by_newline())
}
