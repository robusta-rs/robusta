use robusta_syntax::{
    identifier::{is_horizontal_whitespace, is_horizontal_whitespace_char},
    line_terminator::split_lines,
    unicode_escape::SourceBytes,
};

/// Split a traditional comment interior into physical lines and strip its
/// conventional star margin.
///
/// As in the javadoc tool, a line after the first that has no leading `*`
/// keeps its indentation, so code pasted into `<pre>` without stars keeps its
/// shape.
pub fn traditional_lines(inner: &str) -> Vec<&str> {
    split_lines(inner)
        .enumerate()
        .map(|(index, line)| strip_traditional_margin(line, index == 0))
        .collect()
}

/// Drop indentation, leading asterisks, one following space, and trailing
/// spaces and tabs. A star-less line keeps its indentation unless it is the
/// first. A trailing form feed stays, as javadoc does not skip it before the
/// line break that must follow a snippet's colon.
fn strip_traditional_margin(line: &str, first: bool) -> &str {
    let trimmed = line.trim_start_matches(is_horizontal_whitespace_char);
    let body = if trimmed.starts_with('*') {
        let rest = trimmed.trim_start_matches('*');
        rest.strip_prefix(' ').unwrap_or(rest)
    } else if first {
        trimmed
    } else {
        line
    };
    body.trim_end_matches([' ', '\t'])
}

/// Split a Markdown documentation-comment run into physical content lines.
///
/// Each lexer comment contributes one `///` margin. The source slice spanning
/// a run also contains the intervening terminators and indentation, so remove
/// both before exposing the Markdown content. The common leading spaces and
/// tabs after the markers form the run's margin; indentation beyond that and
/// all trailing whitespace are Markdown content and can encode code or a hard
/// line break.
pub fn markdown_lines(text: &str) -> Vec<&str> {
    markdown_margin(split_lines(text).map(strip_markdown_marker).collect())
}

pub fn markdown_literal_lines(text: &str) -> impl Iterator<Item = Option<&str>> {
    source_lines(text).map(|line| {
        let margin = SourceBytes::new(line)
            .take_while(|&(byte, _)| is_horizontal_whitespace(byte))
            .last()
            .map_or(0, |(_, end)| end);
        line[margin..].strip_prefix("///")
    })
}

/// Raw slices split at translated Java line terminators.
fn source_lines(text: &str) -> impl Iterator<Item = &str> {
    let mut bytes = SourceBytes::new(text).peekable();
    let (mut start, mut done) = (0, false);
    std::iter::from_fn(move || {
        if done {
            return None;
        }
        let mut end = start;
        while let Some((byte, next)) = bytes.next() {
            if matches!(byte, b'\r' | b'\n') {
                let line = &text[start..end];
                start = next;
                if byte == b'\r' && bytes.peek().is_some_and(|&(byte, _)| byte == b'\n') {
                    start = bytes.next().expect("peeked LF").1;
                }
                return Some(line);
            }
            end = next;
        }
        done = true;
        Some(&text[start..])
    })
}

pub fn markdown_margin(mut lines: Vec<&str>) -> Vec<&str> {
    let common_margin = lines
        .iter()
        .filter_map(|line| line.bytes().position(|byte| !matches!(byte, b' ' | b'\t')))
        .min()
        .unwrap_or(0);
    for line in &mut lines {
        *line = line.get(common_margin..).unwrap_or("");
    }
    lines
}

fn strip_markdown_marker(line: &str) -> &str {
    let trimmed = line.trim_start_matches(is_horizontal_whitespace_char);
    trimmed.strip_prefix("///").expect("Markdown Javadoc line starts with `///`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_margin_stars_and_one_space_without_losing_content_indent() {
        assert_eq!(strip_traditional_margin(" * text  ", false), "text");
        assert_eq!(strip_traditional_margin(" ** text  ", false), "text");
        assert_eq!(strip_traditional_margin(" ***   text", false), "  text");
        assert_eq!(strip_traditional_margin(" ** * text", false), "* text");
        assert_eq!(strip_traditional_margin(" ** * text", true), "* text");
        assert_eq!(strip_traditional_margin("*text", false), "text");
        assert_eq!(strip_traditional_margin("   *   spaced", false), "  spaced");
        assert_eq!(strip_traditional_margin("no star", false), "no star");
        assert_eq!(strip_traditional_margin("   *   ", false), "");
        assert_eq!(strip_traditional_margin("\t* tabbed\t", false), "tabbed");
        assert_eq!(strip_traditional_margin(" * {@snippet :\x0c \t", false), "{@snippet :\x0c");
    }

    #[test]
    fn keeps_indentation_of_star_less_lines_after_the_first() {
        assert_eq!(
            traditional_lines("  first\n    code\n        nested\n   "),
            ["first", "    code", "        nested", ""]
        );
    }

    #[test]
    fn strips_each_markdown_margin_without_losing_markdown_whitespace() {
        assert_eq!(
            markdown_lines("/// Summary  \n  ///   indented\r\n\t///"),
            ["Summary  ", "  indented", ""]
        );
    }

    #[test]
    fn literal_markdown_lines_translate_boundaries_and_keep_content_spelling() {
        assert_eq!(
            markdown_margin(
                markdown_literal_lines(r"/// first  \u000d\u000a\u0020/// \u005cu000a")
                    .collect::<Option<Vec<_>>>()
                    .unwrap()
            ),
            vec!["first  ", r"\u005cu000a"]
        );
        assert_eq!(
            markdown_literal_lines("/// first\\u000d\n/// last").collect::<Option<Vec<_>>>(),
            Some(vec![" first", " last"])
        );
        assert!(markdown_literal_lines(r"/// first\u000a\u002f// last").any(|line| line.is_none()));
    }
}
