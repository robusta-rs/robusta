// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter/src/utils/member_chain/mod.rs

mod chain_member;
pub mod simple_argument;

pub use chain_member::MemberChainSegment;

use crate::JavaLabels;
use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::print::call_like_expression::arguments::is_function_like;
use crate::source_text::{character_end, line_breaks};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::is_header_condition;
use robusta_ast::{Expression, MethodCall};
use robusta_formatter_core::{
    FormatElement, FormatElements, IndentWidth, LabelId, best_fitting, text_width, write,
};
use robusta_span::GetSpan;
use simple_argument::SimpleArgument;
use smallvec::SmallVec;

/// Format the chain that ends at `root`, the call whose `write` called this.
///
/// The segments are collected without recursion; each one moves into the
/// arena so the segment it targets can keep it as its parent.
pub fn format_member_chain<'a>(
    f: &mut JavaFormatter<'_, 'a>,
    root: &AstNode<'_, 'a, MethodCall<'a>>,
) {
    MemberChain::new(f, root).fmt(f);
}

/// A chain of calls, rooted at its last call: the member accesses after that
/// call print around the chain.
pub struct MemberChain<'p, 'a> {
    head: AstNode<'p, 'a, Expression<'a>>,
    /// The members after the head, in source order.
    segments: SmallVec<[MemberChainSegment<'p, 'a>; 4]>,
    /// Where each group after the head's group starts in `segments`.
    tail: SmallVec<[usize; 4]>,
    layout: MemberChainLayout,
    /// Whether a comment sits between a member and the code before it.
    has_comment: bool,
}

impl<'p, 'a> MemberChain<'p, 'a> {
    pub fn new(f: &JavaFormatter<'_, 'a>, root: &'p AstNode<'p, 'a, MethodCall<'a>>) -> Self {
        let allocator = f.allocator();
        // The common Java chains (`a.b()` and `a.b.c()`) stay entirely on the stack.
        let mut segments = SmallVec::<[MemberChainSegment<'_, 'a>; 4]>::new();
        let root = MemberChainSegment::method(root, f);
        segments.push(root);
        let mut head = root.target();
        loop {
            let segment = match head.inner() {
                Expression::FieldAccess(_) => MemberChainSegment::Field(
                    head.as_field_access().expect("field access").in_arena(allocator),
                ),
                Expression::MethodCall(call) if call.member().is_some() => {
                    MemberChainSegment::method(
                        head.as_method_call().expect("method call").in_arena(allocator),
                        f,
                    )
                }
                Expression::ArrayAccess(_) => MemberChainSegment::Array(
                    head.as_array_access().expect("array access").in_arena(allocator),
                ),
                _ => break,
            };
            segments.push(segment);
            head = segment.target();
        }
        // A head that is not a call takes the member accesses before the first
        // call, so `this.a.b.c(x).d(y)` gains no break at `.a` or `.b`.
        if head_takes_member_accesses(head.inner()) {
            while !matches!(segments.last(), Some(MemberChainSegment::Method(..))) {
                segments.pop();
            }
        }
        if let Some(innermost) = segments.last() {
            head = innermost.target();
        }
        segments.reverse();

        let merges = segments.first().is_some_and(|first| {
            merges_first_group(
                head.inner(),
                first.has_comment(f),
                matches!(root.parent(), AstNodes::ExpressionStatement(_)),
                f.options().indent_width,
            )
        });
        let mut has_comment = head_has_member_comment(head.inner(), |start, end| {
            f.comments().all_comments_in_range(start, end).next().is_some()
        });
        let mut groups = MemberGroups::default();
        let mut tail = SmallVec::new();
        for (index, segment) in segments.iter().enumerate() {
            let comment = segment.has_comment(f);
            has_comment |= comment;
            if groups.push(segment.kind(), comment).is_some_and(|group| group > 0 || !merges) {
                tail.push(index);
            }
        }
        let layout = MemberChainLayout::new(head.inner(), groups.count(), merges);
        Self { head, segments, tail, layout, has_comment }
    }

    pub fn head(&self) -> &Expression<'a> {
        self.head.inner()
    }

    /// The calls in the chain, from the root.
    pub fn calls(&self) -> impl Iterator<Item = &MethodCall<'a>> {
        self.segments
            .iter()
            .rev()
            .filter_map(|segment| match segment {
                MemberChainSegment::Method(call, _) => Some(call.inner()),
                _ => None,
            })
            .chain(match self.head.inner() {
                Expression::MethodCall(call) => Some(&**call),
                _ => None,
            })
    }

    pub fn is_breakable(&self) -> bool {
        self.layout.is_member_call_chain()
    }

    /// Whether the groups break onto lines of their own, as in Oxc: when more
    /// than two calls have one with a complex argument, the head breaks, the
    /// last call breaks after a call with a lambda, or a group but the last
    /// breaks.
    fn groups_should_break(&self, head: &Group<'a>, tail: &[Group<'a>]) -> bool {
        let mut calls = 0;
        let mut has_complex_args = false;
        for call in self.calls() {
            calls += 1;
            has_complex_args = has_complex_args
                || !call.args.iter().all(|argument| SimpleArgument::from(argument).is_simple());
            if calls > 2 && has_complex_args {
                return true;
            }
        }

        if !tail.is_empty() && will_break(head) {
            return true;
        }

        // The chain ends with its root call, which is the first of `calls()`.
        let has_function_like_argument =
            self.calls().skip(1).any(|call| call.args.iter().any(is_function_like));
        let last = tail.last().unwrap_or(head);
        if will_break(last) && has_function_like_argument {
            return true;
        }

        tail.iter().rev().skip(1).any(will_break)
    }

    /// Whether the group starting at `segments[start]` follows a blank line.
    fn needs_empty_line(&self, start: usize, f: &JavaFormatter<'_, 'a>) -> bool {
        let segment = self.segments[start];
        if matches!(segment, MemberChainSegment::Array(_)) || segment.has_comment(f) {
            // A comment keeps its own blank lines.
            return false;
        }
        let target = segment.target();
        let gap = f.source_text().slice_range(target.span().end, segment.operand_start());
        follows_blank_line(target.inner(), gap)
    }
}

/// A group of members, formatted once for every layout that prints it.
type Group<'a> = Option<FormatElement<'a>>;

fn will_break(group: &Group<'_>) -> bool {
    group.as_ref().is_some_and(FormatElement::will_break)
}

/// Where a chain breaks when it does not fit.
pub struct MemberChainLayout {
    /// The groups that start a line when the chain breaks.
    tail: usize,
    merges_first_group: bool,
    head_is_indivisible: bool,
}

impl MemberChainLayout {
    /// The layout of a chain of `group_count` groups after `head`.
    pub fn new(head: &Expression<'_>, group_count: usize, merges_first_group: bool) -> Self {
        Self {
            tail: group_count - usize::from(merges_first_group),
            merges_first_group,
            head_is_indivisible: head_is_indivisible(head),
        }
    }

    /// Whether more than one group can start a line, or one after a head that
    /// no layout can shorten.
    pub fn is_member_call_chain(&self) -> bool {
        self.tail > 1 || (self.tail > 0 && self.head_is_indivisible)
    }

    /// Whether the chain can break. It also can before one group when a
    /// comment or blank line separates its members, which `separated` tells.
    pub fn can_break(&self, separated: bool) -> bool {
        self.is_member_call_chain() || (self.tail > 0 && separated)
    }

    /// Whether group `group` starts a line when the chain breaks.
    pub fn starts_line(&self, group: usize) -> bool {
        group > 0 || !self.merges_first_group
    }
}

/// What a chain member is, as [`MemberGroups`] reads it.
#[derive(Clone, Copy)]
pub enum MemberKind {
    /// `.name`
    Field,
    /// `.name(arguments)`
    Call,
    /// `[index]`, where `literal` tells whether the index is an integer literal.
    Index { literal: bool },
}

/// Numbers a chain's groups in source order. As in Oxc, a field, call, or
/// index other than an integer literal starts a group after a call, so the
/// members between two calls join the next call: `a()`, `.b.c()`, `[i].d()`.
/// A comment before a member also starts one.
pub struct MemberGroups {
    count: usize,
    after_call: bool,
}

impl Default for MemberGroups {
    fn default() -> Self {
        Self { count: 0, after_call: true }
    }
}

impl MemberGroups {
    /// Record the next member, and return the group it starts.
    pub fn push(&mut self, kind: MemberKind, has_comment: bool) -> Option<usize> {
        let starts = has_comment
            || (self.after_call && !matches!(kind, MemberKind::Index { literal: true }));
        match kind {
            MemberKind::Call => self.after_call = true,
            MemberKind::Field | MemberKind::Index { literal: false } => self.after_call = false,
            MemberKind::Index { literal: true } => {}
        }
        starts.then(|| {
            self.count += 1;
            self.count - 1
        })
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// Whether a chain's first group prints on its head's line, as in Oxc: after
/// `this`, a factory name such as `Stream` or `$`, a field with a factory
/// name, or a name no wider than an indent that starts a statement. A comment
/// before the group's member keeps it off.
pub fn merges_first_group(
    head: &Expression<'_>,
    has_comment: bool,
    starts_statement: bool,
    indent_width: IndentWidth,
) -> bool {
    if has_comment {
        return false;
    }
    match head {
        Expression::This(_) | Expression::QualifiedThis(_) => true,
        Expression::Identifier(name) => {
            is_factory(name.name)
                || (starts_statement
                    && text_width(name.name, indent_width) <= u32::from(indent_width.value()))
        }
        Expression::FieldAccess(access) => is_factory(access.name.name),
        _ => false,
    }
}

/// A capitalized name, or one of only `_` and `$`, which likely names a factory.
fn is_factory(name: &str) -> bool {
    let mut bytes = name.bytes();
    match bytes.next() {
        Some(b'_' | b'$') => bytes.all(|b| matches!(b, b'_' | b'$')),
        Some(c) => c.is_ascii_uppercase(),
        None => false,
    }
}

/// Whether `has_comment_between` finds a comment before the name of a field
/// the head takes.
pub fn head_has_member_comment(
    mut head: &Expression<'_>,
    has_comment_between: impl Fn(u32, u32) -> bool,
) -> bool {
    loop {
        head = match head {
            Expression::FieldAccess(access) => {
                if has_comment_between(access.target.span().end, access.name.span.start) {
                    return true;
                }
                &access.target
            }
            Expression::ArrayAccess(access) => &access.array,
            _ => return false,
        };
    }
}

/// Whether `gap`, the whitespace and `.` between `target` and the member after
/// it, holds a blank line the chain keeps: two line breaks on one side of the
/// `.`. The one after a standalone call goes, as in Oxc.
pub fn follows_blank_line(target: &Expression<'_>, gap: &str) -> bool {
    if matches!(target, Expression::MethodCall(call) if call.member().is_none()) {
        return false;
    }
    let (before, after) = gap.split_at(character_end(gap, b'.').unwrap_or(0));
    line_breaks(before).nth(1).is_some() || line_breaks(after).nth(1).is_some()
}

impl<'a> Format<'a, JavaFormatContext<'a>> for MemberChain<'_, 'a> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let format_head = |f: &mut JavaFormatter<'_, 'a>, end: usize| {
            format_content_without_comments_after(&self.head, self.head.span().end, f);
            if let Some(first) = self.segments.first() {
                first.format_target_comments(f);
            }
            self.format_group(f, 0, end);
        };

        let empty_lines: SmallVec<[bool; 4]> =
            self.tail.iter().map(|&start| self.needs_empty_line(start, f)).collect();
        let separated = self.has_comment || empty_lines.contains(&true);
        if !self.layout.can_break(separated) {
            // One line is the only layout. Nothing in it breaks outside its
            // own groups, so it needs neither interning nor a group.
            format_head(f, self.segments.len());
            return;
        }

        let ends = self.tail.iter().copied().skip(1).chain([self.segments.len()]);
        let head = f.intern(&format_once(|f| format_head(f, self.tail[0])));
        let tail: SmallVec<[Group<'a>; 4]> = self
            .tail
            .iter()
            .zip(ends)
            .map(|(&start, end)| f.intern(&format_once(|f| self.format_group(f, start, end))))
            .collect();

        let write_group = |f: &mut JavaFormatter<'_, 'a>, group: &Group<'a>| {
            if let Some(group) = group {
                f.write_element(group.clone());
            }
        };
        let format_one_line = format_with(|f| {
            write_group(f, &head);
            for group in &tail {
                write_group(f, group);
            }
        });

        let root = *self.segments.last().expect("a chain ends with its root call");
        let dart = is_header_condition(root.parent(), root.span());
        let format_tail = format_with(|f| {
            for (group, &empty_line_before) in tail.iter().zip(&empty_lines) {
                if empty_line_before {
                    write!(f, [empty_line()]);
                } else {
                    write!(f, [hard_line_break()]);
                }
                write_group(f, group);
            }
        });
        // As in Oxc, only the members after the head indent.
        let format_expanded = format_with(|f| {
            write_group(f, &head);
            wrapped_indent(dart, &format_tail).fmt(f);
        });

        let format_content = format_with(|f| {
            if separated || self.groups_should_break(&head, &tail) {
                write!(f, [group(&format_expanded)]);
            } else {
                if will_break(tail.last().unwrap_or(&head)) {
                    write!(f, [expand_parent()]);
                }
                write!(f, [best_fitting!(format_one_line, format_expanded)]);
            }
        });
        write!(f, [labelled(LabelId::of(JavaLabels::MemberChain), &format_content)]);
    }
}

impl<'a> MemberChain<'_, 'a> {
    /// Format `segments[start..end]`, each followed by the comments before the
    /// next member that stay on its side.
    fn format_group(&self, f: &mut JavaFormatter<'_, 'a>, start: usize, end: usize) {
        for index in start..end {
            self.segments[index].format(f);
            if let Some(next) = self.segments.get(index + 1) {
                next.format_target_comments(f);
            }
        }
    }
}

/// Whether the member accesses before a chain's first call join its head, as
/// they do unless the head is itself a call.
pub fn head_takes_member_accesses(head: &Expression<'_>) -> bool {
    !matches!(head, Expression::MethodCall(_))
}

/// Whether the chain head is a single token that no layout can shorten.
///
/// Only such a head makes the dot after it the line's one break position, which
/// is what earns a chain of one group the right to break at all. The exception
/// is deliberate: `"long text".formatted(x)` may wrap before `.formatted`.
fn head_is_indivisible(head: &Expression<'_>) -> bool {
    matches!(
        head,
        Expression::IntLiteral(_)
            | Expression::FloatLiteral(_)
            | Expression::CharLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
    )
}
