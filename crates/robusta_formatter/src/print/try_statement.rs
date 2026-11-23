use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::separated::{format_delimited_frame, format_undelimited_list};
use crate::formatter::trivia::{
    FormatTrailingComments, format_hoisted_leading_comments, format_leading_comments,
};
use crate::print::annotations::format_modifiers_inline;
use crate::print::{FormatWrite, format_name_after_type, write_type_name_separator};
use crate::utils::assignment_like::AssignmentLike;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{
    format_continuation_keyword, write_head_body_separator, write_node_with_terminator,
};
use robusta_ast::{CatchClause, Resource, ResourceDeclaration, TryStatement};
use robusta_formatter_core::write;
use robusta_span::{GetSpan, Span};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, TryStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        token("try").fmt(f);
        if !node.resources.is_empty() {
            space().fmt(f);
            format_resource_specification(f, &self.resources(), node.resources_trailing_semicolon);
        }
        let block = format_once(|f| {
            write_head_body_separator(node.block.span.start, f);
            self.block().fmt(f);
        });
        if !node.catches.is_empty() || node.finally_block.is_some() {
            format_content_without_comments_after(&block, node.block.span.end, f);
        } else {
            block.fmt(f);
        }
        let mut previous_end = node.block.span.end;
        for (index, catch) in self.catches().iter().enumerate() {
            format_continuation_keyword(f, "catch", previous_end);
            previous_end = catch.span.end;
            if index + 1 < node.catches.len() || node.finally_block.is_some() {
                format_content_without_comments_after(&catch, catch.span.end, f);
            } else {
                catch.fmt(f);
            }
        }
        if let Some(finally_block) = self.finally_block() {
            format_continuation_keyword(f, "finally", previous_end);
            write_head_body_separator(finally_block.span.start, f);
            finally_block.fmt(f);
        }
    }
}

fn format_catch_parameter<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    catch: &AstNode<'_, 'ast, CatchClause<'ast>>,
) {
    // The types wrap inside the `catch` parentheses, as dart format wraps an `if`.
    let types = catch.types();
    let count = catch.types.len();
    format_undelimited_list(f, true, count, |f, index| {
        let ty = types.get(index).expect("catch type");
        if index > 0 {
            write!(f, [format_hoisted_leading_comments(ty.span()), token("|"), space()]);
        } else {
            format_modifiers_inline(f, &catch.modifiers());
        }
        if index + 1 == count {
            // The comments before the name lead the name.
            format_content_without_comments_after(&ty, ty.span().end, f);
            write_type_name_separator(catch.name.span.start, f);
            format_content_without_comments_after(&catch.name(), catch.name.span.end, f);
        } else {
            ty.fmt(f);
        }
    });
}

fn format_resource_specification<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    resources: &AstNode<'_, 'ast, robusta_allocator::Vec<'ast, Resource<'ast>>>,
    trailing_semicolon: bool,
) {
    let last_end = resources.last().expect("a resource specification has a resource").span().end;
    let count = resources.len();
    format_delimited_frame(f, "(", ")", None, |f| {
        for (index, resource) in resources.iter().enumerate() {
            if index > 0 {
                soft_line_break_or_space().fmt(f);
            }
            format_resource_entry(f, &resource, index + 1 < count || trailing_semicolon);
        }
        let comments = f.comments().comments_before_character(last_end, b')');
        FormatTrailingComments::Comments(comments).fmt(f);
    });
}

/// A resource and its `;`, whose line's comments trail it.
fn format_resource_entry<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    resource: &AstNode<'_, 'ast, Resource<'ast>>,
    semicolon: bool,
) {
    if semicolon {
        write_node_with_terminator(resource, ";", f);
        let end = f.comments().position_after_character(resource.span().end, b';');
        let comments = f.comments().end_of_line_comments_after(end);
        FormatTrailingComments::Comments(comments).fmt(f);
    } else {
        format_content_without_comments_after(resource, resource.span().end, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ResourceDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_modifiers_inline(f, &self.modifiers());
        format_content_without_comments_after(&self.ty(), self.ty.span().end, f);
        let declarator = format_with(|f| AssignmentLike::ResourceDeclaration(self).fmt(f));
        format_name_after_type(self.name.span.start, &declarator, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CatchClause<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        space().fmt(f);
        token("(").fmt(f);
        let first_start = node.modifiers.first().map_or_else(
            || node.types.first().expect("a catch parameter has a type").span().start,
            |modifier| modifier.span().start,
        );
        let name_end = node.name.span.end;
        let comments = f.comments();
        // The leading comments, those between `catch` and `(` included, start
        // a line where the source does: before the first, and after each, even
        // where the `(` comes between.
        let leading = comments.comments_before(first_start);
        let leading_comment_with_break = leading.iter().enumerate().any(|(index, comment)| {
            let next = leading.get(index + 1).map_or(first_start, |next| next.span.start);
            comment.preceded_by_newline()
                || f.source_text().contains_newline_between(comment.span.end, next)
        });
        let trailing_comment_with_break = comments
            .comments_before_character(name_end, b')')
            .iter()
            .any(|comment| comment.is_line() || comment.preceded_by_newline());
        // The comments before the `)` trail the name, inside the parentheses.
        let trailing = format_once(|f| {
            let comments = f.comments().comments_before_character(name_end, b')');
            FormatTrailingComments::Comments(comments).fmt(f);
        });
        if leading_comment_with_break || trailing_comment_with_break {
            write!(
                f,
                [soft_block_indent(&format_once(|f| {
                    format_leading_comments(Span::empty(first_start)).fmt(f);
                    format_catch_parameter(f, self);
                    trailing.fmt(f);
                }))]
            );
        } else {
            format_catch_parameter(f, self);
            trailing.fmt(f);
        }
        token(")").fmt(f);
        write_head_body_separator(node.block.span.start, f);
        self.block().fmt(f);
    }
}
