use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::trivia::FormatLeadingComments;
use crate::print::call_like_expression::arguments::format_arguments;
use crate::print::member_expression::{
    format_array_access_without_array, format_member_separator, format_member_target_comments,
};
use crate::print::type_parameters::format_type_arguments_before_name;
use crate::utils::statement_body::{format_callee, write_comments_before_opener};
use robusta_ast::{ArrayAccess, Comment, Expression, FieldAccess, MethodCall, MethodMember};
use robusta_span::{GetSpan, Span};

use super::MemberKind;

#[derive(Clone, Copy)]
pub enum MemberChainSegment<'p, 'a> {
    Field(&'p AstNode<'p, 'a, FieldAccess<'a>>),
    Method(&'p AstNode<'p, 'a, MethodCall<'a>>, &'p AstNode<'p, 'a, MethodMember<'a>>),
    Array(&'p AstNode<'p, 'a, ArrayAccess<'a>>),
}

impl<'p, 'a> MemberChainSegment<'p, 'a> {
    /// The segment of a call whose callee is a member.
    pub fn method(call: &'p AstNode<'p, 'a, MethodCall<'a>>, f: &JavaFormatter<'_, 'a>) -> Self {
        let member = call.callee().as_member().expect("a member callee");
        Self::Method(call, member.in_arena(f.allocator()))
    }

    pub(super) fn kind(&self) -> MemberKind {
        match self {
            MemberChainSegment::Field(_) => MemberKind::Field,
            MemberChainSegment::Method(..) => MemberKind::Call,
            MemberChainSegment::Array(access) => {
                MemberKind::Index { literal: matches!(access.index, Expression::IntLiteral(_)) }
            }
        }
    }

    /// Whether a comment sits between the target and the name. The comments
    /// before an array access's `[` stay on the target's side.
    pub(super) fn has_comment(&self, f: &JavaFormatter<'_, 'a>) -> bool {
        if matches!(self, MemberChainSegment::Array(_)) {
            return false;
        }
        let target_end = self.target().span().end;
        f.comments().all_comments_in_range(target_end, self.operand_start()).next().is_some()
    }

    /// Format the comments between the target and this member that stay on
    /// the target's side.
    pub(super) fn format_target_comments(&self, f: &mut JavaFormatter<'_, 'a>) {
        let target = self.target().span();
        if let MemberChainSegment::Array(_) = self {
            write_comments_before_opener(target.end, b'[', f);
        } else {
            format_member_target_comments(target, self.operand_start(), f);
        }
    }

    /// Format the member after its target, with the comments that lead it.
    pub(super) fn format(&self, f: &mut JavaFormatter<'_, 'a>) {
        if let MemberChainSegment::Array(access) = self {
            format_array_access_without_array(f, access);
            return;
        }
        let leading = f.comments().comments_before(self.operand_start());
        format_member_chain_segment(f, *self, leading);
    }

    pub(super) fn target(&self) -> AstNode<'p, 'a, Expression<'a>> {
        match *self {
            MemberChainSegment::Field(access) => access.target(),
            MemberChainSegment::Method(_, member) => member.target(),
            MemberChainSegment::Array(access) => access.array(),
        }
    }

    /// Where the segment starts after its `.`: at explicit type arguments'
    /// `<`, or at the name.
    pub(super) fn operand_start(&self) -> u32 {
        match self {
            MemberChainSegment::Field(access) => access.name.span.start,
            MemberChainSegment::Method(_, member) => {
                member.type_args.as_ref().map_or(member.name.span.start, |args| args.span.start)
            }
            MemberChainSegment::Array(_) => unreachable!("array accesses have no name"),
        }
    }

    pub(super) fn span(&self) -> Span {
        match self {
            MemberChainSegment::Field(access) => access.span,
            MemberChainSegment::Method(call, _) => call.span,
            MemberChainSegment::Array(access) => access.span,
        }
    }

    pub(super) fn parent(&self) -> &'p AstNodes<'p, 'a> {
        match self {
            MemberChainSegment::Field(access) => access.parent(),
            MemberChainSegment::Method(call, _) => call.parent(),
            MemberChainSegment::Array(access) => access.parent(),
        }
    }
}

fn format_member_chain_segment<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    segment: MemberChainSegment<'_, 'a>,
    leading: &[Comment],
) {
    if let Some(first) = leading.first() {
        hard_line_break().fmt(f);
        // Drop the blank line after a standalone call.
        if f.lines_before(first.span) > 1
            && !matches!(segment.target().inner(), Expression::MethodCall(call) if call.member().is_none())
        {
            empty_line().fmt(f);
        }
        FormatLeadingComments::Comments(leading).fmt(f);
    }
    format_member_separator(".", segment.operand_start(), f);
    match segment {
        MemberChainSegment::Field(access) => access.name().fmt(f),
        MemberChainSegment::Array(_) => {
            unreachable!("array accesses format their brackets together")
        }
        MemberChainSegment::Method(call, member) => {
            if let Some(type_args) = member.type_args() {
                format_type_arguments_before_name(f, &type_args);
            }
            format_callee(&member.name(), f);
            format_arguments(f, &call.args(), call.span.end);
        }
    }
}
