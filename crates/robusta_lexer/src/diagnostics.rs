use robusta_diagnostics::RobustaDiagnostic;
use robusta_span::Span;

/// The source is longer than the `u32` span domain admits.
#[cold]
pub fn source_too_long() -> RobustaDiagnostic {
    RobustaDiagnostic::error("source length exceeds 4 GiB limit", Span::empty(0))
}

/// A `/*` comment runs to end of input (JLS §3.7).
#[cold]
pub fn unterminated_block_comment(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("unterminated block comment", span)
}

/// A character that begins no Java token (JLS §3.5).
#[cold]
pub fn unexpected_character(character: char, span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error(format!("unexpected character {character:?}"), span)
}

/// A `\u` escape that is not six well-formed characters (JLS §3.3).
#[cold]
pub fn invalid_unicode_escape(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("invalid Unicode escape", span)
}

/// A numeric form that admits digits was written with none (JLS §3.10.1,
/// §3.10.2). `subject` names the form: `"hexadecimal integer literal"`,
/// `"binary integer literal"`, `"hexadecimal floating-point literal"`,
/// `"floating-point exponent"`, or `"binary exponent"`.
#[cold]
pub fn missing_digits(subject: &'static str, span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error(format!("{subject} must contain at least one digit"), span)
}

/// An underscore leads or trails the digits of an integer literal
/// (JLS §3.10.1).
#[cold]
pub fn integer_underscore(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("underscore must separate digits in integer literal", span)
}

/// An underscore leads or trails the digits of a floating-point literal
/// (JLS §3.10.2).
#[cold]
pub fn float_underscore(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("underscore must separate digits in floating-point literal", span)
}

/// A hexadecimal floating-point literal omits its mandatory `p` exponent
/// (JLS §3.10.2).
#[cold]
pub fn hex_float_requires_exponent(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("hexadecimal floating-point literal requires a binary exponent", span)
}

/// An `8` or `9` appears in a leading-zero octal literal (JLS §3.10.1).
#[cold]
pub fn invalid_octal_digit(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("invalid digit in octal integer literal", span)
}

/// A character literal reaches a line terminator or end of input before its
/// closing quote (JLS §3.10.4).
#[cold]
pub fn unterminated_character_literal(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("unterminated character literal", span)
}

/// `''` — a character literal with no content (JLS §3.10.4).
#[cold]
pub fn empty_character_literal(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("empty character literal", span)
}

/// A character literal holding more than one character (JLS §3.10.4).
#[cold]
pub fn multi_character_literal(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("character literal may only contain one character", span)
}

/// A backslash escape that no production accepts (JLS §3.10.7). `subject`
/// names the enclosing form: `"character literal"`, `"string literal"`, or
/// `"text block"`.
#[cold]
pub fn invalid_escape_sequence(subject: &'static str, span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error(format!("invalid escape sequence in {subject}"), span)
}

/// A string literal reaches a line terminator or end of input before its
/// closing quote (JLS §3.10.5).
#[cold]
pub fn unterminated_string_literal(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("unterminated string literal", span)
}

/// A text block reaches end of input before its closing delimiter
/// (JLS §3.10.6).
#[cold]
pub fn unterminated_text_block(span: Span) -> RobustaDiagnostic {
    RobustaDiagnostic::error("unterminated text block", span)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> Span {
        Span::new(0, 1)
    }

    /// Pins every message the lexer can produce. A wording change is a
    /// user-visible change to `robusta check` output, so it belongs in a diff
    /// that says so.
    #[test]
    fn every_message_is_pinned() {
        let messages = [
            source_too_long().message,
            unterminated_block_comment(span()).message,
            unexpected_character('§', span()).message,
            invalid_unicode_escape(span()).message,
            missing_digits("hexadecimal integer literal", span()).message,
            missing_digits("binary integer literal", span()).message,
            missing_digits("hexadecimal floating-point literal", span()).message,
            missing_digits("floating-point exponent", span()).message,
            missing_digits("binary exponent", span()).message,
            integer_underscore(span()).message,
            float_underscore(span()).message,
            hex_float_requires_exponent(span()).message,
            invalid_octal_digit(span()).message,
            unterminated_character_literal(span()).message,
            empty_character_literal(span()).message,
            multi_character_literal(span()).message,
            invalid_escape_sequence("character literal", span()).message,
            invalid_escape_sequence("string literal", span()).message,
            invalid_escape_sequence("text block", span()).message,
            unterminated_string_literal(span()).message,
            unterminated_text_block(span()).message,
        ];

        assert_eq!(
            messages,
            [
                "source length exceeds 4 GiB limit",
                "unterminated block comment",
                "unexpected character '§'",
                "invalid Unicode escape",
                "hexadecimal integer literal must contain at least one digit",
                "binary integer literal must contain at least one digit",
                "hexadecimal floating-point literal must contain at least one digit",
                "floating-point exponent must contain at least one digit",
                "binary exponent must contain at least one digit",
                "underscore must separate digits in integer literal",
                "underscore must separate digits in floating-point literal",
                "hexadecimal floating-point literal requires a binary exponent",
                "invalid digit in octal integer literal",
                "unterminated character literal",
                "empty character literal",
                "character literal may only contain one character",
                "invalid escape sequence in character literal",
                "invalid escape sequence in string literal",
                "invalid escape sequence in text block",
                "unterminated string literal",
                "unterminated text block",
            ]
        );
    }

    #[test]
    fn every_constructor_reports_an_error_at_its_span() {
        let diagnostic = unterminated_text_block(Span::new(4, 9));
        assert_eq!(diagnostic.severity, robusta_diagnostics::Severity::Error);
        assert_eq!(diagnostic.primary_span(), Span::new(4, 9));
    }
}
