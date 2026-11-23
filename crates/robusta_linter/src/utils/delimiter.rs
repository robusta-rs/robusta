use robusta_span::Span;
use robusta_syntax::unicode_escape::unicode_escape;

use super::pad_fix_with_token_boundary;
use crate::fixer::{RuleFix, RuleFixer};

/// Replace `span` with the source between its `open` and `close` delimiters,
/// or decline when the source is not written with them.
pub fn keep_interior(fixer: RuleFixer<'_>, span: Span, open: &str, close: &str) -> RuleFix {
    replace_delimiters(fixer, span, [open, close], ["", ""])
}

/// Replace the `open` and `close` delimiters of `span` with `replacements`,
/// keeping the source between them, or decline when the source is not
/// written with them.
pub fn replace_delimiters(
    fixer: RuleFixer<'_>,
    span: Span,
    [open, close]: [&str; 2],
    [new_open, new_close]: [&str; 2],
) -> RuleFix {
    let interior = strip_prefix(fixer.source_range(span), open);
    match interior.and_then(|interior| strip_suffix(interior, close)) {
        Some(interior) => {
            let mut replacement = format!("{new_open}{interior}{new_close}");
            pad_fix_with_token_boundary(fixer.source_text(), span, &mut replacement);
            fixer.replace(span, replacement)
        }
        None => fixer.noop(),
    }
}

/// `source` without its leading `prefix`, whose characters may each be written
/// as a Unicode escape (JLS §3.3). `source` starts at a lexer token boundary.
fn strip_prefix<'a>(source: &'a str, prefix: &str) -> Option<&'a str> {
    prefix.chars().try_fold(source, |source, expected| match unicode_escape(source.as_bytes(), 0) {
        Some((value, end)) => (value == u32::from(expected)).then(|| &source[end..]),
        None => source.strip_prefix(expected),
    })
}

/// `source` without its trailing `suffix`, whose characters may each be written
/// as a Unicode escape (JLS §3.3). `source` ends at a lexer token boundary.
pub fn strip_suffix<'a>(source: &'a str, suffix: &str) -> Option<&'a str> {
    suffix.chars().rev().try_fold(source, |source, expected| {
        let escape = source.rfind('\\').and_then(|start| {
            let (value, end) = unicode_escape(source.as_bytes(), start)?;
            (end == source.len()).then_some((value, start))
        });
        match escape {
            Some((value, start)) => (value == u32::from(expected)).then(|| &source[..start]),
            None => source.strip_suffix(expected),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripping_reads_each_character_as_itself_or_as_an_escape() {
        assert_eq!(strip_prefix("(x)", "("), Some("x)"));
        assert_eq!(strip_prefix(r"\u0028x)", "("), Some("x)"));
        assert_eq!(strip_prefix(r"\uuuu0028\u0078)", "("), Some(r"\u0078)"));
        assert_eq!(strip_prefix(r"re\uu0074urn;", "return"), Some(";"));
        assert_eq!(strip_prefix(r"\u0029x)", "("), None);
        assert_eq!(strip_prefix("x)", "("), None);
        assert_eq!(strip_prefix("", "("), None);

        assert_eq!(strip_suffix("(x)", ")"), Some("(x"));
        assert_eq!(strip_suffix(r"(x\u0029", ")"), Some("(x"));
        assert_eq!(strip_suffix(r"10\u006C", "l"), Some("10"));
        assert_eq!(strip_suffix(r"(\u0078\uuuu0029", ")"), Some(r"(\u0078"));
        assert_eq!(strip_suffix(r"(\u0078)", ")"), Some(r"(\u0078"));
        assert_eq!(strip_suffix(r"(x /* \u0029 */ )", ")"), Some(r"(x /* \u0029 */ "));
        assert_eq!(strip_suffix(r"a\u0062c", "bc"), Some("a"));
        assert_eq!(strip_suffix(r"10\u004C", "l"), None);
        assert_eq!(strip_suffix("(x", ")"), None);
        assert_eq!(strip_suffix("", ")"), None);
    }
}
