use crate::{Lexer, TokenKind, diagnostics, span32};

impl Lexer<'_> {
    pub(super) fn scan_number(&mut self) -> TokenKind {
        let start = self.pos;
        if self.peek() == Some(b'0') {
            match self.peek2() {
                Some(b'x' | b'X') => return self.scan_hex_number(start),
                Some(b'b' | b'B') => {
                    self.pos += 2;
                    self.scan_based_integer_digits(
                        start,
                        is_binary_digit,
                        "binary integer literal",
                    );
                    self.scan_integer_suffix();
                    return TokenKind::IntLiteral;
                }
                _ => {}
            }
        }

        self.scan_decimal_or_octal_number(start)
    }

    fn scan_hex_number(&mut self, start: usize) -> TokenKind {
        self.pos += 2; // 0x / 0X
        let (before_dot, leading_underscore, trailing_underscore) =
            self.scan_digits_and_underscores(u8::is_ascii_hexdigit);
        if self.peek() == Some(b'.') {
            self.pos += 1;
            let (after_dot, leading_after_dot, trailing_after_dot) =
                self.scan_digits_and_underscores(u8::is_ascii_hexdigit);
            if !before_dot && !after_dot {
                self.errors.push(diagnostics::missing_digits(
                    "hexadecimal floating-point literal",
                    span32(start, self.pos),
                ));
            }
            if leading_underscore || trailing_underscore || leading_after_dot || trailing_after_dot
            {
                self.report_float_underscore(start);
            }
            if !self.scan_binary_exponent_part(start) {
                self.errors.push(diagnostics::hex_float_requires_exponent(span32(start, self.pos)));
            }
            self.scan_float_suffix();
            return TokenKind::FloatLiteral;
        }

        if self.peek_is_binary_exponent() {
            if !before_dot {
                self.errors.push(diagnostics::missing_digits(
                    "hexadecimal floating-point literal",
                    span32(start, self.pos),
                ));
            }
            if leading_underscore || trailing_underscore {
                self.report_float_underscore(start);
            }
            self.scan_binary_exponent_part(start);
            self.scan_float_suffix();
            return TokenKind::FloatLiteral;
        }

        if !before_dot {
            self.errors.push(diagnostics::missing_digits(
                "hexadecimal integer literal",
                span32(start, self.pos),
            ));
        } else if leading_underscore || trailing_underscore {
            self.errors.push(diagnostics::integer_underscore(span32(start, self.pos)));
        }
        self.scan_integer_suffix();
        TokenKind::IntLiteral
    }

    fn scan_decimal_or_octal_number(&mut self, start: usize) -> TokenKind {
        let starts_with_zero = self.peek() == Some(b'0');
        self.pos += 1;
        let (_, _, trailing_underscore) = self.scan_digits_and_underscores(u8::is_ascii_digit);
        if self.peek() == Some(b'.') {
            self.pos += 1;
            let (_, leading_after_dot, trailing_after_dot) =
                self.scan_digits_and_underscores(u8::is_ascii_digit);
            if trailing_underscore || leading_after_dot || trailing_after_dot {
                self.report_float_underscore(start);
            }
            self.scan_decimal_exponent_part(start);
            self.scan_float_suffix();
            return TokenKind::FloatLiteral;
        }
        if self.peek_is_decimal_exponent() {
            if trailing_underscore {
                self.report_float_underscore(start);
            }
            self.scan_decimal_exponent_part(start);
            self.scan_float_suffix();
            return TokenKind::FloatLiteral;
        }
        if self.peek_is_float_suffix() {
            if trailing_underscore {
                self.report_float_underscore(start);
            }
            self.scan_float_suffix();
            return TokenKind::FloatLiteral;
        }

        if starts_with_zero {
            self.validate_octal_integer(start, trailing_underscore);
        } else if trailing_underscore {
            self.errors.push(diagnostics::integer_underscore(span32(start, self.pos)));
        }
        self.scan_integer_suffix();
        TokenKind::IntLiteral
    }

    pub(super) fn scan_dot_decimal_float(&mut self) -> TokenKind {
        let start = self.pos;
        self.pos += 1; // dot
        let (_, _, trailing_underscore) = self.scan_digits_and_underscores(u8::is_ascii_digit);
        if trailing_underscore {
            self.report_float_underscore(start);
        }
        self.scan_decimal_exponent_part(start);
        self.scan_float_suffix();
        TokenKind::FloatLiteral
    }

    fn scan_based_integer_digits(
        &mut self,
        start: usize,
        is_digit: fn(&u8) -> bool,
        subject: &'static str,
    ) {
        let (saw_digit, leading_underscore, trailing_underscore) =
            self.scan_digits_and_underscores(is_digit);
        if !saw_digit {
            self.errors.push(diagnostics::missing_digits(subject, span32(start, self.pos)));
        } else if leading_underscore || trailing_underscore {
            self.errors.push(diagnostics::integer_underscore(span32(start, self.pos)));
        }
    }

    fn scan_digits_and_underscores(&mut self, is_digit: fn(&u8) -> bool) -> (bool, bool, bool) {
        let mut saw_digit = false;
        let mut leading_underscore = false;
        let mut trailing_underscore = false;
        loop {
            match self.peek() {
                Some(b) if is_digit(&b) => {
                    self.pos += 1;
                    saw_digit = true;
                    trailing_underscore = false;
                }
                Some(b'_') => {
                    self.pos += 1;
                    leading_underscore |= !saw_digit;
                    trailing_underscore = true;
                }
                _ => break,
            }
        }
        (saw_digit, leading_underscore, trailing_underscore)
    }

    fn validate_octal_integer(&mut self, start: usize, trailing_underscore: bool) {
        let invalid_digit = self.src[start..self.pos].iter().any(|b| matches!(b, b'8' | b'9'));
        if trailing_underscore {
            self.errors.push(diagnostics::integer_underscore(span32(start, self.pos)));
        }
        if invalid_digit {
            self.errors.push(diagnostics::invalid_octal_digit(span32(start, self.pos)));
        }
    }

    fn scan_decimal_exponent_part(&mut self, start: usize) -> bool {
        if !self.peek_is_decimal_exponent() {
            return false;
        }
        self.pos += 1;
        self.scan_signed_digits_in_exponent(start, "floating-point exponent");
        true
    }

    fn scan_binary_exponent_part(&mut self, start: usize) -> bool {
        if !self.peek_is_binary_exponent() {
            return false;
        }
        self.pos += 1;
        self.scan_signed_digits_in_exponent(start, "binary exponent");
        true
    }

    fn scan_signed_digits_in_exponent(&mut self, start: usize, subject: &'static str) {
        if matches!(self.peek(), Some(b'+' | b'-')) {
            self.pos += 1;
        }
        let (saw_digit, leading_underscore, trailing_underscore) =
            self.scan_digits_and_underscores(u8::is_ascii_digit);
        if !saw_digit {
            self.errors.push(diagnostics::missing_digits(subject, span32(start, self.pos)));
        } else if leading_underscore || trailing_underscore {
            self.report_float_underscore(start);
        }
    }

    fn scan_integer_suffix(&mut self) {
        if matches!(self.peek(), Some(b'l' | b'L')) {
            self.pos += 1;
        }
    }

    fn scan_float_suffix(&mut self) {
        if self.peek_is_float_suffix() {
            self.pos += 1;
        }
    }

    fn report_float_underscore(&mut self, start: usize) {
        self.errors.push(diagnostics::float_underscore(span32(start, self.pos)));
    }

    fn peek_is_decimal_exponent(&self) -> bool {
        matches!(self.peek(), Some(b'e' | b'E'))
    }

    fn peek_is_binary_exponent(&self) -> bool {
        matches!(self.peek(), Some(b'p' | b'P'))
    }

    fn peek_is_float_suffix(&self) -> bool {
        matches!(self.peek(), Some(b'f' | b'F' | b'd' | b'D'))
    }
}

// By-reference to match the `u8::is_ascii_*` predicates that share the
// `scan_digits_and_underscores` signature.
#[expect(clippy::trivially_copy_pass_by_ref)]
#[inline]
fn is_binary_digit(b: &u8) -> bool {
    matches!(*b, b'0' | b'1')
}
