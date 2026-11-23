use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, terminator_len},
};

use crate::{Lexer, TokenKind, diagnostics, span32};

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
        loop {
            match self.peek() {
                None => {
                    self.errors
                        .push(diagnostics::unterminated_string_literal(span32(start, self.pos)));
                    break;
                }
                Some(b) if is_line_terminator(b) => {
                    self.errors
                        .push(diagnostics::unterminated_string_literal(span32(start, self.pos)));
                    break;
                }
                Some(b'\\') => match self.scan_escape_sequence() {
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
                },
                Some(b'"') => {
                    self.pos += 1;
                    break;
                }
                Some(_) => self.pos += 1,
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
            match self.peek() {
                None => {
                    self.errors.push(diagnostics::unterminated_text_block(span32(start, self.pos)));
                    break;
                }
                Some(b'"') if self.src.get(self.pos..self.pos + 3) == Some(b"\"\"\"") => {
                    self.pos += 3;
                    break;
                }
                Some(b'\\') => match self.scan_text_block_escape_sequence() {
                    EscapeScan::Valid => {}
                    EscapeScan::Invalid => {
                        self.errors.push(diagnostics::invalid_escape_sequence(
                            "text block",
                            span32(start, self.pos),
                        ));
                    }
                    EscapeScan::Unterminated => {
                        self.errors
                            .push(diagnostics::unterminated_text_block(span32(start, self.pos)));
                        break;
                    }
                },
                Some(_) => self.bump_char(),
            }
        }
        TokenKind::StringLiteral
    }
}
