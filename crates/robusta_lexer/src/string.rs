// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_parser/src/lexer/string.rs
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, terminator_len},
};

use crate::{
    Lexer, TokenKind, diagnostics,
    search::{SafeByteMatchTable, safe_byte_match_table},
    span32,
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
    Valid,
    Invalid,
    Unterminated,
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
                EscapeScan::Valid => {}
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
        match self.peek() {
            Some(b'u') => return self.scan_unicode_escape(),
            Some(b'b' | b's' | b't' | b'n' | b'f' | b'r' | b'"' | b'\'' | b'\\') => {
                self.pos += 1;
            }
            Some(b'0'..=b'3') => {
                self.pos += 1;
                for _ in 0..2 {
                    if matches!(self.peek(), Some(b'0'..=b'7')) {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
            }
            Some(b'4'..=b'7') => {
                self.pos += 1;
                if matches!(self.peek(), Some(b'0'..=b'7')) {
                    self.pos += 1;
                }
            }
            None => return EscapeScan::Unterminated,
            Some(b) if is_line_terminator(b) => return EscapeScan::Unterminated,
            Some(_) => {
                self.bump_char();
                return EscapeScan::Invalid;
            }
        }
        EscapeScan::Valid
    }

    fn scan_text_block_escape_sequence(&mut self) -> EscapeScan {
        self.pos += 1; // backslash
        match self.peek() {
            Some(b'\n') => {
                self.pos += 1;
                EscapeScan::Valid
            }
            Some(b'\r') => {
                self.pos += 1;
                if self.peek() == Some(b'\n') {
                    self.pos += 1;
                }
                EscapeScan::Valid
            }
            _ => self.scan_escape_sequence_tail(),
        }
    }

    fn scan_unicode_escape(&mut self) -> EscapeScan {
        // Valid Unicode escapes are already translated by the prepass, so the
        // only `\u` reaching here are unpaired surrogates it left raw (a Rust
        // `String` cannot hold them). Treat such an escape as one escape unit so
        // a literal like `'\uD83D'` is not miscounted as several characters.
        while self.peek() == Some(b'u') {
            self.pos += 1;
        }
        for _ in 0..4 {
            match self.peek() {
                Some(b) if b.is_ascii_hexdigit() => self.pos += 1,
                None | Some(b'\n' | b'\r') => return EscapeScan::Unterminated,
                Some(_) => return EscapeScan::Invalid,
            }
        }
        EscapeScan::Valid
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
                        EscapeScan::Valid => {}
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
                        EscapeScan::Valid => {}
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
