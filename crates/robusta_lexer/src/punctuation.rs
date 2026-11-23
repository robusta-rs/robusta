use crate::{Lexer, TokenKind, diagnostics, span32};

impl Lexer<'_> {
    pub(super) fn scan_punct(&mut self, c: u8) -> TokenKind {
        let start = self.pos;
        self.pos += 1;
        match c {
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b';' => TokenKind::Semicolon,
            b',' => TokenKind::Comma,
            b'.' => {
                if self.peek() == Some(b'.') && self.src.get(self.pos + 1) == Some(&b'.') {
                    self.pos += 2;
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            b'?' => TokenKind::Question,
            b':' => {
                if self.peek() == Some(b':') {
                    self.pos += 1;
                    TokenKind::ColonColon
                } else {
                    TokenKind::Colon
                }
            }
            b'@' => TokenKind::At,
            b'~' => TokenKind::Tilde,
            b'=' => self.then_eq(TokenKind::EqEq, TokenKind::Assign),
            b'!' => self.then_eq(TokenKind::BangEq, TokenKind::Bang),
            b'<' => match self.peek() {
                Some(b'<') => {
                    self.pos += 1;
                    self.then_eq(TokenKind::LtLtEq, TokenKind::LtLt)
                }
                _ => self.then_eq(TokenKind::LtEq, TokenKind::Lt),
            },
            // Each `>` in `List<List<T>>` closes its own type argument list (JLS §3.5)
            b'>' => self.then_eq(TokenKind::GtEq, TokenKind::Gt),
            b'+' => match self.peek() {
                Some(b'+') => {
                    self.pos += 1;
                    TokenKind::PlusPlus
                }
                _ => self.then_eq(TokenKind::PlusEq, TokenKind::Plus),
            },
            b'-' => match self.peek() {
                Some(b'-') => {
                    self.pos += 1;
                    TokenKind::MinusMinus
                }
                Some(b'>') => {
                    self.pos += 1;
                    TokenKind::Arrow
                }
                _ => self.then_eq(TokenKind::MinusEq, TokenKind::Minus),
            },
            b'*' => self.then_eq(TokenKind::StarEq, TokenKind::Star),
            b'/' => self.then_eq(TokenKind::SlashEq, TokenKind::Slash),
            b'%' => self.then_eq(TokenKind::PercentEq, TokenKind::Percent),
            b'^' => self.then_eq(TokenKind::CaretEq, TokenKind::Caret),
            b'&' => match self.peek() {
                Some(b'&') => {
                    self.pos += 1;
                    TokenKind::AmpAmp
                }
                _ => self.then_eq(TokenKind::AmpEq, TokenKind::Amp),
            },
            b'|' => match self.peek() {
                Some(b'|') => {
                    self.pos += 1;
                    TokenKind::PipePipe
                }
                _ => self.then_eq(TokenKind::PipeEq, TokenKind::Pipe),
            },
            _ => {
                // Consume the whole character, not one byte: a token span must
                // never split a multi-byte UTF-8 sequence, or slicing the
                // source by span would panic.
                let ch = self.text[start..].chars().next().unwrap();
                self.pos = start + ch.len_utf8();
                self.errors.push(diagnostics::unexpected_character(ch, span32(start, self.pos)));
                TokenKind::Unknown
            }
        }
    }

    /// Maximal munch for the `x=` operator pairs: consume a trailing `=` and
    /// yield `with_eq`, else yield `bare`.
    fn then_eq(&mut self, with_eq: TokenKind, bare: TokenKind) -> TokenKind {
        if self.peek() == Some(b'=') {
            self.pos += 1;
            with_eq
        } else {
            bare
        }
    }
}
