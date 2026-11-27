// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/print/member_expression.rs

use crate::JavaLabels;
use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::separated::format_delimited_frame;
use crate::formatter::trivia::{format_hoisted_leading_comments, format_leading_comments};
use crate::print::names::format_dotted_expression_name;
use crate::print::{FormatWrite, format_wrapped_continuation};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::is_header_condition;
use crate::utils::statement_body::{write_comments_before_opener, write_comments_between_blocks};
use robusta_ast::{ArrayAccess, Expression, FieldAccess};
use robusta_formatter_core::{FormatElements, LabelId, format_args, write};
use robusta_span::{GetSpan, Span};

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ArrayAccess<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let array = self.array();
        let array_end = array.span().end;
        format_content_without_comments_after(&array, array_end, f);
        write_comments_before_opener(array_end, b'[', f);
        format_array_access_without_array(f, self);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, FieldAccess<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.is_dotted_name() {
            format_dotted_expression_name(self, f);
            return;
        }
        let object = self.target();
        let object_span = object.span();
        let is_member_chain = format_member_object(&object, object_span.end, f);
        let layout = MemberLayout::new(Some(object.inner()), self.parent(), is_member_chain);
        let in_header = is_header_condition(self.parent(), self.span);
        let member = Member { separator: ".", start: self.name.span.start, layout, in_header };
        format_member_after_object(object_span, member, &self.name(), f);
    }
}

/// Print the object before a `.` or `::` without the comments after `end`,
/// and return whether it printed as a member chain that can break.
pub fn format_member_object<'a>(
    object: &impl Format<'a, JavaFormatContext<'a>>,
    end: u32,
    f: &mut JavaFormatter<'_, 'a>,
) -> bool {
    let start = f.elements().len();
    format_content_without_comments_after(object, end, f);
    f.elements()[start..].has_label(LabelId::of(JavaLabels::MemberChain))
}

/// `[index]`, after the array that is already printed.
pub fn format_array_access_without_array<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    access: &AstNode<'_, 'a, ArrayAccess<'a>>,
) {
    format_delimited_frame(f, "[", "]", None, |f| access.index().fmt(f));
}

/// The comments between an object and the name after its `.` or `::`:
/// `a. // c\n b` -> `a // c\n.b`. The object's trailing comments and the
/// others before the separator stay on the object's side, and those after it
/// move in front of it when one of them ends its line. Returns `false` when a
/// comment ended the line before the `.` or `::`, so the caller adds no break.
pub fn format_comments_around_member_separator(
    object: Span,
    name_start: u32,
    separator: &str,
    f: &mut JavaFormatter<'_, '_>,
) -> bool {
    let mut needs_separator = format_member_target_comments(object, name_start, f);
    if !f.comments().has_comment_before(name_start) {
        return needs_separator;
    }
    let before = f.comments().comments_before_character(object.end, separator.as_bytes()[0]);
    needs_separator &= write_comments_between_blocks(before, f);
    if f.comments().has_leading_own_line_comment(name_start) {
        write!(f, [hard_line_break(), format_hoisted_leading_comments(Span::empty(name_start))]);
        return false;
    }
    needs_separator
}

pub fn format_member_target_comments(
    object: Span,
    name_start: u32,
    f: &mut JavaFormatter<'_, '_>,
) -> bool {
    // The name is inside the member, so a comment that starts its line leads it.
    let member = Span::new(object.start, name_start + 1);
    let trailing = f.comments().get_trailing_comments(member, object, name_start);
    write_comments_between_blocks(trailing, f)
}

/// Print the `.` or `::`, and the comments that
/// [`format_comments_around_member_separator`] left to lead the member.
pub fn format_member_separator(
    separator: &'static str,
    name_start: u32,
    f: &mut JavaFormatter<'_, '_>,
) {
    write!(f, [separator, format_leading_comments(Span::empty(name_start))]);
}

/// Where the line may break between an object and the member after its `.`
/// or `::`.
#[derive(Debug, Clone, Copy)]
pub enum MemberLayout {
    /// The member stays on its object's line unless nothing else can break.
    NoBreak,
    /// The line breaks after the object when the member does not fit after it.
    BreakAfterObject,
}

impl MemberLayout {
    /// The layout of the member after `object`, which is `None` for a type
    /// before `::`. `parent` holds the member, and `object_is_member_chain`
    /// tells whether `object` prints as a chain that can break.
    pub fn new(
        object: Option<&Expression<'_>>,
        parent: &AstNodes<'_, '_>,
        object_is_member_chain: bool,
    ) -> Self {
        let is_nested = match parent {
            AstNodes::AssignmentExpression(_)
            | AstNodes::VariableDeclarator(_)
            | AstNodes::ResourceDeclaration(_) => {
                let is_call_with_arguments =
                    matches!(object, Some(Expression::MethodCall(call)) if !call.args.is_empty());
                if is_call_with_arguments || object_is_member_chain {
                    return Self::NoBreak;
                }
                false
            }
            _ => is_member_access(parent),
        };

        // A break after a single word would leave it alone on its line for one
        // member.
        if !is_nested
            && matches!(
                object,
                Some(Expression::Identifier(_) | Expression::This(_) | Expression::Super(_))
            )
        {
            return Self::NoBreak;
        }

        let mut ancestor = parent;
        while is_member_access(ancestor) {
            ancestor = ancestor.parent();
        }
        match ancestor {
            // The member is in the head of the chain that calls it, which never
            // breaks.
            AstNodes::MethodMember(_) => Self::NoBreak,
            AstNodes::AssignmentExpression(assignment)
                if !matches!(
                    assignment.target.without_parentheses(),
                    Expression::Identifier(_)
                ) =>
            {
                Self::NoBreak
            }
            _ => Self::BreakAfterObject,
        }
    }
}

fn is_member_access(node: &AstNodes<'_, '_>) -> bool {
    matches!(node, AstNodes::FieldAccess(_) | AstNodes::ArrayAccess(_))
}

/// A member after its object's `.` or `::`.
#[derive(Clone, Copy)]
pub struct Member {
    pub separator: &'static str,
    /// Where the member starts after the separator.
    pub start: u32,
    pub layout: MemberLayout,
    /// Whether the member is in a header's condition, whose wrapped lines
    /// follow dart format.
    pub in_header: bool,
}

/// The comments around the separator after `object`, which is already
/// printed, then the separator and `name`, with the break before them that the
/// member's layout allows. The comments print on the member's wrapped line, so
/// one that ends the line starts the member on the next line, indented as the
/// break would indent it.
pub fn format_member_after_object<'a>(
    object: Span,
    member: Member,
    name: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let Member { separator, start, layout, in_header } = member;
    let member = format_with(|f| {
        format_member_separator(separator, start, f);
        name.fmt(f);
    });
    if !f.comments().has_comment_before(start) {
        format_member_with_layout(layout, in_header, &member, f);
        return;
    }
    let comments_and_member = format_once(|f| {
        if format_comments_around_member_separator(object, start, separator, f) {
            // The line is still open, so the layout's own break indents the member.
            let layout = format_once(|f| format_member_with_layout(layout, in_header, &member, f));
            if in_header { dedent(&dedent(&layout)).fmt(f) } else { dedent(&layout).fmt(f) }
        } else {
            member.fmt(f);
        }
    });
    wrapped_indent(in_header, &comments_and_member).fmt(f);
}

/// The break before `member` that `layout` allows. A [`MemberLayout::NoBreak`]
/// member breaks only when nothing else makes the line fit, so its group holds
/// just the break.
fn format_member_with_layout<'a>(
    layout: MemberLayout,
    in_header: bool,
    member: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    match layout {
        MemberLayout::NoBreak => {
            format_wrapped_continuation(soft_line_break(), member, true, in_header, f);
        }
        MemberLayout::BreakAfterObject => {
            write!(
                f,
                [group(&wrapped_indent(in_header, &format_args!(soft_line_break(), member)))]
            );
        }
    }
}
