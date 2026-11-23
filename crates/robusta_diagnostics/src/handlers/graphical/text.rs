//! Visible spellings for control characters in diagnostic text.

use std::{borrow::Cow, fmt};

/// Byte scan for C0, DEL, and UTF-8 encoded C1 controls.
pub(super) fn has_controls(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.iter().enumerate().any(|(i, &byte)| {
        byte.is_ascii_control()
            || (byte == 0xc2 && bytes.get(i + 1).is_some_and(|b| (0x80..=0x9f).contains(b)))
    })
}

/// Preserve prose line breaks and tabs; escape controls before applying styles or wrapping.
pub(super) fn escape_controls(text: &str) -> Cow<'_, str> {
    if !has_controls(text) {
        return Cow::Borrowed(text);
    }
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\n' | '\t') {
            escaped.push(c);
        } else {
            write_visible_char(&mut escaped, c).expect("writing to a String cannot fail");
        }
    }
    Cow::Owned(escaped)
}

pub(super) fn write_visible_char(f: &mut impl fmt::Write, c: char) -> fmt::Result {
    match c {
        '\0'..='\u{1f}' => {
            f.write_char(char::from_u32(0x2400 + u32::from(c)).expect("valid control picture"))
        }
        '\u{7f}' => f.write_char('\u{2421}'),
        '\u{80}'..='\u{9f}' => write!(f, "<U+{:04X}>", u32::from(c)),
        _ => f.write_char(c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_text_is_borrowed() {
        for text in ["", "plain diagnostic", "Café 火 e\u{301}", "non-breaking\u{a0}space"] {
            assert!(matches!(escape_controls(text), Cow::Borrowed(_)));
        }
    }

    #[test]
    fn every_control_has_a_visible_spelling() {
        for code in (0..=0x1f).chain(0x7f..=0x9f) {
            let c = char::from_u32(code).unwrap();
            let mut visible = String::new();
            write_visible_char(&mut visible, c).unwrap();
            assert!(!visible.chars().any(char::is_control));
            assert_eq!(
                unicode_width::UnicodeWidthStr::width(visible.as_str()),
                if code < 0x80 { 1 } else { 8 }
            );
        }
        assert_eq!(escape_controls("a\tline\nb\r\n"), "a\tline\nb␍\n");
    }
}
