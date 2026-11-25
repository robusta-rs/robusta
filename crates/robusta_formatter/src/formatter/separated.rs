use crate::formatter::prelude::*;
use robusta_formatter_core::{GroupId, format_args, write};
use robusta_span::GetSpan;

/// A bracketed list's frame: `open`, the entries one level deeper, and
/// `close`, breaking together as the group `group_id`.
pub fn format_delimited_frame<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    open: &'static str,
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

/// An undelimited list whose entries own their separators and comments,
/// with wrapped lines as [`wrapped_indent`] places them.
pub fn format_undelimited_list<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    dart: bool,
    count: usize,
    render: impl FnMut(&mut JavaFormatter<'_, 'ast>, usize),
) {
    write!(
        f,
        [group(&wrapped_indent(
            dart,
            &format_once(|f| {
                format_list_entries(f, count, render);
            }),
        ))]
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
