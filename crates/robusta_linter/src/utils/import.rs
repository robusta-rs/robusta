use robusta_span::Span;
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, terminator_len},
};

use crate::utils::follows_documentation;
use crate::{Fix, LintContext};

/// Delete an import together with its own physical line. Returns none unless
/// the import occupies its line alone (only whitespace around it), so the
/// deletion is a single obvious edit that cannot disturb an adjacent import,
/// its separators, or a comment. Also returns none after a documentation
/// comment, which the deletion could hand to the next declaration.
// A terminator is at most two bytes, so its length always fits `u32`.
#[expect(clippy::cast_possible_truncation)]
pub fn delete_import_line(context: &LintContext<'_, '_, '_>, import: Span) -> Option<Fix> {
    let source = context.source_text();
    let bytes = source.as_bytes();

    // Extend left over the line's leading indentation; only horizontal
    // whitespace may sit between the line start and the `import` keyword.
    let mut start = import.start;
    while start > 0 && is_horizontal_whitespace(bytes[start as usize - 1]) {
        start -= 1;
    }
    // Anything other than a line break (or start of file) before the indentation
    // means the import shares its line with other content: ambiguous, no fix.
    if !(start == 0 || is_line_terminator(bytes[start as usize - 1])) {
        return None;
    }

    // Extend right over trailing horizontal whitespace, then consume exactly one
    // line terminator. A non-whitespace tail (a trailing comment, another
    // import) makes the edit ambiguous.
    let mut end = import.end;
    while (end as usize) < bytes.len() && is_horizontal_whitespace(bytes[end as usize]) {
        end += 1;
    }
    match bytes.get(end as usize) {
        // EOF: the final line carries no terminator to remove.
        None => {}
        Some(&byte) if is_line_terminator(byte) => {
            end += terminator_len(source, end as usize) as u32;
        }
        // Non-whitespace after the import on its own line.
        Some(_) => return None,
    }

    let span = Span::new(start, end);
    // The whitespace guards above already exclude a comment around the import;
    // keep the check to guarantee no comment loses its owner where one is written
    // inside the declaration itself (`import /* x */ a.b.C;`).
    if context.has_comments_between(span) {
        return None;
    }
    // Even before another import, since a fix may delete that one too.
    if follows_documentation(context.comments(), import) {
        return None;
    }
    Some(Fix::delete(span))
}
