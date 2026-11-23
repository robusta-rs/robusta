//! Java line terminators: LF, CR, and CRLF (JLS §3.4).

use robusta_span::Span;

/// Whether `byte` starts a line terminator: `\n`, or the `\r` of `\r` /
/// `\r\n`.
#[inline]
pub const fn is_line_terminator(byte: u8) -> bool {
    matches!(byte, b'\n' | b'\r')
}

/// Byte length of the line terminator starting at `index`: 2 for `\r\n`,
/// 1 for `\n` or a lone `\r`, and 0 when `index` is not at a terminator.
///
/// `source[index..index + terminator_len(source, index)]` therefore always
/// steps over exactly one logical line break, or stays in place.
#[inline]
pub fn terminator_len(source: &str, index: usize) -> usize {
    match source.as_bytes().get(index) {
        Some(b'\r') if source.as_bytes().get(index + 1) == Some(&b'\n') => 2,
        Some(b'\n' | b'\r') => 1,
        _ => 0,
    }
}

/// Whether `text` contains any line terminator.
#[inline]
pub fn contains_line_terminator(text: &str) -> bool {
    text.bytes().any(is_line_terminator)
}

/// Number of line terminators in `text`, counting `\r\n` once.
pub fn terminator_count(text: &str) -> usize {
    split_lines(text).count() - 1
}

/// Byte offset of the first character of the line containing `offset`.
// Spans bound source offsets to `u32`, so positions computed from an in-span
// offset cannot truncate.
#[expect(clippy::cast_possible_truncation)]
pub fn line_start(source: &str, offset: u32) -> u32 {
    source.as_bytes()[..offset as usize]
        .iter()
        .rposition(|&byte| is_line_terminator(byte))
        .map_or(0, |position| position + 1) as u32
}

/// Byte offset where the content of the line containing `offset` ends,
/// excluding the line's terminator.
// Spans bound source offsets to `u32`, so positions computed from an in-span
// offset cannot truncate.
#[expect(clippy::cast_possible_truncation)]
pub fn line_content_end(source: &str, offset: u32) -> u32 {
    source.as_bytes()[offset as usize..]
        .iter()
        .position(|&byte| is_line_terminator(byte))
        .map_or(source.len(), |position| offset as usize + position) as u32
}

/// The line containing `offset`, without its terminator.
pub fn line_span(source: &str, offset: u32) -> Span {
    Span::new(line_start(source, offset), line_content_end(source, offset))
}

/// Split `text` into lines on `\n`, `\r`, and `\r\n`, yielding each line
/// without its terminator.
///
/// Always yields at least one line; text ending in a terminator yields a
/// final empty line, like [`str::split`].
pub fn split_lines(text: &str) -> SplitLines<'_> {
    SplitLines { text, position: 0, finished: false }
}

/// Split `text` into lines on `\n`, `\r`, and `\r\n`, retaining each line's
/// terminator.
///
/// Empty input and text ending in a terminator do not yield an extra empty
/// line. This is the endings-preserving counterpart to [`split_lines`].
pub fn split_lines_with_endings(text: &str) -> SplitLinesWithEndings<'_> {
    SplitLinesWithEndings { text, position: 0 }
}

/// Iterator returned by [`split_lines`].
#[derive(Debug, Clone)]
pub struct SplitLines<'text> {
    text: &'text str,
    position: usize,
    finished: bool,
}

impl<'text> Iterator for SplitLines<'text> {
    type Item = &'text str;

    fn next(&mut self) -> Option<&'text str> {
        if self.finished {
            return None;
        }
        let bytes = self.text.as_bytes();
        let mut end = self.position;
        while end < bytes.len() && !is_line_terminator(bytes[end]) {
            end += 1;
        }
        let line = &self.text[self.position..end];
        if end == bytes.len() {
            self.finished = true;
        }
        self.position = end + terminator_len(self.text, end);
        Some(line)
    }
}

/// Iterator returned by [`split_lines_with_endings`].
#[derive(Debug, Clone)]
pub struct SplitLinesWithEndings<'text> {
    text: &'text str,
    position: usize,
}

impl<'text> Iterator for SplitLinesWithEndings<'text> {
    type Item = &'text str;

    fn next(&mut self) -> Option<&'text str> {
        if self.position == self.text.len() {
            return None;
        }
        let bytes = self.text.as_bytes();
        let mut end = self.position;
        while end < bytes.len() && !is_line_terminator(bytes[end]) {
            end += 1;
        }
        end += terminator_len(self.text, end);
        let line = &self.text[self.position..end];
        self.position = end;
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminator_len_steps_over_each_spelling_once() {
        assert_eq!(terminator_len("a\nb", 1), 1);
        assert_eq!(terminator_len("a\rb", 1), 1);
        assert_eq!(terminator_len("a\r\nb", 1), 2);
        assert_eq!(terminator_len("ab", 1), 0);
        assert_eq!(terminator_len("a", 1), 0);
    }

    #[test]
    fn line_span_covers_the_line_without_its_terminator() {
        let source = "one\r\ntwo\rthree";
        assert_eq!(line_span(source, 0), Span::new(0, 3));
        assert_eq!(line_span(source, 6), Span::new(5, 8));
        assert_eq!(line_span(source, 14), Span::new(9, 14));
        assert_eq!(line_span("", 0), Span::new(0, 0));
    }

    #[test]
    fn split_lines_treats_every_terminator_spelling_as_one_break() {
        let lines: Vec<&str> = split_lines("a\nb\rc\r\nd").collect();
        assert_eq!(lines, ["a", "b", "c", "d"]);
    }

    #[test]
    fn split_lines_yields_a_final_empty_line_after_a_trailing_terminator() {
        let lines: Vec<&str> = split_lines("a\r\n").collect();
        assert_eq!(lines, ["a", ""]);
        let lines: Vec<&str> = split_lines("").collect();
        assert_eq!(lines, [""]);
    }

    #[test]
    fn split_lines_with_endings_retains_each_terminator_spelling() {
        let lines: Vec<&str> = split_lines_with_endings("a\nb\rc\r\nd").collect();
        assert_eq!(lines, ["a\n", "b\r", "c\r\n", "d"]);
        assert_eq!(split_lines_with_endings("a\r\n").collect::<Vec<_>>(), ["a\r\n"]);
        assert!(split_lines_with_endings("").next().is_none());
    }

    #[test]
    fn terminator_count_counts_crlf_once() {
        assert_eq!(terminator_count("a\r\nb\nc\r"), 3);
        assert_eq!(terminator_count("abc"), 0);
        assert!(!contains_line_terminator("abc"));
        assert!(contains_line_terminator("a\rc"));
    }
}
