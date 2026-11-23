use std::borrow::Cow;

use memchr::{memchr, memmem};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_span::Span;

use crate::{diagnostics, offset32, span32};

/// Source after Unicode-escape translation, with a map back to original spans.
pub struct Translated<'a> {
    /// The cooked text the byte-indexed lexer runs over. Borrows the original
    /// verbatim when no eligible Unicode escape was translated.
    pub cooked: Cow<'a, str>,
    /// Run-start anchors `(cooked_offset, original_offset)`, ascending. Between
    /// anchors, cooked and original advance byte-for-byte, so an offset maps by
    /// adding the anchor's constant delta. Empty means the identity map.
    map: Vec<(u32, u32)>,
    /// Errors raised while translating (malformed escapes), in source order.
    pub errors: Vec<RobustaDiagnostic>,
}

impl Translated<'_> {
    /// Whether cooked offsets differ from their original-source offsets.
    pub(crate) fn has_span_map(&self) -> bool {
        !self.map.is_empty()
    }

    /// Map a cooked byte offset back to an original-source byte offset.
    pub(crate) fn map_offset(&self, cooked: u32) -> u32 {
        if self.map.is_empty() {
            return cooked;
        }
        // The last anchor at or before `cooked`; `map[0]` is `(0, 0)`, so a
        // non-empty map always has one.
        let idx = self.map.partition_point(|&(c, _)| c <= cooked) - 1;
        let (anchor_cooked, anchor_original) = self.map[idx];
        anchor_original + (cooked - anchor_cooked)
    }

    /// Map a cooked span back to original-source coordinates.
    pub(crate) fn map_span(&self, span: Span) -> Span {
        Span::new(self.map_offset(span.start), self.map_offset(span.end))
    }

    /// Map non-overlapping spans in source order with one forward anchor scan.
    ///
    /// The input endpoints must be nondecreasing: each span's start must be at
    /// or after the preceding span's end. Call this separately for independent
    /// ordered collections so each collection begins with a fresh anchor.
    pub(crate) fn map_spans_in_order<'span>(
        &self,
        spans: impl IntoIterator<Item = &'span mut Span>,
    ) {
        let mut mapper = self.ordered_span_mapper();
        for span in spans {
            *span = mapper.map_span(*span);
        }
    }

    pub(crate) fn ordered_span_mapper(&self) -> OrderedSpanMapper<'_> {
        OrderedSpanMapper { map: &self.map, anchor_idx: 0, previous_end: 0 }
    }
}

/// A forward cursor over the translation anchors for one ordered collection.
pub struct OrderedSpanMapper<'a> {
    map: &'a [(u32, u32)],
    anchor_idx: usize,
    previous_end: u32,
}

impl OrderedSpanMapper<'_> {
    pub(crate) fn map_span(&mut self, cooked: Span) -> Span {
        debug_assert!(
            self.previous_end <= cooked.start,
            "ordered span mapping requires non-overlapping spans in source order"
        );
        debug_assert!(cooked.start <= cooked.end, "span endpoints must be ordered");
        self.previous_end = cooked.end;
        if self.map.is_empty() {
            return cooked;
        }
        Span::new(self.map_offset(cooked.start), self.map_offset(cooked.end))
    }

    #[inline]
    fn map_offset(&mut self, cooked: u32) -> u32 {
        while self.anchor_idx + 1 < self.map.len() && self.map[self.anchor_idx + 1].0 <= cooked {
            self.anchor_idx += 1;
        }
        let (anchor_cooked, anchor_original) = self.map[self.anchor_idx];
        anchor_original + (cooked - anchor_cooked)
    }
}

/// What an eligible `\u…` sequence translated to.
enum Escape {
    /// A valid non-surrogate escape decoding to `ch`, ending at byte `end`.
    Char { ch: char, end: usize },
    /// A valid escape whose code point is an unpaired surrogate; left raw.
    Surrogate { value: u32, end: usize },
    /// Fewer than four hex digits after the `u`s; left raw and diagnosed.
    Malformed { end: usize },
}

/// Translate Unicode escapes in `source` ahead of lexing.
pub fn translate(source: &str) -> Translated<'_> {
    let bytes = source.as_bytes();
    // A Unicode escape needs a backslash; without one the source is already
    // cooked and every offset maps to itself. This keeps ASCII-heavy files free.
    if memchr(b'\\', bytes).is_none() {
        return Translated { cooked: Cow::Borrowed(source), map: Vec::new(), errors: Vec::new() };
    }
    // A backslash that never leads a `u` is an ordinary string escape, so the
    // source is still already cooked. Both prefilters are whole-source scans
    // and belong to the same searcher the comment scanners use.
    if memmem::find(bytes, b"\\u").is_none() {
        return Translated { cooked: Cow::Borrowed(source), map: Vec::new(), errors: Vec::new() };
    }

    // The cooked buffer is allocated lazily on the first translated escape, so
    // sources with backslashes but no eligible `\u` still borrow the original.
    let mut cooked: Option<String> = None;
    let mut map: Vec<(u32, u32)> = Vec::new();
    let mut errors: Vec<RobustaDiagnostic> = Vec::new();
    let mut i = 0;
    // Whether a backslash at `i` would be eligible: true when an even number of
    // contiguous backslashes precede it (JLS §3.3). Reset by any other char.
    let mut eligible = true;

    while i < bytes.len() {
        if bytes[i] == b'\\' {
            if eligible && bytes.get(i + 1) == Some(&b'u') {
                match scan_escape(bytes, i) {
                    Escape::Char { ch, end } => {
                        let buf = cooked.get_or_insert_with(|| {
                            let mut buffer = String::with_capacity(source.len());
                            buffer.push_str(&source[..i]);
                            buffer
                        });
                        buf.push(ch);
                        // Anchor the byte-for-byte run resuming after the escape.
                        map.push((offset32(buf.len()), offset32(end)));
                        i = end;
                        eligible = true;
                        continue;
                    }
                    Escape::Surrogate { value, end } => {
                        if let Some((ch, pair_end)) = surrogate_pair(bytes, value, end) {
                            let buf = cooked.get_or_insert_with(|| {
                                let mut buffer = String::with_capacity(source.len());
                                buffer.push_str(&source[..i]);
                                buffer
                            });
                            buf.push(ch);
                            map.push((offset32(buf.len()), offset32(pair_end)));
                            i = pair_end;
                        } else {
                            copy_raw(&mut cooked, source, i, end);
                            i = end;
                        }
                        eligible = true;
                        continue;
                    }
                    Escape::Malformed { end } => {
                        errors.push(diagnostics::invalid_unicode_escape(span32(i, end)));
                        copy_raw(&mut cooked, source, i, end);
                        i = end;
                        eligible = true;
                        continue;
                    }
                }
            }
            // A backslash that does not begin an escape: copy it and flip the
            // contiguous-backslash parity so the next one is not eligible.
            if let Some(buf) = &mut cooked {
                buf.push('\\');
            }
            i += 1;
            eligible = !eligible;
        } else {
            let ch_len = char_len(bytes[i]);
            if let Some(buf) = &mut cooked {
                buf.push_str(&source[i..i + ch_len]);
            }
            i += ch_len;
            eligible = true;
        }
    }

    match cooked {
        Some(cooked) => {
            // Seed the byte-for-byte run before the first translated escape.
            map.insert(0, (0, 0));
            Translated { cooked: Cow::Owned(cooked), map, errors }
        }
        None => Translated { cooked: Cow::Borrowed(source), map: Vec::new(), errors },
    }
}

/// Scan an eligible `\u…` sequence starting at `start` (`bytes[start]` is `\`,
/// `bytes[start + 1]` is `u`). Consumes one or more `u`s and four hex digits.
fn scan_escape(bytes: &[u8], start: usize) -> Escape {
    let mut j = start + 1;
    while bytes.get(j) == Some(&b'u') {
        j += 1;
    }
    let mut value: u32 = 0;
    for _ in 0..4 {
        match bytes.get(j) {
            Some(&d) if d.is_ascii_hexdigit() => {
                value = value * 16 + u32::from(hex_digit(d));
                j += 1;
            }
            _ => return Escape::Malformed { end: j },
        }
    }
    match char::from_u32(value) {
        // `from_u32` rejects exactly the surrogate range for a 4-hex value.
        Some(ch) => Escape::Char { ch, end: j },
        None => Escape::Surrogate { value, end: j },
    }
}

/// Combine adjacent high/low surrogate escapes into the supplementary Unicode
/// scalar they encode. The two escapes are independently eligible because the
/// first ends in a hexadecimal digit, not a backslash.
fn surrogate_pair(bytes: &[u8], high: u32, low_start: usize) -> Option<(char, usize)> {
    if !(0xd800..=0xdbff).contains(&high)
        || bytes.get(low_start) != Some(&b'\\')
        || bytes.get(low_start + 1) != Some(&b'u')
    {
        return None;
    }
    let Escape::Surrogate { value: low, end } = scan_escape(bytes, low_start) else {
        return None;
    };
    if !(0xdc00..=0xdfff).contains(&low) {
        return None;
    }
    let scalar = 0x1_0000 + ((high - 0xd800) << 10) + (low - 0xdc00);
    char::from_u32(scalar).map(|ch| (ch, end))
}

/// Copy `source[from..to]` verbatim into the cooked buffer when it exists. Used
/// for escapes left untranslated; the range is all ASCII, so it is on char
/// boundaries. No anchor is needed because a raw copy preserves byte length.
fn copy_raw(cooked: &mut Option<String>, source: &str, from: usize, to: usize) {
    if let Some(buf) = cooked {
        buf.push_str(&source[from..to]);
    }
}

#[inline]
fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        _ => b - b'A' + 10,
    }
}

/// The UTF-8 length of the character whose leading byte is `b`.
#[inline]
fn char_len(b: u8) -> usize {
    match b {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cooked(source: &str) -> String {
        translate(source).cooked.into_owned()
    }

    #[test]
    fn source_without_backslash_is_borrowed_identity() {
        let t = translate("class A {}");
        assert!(matches!(t.cooked, Cow::Borrowed(_)));
        assert_eq!(t.map.len(), 0);
        assert_eq!(t.map_offset(4), 4);
    }

    #[test]
    fn backslashes_without_eligible_escape_stay_borrowed() {
        // A string with a real `\n` escape has a backslash but nothing to
        // translate, so it must still borrow.
        let t = translate(r#""a\nb""#);
        assert!(matches!(t.cooked, Cow::Borrowed(_)));
    }

    #[test]
    fn translates_escape_in_keyword() {
        assert_eq!(cooked(r"cl\u0061ss A {}"), "class A {}");
    }

    #[test]
    fn multiple_u_letters_are_allowed() {
        assert_eq!(cooked(r"\uuuu0061"), "a");
    }

    #[test]
    fn escaped_backslash_keeps_unicode_text_raw() {
        // `\\` is an even backslash run, so the second backslash is not
        // eligible and `u2126` stays literal (JLS §3.3).
        assert_eq!(cooked(r#""\\u2126""#), r#""\\u2126""#);
    }

    #[test]
    fn translation_is_not_recursive() {
        // `\u005c` decodes to a backslash, which must not start a new escape.
        assert_eq!(cooked(r"\u005cu005a"), r"\u005a");
    }

    #[test]
    fn odd_backslash_run_still_translates() {
        // Three backslashes: the first two stay raw, the third is eligible.
        assert_eq!(cooked(r"\\\u0061"), r"\\a");
    }

    #[test]
    fn escaped_slashes_become_a_comment_prefix() {
        assert_eq!(cooked(r"\u002f\u002f x"), "// x");
    }

    #[test]
    fn newline_escape_translates_to_a_real_line_terminator() {
        assert_eq!(cooked(r"a\u000ab"), "a\nb");
    }

    #[test]
    fn surrogate_escape_is_left_raw() {
        // A Rust String cannot hold an unpaired surrogate, so keep the spelling.
        assert_eq!(cooked(r"\uD83D"), r"\uD83D");
    }

    #[test]
    fn surrogate_pair_escapes_translate_to_a_supplementary_scalar() {
        let translated = translate(r"\uD835\uDC82x");
        assert_eq!(translated.cooked, "𝒂x");
        assert_eq!(translated.map_offset(0), 0);
        assert_eq!(translated.map_offset(4), 12);
        assert_eq!(translated.map_offset(5), 13);
    }

    #[test]
    fn malformed_escape_is_left_raw_and_diagnosed() {
        let t = translate(r"\uXY");
        assert_eq!(t.cooked, r"\uXY");
        assert_eq!(t.errors.len(), 1);
        assert!(t.errors[0].message.contains("Unicode escape"));
    }

    #[test]
    fn offsets_map_back_through_a_translated_escape() {
        // `cl\u0061ss` -> cooked `class` (5 bytes) over 10 original bytes.
        let t = translate(r"cl\u0061ss");
        assert_eq!(t.map_offset(0), 0); // before the escape: identity
        assert_eq!(t.map_offset(2), 2); // start of the decoded 'a'
        assert_eq!(t.map_offset(3), 8); // after the escape
        assert_eq!(t.map_offset(5), 10); // end of `ss`
    }

    #[test]
    fn offsets_map_across_multiple_escapes() {
        // `\u0061\u0062` -> cooked `ab` (2 bytes) over 12 original bytes.
        let t = translate(r"\u0061\u0062");
        assert_eq!(t.map_offset(0), 0);
        assert_eq!(t.map_offset(1), 6);
        assert_eq!(t.map_offset(2), 12);
    }

    #[test]
    fn ordered_span_mapping_matches_random_access_across_multiple_anchors() {
        let translated = translate(r"a\u0062cd\u0065fg");
        assert_eq!(translated.cooked, "abcdefg");

        // The middle spans cross the first and second translation anchors.
        let mut spans = [Span::new(0, 1), Span::new(1, 3), Span::new(3, 6), Span::new(6, 7)];
        let expected = spans.map(|span| translated.map_span(span));

        translated.map_spans_in_order(spans.iter_mut());

        assert_eq!(spans, expected);
    }
}
