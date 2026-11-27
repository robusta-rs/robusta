use std::borrow::Cow;

use robusta_ast::Expression;
use robusta_formatter_core::{LINE_TERMINATORS, arena_cow_str, normalize_newlines, write};
use robusta_syntax::line_terminator::split_lines;

use crate::formatter::prelude::*;
use crate::utils::expression::leftmost;

const DELIMITER: &str = r#"""""#;

/// Writes the text block `raw` (JLS §3.10.6). Its lines take the indentation
/// of the line the opening `"""` is on, one level deeper unless the opening
/// starts that line, plus the indentation the compiler leaves them. With
/// `text_blocks`, a closing `"""` after text moves to the next line.
pub fn format_text_block<'a>(raw: &'a str, f: &mut JavaFormatter<'_, 'a>) {
    let Some(mut lines) = text_block_lines(raw) else {
        // The IR carries LF as its only break, so CR and CRLF fold to LF here
        // and the printer re-emits the configured line ending. JLS §3.10.6
        // normalizes them the same way before anything else.
        let normalized = normalize_newlines(raw, LINE_TERMINATORS);
        text(arena_cow_str(&normalized, f)).fmt(f);
        return;
    };

    let content = format_once(|f| {
        let mut line = lines.next().expect("a text block has an opening line");
        let mut blank_lines = 0;
        for next in lines {
            if next.is_empty() {
                blank_lines += 1;
                continue;
            }
            write_line(line, blank_lines, f);
            hard_line_break().fmt(f);
            line = next;
            blank_lines = 0;
        }
        match text_before_closing(line).filter(|_| f.options().text_blocks) {
            // A `\` keeps the line break out of the string (JLS §3.10.7). The
            // compiler keeps white space before it, but drops it before `"""`.
            Some(text_line) => {
                write!(f, [text(text_line), token("\\"), hard_line_break(), token(DELIMITER)]);
            }
            None => write_line(line, blank_lines, f),
        }
    });
    indent_from_line(&content).fmt(f);
}

/// Whether `expression` is a text block placed as an array literal, as one is
/// by default. With `text_blocks`, its lines break every group around it
/// instead, so its opening `"""` starts a wrapped line.
pub fn is_array_like_text_block(expression: &Expression<'_>, f: &JavaFormatter<'_, '_>) -> bool {
    matches!(expression, Expression::StringLiteral(literal) if literal.is_text_block())
        && !f.options().text_blocks
}

/// Whether the line breaks before `expression` because, with `text_blocks`, it
/// starts with a text block, whose opening `"""` starts a wrapped line.
pub fn breaks_before_text_block(expression: &Expression<'_>, f: &JavaFormatter<'_, '_>) -> bool {
    f.options().text_blocks
        && matches!(leftmost(expression), Expression::StringLiteral(literal) if literal.is_text_block())
}

/// The text before the closing `"""` on the last of [`text_block_lines`],
/// without the white space after it, or `None` when the closing `"""` starts
/// its line.
pub fn text_before_closing(line: &str) -> Option<&str> {
    let text = line.strip_suffix(DELIMITER)?.trim_end_matches(is_java_white_space);
    (!text.is_empty()).then_some(text)
}

/// The lines of the text block `raw` without the indentation the compiler
/// removes, or `None` when a Unicode escape could spell part of it.
///
/// The first line is the opening `"""` and any white space after it. A blank
/// line is empty, and a closing `"""` that starts its line has no white space
/// before it. The compiler removes the indentation every content line and the
/// closing line share, as `String.stripIndent` does, so any indentation the
/// same lines share in its place keeps the string's value.
pub fn text_block_lines(raw: &str) -> Option<impl Iterator<Item = &str>> {
    // White space and line breaks count only after Unicode escapes are
    // translated, so a text block that has one keeps its lines as written.
    if matches!(robusta_lexer::translate(raw).cooked, Cow::Owned(_)) {
        return None;
    }

    let incidental = split_lines(raw)
        .skip(1)
        .filter(|line| !is_blank(line))
        .map(|line| line.chars().take_while(|&c| is_java_white_space(c)).count())
        .min()
        .unwrap_or(0);

    Some(split_lines(raw).enumerate().map(move |(index, line)| {
        if index == 0 {
            line
        } else if is_blank(line) {
            ""
        } else if line.trim_start_matches(is_java_white_space) == DELIMITER {
            DELIMITER
        } else {
            line.char_indices().nth(incidental).map_or("", |(start, _)| &line[start..])
        }
    }))
}

/// Writes `line` and the `blank_lines` after it. A line break on an empty line
/// prints nothing, so the breaks of the blank lines end the text before them,
/// and the break after them only starts the next line at its indentation.
fn write_line<'a>(line: &'a str, blank_lines: usize, f: &mut JavaFormatter<'_, 'a>) {
    if blank_lines == 0 {
        text(line).fmt(f);
    } else {
        let line = format!("{line}{}", "\n".repeat(blank_lines + 1));
        text(f.allocator().alloc_str(&line)).fmt(f);
    }
}

fn is_blank(line: &str) -> bool {
    line.chars().all(is_java_white_space)
}

/// Java's `Character.isWhitespace`, which `String.stripIndent` counts as one
/// character each: a space, line, or paragraph separator other than a no-break
/// space, or one of the control characters tab through carriage return and
/// the four information separators.
fn is_java_white_space(c: char) -> bool {
    matches!(
        c,
        '\t'..='\r'
            | '\u{1C}'..='\u{20}'
            | '\u{1680}'
            | '\u{2000}'..='\u{2006}'
            | '\u{2008}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{205F}'
            | '\u{3000}'
    )
}
