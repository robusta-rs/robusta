pub mod arguments;

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::separated::argument_list_end;
use crate::print::call_like_expression::arguments::format_arguments;
use crate::print::names::{class_type_wraps_line, format_class_type_name};
use crate::print::type_parameters::format_type_arguments_before_name;
use crate::print::{FormatWrite, format_dot_before_nodeless_operand};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::member_chain::format_member_chain;
use crate::utils::statement_body::{
    comment_ends_line_before_opener, format_callee, format_head_body, format_on_wrapped_line,
    write_comments_between_blocks,
};
use robusta_allocator::Vec;
use robusta_ast::{
    ClassBody, ClassInstanceCreation, Expression, MethodCall, MethodMember,
    QualifiedClassInstanceCreation, TypeArguments,
};
use robusta_span::{GetSpan, Span};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ClassInstanceCreation<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_new_keyword(f, self.constructor_type_args());
        format_creation(f, &self.ty(), self.ty.span().end, &self.args(), self.body());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedClassInstanceCreation<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let qualifier = self.qualifier();
        let span = qualifier.span();
        format_content_without_comments_after(&qualifier, span.end, f);
        let creation = format_with(|f| {
            format_new_keyword(f, self.constructor_type_args());
            let class_end =
                self.type_args.as_ref().map_or(self.name.span.end, |args| args.span.end);
            let class = format_with(|f| {
                format_class_type_name(
                    f,
                    &self.annotations(),
                    &self.name(),
                    self.type_args(),
                    false,
                );
            });
            format_creation(f, &class, class_end, &self.args(), self.body());
        });
        format_dot_before_nodeless_operand(span, &creation, f);
    }
}

/// Whether a comment that ends its line comes before the type arguments of one
/// of the names of the class that `creation` creates, so the code after them
/// continues a wrapped line.
fn created_class_wraps_line(f: &JavaFormatter<'_, '_>, creation: &AstNodes<'_, '_>) -> bool {
    match creation {
        AstNodes::ClassInstanceCreation(creation) => class_type_wraps_line(f, &creation.ty),
        AstNodes::QualifiedClassInstanceCreation(creation) => {
            creation.type_args.is_some()
                && comment_ends_line_before_opener(creation.name.span.end, f)
        }
        _ => unreachable!("only a class instance creation creates a class"),
    }
}

/// `new` and its constructor type arguments, up to the class.
fn format_new_keyword<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    constructor_type_args: Option<AstNode<'_, 'a, TypeArguments<'a>>>,
) {
    "new".fmt(f);
    if let Some(constructor_type_args) = constructor_type_args {
        write_comments_between_blocks(
            f.comments().comments_before(constructor_type_args.span.start),
            f,
        );
        space().fmt(f);
        format_type_arguments_before_name(f, &constructor_type_args);
    }
    space().fmt(f);
}

/// The created class and what follows it. When a comment that ends its line
/// comes before the class's type arguments, the class's printer indents them
/// as a wrapped line, and the arguments and body continue that line.
fn format_creation<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    class: &impl Format<'a, JavaFormatContext<'a>>,
    class_end: u32,
    args: &AstNode<'_, 'a, Vec<'a, Expression<'a>>>,
    body: Option<AstNode<'_, 'a, ClassBody<'a>>>,
) {
    // A class body can follow the arguments, so bound the class's comments at `(`.
    let open_end = f.comments().position_after_character(class_end, b'(');
    let close_end = argument_list_end(f, args, open_end);
    let class_comment_end =
        if args.is_empty() && f.comments().has_comment_in_span(Span::new(open_end, close_end)) {
            // Lay out deferred comments with the empty list's dangling comments.
            f.comments()
                .all_comments_in_range(class_end, open_end)
                .find(|comment| comment.is_line() || comment.preceded_by_newline())
                .map_or(open_end, |comment| comment.span.start)
        } else {
            open_end
        };
    format_content_without_comments_after(class, class_comment_end, f);
    // The class's trailing comments ride to the end of the line, outside the indent.
    let rest = format_with(|f| {
        format_arguments(f, args, close_end);
        if let Some(body) = &body {
            format_head_body(body, f);
        }
    });
    format_on_wrapped_line(created_class_wraps_line(f, args.parent()), &rest, f);
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MethodCall<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.member().is_some() {
            format_member_chain(f, self);
        } else {
            // A method call without a receiver: `foo(a)`.
            format_callee(&self.callee(), f);
            format_arguments(f, &self.args(), self.span.end);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MethodMember<'a>> {
    fn write(&self, _f: &mut JavaFormatter<'_, 'a>) {
        unreachable!("the member chain prints a call's callee");
    }
}
