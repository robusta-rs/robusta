use std::{borrow::Cow, ops::Range};

use markdown::{ParseOptions, mdast::Node, to_mdast};

use super::{code_gap, columns, indented, next_column, quote_content, setext, span};

mod lazy;
mod underline;

/// Text to insert at an offset so that markdown-rs reads the content as CommonMark does.
type Repair<'a> = (usize, Cow<'a, str>);

/// Parse `content` with text inserted where markdown-rs would otherwise read it unlike CommonMark.
pub fn parse(content: &str, options: &ParseOptions) -> Option<Node> {
    let mut underlines = underline::Underlines::new(content);
    let mut repairs = Vec::new();
    let mut tabs = Vec::new();
    let mut root = parse_repaired(content, underlines.breaks(), options)?;
    // Each repair changes how later lines read, so look again after each parse.
    loop {
        if !underlines.settle(&root, content, &repairs) {
            // Tabs follow the latest reading alone, so that no padding stays in what became code.
            let mut measured = Vec::new();
            wide_tabs(&root, content, &mut measured);
            measured.sort_unstable();
            measured.dedup();
            // The other repairs read the lines as the tabs measure them.
            if measured == tabs {
                let known = repairs.len();
                unguarded_paragraphs(&root, content, &mut repairs);
                repairs.extend(next_guard_repair(&root, content, &repairs));
                repairs.sort_unstable();
                repairs.dedup();
                if repairs.len() == known {
                    break;
                }
            }
            tabs = measured;
            underlines.unsettle();
        }
        let mut all = [repairs.as_slice(), tabs.as_slice(), underlines.breaks()].concat();
        all.sort_unstable();
        root = parse_repaired(content, &all, options)?;
    }
    if underlines.any_kept() || repairs.iter().any(|(_, text)| text.ends_with(BREAK)) {
        remove_inserted_breaks(&mut root);
    }
    Some(root)
}

/// The thematic break that repairs insert to close blocks.
const BREAK: &str = "***";

/// A thematic break on a line of its own before `line`, behind the quote markers and indentation
/// before the text at `start` on it, which closes the blocks that text is not in.
fn break_before(content: &str, line: usize, start: usize) -> Repair<'_> {
    let text = content.len() - content[start..].trim_start_matches([' ', '\t']).len();
    (line - 1, format!("{}{BREAK}", &content[line - 1..text]).into())
}

/// Remove the thematic breaks that repairs inserted, which span no source text.
fn remove_inserted_breaks(node: &mut Node) {
    let Some(children) = node.children_mut() else { return };
    children.retain(|child| {
        !matches!(child, Node::ThematicBreak(_))
            || span(child).is_none_or(|range| !range.is_empty())
    });
    for child in children {
        remove_inserted_breaks(child);
    }
}

// Where a container closes, markdown-rs lets indented code or HTML interrupt a paragraph
// that starts with one of these bytes: `- a\n\n{x}\n    y` reads `y` as code.
const UNGUARDED_STARTS: &[u8] = b"#$*_`ei{~";

/// A space before each one-line paragraph that starts with an unguarded byte and has a line after it.
fn unguarded_paragraphs(node: &Node, content: &str, repairs: &mut Vec<Repair<'_>>) {
    if !matches!(node, Node::Paragraph(_)) {
        for child in node.children().into_iter().flatten() {
            unguarded_paragraphs(child, content, repairs);
        }
    } else if let Some(range) = span(node)
        && UNGUARDED_STARTS.contains(&content.as_bytes()[range.start])
        && !content[range.clone()].contains('\n')
        // A space after a list marker would move the item's content, and is not needed there.
        && let line = content[..range.start].rfind('\n').map_or(0, |newline| newline + 1)
        && code_gap(&content[line..range.start])
        && content[range.end..]
            .split('\n')
            .nth(1)
            .is_some_and(|line| !quote_content(line).trim().is_empty())
    {
        // A quote marker takes the first space as its own.
        let space = if content[..range.start].ends_with('>') { "  " } else { " " };
        repairs.push((range.start, space.into()));
    }
}

/// A thematic break before the first line of `block`, a paragraph in `container`, that starts
/// an ordered item zero-padded to 1, as in `a\n01. b`: CommonMark lets the item interrupt the
/// paragraph, markdown-rs only `1.`. Unlike a blank line, the break keeps the container tight.
fn padded_first_item<'a>(container: &Node, block: &Node, content: &'a str) -> Option<Repair<'a>> {
    let range = span(block).filter(|_| matches!(block, Node::Paragraph(_) | Node::Heading(_)))?;
    let line_prefix = |start: usize| {
        &content[content[..start].rfind('\n').map_or(0, |newline| newline + 1)..start]
    };
    // The quote markers and content column of the container, from the line of its first block.
    let first = container.children()?.first().and_then(span);
    let (quotes, base) = match (container, first) {
        (Node::ListItem(_), Some(first)) => quoted_indent(line_prefix(first.start)),
        (Node::Blockquote(_), Some(first)) => (quoted_indent(line_prefix(first.start)).0, 0),
        _ => (0, 0),
    };
    content[range.clone()].match_indices('\n').find_map(|(newline, _)| {
        let line = range.start + newline + 1;
        let text = content[line..].trim_start_matches([' ', '\t', '>']);
        let marker = content.len() - text.len();
        let digits = text.bytes().take_while(u8::is_ascii_digit).count();
        let (line_quotes, indent) = quoted_indent(&content[line..marker]);
        (line_quotes == quotes
            && (base..base + 4).contains(&indent)
            && digits > 1
            && text[..digits].trim_start_matches('0') == "1"
            && after_list_marker(text).is_some_and(|rest| !blank_first_line(rest)))
        .then(|| break_before(content, line, marker))
    })
}

/// The quote markers in `prefix`, and the columns from the content of the last one, after the
/// space it takes, to the end of `prefix`.
fn quoted_indent(prefix: &str) -> (usize, usize) {
    let (quotes, column) = quote_columns(prefix);
    (quotes.len(), column.saturating_sub(quotes.last().map_or(0, |quote| quote.1)))
}

/// Spaces before each tab wider than one column in the indentation that markdown-rs measures
/// in bytes: the line of a list item up to its content, and the line of a code fence up to the
/// fence. Otherwise `-\ta\n\n  b` keeps `b` in the item, and `>\t- a\n>\n>\t  b` does not.
fn wide_tabs(node: &Node, content: &str, repairs: &mut Vec<Repair<'_>>) {
    let end = span(node).and_then(|range| match node {
        Node::ListItem(item) => {
            let line_end =
                content[range.start..].find('\n').map_or(content.len(), |end| range.start + end);
            Some(
                item.children
                    .first()
                    .and_then(span)
                    .map_or(line_end, |child| child.start.min(line_end)),
            )
        }
        Node::Code(_) if matches!(content.as_bytes()[range.start], b'`' | b'~') => {
            Some(range.start)
        }
        _ => None,
    });
    if let Some(end) = end {
        let line = content[..end].rfind('\n').map_or(0, |newline| newline + 1);
        let mut column = 0;
        for (index, byte) in content.bytes().enumerate().take(end).skip(line) {
            let next = next_column(column, byte);
            if next - column > 1 {
                repairs.push((index, (&"   "[..next - column - 1]).into()));
            }
            column = next;
        }
    }
    for child in node.children().into_iter().flatten() {
        wide_tabs(child, content, repairs);
    }
}

/// A break before the first block that markdown-rs reads with the guard of the paragraph before
/// it, so that `a\n> <b>` starts an HTML block, `a\n>     b` indented code, and `a\n> 2. c` and
/// `[a]: b\n2. c` a list, indentation before the first list it reads without that guard, so that
/// `- a\n\nb\n2. c` continues the paragraph `b`, the containers of the paragraph before the first
/// lazy line it does not continue, or a line ending after it when they would make it an underline,
/// or a break before the first line it reads lazily after a setext heading or the first
/// zero-padded item it reads with the guard. Later ones wait, as that block can take them in.
fn next_guard_repair<'a>(
    node: &Node,
    content: &'a str,
    repairs: &[Repair<'_>],
) -> Option<Repair<'a>> {
    let children = node.children()?;
    for (index, child) in children.iter().enumerate() {
        if index > 0
            && let Some(repair) = guard_repair(&children[index - 1], child, content)
            && !repairs.contains(&repair)
        {
            return Some(repair);
        }
        if let Some(repair) = padded_first_item(node, child, content)
            && !repairs.contains(&repair)
        {
            return Some(repair);
        }
        if let Some(repair) = next_guard_repair(child, content, repairs) {
            return Some(repair);
        }
    }
    None
}

/// The text that makes markdown-rs read `block`, on the line after `before`, with the guard of
/// a paragraph exactly where CommonMark does.
fn guard_repair<'a>(before: &Node, block: &Node, content: &'a str) -> Option<Repair<'a>> {
    let (before_range, range) = (span(before)?, span(block)?);
    let gap = &content[before_range.end..range.start];
    if gap.matches('\n').count() != 1 {
        return None;
    }
    let line = content[..range.start].rfind('\n')? + 1;
    // After a line that closes a list item, markdown-rs drops the guard of the paragraph that
    // line starts, so any list interrupts it, not only a non-empty one starting at 1.
    if let (Node::Paragraph(_), Node::List(list)) = (before, block)
        && let text = content[range.start..].trim_start_matches([' ', '\t'])
        && let Some(rest) = after_list_marker(text)
        && let empty = blank_first_line(rest)
        && (empty || list.start.is_some_and(|start| start != 1))
    {
        // A lone dash underlines the paragraph.
        let marker = content.len() - text.len();
        return Some((marker, if empty && text.starts_with('-') { "-" } else { "    " }.into()));
    }
    if let Some(repair) = lazy::continuation(before, block, content) {
        return Some(repair);
    }
    // A setext underline closes its paragraph, so no line after it is lazy, but markdown-rs
    // keeps a paragraph line that lacks the heading's containers in them, as in `- a\n  ==\nb`.
    // A break before the line, behind the same markers, closes them first.
    if setext(before, content)
        && matches!(block, Node::Paragraph(_) | Node::Definition(_))
        && lazy::lazy_line_containers(content, before_range.start, line).is_some()
    {
        return Some(break_before(content, line, range.start));
    }
    reads_stale_guard(before, block, content).then(|| break_before(content, line, range.start))
}

/// Whether markdown-rs reads `block` with the guard of a paragraph that `before` does not leave open.
fn reads_stale_guard(before: &Node, block: &Node, content: &str) -> bool {
    // A container opened on the next line starts with no paragraph open inside it.
    if let Some(first) = first_contained_block(block) {
        return matches!(before, Node::Paragraph(_) | Node::Definition(_))
            && (opens_with_tag_or_code(first, content) || opens_with_list_marker(first, content));
    }
    // A definition leaves no paragraph for a list item to interrupt.
    matches!(before, Node::Definition(_)) && opens_with_list_marker(block, content)
}

/// The first block inside a block quote or list, through the containers it opens with.
fn first_contained_block(mut node: &Node) -> Option<&Node> {
    if !matches!(node, Node::Blockquote(_) | Node::List(_)) {
        return None;
    }
    while matches!(node, Node::Blockquote(_) | Node::List(_) | Node::ListItem(_)) {
        node = node.children()?.first()?;
    }
    Some(node)
}

/// The column of each quote marker in `prefix` with the column its content starts at, after
/// the space or tab column the marker takes, and the column at the end of `prefix`.
pub fn quote_columns(prefix: &str) -> (Vec<(usize, usize)>, usize) {
    let mut quotes = Vec::new();
    let mut column = 0;
    let mut bytes = prefix.bytes().peekable();
    while let Some(byte) = bytes.next() {
        let next = next_column(column, byte);
        if byte == b'>' {
            quotes.push((column, next + usize::from(matches!(bytes.peek(), Some(b' ' | b'\t')))));
        }
        column = next;
    }
    (quotes, column)
}

/// Whether a paragraph or setext heading starts with a tag, or four columns in as code does.
fn opens_with_tag_or_code(node: &Node, content: &str) -> bool {
    let tag = |range: Range<usize>| content[range].trim_start_matches([' ', '\t']).starts_with('<');
    matches!(node, Node::Paragraph(_) | Node::Heading(_))
        && span(node).is_some_and(|range| indented(content, range.clone()) || tag(range))
}

/// Whether a paragraph or setext heading starts with a list marker, as an item that cannot
/// interrupt a paragraph does.
fn opens_with_list_marker(node: &Node, content: &str) -> bool {
    let Some(range) = span(node) else { return false };
    matches!(node, Node::Paragraph(_) | Node::Heading(_))
        && !indented(content, range.clone())
        && after_list_marker(content[range].trim_start_matches([' ', '\t'])).is_some()
}

/// The text after the list marker that `text` starts with.
fn after_list_marker(text: &str) -> Option<&str> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    let marker = match digits {
        0 if text.starts_with(['-', '+', '*']) => 1,
        1..=9 if text[digits..].starts_with(['.', ')']) => digits + 1,
        _ => return None,
    };
    let rest = &text[marker..];
    rest.chars().next().is_none_or(|ch| matches!(ch, ' ' | '\t' | '\n')).then_some(rest)
}

/// Whether the line after a list marker is blank.
fn blank_first_line(rest: &str) -> bool {
    rest[..rest.find('\n').unwrap_or(rest.len())].trim_matches([' ', '\t']).is_empty()
}

/// Parse `content` with `repairs` inserted, with positions in `content`.
fn parse_repaired(content: &str, repairs: &[Repair<'_>], options: &ParseOptions) -> Option<Node> {
    if repairs.is_empty() {
        return to_mdast(content, options).ok();
    }
    let size = repairs.iter().map(|repair| repair.1.len()).sum::<usize>();
    let mut repaired = String::with_capacity(content.len() + size);
    let mut copied = 0;
    for (start, text) in repairs {
        repaired.push_str(&content[copied..*start]);
        repaired.push_str(text);
        copied = *start;
    }
    repaired.push_str(&content[copied..]);
    let mut root = to_mdast(&repaired, options).ok()?;
    restore_positions(&mut root, content, repairs);
    Some(root)
}

/// Move the positions of `node` back to `content`, before `repairs` were inserted.
fn restore_positions(node: &mut Node, content: &str, repairs: &[Repair<'_>]) {
    if let Some(position) = node.position_mut() {
        for point in [&mut position.start, &mut position.end] {
            let mut inserted = 0;
            for (start, text) in repairs {
                let at = start + inserted;
                if point.offset <= at {
                    break;
                }
                let passed = (point.offset - at).min(text.len());
                inserted += passed;
                point.line -= text[..passed].matches('\n').count();
            }
            point.offset -= inserted;
            let line = content[..point.offset].rfind('\n').map_or(0, |newline| newline + 1);
            // Spaces before a tab keep the columns after it.
            if repairs.iter().any(|(start, text)| {
                (line..=point.offset).contains(start)
                    && (content.as_bytes().get(*start) != Some(&b'\t') || text.contains('>'))
            }) {
                point.column = columns(&content[line..point.offset]) + 1;
            }
        }
    }
    for child in node.children_mut().into_iter().flatten() {
        restore_positions(child, content, repairs);
    }
}
