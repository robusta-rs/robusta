mod comment;
mod deletion;
mod delimiter;
mod import;

use robusta_span::Span;
use robusta_syntax::{identifier::identifier_part, unicode_escape::unicode_escape};

pub use self::comment::follows_documentation;
pub use self::deletion::{normalize_deletions, removal_span};
pub use self::delimiter::{keep_interior, replace_delimiters, strip_suffix};
pub use self::import::{delete_import, imports_java_lang_type};

// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_linter/src/utils/mod.rs
/// Pads replacement text with spaces when needed to preserve token boundaries
/// with neighboring source characters: `return(value)` -> `return value`.
pub fn pad_fix_with_token_boundary(source_text: &str, span: Span, replacement: &mut String) {
    let before = last_char(&source_text[..span.start as usize]);
    let after = first_char(&source_text[span.end as usize..]);
    let needs_pad_start = joins_token(before, first_char(replacement));
    let needs_pad_end = joins_token(last_char(replacement), after);

    if needs_pad_start {
        replacement.insert(0, ' ');
    }
    if needs_pad_end {
        replacement.push(' ');
    }
}

/// The first character of `text`, with a Unicode escape decoded (JLS §3.3).
fn first_char(text: &str) -> Option<char> {
    match unicode_escape(text.as_bytes(), 0) {
        Some((value, _)) => Some(char::from_u32(value).unwrap_or('\\')),
        None => text.chars().next(),
    }
}

/// The last character of `text`, with a Unicode escape decoded (JLS §3.3).
fn last_char(text: &str) -> Option<char> {
    let escape = text.rfind('\\').and_then(|start| {
        unicode_escape(text.as_bytes(), start).filter(|&(_, end)| end == text.len())
    });
    match escape {
        Some((value, _)) => Some(char::from_u32(value).unwrap_or('\\')),
        None => text.chars().next_back(),
    }
}

/// Whether `left` and `right` side by side could continue one token: an
/// identifier, or `++` or `--`. A backslash left undecoded, such as one that
/// escapes a surrogate, counts as an identifier part.
fn joins_token(left: Option<char>, right: Option<char>) -> bool {
    let is_part = |character: char| character == '\\' || identifier_part(character).is_some();
    match (left, right) {
        (Some(left), Some(right)) => {
            (is_part(left) && is_part(right)) || (left == right && matches!(left, '+' | '-'))
        }
        _ => false,
    }
}
