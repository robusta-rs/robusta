// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_formatter/src/source_text.rs

//! Java source-text scanning that depends on Java lexical rules.
//!
//! These extend the language-agnostic [`SourceText`], which owns only
//! mechanical byte and offset access, with Java lexis: line terminators and
//! whitespace after Unicode escape translation (JLS §3.3).

use robusta_ast::Comment;
use robusta_formatter_core::SourceText;
use robusta_span::Span;
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::contains_line_terminator,
    unicode_escape::{SourceBytes, skip_whitespace, unicode_escape},
};

/// Where the first `character` in `text` ends, written as itself or as a
/// Unicode escape.
pub fn character_end(text: &str, character: u8) -> Option<usize> {
    let bytes = text.as_bytes();
    let (mut position, mut eligible) = (0, true);
    while let Some(&byte) = bytes.get(position) {
        let escape = unicode_escape(bytes, position).filter(|_| eligible);
        let (value, end) = escape.unwrap_or((u32::from(byte), position + 1));
        if value == u32::from(character) {
            return Some(end);
        }
        // A backslash that begins no escape makes the next one ineligible.
        eligible = escape.is_some() || byte != b'\\' || !eligible;
        position = end;
    }
    None
}

/// The line terminators in `text`, written as themselves or as Unicode
/// escapes, with `\r\n` as one.
pub fn line_breaks(text: &str) -> impl Iterator<Item = u8> + '_ {
    let mut previous = 0;
    SourceBytes::new(text).map(|(byte, _)| byte).filter(move |&byte| {
        let is_break = matches!(byte, b'\r' | b'\n') && !(byte == b'\n' && previous == b'\r');
        previous = byte;
        is_break
    })
}

/// Java-specific source scanning, layered on the core mechanical [`SourceText`].
pub trait SourceTextExt {
    /// Check if range contains line terminators.
    fn contains_newline_between(&self, start: u32, end: u32) -> bool;

    /// Whether only horizontal whitespace follows `position` on its line.
    fn ends_line(&self, position: u32) -> bool;

    /// Where the `count` characters from `from` end, each written as itself or
    /// as a Unicode escape.
    fn characters_end(&self, from: u32, count: usize) -> u32;

    /// Whether every character between `start` and `end`, each written as
    /// itself or as a Unicode escape, satisfies `predicate`.
    fn all_characters_match(&self, start: u32, end: u32, predicate: impl Fn(u8) -> bool) -> bool;

    /// The run of `comments` after `from` with no line breaks between them.
    fn comments_on_line_after<'c>(&self, comments: &'c [Comment], from: u32) -> &'c [Comment];

    /// The line terminators in the gap after a comment that a following node's
    /// leading pass claims, up to the first other token before `limit`. A `;`,
    /// `,` or parenthesis in the gap prints elsewhere, so it is
    /// transparent, and one alone on its line takes that line with it.
    /// A closing parenthesis may be written as a Unicode escape.
    ///
    /// `None` counts up to the next token, skipping nothing, and gives `0`
    /// at the end of the text.
    fn lines_after_skipping_terminators(&self, end: u32, limit: Option<u32>) -> usize;

    /// The line breaks in the whitespace before `span`, or before the first
    /// unprinted comment when it ends before `span` and so leads it.
    ///
    /// The scan starts after the last byte that cannot belong to whitespace or a
    /// Unicode escape, so escaped line terminators count.
    fn get_lines_before(&self, span: Span, first_unprinted_comment: Option<Span>) -> usize;
}

impl SourceTextExt for SourceText<'_> {
    fn contains_newline_between(&self, start: u32, end: u32) -> bool {
        if start >= end {
            return false;
        }
        let text = self.slice_range(start, end);
        if !text.as_bytes().contains(&b'\\') {
            return contains_line_terminator(text);
        }
        SourceBytes::new(text).any(|(byte, _)| matches!(byte, b'\r' | b'\n'))
    }

    fn ends_line(&self, position: u32) -> bool {
        SourceBytes::new(&self[position as usize..])
            .find(|&(byte, _)| !is_horizontal_whitespace(byte))
            .is_none_or(|(byte, _)| matches!(byte, b'\r' | b'\n'))
    }

    #[expect(clippy::cast_possible_truncation, reason = "parsed source spans fit u32")]
    fn characters_end(&self, from: u32, count: usize) -> u32 {
        let bytes = self.as_bytes();
        let mut position = from as usize;
        for _ in 0..count {
            position = unicode_escape(bytes, position).map_or(position + 1, |(_, end)| end);
        }
        position as u32
    }

    fn all_characters_match(&self, start: u32, end: u32, predicate: impl Fn(u8) -> bool) -> bool {
        SourceBytes::new(self.slice_range(start, end)).all(|(byte, _)| predicate(byte))
    }

    fn comments_on_line_after<'c>(&self, comments: &'c [Comment], from: u32) -> &'c [Comment] {
        let mut end = from;
        for (index, comment) in comments.iter().enumerate() {
            if skip_whitespace(self, end, is_horizontal_whitespace) != comment.span.start {
                return &comments[..index];
            }
            end = comment.span.end;
        }
        comments
    }

    fn lines_after_skipping_terminators(&self, end: u32, limit: Option<u32>) -> usize {
        let text = match limit {
            Some(limit) if end >= limit => return 0,
            Some(limit) => self.slice_range(end, limit),
            None => &self[end as usize..],
        };
        let mut count = 0;
        // One break behind a skipped separator merges into the one before it.
        let mut discount_next = false;
        let mut previous_cr = false;
        let mut bytes = SourceBytes::new(text);
        while let Some((byte, _)) = bytes.next_with_escapes(|byte| {
            byte.is_ascii_whitespace() || (limit.is_some() && byte == b')')
        }) {
            match byte {
                b';' | b',' | b'(' | b')' if limit.is_some() => discount_next = count > 0,
                b'\n' if previous_cr => {}
                b'\r' | b'\n' if discount_next => discount_next = false,
                b'\r' | b'\n' => count += 1,
                byte if !byte.is_ascii_whitespace() => return count,
                _ => {}
            }
            previous_cr = byte == b'\r';
        }
        if limit.is_some() { count } else { 0 }
    }

    fn get_lines_before(&self, span: Span, first_unprinted_comment: Option<Span>) -> usize {
        let mut start = span.start;
        // Should skip the leading comments of the node.
        if let Some(comment) = first_unprinted_comment
            && comment.end <= start
        {
            start = comment.start;
        }
        let bytes = &self.as_bytes()[..start as usize];
        let scan_start = bytes
            .iter()
            .rposition(|&byte| {
                !(byte.is_ascii_whitespace()
                    || byte.is_ascii_hexdigit()
                    || matches!(byte, b'\\' | b'u'))
            })
            .map_or(0, |index| index + 1);
        if scan_start == bytes.len() {
            return 0;
        }
        let mut count = 0;
        let mut previous_cr = false;
        let mut after_token = scan_start > 0;
        for (byte, _) in SourceBytes::new(&self[scan_start..start as usize]) {
            match byte {
                b'\n' if previous_cr => {}
                b'\r' | b'\n' => count += 1,
                byte if !byte.is_ascii_whitespace() => {
                    count = 0;
                    after_token = true;
                }
                _ => {}
            }
            previous_cr = byte == b'\r';
        }
        // Nothing comes before the first line of the file.
        if after_token { count } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The line breaks [`SourceTextExt::get_lines_before`] counts before `b`
    /// in `a{gap}b`.
    fn lines_before_b(gap: &str) -> usize {
        let source = format!("a{gap}b");
        let start = u32::try_from(source.len() - 1).unwrap();
        SourceText::new(&source).get_lines_before(Span::new(start, start + 1), None)
    }

    /// Whether [`SourceTextExt::contains_newline_between`] finds a line
    /// terminator in all of `gap`.
    fn contains_newline(gap: &str) -> bool {
        SourceText::new(gap).contains_newline_between(0, u32::try_from(gap.len()).unwrap())
    }

    #[test]
    fn lines_count_the_gaps_around_each_declaration() {
        for (source, before, after) in [
            ("int x = 1;\n\nint y = 2;\n\n\nint z = 3;\n", [0, 2, 3], [2, 3, 0]),
            ("int x = 1;\r\n\r\nint y = 2;\r\n\r\n\r\nint z = 3;\r\n", [0, 2, 3], [2, 3, 0]),
            ("int x = 1;\n\r\nint y = 2;\r\n\nint z = 3;\n", [0, 2, 2], [2, 2, 0]),
        ] {
            let source_text = SourceText::new(source);
            let spans = ["x", "y", "z"].map(|name| {
                let start = u32::try_from(source.find(&format!("int {name}")).unwrap()).unwrap();
                Span::new(start, start + 10)
            });
            let lines_before = spans.map(|span| source_text.get_lines_before(span, None));
            let lines_after =
                spans.map(|span| source_text.lines_after_skipping_terminators(span.end, None));
            assert_eq!((lines_before, lines_after), (before, after), "{source:?}");
        }
    }

    #[test]
    fn logical_lines_coalesce_raw_and_escaped_crlf() {
        for gap in [r"\u000a", r"\uu000D", r"\uuu000d\u000A", "\\u000d\n", "\r\\uu000a"] {
            assert!(contains_newline(gap), "{gap:?}");
            assert_eq!(lines_before_b(gap), 1, "{gap:?}");
            assert_eq!(line_breaks(gap).count(), 1, "{gap:?}");
        }
        assert_eq!(lines_before_b(r"\u000d\u000a\u0020\u000a"), 2);
        assert_eq!(line_breaks(r"\u000d\u000a\u0020\u000a").count(), 2);
    }

    #[test]
    fn bounded_leading_comment_scan_skips_an_escaped_closing_paren() {
        for separator in [")", r"\u0029"] {
            let gap = format!("{separator}\r\nx");
            let limit = u32::try_from(gap.len() - 1).unwrap();
            let source = SourceText::new(&gap);
            assert_eq!(source.lines_after_skipping_terminators(0, Some(limit)), 1);
            assert_eq!(source.lines_after_skipping_terminators(0, None), 0);
        }
    }

    #[test]
    fn line_terminator_scans_ignore_ineligible_escapes() {
        for gap in [r"\\u000a", r"\u005cu000a", r"\u000", r"\u2028", r"\u0029"] {
            assert!(!contains_newline(gap), "{gap:?}");
            assert_eq!(lines_before_b(gap), 0, "{gap:?}");
            assert_eq!(line_breaks(gap).count(), 0, "{gap:?}");
        }
        assert!(contains_newline(r"\\\u000a"));
    }

    #[test]
    fn newline_checks_treat_cr_and_crlf_as_line_terminators() {
        assert!(contains_newline("a\r"));
        assert!(contains_newline("a\r\nb"));
        assert_eq!(line_breaks(",\r\n// c\r/* d\n */").count(), 3);
    }

    #[test]
    fn character_scans_read_unicode_escapes() {
        for gap in ["|", r" \u007c", r"\uu007C", r"\u0020\u007c", r"\u005c\u007c"] {
            assert!(character_end(gap, b'|').is_some(), "{gap:?}");
        }
        assert_eq!(character_end(r" \uu007C |", b'|'), Some(8));
        for gap in ["", r"\\u007c", r"\u005cu007c", r"\u007", r"\u007d"] {
            assert_eq!(character_end(gap, b'|'), None, "{gap:?}");
        }
    }
}
