use std::borrow::Cow;

use unicode_general_category::{GeneralCategory, get_general_category};

/// Whether `byte` is Java horizontal whitespace: space, tab, or form feed
/// (JLS §3.6). Line terminators are classified separately.
#[inline]
pub const fn is_horizontal_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | 0x0c)
}

/// [`is_horizontal_whitespace`] for call sites that match on `char`, such as
/// [`str::trim_start_matches`]. The set is the byte form's, so the two cannot
/// drift apart.
#[inline]
pub const fn is_horizontal_whitespace_char(character: char) -> bool {
    character.is_ascii() && is_horizontal_whitespace(character as u8)
}

#[inline]
pub fn is_identifier_start(ch: char) -> bool {
    if ch.is_ascii() {
        is_ascii_ident_start(ch as u8)
    } else {
        is_ident_start_category(get_general_category(ch))
    }
}

#[inline]
pub fn is_ascii_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$')
}

fn is_ident_start_category(category: GeneralCategory) -> bool {
    matches!(
        category,
        GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
            | GeneralCategory::LetterNumber
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::ConnectorPunctuation
    )
}

pub enum IdentifierPart {
    Significant,
    Ignorable,
}

pub fn identifier_part(ch: char) -> Option<IdentifierPart> {
    if ch.is_ascii() {
        return is_ascii_ident_continue(ch as u8).then_some({
            if is_ascii_ident_ignorable(ch as u8) {
                IdentifierPart::Ignorable
            } else {
                IdentifierPart::Significant
            }
        });
    }
    let category = get_general_category(ch);
    // javac retains supplementary format characters despite JLS §3.8.
    if category == GeneralCategory::Format && ch <= '\u{ffff}' {
        Some(IdentifierPart::Ignorable)
    } else if is_ident_start_category(category)
        || matches!(
            category,
            GeneralCategory::DecimalNumber
                | GeneralCategory::SpacingMark
                | GeneralCategory::NonspacingMark
                | GeneralCategory::Format
        )
    {
        Some(IdentifierPart::Significant)
    } else if matches!(ch, '\u{0080}'..='\u{009f}') {
        Some(IdentifierPart::Ignorable)
    } else {
        None
    }
}

#[inline]
pub fn is_ascii_ident_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$') || is_ascii_ident_ignorable(byte)
}

#[inline]
pub fn is_ascii_ident_ignorable(byte: u8) -> bool {
    matches!(byte, b'\x00'..=b'\x08' | b'\x0e'..=b'\x1b' | b'\x7f')
}

fn is_identifier_ignorable(ch: char) -> bool {
    matches!(identifier_part(ch), Some(IdentifierPart::Ignorable))
}

/// Remove BMP identifier-ignorable characters from a cooked binding spelling.
pub fn identifier_name(name: &str) -> Cow<'_, str> {
    if name.bytes().all(|byte| (b' '..b'\x7f').contains(&byte)) {
        return Cow::Borrowed(name);
    }
    let Some(first) = name.find(is_identifier_ignorable) else {
        return Cow::Borrowed(name);
    };
    Cow::Owned(remove_ignorable_characters(name, first))
}

pub fn remove_ignorable_characters(name: &str, first: usize) -> String {
    let mut identity = String::with_capacity(name.len());
    identity.push_str(&name[..first]);
    identity.extend(name[first..].chars().filter(|&ch| !is_identifier_ignorable(ch)));
    identity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_whitespace_matches_the_jls_set() {
        assert!(is_horizontal_whitespace(b' '));
        assert!(is_horizontal_whitespace(b'\t'));
        assert!(is_horizontal_whitespace(0x0c));
        assert!(!is_horizontal_whitespace(b'\n'));
        assert!(!is_horizontal_whitespace(b'\r'));
        assert!(!is_horizontal_whitespace(0x0b));
    }

    #[test]
    fn the_char_form_matches_the_byte_form_over_ascii_and_rejects_the_rest() {
        for byte in 0..=127u8 {
            assert_eq!(
                is_horizontal_whitespace_char(byte as char),
                is_horizontal_whitespace(byte),
                "disagreed on byte {byte}"
            );
        }
        // Unicode space separators are not Java white space (JLS §3.6).
        assert!(!is_horizontal_whitespace_char('\u{a0}'));
        assert!(!is_horizontal_whitespace_char('\u{3000}'));
    }
}
