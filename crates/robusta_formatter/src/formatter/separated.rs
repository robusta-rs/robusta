use crate::formatter::prelude::*;
use crate::formatter::trivia::{format_dangling_comments, format_opening_paren_comment};
use robusta_formatter_core::{GroupId, format_args, write};
use robusta_span::{GetSpan, Span};

/// A bracketed list's frame: `open`, the entries one level deeper, and
/// `close`, breaking together as the group `group_id`.
pub fn format_delimited_frame<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    open: impl Format<'ast, JavaFormatContext<'ast>>,
    close: &'static str,
    group_id: Option<GroupId>,
    entries: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    write!(
        f,
        [group(&format_args!(open, soft_block_indent(&format_once(entries)), close))
            .with_group_id(group_id)]
    );
}

/// [`format_delimited_frame`] in parentheses whose entries start at
/// `entries_start`. A line comment on the `(`'s line stays there, and the
/// frame breaks.
pub fn format_parenthesized_frame<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    entries_start: u32,
    group_id: Option<GroupId>,
    entries: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    let open = format_with(|f| {
        "(".fmt(f);
        format_opening_paren_comment(entries_start, f);
    });
    format_delimited_frame(f, open, ")", group_id, entries);
}

/// What a `(` holds when a line comment before `content_start` stays on the
/// `(`'s line: the comment, then `content` one level deeper, breaking as one
/// group so `)` starts a line of its own. `content` stays on one line where it
/// fits. Returns `false`, printing nothing, when no such comment is pending.
pub fn format_after_opening_paren_comment<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    content_start: u32,
    content: &impl Format<'ast, JavaFormatContext<'ast>>,
) -> bool {
    if f.comments().opening_paren_line_comment(content_start).is_empty() {
        return false;
    }
    let comments = format_with(|f| {
        format_opening_paren_comment(content_start, f);
    });
    write!(f, [group(&format_args!(comments, soft_block_indent(&group(content))))]);
    true
}

/// Empty parentheses ending at `span.end`, which keep the comments between
/// them, so `f(/* why */)` does not push the comment out of the parentheses.
/// A line comment on the `(`'s line stays there, with `)` on a line of its own.
pub fn format_empty_parentheses(f: &mut JavaFormatter<'_, '_>, span: Span) {
    "(".fmt(f);
    if format_opening_paren_comment(span.end, f) {
        write!(
            f,
            [format_dangling_comments(span).with_soft_block_indent(), line_suffix_boundary(), ")"]
        );
    } else {
        write!(f, [format_dangling_comments(span).with_soft_block_indent(), ")"]);
    }
}

/// An undelimited list whose entries own their separators and comments,
/// with wrapped lines one level deeper.
pub fn format_undelimited_list<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    count: usize,
    render: impl FnMut(&mut JavaFormatter<'_, 'ast>, usize),
) {
    write!(
        f,
        [group(&indent(&format_once(|f| {
            format_list_entries(f, count, render);
        })))]
    );
}

fn format_list_entries<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    count: usize,
    mut render: impl FnMut(&mut JavaFormatter<'_, 'ast>, usize),
) {
    for index in 0..count {
        if index > 0 {
            soft_line_break_or_space().fmt(f);
        }
        render(f, index);
    }
}

/// The source end of an argument list's `)`, including an empty list.
pub fn argument_list_end<T: GetSpan>(f: &JavaFormatter<'_, '_>, items: &[T], open: u32) -> u32 {
    let items_end = items.last().map_or(open, |item| item.span().end);
    f.comments().position_after_character(items_end, b')')
}
