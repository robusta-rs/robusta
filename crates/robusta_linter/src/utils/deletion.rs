use robusta_span::Span;
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{contains_line_terminator, line_content_end, line_start, terminator_len},
};

use crate::Fix;

/// The span deleting `item`, with its lines when nothing else is on them,
/// otherwise with the whitespace on one side.
// Parser spans are bounded to `u32`, so the source positions converted back
// to `Span` offsets below cannot truncate.
#[expect(clippy::cast_possible_truncation)]
pub fn removal_span(source: &str, item: Span) -> Span {
    if let Some(lines) = whole_lines(source, item) {
        return lines;
    }
    let bytes = source.as_bytes();
    let line_start = line_start(source, item.start) as usize;
    let line_end = line_content_end(source, item.end) as usize;
    let leading = source[line_start..item.start as usize].bytes().all(is_horizontal_whitespace);

    let mut start = item.start as usize;
    while !leading && start > line_start && is_horizontal_whitespace(bytes[start - 1]) {
        start -= 1;
    }
    if start < item.start as usize {
        return Span::new(start as u32, item.end);
    }

    // `@SuppressWarnings("robusta:x") // reason` -> `// reason`
    let mut end = item.end as usize;
    while end < line_end && is_horizontal_whitespace(bytes[end]) {
        end += 1;
    }
    if !leading && source[end..line_end].starts_with("//") {
        end = item.end as usize;
    }
    Span::new(item.start, end as u32)
}

/// Merge the overlapping or touching deletions among `edits`, and widen a
/// deletion that leaves its line blank to the whole line.
pub fn normalize_deletions(source: &str, edits: &mut Vec<Fix>) {
    coalesce_deletions(edits);
    for edit in &mut *edits {
        // One that already spans a line break may end at a line start, where
        // widening would take the next line too.
        if edit.content.is_empty() && !contains_line_terminator(edit.span.source_text(source)) {
            edit.span = whole_lines(source, edit.span).unwrap_or(edit.span);
        }
    }
    coalesce_deletions(edits);
}

fn coalesce_deletions(edits: &mut Vec<Fix>) {
    edits.sort_by_key(|edit| (edit.span.start, edit.span.end));
    let mut normalized = Vec::<Fix>::with_capacity(edits.len());
    for edit in edits.drain(..) {
        if let Some(previous) = normalized.last_mut()
            && previous.content.is_empty()
            && edit.content.is_empty()
            && edit.span.start <= previous.span.end
        {
            previous.span.end = previous.span.end.max(edit.span.end);
        } else {
            normalized.push(edit);
        }
    }
    *edits = normalized;
}

/// The lines `span` is on, with the last one's terminator, when only
/// horizontal whitespace surrounds it there.
// Parser spans are bounded to `u32`, so the line offsets cannot truncate.
#[expect(clippy::cast_possible_truncation)]
fn whole_lines(source: &str, span: Span) -> Option<Span> {
    let line_start = line_start(source, span.start) as usize;
    let line_end = line_content_end(source, span.end) as usize;
    let before = &source[line_start..span.start as usize];
    let after = &source[span.end as usize..line_end];
    if !before.bytes().all(is_horizontal_whitespace) || !after.bytes().all(is_horizontal_whitespace)
    {
        return None;
    }
    let end = line_end + terminator_len(source, line_end);
    Some(Span::new(line_start as u32, end as u32))
}
