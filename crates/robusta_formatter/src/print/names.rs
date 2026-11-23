use robusta_allocator::Vec;
use robusta_ast::{
    Annotation, BindingIdentifier, FieldAccess, IdentifierName, IdentifierReference,
    LabelIdentifier, QualifiedClassType, QualifiedName, QualifiedTypeName, SimpleClassType,
    TypeArguments,
};
use robusta_formatter_core::Format;
use robusta_span::{GetSpan, Span};
use smallvec::SmallVec;

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::trivia::FormatTrailingComments;
use crate::print::annotations::format_type_annotations;
use crate::print::member_expression::{
    Member, MemberLayout, format_comments_around_member_separator, format_member_after_object,
    format_member_separator,
};
use crate::print::{FormatWrite, format_indented_continuation_with_mode};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::is_header_condition;
use crate::utils::statement_body::format_after_comments_before_opener;

// A name keeps its source spelling, escapes included.
fn format_identifier(span: Span, f: &mut JavaFormatter<'_, '_>) {
    text_without_whitespace(f.source_text().slice_range(span.start, span.end)).fmt(f);
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, IdentifierName<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_identifier(self.span, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, IdentifierReference<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_identifier(self.span, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, BindingIdentifier<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_identifier(self.span, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, LabelIdentifier<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_identifier(self.span, f);
    }
}

/// Walk a dotted name from its last node to the left, without recursion, and
/// return what starts it and `$part` of each other node, last first. Each node
/// moves into the arena to be the parent of the next.
macro_rules! dotted_parts {
    ($node:expr, $qualifier:ident, $as_qualified:ident, $it:ident => $part:expr, $f:expr) => {{
        let allocator = $f.allocator();
        let mut parts = SmallVec::<[_; 4]>::new();
        let $it = $node;
        parts.push($part);
        let mut qualifier = $it.$qualifier();
        while let Some(qualified) = qualifier.$as_qualified() {
            let $it = qualified.in_arena(allocator);
            parts.push($part);
            qualifier = $it.$qualifier();
        }
        (qualifier, parts)
    }};
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedName<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let wrap = !matches!(
            self.parent(),
            AstNodes::PackageDeclaration(_) | AstNodes::ImportDeclaration(_)
        );
        let (first, names) = dotted_parts!(self, qualifier, as_qualified, it => it.name(), f);
        format_dotted_names(&first, &names, wrap, f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedTypeName<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let (first, names) = dotted_parts!(self, qualifier, as_qualified, it => it.name(), f);
        format_dotted_names(&first, &names, true, f);
    }
}

/// `a.b.c`, a dotted expression name. `node` is its last field access. Each
/// `.` takes the layout of a field access after its object.
pub fn format_dotted_expression_name<'a>(
    node: &AstNode<'_, 'a, FieldAccess<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let (first, accesses) = dotted_parts!(node, target, as_field_access, it => *it, f);
    let layout_of = |access: &AstNode<'_, 'a, FieldAccess<'a>>| {
        MemberLayout::new(Some(&access.target), access.parent(), false)
    };
    let last_layout = layout_of(node);
    // Every field access but `node` is the object of the next one, so their
    // dots share a layout, found from the one nearest `node`.
    let inner_layout = accesses.get(1).map_or(last_layout, layout_of);
    let in_header = is_header_condition(node.parent(), node.span);
    format_content_without_comments_after(&first, first.span().end, f);
    for (index, access) in accesses.iter().enumerate().rev() {
        let layout = if index == 0 { last_layout } else { inner_layout };
        let member = Member { separator: ".", start: access.name.span.start, layout, in_header };
        format_member_after_object(access.target.span(), member, &access.name(), f);
    }
}

/// `first`, then `names` from the last to the one after `first`.
fn format_dotted_names<'p, 'a: 'p>(
    first: &(impl Format<'a, JavaFormatContext<'a>> + GetSpan),
    names: &[AstNode<'p, 'a, IdentifierName<'a>>],
    wrap: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let parts = names.iter().rev().map(|name| (name.span, name));
    format_dotted_name(first, parts, wrap, f, Format::fmt);
}

fn format_dotted_name<'a, Part>(
    first: &(impl Format<'a, JavaFormatContext<'a>> + GetSpan),
    parts: impl Iterator<Item = (Span, Part)>,
    wrap: bool,
    f: &mut JavaFormatter<'_, 'a>,
    format_after_dot: impl Fn(&Part, &mut JavaFormatter<'_, 'a>),
) {
    let mut previous = first.span();
    format_content_without_comments_after(first, previous.end, f);
    for (span, part) in parts {
        let needs_separator = format_comments_around_member_separator(previous, span.start, ".", f);
        let suffix = format_with(|f| {
            format_member_separator(".", span.start, f);
            format_after_dot(&part, f);
        });
        if wrap && needs_separator {
            format_indented_continuation_with_mode(soft_line_break(), &suffix, true, f);
        } else {
            suffix.fmt(f);
        }
        previous = span;
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SimpleClassType<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let created = is_created_class(self.parent());
        format_class_type_name(f, &self.annotations(), &self.name(), self.args(), created);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, QualifiedClassType<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let (first, types) = dotted_parts!(self, qualifier, as_qualified, it => *it, f);
        let parts = types.iter().rev().map(|ty| (Span::new(last_name_start(ty), ty.span.end), ty));
        format_dotted_name(&first, parts, true, f, |ty, f| {
            let created = is_created_class(ty.parent());
            format_class_type_name(f, &ty.annotations(), &ty.name(), ty.args(), created);
        });
    }
}

/// Whether a class type with this parent names the class an instance creation
/// creates.
fn is_created_class(parent: &AstNodes<'_, '_>) -> bool {
    matches!(parent, AstNodes::ClassInstanceCreation(_))
}

/// Where the last name starts, with its annotations.
fn last_name_start(ty: &QualifiedClassType<'_>) -> u32 {
    ty.annotations.first().map_or(ty.name.span.start, |first| first.span.start)
}

/// One name of a class type, with its annotations and type arguments. The
/// comments before a `created` class's type arguments stay before the `<`.
/// Before any other type's `<`, those on the name's line trail the name and
/// those on later lines lead the type arguments, so a block comment that ends
/// its line prints as it does once joined to the `<`.
pub fn format_class_type_name<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    annotations: &AstNode<'_, 'a, Vec<'a, Annotation<'a>>>,
    name: &(impl Format<'a, JavaFormatContext<'a>> + GetSpan),
    args: Option<AstNode<'_, 'a, TypeArguments<'a>>>,
    created: bool,
) {
    format_type_annotations(f, annotations);
    let Some(args) = args else {
        name.fmt(f);
        return;
    };
    let end = name.span().end;
    format_content_without_comments_after(name, end, f);
    if created {
        format_after_comments_before_opener(end, b'<', &args, f);
    } else {
        let comments = f.comments().comments_before_character(end, b'<');
        let same_line =
            comments.iter().take_while(|comment| !comment.preceded_by_newline()).count();
        FormatTrailingComments::Comments(&comments[..same_line]).fmt(f);
        args.fmt(f);
    }
}
