use crate::ast_nodes::AstNode;
use crate::ast_nodes::impls::ast_nodes::format_node_leading_comments;
use crate::formatter::prelude::*;
use crate::formatter::separated::{format_empty_parentheses, format_parenthesized_frame};
use crate::print::annotations::{format_modifiers_inline, format_type_annotations};
use crate::print::array_dimension::format_array_type_dimensions;
use crate::print::{FormatWrite, format_name_after_type, format_qualified_keyword};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{
    format_after_comments_before_opener, write_comments_between_blocks,
};
use robusta_allocator::Vec;
use robusta_ast::{
    Annotation, BindingIdentifier, FormalParameter, FormalParameters, Modifier, ReceiverParameter,
    RecordComponent, RecordHeader,
};
use robusta_formatter_core::{GroupId, format_args};
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, RecordHeader<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_record_header(f, self);
    }
}

/// A record header's `(…)`, with comments as in [`format_parameter_list`].
///
/// Returns the group the components break in, which an empty header does not
/// have.
pub fn format_record_header<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    header: &AstNode<'_, 'ast, RecordHeader<'ast>>,
) -> Option<GroupId> {
    write_comments_between_blocks(f.comments().comments_before(header.span.start), f);
    let components = header.components();
    let Some(first) = components.first() else {
        format_empty_parentheses(f, header.span);
        return None;
    };
    let group_id = f.state().group_id("record_components");
    format_parenthesized_frame(f, first.span.start, Some(group_id), |f| {
        f.join_with(&format_args!(",", soft_line_break_or_space())).entries(components.iter());
    });
    Some(group_id)
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, FormalParameters<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_parameter_list(f, self);
    }
}

/// A parameter list's `(…)`, or a concise lambda's parameter. The comments
/// before the `(` stay before it, and an empty list keeps a comment between its
/// delimiters, so `void i(/* empty */)` does not push the comment into the body.
/// A line comment right after the `(` stays on its line, and the list breaks.
///
/// Returns the group the entries break in, which an empty or concise list does
/// not have.
pub fn format_parameter_list<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    list: &AstNode<'_, 'ast, FormalParameters<'ast>>,
) -> Option<GroupId> {
    write_comments_between_blocks(f.comments().comments_before(list.span.start), f);
    let receiver = list.receiver();
    let parameters = list.items();
    if !list.has_parentheses() {
        let parameter = parameters.get(0).expect("a concise lambda parameter");
        parameter.fmt(f);
        return None;
    }
    let Some(first_start) = receiver
        .as_ref()
        .map(|receiver| receiver.span.start)
        .or_else(|| parameters.first().map(|parameter| parameter.span.start))
    else {
        format_empty_parentheses(f, list.span);
        return None;
    };
    let group_id = f.state().group_id("parameters");
    format_parenthesized_frame(f, first_start, Some(group_id), |f| {
        f.join_with(&format_args!(",", soft_line_break_or_space()))
            .entries(receiver)
            .entries(parameters.iter());
    });
    Some(group_id)
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, FormalParameter<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        Parameter::Formal(*self).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, RecordComponent<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        Parameter::Record(*self).fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ReceiverParameter<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_receiver_parameter(f, self);
    }
}

fn format_receiver_parameter<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    receiver: &AstNode<'_, 'ast, ReceiverParameter<'ast>>,
) {
    format_modifiers_inline(f, &receiver.modifiers());
    format_content_without_comments_after(&receiver.ty(), receiver.ty.span().end, f);
    if let Some(qualifier) = receiver.qualifier() {
        let qualified = format_once(|f| format_qualified_keyword(f, &qualifier, "this"));
        format_name_after_type(qualifier.span().start, &qualified, f);
    } else {
        let this_start = f.comments().position_after_character(receiver.ty.span().end, b't') - 1;
        let this = format_once(|f| {
            format_node_leading_comments(&receiver.as_ast_nodes(), this_start, f);
            "this".fmt(f);
        });
        format_name_after_type(this_start, &this, f);
    }
}

enum Parameter<'parent, 'a> {
    Formal(AstNode<'parent, 'a, FormalParameter<'a>>),
    Record(AstNode<'parent, 'a, RecordComponent<'a>>),
}

impl<'a> Parameter<'_, 'a> {
    fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        match self {
            Self::Formal(node) => node.modifiers(),
            Self::Record(node) => node.modifiers(),
        }
    }

    /// Formats the type and returns where it ends, or `None` for an untyped
    /// lambda parameter.
    fn format_type(&self, f: &mut JavaFormatter<'_, 'a>) -> Option<u32> {
        match self {
            Self::Formal(node) => node.ty().map(|ty| {
                let end = ty.span().end;
                format_content_without_comments_after(&ty, end, f);
                end
            }),
            Self::Record(node) => {
                let ty = node.ty();
                let end = ty.span().end;
                format_content_without_comments_after(&ty, end, f);
                Some(end)
            }
        }
    }

    fn varargs(&self) -> bool {
        match self {
            Self::Formal(node) => node.varargs,
            Self::Record(node) => node.varargs,
        }
    }

    fn varargs_annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        match self {
            Self::Formal(node) => node.varargs_annotations(),
            Self::Record(node) => node.varargs_annotations(),
        }
    }

    fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        match self {
            Self::Formal(node) => node.name(),
            Self::Record(node) => node.name(),
        }
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for Parameter<'_, 'a> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let name = self.name();
        format_modifiers_inline(f, &self.modifiers());
        let type_end = self.format_type(f);
        if let Some(type_end) = type_end
            && self.varargs()
        {
            format_varargs(f, type_end, &self.varargs_annotations());
        }
        let name_and_dims = format_with(|f| {
            if let Self::Formal(node) = self
                && !node.dims.is_empty()
            {
                format_content_without_comments_after(&name, name.span.end, f);
                format_array_type_dimensions(f, name.span.end, &node.dims());
            } else {
                name.fmt(f);
            }
        });
        if type_end.is_some() {
            format_name_after_type(name.span.start, &name_and_dims, f);
        } else {
            name_and_dims.fmt(f);
        }
    }
}

/// An ellipsis and its annotations. The comments before the first annotation
/// lead it.
pub fn format_varargs<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    type_end: u32,
    annotations: &AstNode<'_, 'a, Vec<'a, Annotation<'a>>>,
) {
    if annotations.is_empty() {
        format_after_comments_before_opener(type_end, b'.', &"...", f);
    } else {
        space().fmt(f);
        format_type_annotations(f, annotations);
        "...".fmt(f);
    }
}
