use crate::ast_nodes::AstNode;
use crate::ast_nodes::impls::ast_nodes::format_node_leading_comments;
use crate::formatter::prelude::*;
use crate::formatter::trivia::{
    documentation_comment_ends_line, format_documentation_comments_before,
};
use crate::print::annotations::format_modifiers_stacked;
use crate::print::array_dimension::format_array_type_dimensions;
use crate::print::class::{FormatDeclarationClause, format_declaration_header};
use crate::print::names::class_type_wraps_line;
use crate::print::parameters::format_parameter_list;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::{
    FormatWrite, comments_move_name, format_name_after_type, format_name_after_type_with_indent,
};
use crate::utils::assignment_like::AssignmentLike;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{format_head_body, format_on_wrapped_line_with_indent};
use robusta_allocator::Vec;
use robusta_ast::{
    AnnotationInterfaceElementDeclaration, ArrayTypeDimension, Comment,
    CompactConstructorDeclaration, ConstructorDeclaration, FormalParameters, MethodDeclaration,
    MethodResult, Modifier, Type, TypeParameters,
};
use robusta_formatter_core::{format_args, write};
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, MethodDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let method = self.inner();
        let signature = Signature {
            modifiers: self.modifiers(),
            type_params: self.type_params(),
            return_type: Some(self.return_type()),
            name: self.name(),
            parameters: self.params(),
            dimensions: Some(self.post_paren_dimensions()),
            throws: self.throws(),
        };
        let header_end = format_signature(f, &signature);
        if let Some(body) = self.body() {
            format_head_body(&body, f);
            if body.statements.is_empty() {
                signature.mark_name_comment_moved_behind(method.span.end, f);
            }
        } else {
            // The header, already printed, is the content before the `;`.
            FormatContentWithSemicolon::new(&(), header_end, method.span.end).fmt(f);
            signature.mark_name_comment_moved_behind(method.span.end, f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ConstructorDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let signature = Signature {
            modifiers: self.modifiers(),
            type_params: self.type_params(),
            return_type: None,
            name: self.name(),
            parameters: self.params(),
            dimensions: None,
            throws: self.throws(),
        };
        format_signature(f, &signature);
        let body = self.body();
        format_head_body(&body, f);
        if body.statements.is_empty() {
            signature.mark_name_comment_moved_behind(self.span.end, f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CompactConstructorDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        format_modifiers_stacked(f, &self.modifiers());
        format_content_without_comments_after(&self.name(), node.name.span.end, f);
        format_head_body(&self.body(), f);
    }
}

/// A method or constructor header through its `throws` clause.
struct Signature<'parent, 'a, Name> {
    modifiers: AstNode<'parent, 'a, Vec<'a, Modifier<'a>>>,
    type_params: Option<AstNode<'parent, 'a, TypeParameters<'a>>>,
    /// `None` for a constructor.
    return_type: Option<AstNode<'parent, 'a, MethodResult<'a>>>,
    name: AstNode<'parent, 'a, Name>,
    parameters: AstNode<'parent, 'a, FormalParameters<'a>>,
    /// A method's obsolete dimensions after the parameter list (`int f()[]`).
    dimensions: Option<AstNode<'parent, 'a, Vec<'a, ArrayTypeDimension<'a>>>>,
    throws: AstNode<'parent, 'a, Vec<'a, Type<'a>>>,
}

impl<Name: GetSpan> Signature<'_, '_, Name> {
    /// The line comment that ends the line of the name, or of a constructor's
    /// type parameters, when it rides past an empty `()`.
    fn name_comment_past_parameters(&self, f: &JavaFormatter<'_, '_>) -> Option<Comment> {
        let parameters = &self.parameters;
        if parameters.receiver.is_some() || !parameters.items.is_empty() {
            return None;
        }
        let start = match (&self.return_type, &self.type_params) {
            (None, Some(_)) if self.name_keeps_line(f) => return None,
            (None, Some(type_params)) => type_params.span.end,
            _ => self.name.span().start,
        };
        f.comments().printed_line_comment_after(start).filter(|comment| {
            comment.span.end <= parameters.span.start
                && !comment.preceded_by_newline()
                && !comment.is_documentation()
        })
    }

    /// Whether a constructor's name moves to the line after its type
    /// parameters, as after a return type: in front of a line comment ending
    /// their line, the name would bring the block comment after it, which stays
    /// before the `(`, onto that line.
    fn name_keeps_line(&self, f: &JavaFormatter<'_, '_>) -> bool {
        let Some(type_params) = &self.type_params else {
            return false;
        };
        let comments = f.comments();
        let name = self.name.span();
        comments
            .all_comments_in_range(type_params.span.end, name.start)
            .take_while(|comment| !comment.preceded_by_newline())
            .last()
            .is_some_and(Comment::is_line)
            && comments
                .all_comments_in_range(name.end, self.parameters.span.start)
                .next()
                .is_some_and(|comment| comment.is_block() && !comment.preceded_by_newline())
    }

    /// Record [`Self::name_comment_past_parameters`] as moved behind the
    /// declaration ending at `end`. Called after an empty body or a `;`: with
    /// no `throws` clause, no line breaks between them and the `()`.
    fn mark_name_comment_moved_behind(&self, end: u32, f: &JavaFormatter<'_, '_>) {
        if self.throws.is_empty()
            && let Some(comment) = self.name_comment_past_parameters(f)
        {
            f.comments().mark_line_comment_moved_behind(end, &comment);
        }
    }
}

/// Returns the end of the header, where the body or `;` follows.
fn format_signature<'parent, 'a, Name: GetSpan>(
    f: &mut JavaFormatter<'_, 'a>,
    signature: &Signature<'parent, 'a, Name>,
) -> u32
where
    AstNode<'parent, 'a, Name>: Format<'a, JavaFormatContext<'a>>,
{
    format_modifiers_stacked(f, &signature.modifiers);
    let Some(return_type) = signature.return_type else {
        let Some(type_params) = &signature.type_params else {
            return format_name_through_throws(f, signature);
        };
        let name_start = signature.name.span().start;
        let after_params = f.comments().comments_in_range(type_params.span.end, name_start);
        if !documentation_comment_ends_line(after_params) && !signature.name_keeps_line(f) {
            type_params.fmt(f);
            space().fmt(f);
            return format_name_through_throws(f, signature);
        }
        // The name moves to the next line, as after a return type.
        format_content_without_comments_after(type_params, type_params.span.end, f);
        let mut header_end = 0;
        let rest = format_once(|f| header_end = format_name_through_throws(f, signature));
        format_name_after_type_with_indent(name_start, &rest, false, true, f);
        return header_end;
    };
    let mut header_end = 0;
    // Measured flat, the return type and name reach the `(`, so a break after
    // the type parameters comes before one inside the return type.
    let measure_flat = signature.type_params.is_some();
    let result_through_throws = format_once(|f| {
        let ty = format_once(|f| {
            format_content_without_comments_after(&return_type, return_type.span().end, f);
        });
        if measure_flat {
            // Outside the group, a comment that starts a line does not break the type.
            format_node_leading_comments(return_type.parent(), return_type.span().start, f);
            group(&ty).measure_flat().fmt(f);
        } else {
            ty.fmt(f);
        }
        let rest = format_once(|f| header_end = format_name_through_throws(f, signature));
        let name_start = signature.name.span().start;
        let name = format_once(|f| {
            format_name_after_type_with_indent(name_start, &rest, measure_flat, true, f);
        });
        // The name continues the line of the return type's moved type arguments.
        let class = match return_type.inner() {
            MethodResult::Array(array) => array.element.as_class_type(),
            result => result.as_class_type(),
        };
        let wrapped = class.is_some_and(|class| class_type_wraps_line(f, class));
        format_on_wrapped_line_with_indent(wrapped, true, &name, f);
    });
    let Some(type_params) = &signature.type_params else {
        result_through_throws.fmt(f);
        return header_end;
    };
    // The header breaks after type parameters that fit on their line.
    let type_params_id = f.state().group_id("type_parameters");
    let type_params_only = format_once(|f| {
        format_content_without_comments_after(type_params, type_params.span.end, f);
    });
    write!(f, [group(&type_params_only).with_group_id(Some(type_params_id))]);
    // Outside the group, a line comment after `>` does not break the list.
    type_params.format_trailing_comments(f);
    let result_through_throws = result_through_throws.memoized();
    let return_type_start = return_type.span().start;
    let name_start = signature.name.span().start;
    // A name that a comment moves leaves the return type's line, so the header
    // breaks after the type parameters first.
    let name_moves =
        comments_move_name(f.comments().comments_in_range(return_type.span().end, name_start));
    let broken_after = format_once(|f| {
        if name_moves {
            wrapped_indent(true, &format_args!(hard_line_break(), result_through_throws)).fmt(f);
        } else {
            format_name_after_type_with_indent(
                return_type_start,
                &result_through_throws,
                false,
                true,
                f,
            );
        }
    });
    write!(
        f,
        [
            if_group_fits_on_line(&broken_after).with_group_id(Some(type_params_id)),
            if_group_breaks(&format_args!(space(), result_through_throws))
                .with_group_id(Some(type_params_id)),
        ]
    );
    header_end
}

/// The name, parameters and `throws` clause. Returns the header's end.
fn format_name_through_throws<'parent, 'a, Name: GetSpan>(
    f: &mut JavaFormatter<'_, 'a>,
    signature: &Signature<'parent, 'a, Name>,
) -> u32
where
    AstNode<'parent, 'a, Name>: Format<'a, JavaFormatContext<'a>>,
{
    let name = &signature.name;
    let name_end = name.span().end;
    let params_start = signature.parameters.span.start;
    format_content_without_comments_after(name, name_end, f);
    // Otherwise the parameter list prints the comments after its `(`.
    if !documentation_comment_ends_line(f.comments().comments_before(params_start)) {
        name.format_trailing_comments(f);
        format_documentation_comments_before(name_end, params_start, f);
    }
    let params_end = signature.parameters.span.end;
    let head_end = signature
        .dimensions
        .and_then(|dims| dims.last().map(|last| last.span.end))
        .unwrap_or(params_end);
    let clause = FormatDeclarationClause::types("throws", signature.throws);
    format_declaration_header(f, head_end, clause, |f| {
        let list = format_parameter_list(f, &signature.parameters);
        if let Some(dimensions) = signature.dimensions {
            // The name's line comment also rides past each `[]` before the
            // next comment, so it moves behind the last of them.
            if let Some(comment) = signature.name_comment_past_parameters(f) {
                let comments = f.comments();
                let end = dimensions
                    .iter()
                    .take_while(|dimension| !comments.has_comment_before(dimension.span.end))
                    .last()
                    .map_or(params_end, |dimension| dimension.span.end);
                comments.mark_line_comment_moved_behind(end, &comment);
            }
            format_array_type_dimensions(f, params_end, &dimensions);
        }
        list
    })
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, AnnotationInterfaceElementDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let element = AssignmentLike::AnnotationInterfaceElementDeclaration(self);
        let content_end = self
            .default_value
            .as_ref()
            .map_or_else(|| element.left_end(f), |default_value| default_value.span().end);
        // The type stays outside the assignment layout, so a broken `default`
        // value stays one level below a moved name.
        let content = format_once(|f| {
            format_modifiers_stacked(f, &self.modifiers());
            format_content_without_comments_after(&self.ty(), self.ty.span().end, f);
            format_name_after_type(self.name.span.start, &element, f);
        });
        FormatContentWithSemicolon::new(&content, content_end, self.span.end).fmt(f);
    }
}
