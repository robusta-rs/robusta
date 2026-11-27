use robusta_allocator::Vec;
use robusta_ast::{Annotation, ArrayCreation, ArrayDimension, ArrayTypeDimension, Type};
use robusta_formatter_core::{format_args, write};
use robusta_span::{GetSpan, Span};

use crate::ast_nodes::{AstNode, AstNodeListElement};
use crate::formatter::separated::format_delimited_frame;
use crate::formatter::{prelude::*, trivia::format_dangling_comments};
use crate::print::{FormatWrite, annotations::format_type_annotations};
use crate::utils::{
    format_node_without_trailing_comments::format_content_without_comments_after,
    statement_body::{
        comment_ends_line_before_opener, format_after_comments_before_opener,
        format_on_wrapped_line,
    },
};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayCreation<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        write!(f, ["new", space()]);
        let element_end = self.element.span().end;
        let annotated = |dimension: &ArrayDimension<'_>| !dimension.annotations.is_empty();
        let wrapped = dimensions_wrap_line(f, element_end, &self.dims, annotated);
        format_content_without_comments_after(&self.element(), element_end, f);
        format_dimensions(f, element_end, &self.dims(), annotated);
        if let Some(init) = self.init() {
            format_on_wrapped_line(wrapped, &format_args!(space(), init), f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayDimension<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_annotations(f, &self.annotations());
        if let Some(length) = self.length() {
            format_delimited_frame(f, "[", "]", None, |f| length.fmt(f));
        } else {
            let open_end = f.comments().position_after_character(
                self.annotations.last().map_or(self.span.start, |last| last.span.end),
                b'[',
            );
            write!(
                f,
                [
                    "[",
                    format_dangling_comments(Span::new(open_end, self.span.end))
                        .with_soft_block_indent(),
                    "]"
                ]
            );
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayTypeDimension<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_dimension(f, &self.annotations(), self.span.start, self.span.end);
    }
}

/// A dimension's type-use annotations and its `[]`, which ends at `end`.
/// `preceding_end` is where the text before the dimension ends.
pub(super) fn format_type_dimension<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    annotations: &AstNode<'_, 'a, Vec<'a, Annotation<'a>>>,
    preceding_end: u32,
    end: u32,
) {
    format_type_annotations(f, annotations);
    let open_end = f.comments().position_after_character(
        annotations.last().map_or(preceding_end, |last| last.span.end),
        b'[',
    );
    write!(
        f,
        ["[", format_dangling_comments(Span::new(open_end, end)).with_soft_block_indent(), "]"]
    );
}

/// Dimensions written apart from the type they extend, after a declarator's
/// name (`int a[]`) or a method's parameter list (`int f()[]`). `preceding_end`
/// is where the text before the first dimension ends; returns where the last
/// one ends.
pub fn format_array_type_dimensions<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    preceding_end: u32,
    dimensions: &AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>>,
) -> u32 {
    format_dimensions(f, preceding_end, dimensions, |dimension| !dimension.annotations.is_empty())
}

/// A dimension after the text ending at `preceding_end`, separated from it by
/// the comments before its `[`, or by a space before its first annotation,
/// where the leading pass of the node starting there prints them.
pub(super) fn format_dimension_after<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    preceding_end: u32,
    annotated: bool,
    dimension: &impl Format<'a, JavaFormatContext<'a>>,
) {
    if annotated {
        space().fmt(f);
        dimension.fmt(f);
    } else {
        format_after_comments_before_opener(preceding_end, b'[', dimension, f);
    }
}

/// [`dimensions_wrap_line`] for dimensions written apart from the type they
/// extend, as [`format_array_type_dimensions`] prints them.
pub fn array_type_dimensions_wrap_line(
    f: &JavaFormatter<'_, '_>,
    preceding_end: u32,
    dimensions: &[ArrayTypeDimension<'_>],
) -> bool {
    dimensions_wrap_line(f, preceding_end, dimensions, |dimension| {
        !dimension.annotations.is_empty()
    })
}

/// Whether a comment that ends its line comes before the `[` of one of
/// `dimensions`, after the text ending at `preceding_end`, so the code after
/// them continues a wrapped line.
fn dimensions_wrap_line<D: GetSpan>(
    f: &JavaFormatter<'_, '_>,
    preceding_end: u32,
    dimensions: &[D],
    annotated: fn(&D) -> bool,
) -> bool {
    let mut preceding_end = preceding_end;
    dimensions.iter().any(|dimension| {
        let wraps = !annotated(dimension) && comment_ends_line_before_opener(preceding_end, f);
        preceding_end = dimension.span().end;
        wraps
    })
}

/// Whether a comment that ends its line comes before the `[` of one of `ty`'s
/// dimensions, so the code after `ty` continues a wrapped line.
pub fn type_wraps_line(f: &JavaFormatter<'_, '_>, ty: &Type<'_>) -> bool {
    let Type::Array(array) = ty else {
        return false;
    };
    let element_end = array.element.span().end;
    (array.annotations.is_empty() && comment_ends_line_before_opener(element_end, f))
        || type_wraps_line(f, &array.element)
}

/// `dimensions` after the text ending at `preceding_end`; returns where the
/// last one ends.
fn format_dimensions<'a, D: AstNodeListElement>(
    f: &mut JavaFormatter<'_, 'a>,
    preceding_end: u32,
    dimensions: &AstNode<'_, 'a, Vec<'a, D>>,
    annotated: fn(&D) -> bool,
) -> u32
where
    for<'p> AstNode<'p, 'a, D>: Format<'a, JavaFormatContext<'a>>,
{
    let mut preceding_end = preceding_end;
    for dimension in dimensions {
        let end = dimension.span().end;
        let content = format_with(|f| format_content_without_comments_after(&dimension, end, f));
        format_dimension_after(f, preceding_end, annotated(dimension.inner()), &content);
        preceding_end = end;
    }
    preceding_end
}
