// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_parser/src/lexer/string.rs
use std::borrow::Cow;

use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, terminator_len},
};

use crate::{
    Lexer, TokenKind, diagnostics,
    search::{SafeByteMatchTable, safe_byte_match_table},
    span32, unicode,
};

static STRING_END_TABLE: SafeByteMatchTable =
    safe_byte_match_table!(|b| matches!(b, b'"' | b'\r' | b'\n' | b'\\'));

static TEXT_BLOCK_END_TABLE: SafeByteMatchTable =
    safe_byte_match_table!(|b| matches!(b, b'"' | b'\\'));

/// Find the next quote, escape, or line break after an escape sequence.
#[inline(always)]
fn find_string_chunk_end(bytes: &[u8]) -> Option<usize> {
    const PREFIX_LEN: usize = 16;
    // Handle short chunks inline before paying for the two bulk searches.
    let Some(prefix) = bytes.get(..PREFIX_LEN) else {
        return bytes.iter().position(|&byte| STRING_END_TABLE.matches(byte));
    };
    for (index, &byte) in prefix.iter().enumerate() {
        if STRING_END_TABLE.matches(byte) {
            return Some(index);
        }
    }

    let remaining = &bytes[PREFIX_LEN..];
    let end = memchr::memchr3(b'"', b'\\', b'\n', remaining);
    // A carriage return before the other delimiters ends the chunk first.
    memchr::memchr(b'\r', &remaining[..end.unwrap_or(remaining.len())])
        .or(end)
        .map(|index| PREFIX_LEN + index)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeScan {
    /// A valid escape and the character it spells, or `None` for a line
    /// continuation or an unpaired surrogate, which no `char` holds.
    Valid(Option<char>),
    Invalid,
    Unterminated,
}

/// The value of the ordinary string literal spelled `raw`, quotes included:
/// Unicode escapes translate first (JLS §3.3), then escape sequences
/// (JLS §3.10.7). Borrows `raw` when neither occurs.
///
/// `None` for a text block, a malformed literal, or a value with an unpaired
/// surrogate, which a Rust string cannot hold.
pub fn string_literal_value(raw: &str) -> Option<Cow<'_, str>> {
    match unicode::translate(raw).cooked {
        Cow::Borrowed(raw) => decode_string_content(string_content(raw)?),
        Cow::Owned(raw) => {
            decode_string_content(string_content(&raw)?).map(|value| Cow::Owned(value.into_owned()))
        }
    }
}

/// The text between an ordinary string literal's quotes.
fn string_content(raw: &str) -> Option<&str> {
    if raw.starts_with("\"\"\"") {
        return None;
    }
    raw.strip_circumfix('"', '"')
}

/// Decode the escape sequences in Unicode-translated string content.
fn decode_string_content(content: &str) -> Option<Cow<'_, str>> {
    let bytes = content.as_bytes();
    let Some(mut backslash) = memchr::memchr(b'\\', bytes) else {
        return Some(Cow::Borrowed(content));
    };
    let mut value = String::with_capacity(content.len());
    let mut chunk_start = 0;
    loop {
        value.push_str(&content[chunk_start..backslash]);
        let (EscapeScan::Valid(Some(ch)), end) = decode_escape_sequence(content, backslash + 1)
        else {
            return None;
        };
        value.push(ch);
        chunk_start = end;
        match memchr::memchr(b'\\', &bytes[end..]) {
            Some(offset) => backslash = end + offset,
            None => break,
        }
    }
    value.push_str(&content[chunk_start..]);
    Some(Cow::Owned(value))
}

/// Decode the escape sequence whose backslash precedes `text[pos]`,
/// returning it and the offset after it.
fn decode_escape_sequence(text: &str, mut pos: usize) -> (EscapeScan, usize) {
    let bytes = text.as_bytes();
    let ch = match bytes.get(pos) {
        Some(b'u') => return decode_unicode_escape(bytes, pos),
        Some(&b) if is_line_terminator(b) => return (EscapeScan::Unterminated, pos),
        None => return (EscapeScan::Unterminated, pos),
        Some(b'b') => '\u{8}',
        Some(b's') => ' ',
        Some(b't') => '\t',
        Some(b'n') => '\n',
        Some(b'f') => '\u{c}',
        Some(b'r') => '\r',
        Some(&b @ (b'"' | b'\'' | b'\\')) => char::from(b),
        Some(&b @ b'0'..=b'7') => {
            // `\0` to `\377`: a leading digit above 3 takes one more digit, not two.
            let max_len = if b <= b'3' { 3 } else { 2 };
            let mut value = 0;
            let mut len = 0;
            while len < max_len
                && let Some(&digit @ b'0'..=b'7') = bytes.get(pos)
            {
                value = value * 8 + (digit - b'0');
                pos += 1;
                len += 1;
            }
            return (EscapeScan::Valid(Some(char::from(value))), pos);
        }
        Some(_) => {
            let ch_len = text[pos..].chars().next().map_or(1, char::len_utf8);
            return (EscapeScan::Invalid, pos + ch_len);
        }
    };
    (EscapeScan::Valid(Some(ch)), pos + 1)
}

/// Scan a `\u` escape left raw by Unicode translation.
fn decode_unicode_escape(bytes: &[u8], mut pos: usize) -> (EscapeScan, usize) {
    // Valid Unicode escapes are already translated by the prepass, so the
    // only `\u` reaching here are unpaired surrogates it left raw (a Rust
    // `String` cannot hold them). Treat such an escape as one escape unit so
    // a literal like `'\uD83D'` is not miscounted as several characters.
    while bytes.get(pos) == Some(&b'u') {
        pos += 1;
    }
    for _ in 0..4 {
        match bytes.get(pos) {
            Some(b) if b.is_ascii_hexdigit() => pos += 1,
            None | Some(b'\n' | b'\r') => return (EscapeScan::Unterminated, pos),
            Some(_) => return (EscapeScan::Invalid, pos),
        }
    }
    (EscapeScan::Valid(None), pos)
}

impl Lexer<'_> {
    pub(super) fn scan_char(&mut self) -> TokenKind {
        let start = self.pos;
        self.pos += 1; // opening quote
        let mut malformed_content = false;
        match self.peek() {
            None | Some(b'\n' | b'\r') => {
                self.errors
                    .push(diagnostics::unterminated_character_literal(span32(start, self.pos)));
                return TokenKind::CharLiteral;
            }
            Some(b'\'') => {
                self.pos += 1;
                self.errors.push(diagnostics::empty_character_literal(span32(start, self.pos)));
                return TokenKind::CharLiteral;
            }
            Some(b'\\') => match self.scan_escape_sequence() {
                EscapeScan::Valid(_) => {}
                EscapeScan::Invalid => {
                    malformed_content = true;
                    self.errors.push(diagnostics::invalid_escape_sequence(
                        "character literal",
                        span32(start, self.pos),
                    ));
                }
                EscapeScan::Unterminated => {
                    self.errors
                        .push(diagnostics::unterminated_character_literal(span32(start, self.pos)));
                    return TokenKind::CharLiteral;
                }
            },
            Some(_) => self.bump_char(),
        }

        if self.peek() == Some(b'\'') {
            self.pos += 1;
        } else if !matches!(self.peek(), None | Some(b'\n' | b'\r')) {
            while !matches!(self.peek(), None | Some(b'\'' | b'\n' | b'\r')) {
                self.bump_char();
            }
            if self.peek() == Some(b'\'') {
                self.pos += 1;
            }
            if !malformed_content {
                self.errors.push(diagnostics::multi_character_literal(span32(start, self.pos)));
            }
        } else {
            self.errors.push(diagnostics::unterminated_character_literal(span32(start, self.pos)));
        }

        TokenKind::CharLiteral
    }

    fn scan_escape_sequence(&mut self) -> EscapeScan {
        self.pos += 1; // backslash
        self.scan_escape_sequence_tail()
    }

    fn scan_escape_sequence_tail(&mut self) -> EscapeScan {
        let (escape, end) = decode_escape_sequence(self.text, self.pos);
        self.pos = end;
        escape
    }

    fn scan_text_block_escape_sequence(&mut self) -> EscapeScan {
        self.pos += 1; // backslash
        match self.peek() {
            Some(b'\n') => {
                self.pos += 1;
                EscapeScan::Valid(None)
            }
            Some(b'\r') => {
                self.pos += 1;
                if self.peek() == Some(b'\n') {
                    self.pos += 1;
                }
                EscapeScan::Valid(None)
            }
            _ => self.scan_escape_sequence_tail(),
        }
    }

    pub(super) fn scan_string(&mut self) -> TokenKind {
        let start = self.pos;
        if let Some(content_start) = self.text_block_content_start() {
            return self.scan_text_block(start, content_start);
        }
        self.pos += 1; // opening quote
        let mut next_byte = self.byte_search(&STRING_END_TABLE);
        loop {
            match next_byte {
                Some(b'"') => {
                    self.pos += 1;
                    break;
                }
                Some(b'\\') => {
                    match self.scan_escape_sequence() {
                        EscapeScan::Valid(_) => {}
                        EscapeScan::Invalid => {
                            self.errors.push(diagnostics::invalid_escape_sequence(
                                "string literal",
                                span32(start, self.pos),
                            ));
                        }
                        EscapeScan::Unterminated => {
                            self.errors.push(diagnostics::unterminated_string_literal(span32(
                                start, self.pos,
                            )));
                            break;
                        }
                    }
                    let remaining = &self.src[self.pos..];
                    next_byte = if let Some(end) = find_string_chunk_end(remaining) {
                        self.pos += end;
                        Some(remaining[end])
                    } else {
                        self.pos = self.src.len();
                        None
                    };
                }
                // The end of the source, or a line break
                _ => {
                    self.errors
                        .push(diagnostics::unterminated_string_literal(span32(start, self.pos)));
                    break;
                }
            }
        }
        TokenKind::StringLiteral
    }

    fn text_block_content_start(&self) -> Option<usize> {
        if self.src.get(self.pos..self.pos + 3)? != b"\"\"\"" {
            return None;
        }

        let mut n = self.pos + 3;
        while self.src.get(n).is_some_and(|&byte| is_horizontal_whitespace(byte)) {
            n += 1;
        }
        match terminator_len(self.text, n) {
            0 => None,
            len => Some(n + len),
        }
    }

    fn scan_text_block(&mut self, start: usize, content_start: usize) -> TokenKind {
        self.pos = content_start;
        loop {
            match self.byte_search(&TEXT_BLOCK_END_TABLE) {
                None => {
                    self.errors.push(diagnostics::unterminated_text_block(span32(start, self.pos)));
                    break;
                }
                Some(b'"') => {
                    if self.src[self.pos..].starts_with(b"\"\"\"") {
                        self.pos += 3;
                        break;
                    }
                    self.pos += 1;
                }
                Some(next_byte) => {
                    // `TEXT_BLOCK_END_TABLE` only matches `"` and `\`
                    debug_assert_eq!(next_byte, b'\\');
                    match self.scan_text_block_escape_sequence() {
                        EscapeScan::Valid(_) => {}
                        EscapeScan::Invalid => {
                            self.errors.push(diagnostics::invalid_escape_sequence(
                                "text block",
                                span32(start, self.pos),
                            ));
                        }
                        EscapeScan::Unterminated => {
                            self.errors.push(diagnostics::unterminated_text_block(span32(
                                start, self.pos,
                            )));
                            break;
                        }
                    }
                }
            }
        }
        TokenKind::StringLiteral
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::string_literal_value;

    #[test]
    fn unescaped_value_borrows_the_literal() {
        assert!(matches!(string_literal_value(r#""plain""#), Some(Cow::Borrowed("plain"))));
        assert!(matches!(string_literal_value(r#""""#), Some(Cow::Borrowed(""))));
    }

    #[test]
    fn escape_sequences_decode_to_their_characters() {
        for (raw, value) in [
            (r#""\b\s\t\n\f\r""#, "\u{8} \t\n\u{c}\r"),
            (r#""\"\'\\""#, "\"'\\"),
            (r#""\0\7\08\101\377\400\47""#, "\0\u{7}\08A\u{ff} 0'"),
            (r#""\162obusta""#, "robusta"),
        ] {
            assert_eq!(string_literal_value(raw).as_deref(), Some(value), "{raw}");
        }
    }

    #[test]
    fn unicode_escapes_translate_before_escape_sequences() {
        for (raw, value) in [
            (r#""\u0072obusta""#, "robusta"),
            (r#""\u005cn""#, "\n"),
            (r#""\u005c\u005c""#, "\\"),
            (r#""\\u0041""#, "\\u0041"),
            (r"\u0022a\u0022", "a"),
            (r#""\uD83D\uDE00""#, "\u{1f600}"),
        ] {
            assert_eq!(string_literal_value(raw).as_deref(), Some(value), "{raw}");
        }
    }

    #[test]
    fn text_blocks_and_unrepresentable_values_have_none() {
        for raw in [
            "\"\"\"\n    text\"\"\"",
            r#""\uD83D""#,
            r#""\q""#,
            r#""\8""#,
            r#""open"#,
            r#""trailing\""#,
        ] {
            assert_eq!(string_literal_value(raw), None, "{raw}");
        }
    }
}
