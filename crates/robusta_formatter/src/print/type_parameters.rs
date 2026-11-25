// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_formatter/src/print/type_parameters.rs

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::separated::format_delimited_frame;
use crate::formatter::trivia::{
    format_dangling_comments, format_hoisted_leading_comments, format_leading_comments,
};
use crate::print::FormatWrite;
use crate::print::annotations::format_type_annotations;
use crate::print::intersection_type::{format_additional_bound, format_type_bounds};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use robusta_ast::{
    ClassType, TypeArgument, TypeArguments, TypeParameter, TypeParameters, WildcardBound,
    WildcardTypeArgument, match_class_type,
};
use robusta_formatter_core::{format_args, write};
use robusta_span::{GetSpan, Span};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, TypeParameters<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_delimited_frame(f, "<", ">", None, |f| {
            f.join_with(&format_args!(",", soft_line_break_or_space()))
                .entries(self.params().iter());
        });
    }
}

/// A class, interface or record's type parameters, indented twice as far when
/// `has_clauses` puts clauses after them.
pub fn format_type_declaration_parameters<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    type_params: &AstNode<'_, 'a, TypeParameters<'a>>,
    has_clauses: bool,
) {
    let parameters = type_params.params();
    let last = parameters.last().expect("type parameter").span;
    let list = format_once(|f| {
        format_type_header_list(f, type_params.span, last, |f| {
            f.join_with(&format_args!(",", soft_line_break_or_space())).entries(parameters.iter());
        });
    });
    if has_clauses {
        wrapped_indent(true, &list).fmt(f);
    } else {
        list.fmt(f);
    }
}

/// `<`, then a list that breaks after it, stays on the next line if it fits
/// there, and otherwise puts each item on its own line, with `>` after the last.
/// `>` starts a line after a comment that owns its line, or where nothing before
/// it could break to make it fit.
fn format_type_header_list<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    list: Span,
    last: Span,
    entries: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    let close = list.end - 1;
    let comment_owns_line = f
        .comments()
        .all_comments_in_range(last.end, close)
        .any(|&comment| comment.is_line() || comment.preceded_by_newline());
    let closing = format_with(|f| {
        if comment_owns_line {
            write!(f, [hard_line_break(), ">"]);
        } else {
            write!(f, [group(&format_args!(soft_line_break(), ">")).measure_flat()]);
        }
    });
    let entries = format_once(entries);
    write!(
        f,
        [
            "<",
            group(&wrapped_indent(
                true,
                &format_args!(soft_line_break(), group(&format_args!(entries, closing)))
            ))
        ]
    );
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, TypeArguments<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let arguments = self.args();
        let Some(last) = arguments.last().map(GetSpan::span) else {
            // `<>`
            write!(f, ["<", format_dangling_comments(self.span).with_soft_block_indent(), ">"]);
            return;
        };
        let entries = |f: &mut JavaFormatter<'_, 'a>| {
            f.join_with(&format_args!(",", soft_line_break_or_space())).entries(arguments.iter());
        };
        if !is_in_type_declaration_header(self.parent()) {
            // `f<T // c` + `>()`: a comment before `>` needs the broken layout to stay in place.
            let hug = matches!(self.args.as_ref(), [only] if is_simple_type(only))
                && f.comments().all_comments_in_range(last.end, self.span.end).next().is_none();
            if hug {
                // Surrounding breaks are tried first, and the list splits only if they fail.
                write!(
                    f,
                    [group(&format_args!("<", soft_block_indent(&format_once(entries)), ">"))
                        .measure_flat()]
                );
            } else {
                format_delimited_frame(f, "<", ">", None, entries);
            }
            return;
        }
        let Some(components) = f.context().record_components_group() else {
            format_type_header_list(f, self.span, last, entries);
            return;
        };
        // After split components, the list wraps as any other list does.
        let interned = f.intern(&format_once(entries)).expect("a type argument");
        let entries = format_with(|f| f.write_element(interned.clone()));
        let after_split = format_with(|f| {
            format_delimited_frame(f, "<", ">", None, |f| entries.fmt(f));
        });
        let otherwise = format_with(|f| {
            format_type_header_list(f, self.span, last, |f| entries.fmt(f));
        });
        write!(
            f,
            [
                if_group_breaks(&after_split).with_group_id(Some(components)),
                if_group_fits_on_line(&otherwise).with_group_id(Some(components)),
            ]
        );
    }
}

/// Whether `argument` is a primitive type, a class type without type arguments,
/// or `?`, none of them annotated.
fn is_simple_type(argument: &TypeArgument<'_>) -> bool {
    match argument {
        TypeArgument::Wildcard(wildcard) => {
            wildcard.annotations.is_empty() && wildcard.bound.is_none()
        }
        TypeArgument::Primitive(primitive) => primitive.annotations.is_empty(),
        TypeArgument::Array(_) => false,
        it @ match_class_type!(TypeArgument) => {
            it.to_class_type().self_and_qualifiers().all(|ty| match ty {
                ClassType::Simple(ty) => ty.annotations.is_empty() && ty.args.is_none(),
                ClassType::Qualified(ty) => ty.annotations.is_empty() && ty.args.is_none(),
            })
        }
    }
}

/// Type arguments written before a name, as in `this.<T>m()` or `new <T> B()`.
/// The comments after them lead the name rather than trail them.
pub fn format_type_arguments_before_name<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    arguments: &AstNode<'_, 'a, TypeArguments<'a>>,
) {
    format_content_without_comments_after(arguments, arguments.span.end, f);
}

/// Whether a type at `node` is part of a type declaration's type parameters or
/// clauses, rather than a method's or an annotation's inside them.
fn is_in_type_declaration_header(mut node: &AstNodes<'_, '_>) -> bool {
    loop {
        match node {
            AstNodes::SimpleClassType(_)
            | AstNodes::QualifiedClassType(_)
            | AstNodes::TypeArguments(_)
            | AstNodes::ArrayType(_)
            | AstNodes::WildcardTypeArgument(_)
            | AstNodes::WildcardBound(_)
            | AstNodes::TypeParameter(_)
            | AstNodes::TypeParameters(_) => node = node.parent(),
            parent => return is_type_declaration(parent),
        }
    }
}

/// Whether `start` begins the first item of a type declaration's type
/// parameters, or of a type argument list inside its header. `node` is the
/// item's parent.
pub fn starts_type_declaration_list(node: &AstNodes<'_, '_>, start: u32) -> bool {
    let first = match node {
        AstNodes::TypeParameters(parent) => parent
            .params
            .first()
            .map(|first| first.span)
            .filter(|_| is_type_declaration(node.parent())),
        AstNodes::TypeArguments(parent) => {
            parent.args.first().map(GetSpan::span).filter(|_| is_in_type_declaration_header(node))
        }
        _ => None,
    };
    first.is_some_and(|first| first.start == start)
}

fn is_type_declaration(node: &AstNodes<'_, '_>) -> bool {
    matches!(
        node,
        AstNodes::ClassDeclaration(_)
            | AstNodes::InterfaceDeclaration(_)
            | AstNodes::EnumDeclaration(_)
            | AstNodes::RecordDeclaration(_)
    )
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, TypeParameter<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_annotations(f, &self.annotations());
        self.name().fmt(f);
        if let Some(first) = self.bounds.first() {
            let type_declaration = is_type_declaration(self.parent().parent());
            write!(
                f,
                [
                    group(&wrapped_indent(type_declaration, &soft_line_break_or_space()))
                        .measure_flat(),
                    "extends"
                ]
            );
            if type_declaration {
                let bounds = format_once(|f| format_type_declaration_bounds(f, self));
                write!(
                    f,
                    [group(&wrapped_indent(
                        true,
                        &format_args!(
                            line_suffix_boundary(),
                            soft_line_break_or_space(),
                            format_leading_comments(first.span()),
                            wrapped_indent(true, &bounds)
                        )
                    ))]
                );
                return;
            }
            // A comment on a line of its own stays there, so the bound follows it.
            let has_own_line_comment =
                f.comments().has_leading_own_line_comment(first.span().start);
            let group_id = f.state().group_id("bounds");
            write!(
                f,
                [
                    group(&indent(&format_args!(
                        line_suffix_boundary(),
                        soft_line_break_or_space()
                    )))
                    .with_group_id(Some(group_id))
                    .should_expand(has_own_line_comment),
                    indent_if_group_breaks(
                        &format_with(|f| format_type_bounds(f, self.bounds().iter())),
                        group_id
                    )
                ]
            );
        }
    }
}

/// The bounds of a type declaration's type parameter, filled: `& B` stays on the
/// line when it fits and follows a bound that split on a new one.
fn format_type_declaration_bounds<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    parameter: &AstNode<'_, 'a, TypeParameter<'a>>,
) {
    let mut filler = f.fill();
    for (index, bound) in parameter.bounds().iter().enumerate() {
        let separator = format_with(|f| {
            if f.comments().has_leading_own_line_comment(bound.span().start) {
                write!(f, [hard_line_break(), format_hoisted_leading_comments(bound.span())]);
            } else {
                soft_line_break_or_space().fmt(f);
            }
        });
        if index == 0 {
            filler.entry(&separator, &group(&bound));
        } else {
            filler.entry(&separator, &group(&format_with(|f| format_additional_bound(f, &bound))));
        }
    }
    filler.finish();
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, WildcardTypeArgument<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_annotations(f, &self.annotations());
        "?".fmt(f);
        if let Some(bound) = self.bound() {
            write!(f, [space(), bound]);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, WildcardBound<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.kind.as_str().fmt(f);
        if is_in_type_declaration_header(self.parent()) {
            write!(
                f,
                [group(&wrapped_indent(
                    true,
                    &format_args!(soft_line_break_or_space(), self.ty())
                ))]
            );
        } else {
            write!(f, [space(), self.ty()]);
        }
    }
}
