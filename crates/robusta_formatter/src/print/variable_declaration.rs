// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/variable_declaration.rs

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::print::FormatWrite;
use crate::print::annotations::{format_modifiers_grouped, format_modifiers_inline};
use crate::print::array_dimension::type_wraps_line;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::utils::assignment_like::AssignmentLike;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::format_on_wrapped_line;
use robusta_allocator::Vec;
use robusta_ast::{FieldDeclaration, LocalVariableDeclaration, VariableDeclarator};
use robusta_formatter_core::{format_args, write};
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, FieldDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        format_modifiers_grouped(f, &self.modifiers());
        let wrapped = type_wraps_line(f, &node.ty);
        let declarators = &self.declarators();
        format_type_and_declarators(f, &self.ty(), wrapped, declarators, Some(node.span.end));
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, LocalVariableDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let wrapped = node.ty.as_type().is_some_and(|ty| type_wraps_line(f, ty));
        let (ty, declarators) = (&self.ty(), &self.declarators());
        // A basic `for` initializer uses the same production, but its semicolon
        // separates the header's clauses, so the `for` header prints it.
        if matches!(self.parent(), AstNodes::ForStatement(_)) {
            format_modifiers_inline(f, &self.modifiers());
            format_type_and_declarators(f, ty, wrapped, declarators, None);
        } else {
            format_modifiers_grouped(f, &self.modifiers());
            format_type_and_declarators(f, ty, wrapped, declarators, Some(node.span.end));
        }
    }
}

/// Format the type and `VariableDeclaratorList` fields and locals share
/// (JLS §8.3, §14.4), then the `;` ending the declaration at `node_end`, when
/// the declaration prints one. The declarators continue a line `wrapped` by a
/// comment before one of the type's `[`s.
fn format_type_and_declarators<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    ty: &(impl Format<'a, JavaFormatContext<'a>> + GetSpan),
    wrapped: bool,
    declarators: &AstNode<'_, 'a, Vec<'a, VariableDeclarator<'a>>>,
    node_end: Option<u32>,
) {
    let type_end = ty.span().end;
    let content_end = declarators.last().map_or(type_end, |last| last.span.end);
    let content = format_with(|f| {
        let ty = format_with(|f| format_content_without_comments_after(ty, type_end, f));
        let declarators = format_with(|f| format_on_wrapped_line(wrapped, declarators, f));
        write!(f, [group(&format_args!(ty, declarators))]);
    });
    match node_end {
        Some(node_end) => FormatContentWithSemicolon::new(&content, content_end, node_end).fmt(f),
        None => content.fmt(f),
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Vec<'a, VariableDeclarator<'a>>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let length = self.len();

        let is_parent_for_loop = matches!(self.parent().parent(), AstNodes::ForStatement(_));

        let has_any_initializer = self.iter().any(|declarator| declarator.init.is_some());

        let format_separator = format_with(|f| {
            if !is_parent_for_loop && has_any_initializer {
                write!(f, hard_line_break());
            } else {
                write!(f, soft_line_break_or_space());
            }
        });

        let mut declarators = self.iter();
        let Some(first_declarator) = declarators.next() else {
            return;
        };

        if length == 1 {
            super::format_name_after_type(first_declarator.span.start, &first_declarator, f);
            return;
        }

        // Later declarators line up below a moved first one.
        write!(
            f,
            indent(&format_once(|f| {
                super::write_type_name_separator(first_declarator.span.start, f);
                first_declarator.fmt(f);
                for declarator in declarators {
                    write!(f, [",", format_separator, declarator]);
                }
            }))
        );
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, VariableDeclarator<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        AssignmentLike::VariableDeclarator(self).fmt(f);
    }
}
