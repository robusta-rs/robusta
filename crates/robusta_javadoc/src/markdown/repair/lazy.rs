use std::ops::Range;

use markdown::{ParseOptions, mdast::Node, to_mdast};

use super::{Repair, quote_columns, underline::underline_line};
use crate::markdown::{columns, indented_range, span};

/// The text that makes markdown-rs continue, with `block` on a lazy line, the paragraph that
/// `before` leaves open in a container. A lazy line continues such a paragraph unless it starts
/// a block that can interrupt one, but markdown-rs closes the container before complete HTML,
/// as in `- a\n<b>`, and before a line that starts like a fence or HTML, as in `- a\n~`.
pub fn continuation<'a>(before: &Node, block: &Node, content: &'a str) -> Option<Repair<'a>> {
    let (paragraph, start) = (last_paragraph(before, content)?, span(block)?.start);
    if content[paragraph.end..start].matches('\n').count() != 1
        || !continues_paragraph(block, content)
    {
        return None;
    }
    let line = content[..start].rfind('\n')? + 1;
    // The containers would make a last line of underline marks underline the paragraph, as in
    // `> a\n--`, which markdown-rs reads lazily before a line ending.
    if underline_line(&content[line..]) {
        return Some((content.len(), "\n".into()));
    }
    lazy_line_containers(content, paragraph.start, line)
}

/// The paragraph that a block quote or list ends with, through the containers it ends with,
/// from the first of the definitions that markdown-rs reads at its start.
fn last_paragraph(node: &Node, content: &str) -> Option<Range<usize>> {
    if !matches!(node, Node::Blockquote(_) | Node::List(_)) {
        return None;
    }
    let mut children = node.children()?;
    while let Some(last @ (Node::Blockquote(_) | Node::List(_) | Node::ListItem(_))) =
        children.last()
    {
        children = last.children()?;
    }
    let paragraph = |node: &Node| {
        matches!(node, Node::Paragraph(_) | Node::Definition(_))
            && indented_range(node, content).is_none()
    };
    let (last, before) = children.split_last().filter(|(last, _)| paragraph(last))?;
    let mut range = span(last)?;
    for definition in before.iter().rev() {
        match span(definition) {
            Some(previous)
                if matches!(definition, Node::Definition(_))
                    && paragraph(definition)
                    && content[previous.end..range.start].matches('\n').count() == 1 =>
            {
                range.start = previous.start;
            }
            _ => break,
        }
    }
    Some(range)
}

/// Whether `block` reads as paragraph text after a paragraph line: a paragraph, or HTML that
/// cannot interrupt one, which no extension changes.
fn continues_paragraph(block: &Node, content: &str) -> bool {
    match (block, span(block)) {
        (Node::Paragraph(_), _) => true,
        (Node::Html(_), Some(range)) => {
            // The block can start inside a tab that its container takes part of.
            let line = content[range.start..].split('\n').next().unwrap_or_default();
            let line = line.trim_start_matches([' ', '\t']);
            to_mdast(&format!("a\n{line}"), &ParseOptions::default()).is_ok_and(|root| {
                matches!(root.children().map(Vec::as_slice), Some([Node::Paragraph(_)]))
            })
        }
        _ => false,
    }
}

/// The quote markers and indentation that the lazy line at `line` lacks to continue the
/// containers of the paragraph or setext heading at `paragraph`. Missing quote markers go after
/// the last one the line has, and indentation alone before its text.
pub fn lazy_line_containers(content: &str, paragraph: usize, line: usize) -> Option<Repair<'_>> {
    let text = line + content[line..].find(|ch| !matches!(ch, ' ' | '\t' | '>'))?;
    let first = content[..paragraph].rfind('\n').map_or(0, |newline| newline + 1);
    let (quotes, end) = quote_columns(&content[first..paragraph]);
    let (matched, mut column) = quote_columns(&content[line..text]);
    let missing = quotes.get(matched.len()..)?;
    let start = match (matched.len(), missing.is_empty()) {
        (count, false) if count > 0 => {
            line + content[line..text].match_indices('>').nth(count - 1)?.0 + 1
        }
        (_, false) => line,
        (_, true) => text,
    };
    column = column.min(columns(&content[line..start]));
    // Columns count from the content of the last quote the line shares with the paragraph line,
    // whose marker takes the first space after it.
    let (mut from, mut base) = match matched.len() {
        0 => (0, 0),
        count => (matched[count - 1].0 + 2, quotes[count - 1].1),
    };
    let mut insert = String::new();
    for &(marker, quote_content) in missing {
        let at = (from + marker).checked_sub(base)?;
        insert.extend(std::iter::repeat_n(' ', at.checked_sub(column)?));
        insert.push_str("> ");
        column = at + 2;
        (from, base) = (column, quote_content);
    }
    let at = (from + end).checked_sub(base)?;
    insert.extend(std::iter::repeat_n(' ', at.saturating_sub(column)));
    (!insert.is_empty()).then(|| (start, insert.into()))
}
