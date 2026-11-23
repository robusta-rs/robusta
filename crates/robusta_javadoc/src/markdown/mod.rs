use std::ops::Range;

use markdown::{
    Constructs, ParseOptions,
    mdast::{Code, Node},
};

mod repair;

/// Parse the content of a Markdown comment, with container code recovered.
pub fn parse(content: &str) -> Option<Node> {
    let options = ParseOptions { constructs: Constructs::gfm(), ..ParseOptions::default() };
    let mut root = repair::parse(content, &options)?;
    recover(&mut root, content);
    Some(root)
}

// markdown-rs can split one container code block into code, paragraphs, or definitions.
fn recover(node: &mut Node, content: &str) {
    let item = matches!(node, Node::ListItem(_));
    let container_end = span(node).map_or(content.len(), |range| range.end);
    let Some(children) = node.children_mut() else { return };
    for child in children.iter_mut() {
        recover(child, content);
        // A fenced block that its container closes after a blank line ends past the container,
        // and markdown-rs drops the last line ending from its value, which CommonMark keeps. A
        // block holding only that blank line reads the same in both.
        if let Some(range) = span(child)
            && range.end > container_end
            && let lines = &content[range.start..block_end(content, &range)]
            && lines.starts_with(['`', '~'])
            && lines.split_once('\n').is_some_and(|(_, body)| body.contains('\n'))
            && let Node::Code(code) = child
        {
            code.value.push('\n');
        }
        // An open fenced block holds every line to its container's end. markdown-rs drops a
        // last line that is blank and no line ending follows, from the value and sometimes from
        // the span. The value of a block holding no other line reads the same in both.
        if let Some(range) = span(child)
            && content[range.start..].starts_with(['`', '~'])
            && let end = if content[..range.end].ends_with('\n') {
                container_end.max(range.end)
            } else {
                range.end
            }
            && let Some((lines, last)) = content[range.start..end].rsplit_once('\n')
            && !last.is_empty()
            && code_gap(last)
            && range.end > range.start + lines.len()
            && let Node::Code(Code { value, position: Some(position), .. }) = child
        {
            if lines.matches('\n').count() > value.matches('\n').count() {
                value.push('\n');
            }
            if end > range.end {
                position.end.offset = end;
                position.end.column += columns(last);
            }
        }
    }
    let separated = |pair: &[Node]| {
        let (Some(before), Some(after)) = (span(&pair[0]), span(&pair[1])) else { return true };
        content[before.end..after.start].bytes().filter(|byte| *byte == b'\n').count() > 1
    };
    let tight = item
        && !children
            .windows(2)
            .any(|pair| indented_range(&pair[0], content).is_none() && separated(pair));
    let mut changed = false;
    let mut index = 0;
    while index < children.len() {
        // Indented lines right after a definition continue its paragraph.
        let continues = index > 0
            && matches!(children[index - 1], Node::Definition(_))
            && !separated(&children[index - 1..=index]);
        let Some(range) = indented_range(&children[index], content).filter(|_| !continues) else {
            index += 1;
            continue;
        };
        let mut end = index + 1;
        let mut combined = range.clone();
        while let Some(next) = children.get(end).and_then(|node| indented_range(node, content)) {
            if !code_gap(&content[combined.end..next.start]) {
                break;
            }
            combined.end = next.end;
            end += 1;
        }
        // Indented code consumes trailing blank lines without loosening its list.
        if tight
            && let Some(next) = children.get(end).and_then(span)
            && let gap = &content[combined.end..next.start]
            && code_gap(gap)
            && let Some(newline) = gap.rfind('\n')
        {
            combined.end += newline + 1;
        }
        let column = children[index].position().map_or(0, |position| position.start.column - 1);
        let Some(code) = IndentedCode::new(content, combined.clone(), column) else {
            index = end;
            continue;
        };
        let mut position = children[index].position().cloned();
        if let (Some(position), Some(last)) = (&mut position, children[end - 1].position()) {
            position.end = last.end.clone();
            if combined.end > position.end.offset {
                position.end.line += content[position.end.offset..combined.end]
                    .bytes()
                    .filter(|byte| *byte == b'\n')
                    .count();
                position.end.column = 1;
            }
            position.end.offset = combined.end;
        }
        changed |= end > index + 1
            || combined.end != range.end
            || !matches!(children[index], Node::Code(_));
        children[index] = Node::Code(Code { value: code.value, position, lang: None, meta: None });
        children.drain(index + 1..end);
        index += 1;
    }
    if changed && let Node::ListItem(item) = node {
        item.spread = item.children.windows(2).any(separated);
    }
}

/// Indented code, including leaves misparsed as paragraphs or definitions.
pub fn indented_range(node: &Node, content: &str) -> Option<Range<usize>> {
    let range = span(node)?;
    match node {
        Node::Code(_) if matches!(content.as_bytes()[range.start], b' ' | b'\t') => Some(range),
        Node::Paragraph(_) | Node::Definition(_) if indented(content, range.clone()) => Some(range),
        _ => None,
    }
}

/// Whether the text at `range` starts four or more columns into it.
fn indented(content: &str, range: Range<usize>) -> bool {
    let padding =
        content[range.clone()].bytes().take_while(|byte| matches!(byte, b' ' | b'\t')).count();
    let line = content[..range.start].rfind('\n').map_or(0, |newline| newline + 1);
    columns(&content[line..range.start + padding]) - columns(&content[line..range.start]) >= 4
}

/// Whether `gap` holds only blank lines and quote markers.
pub fn code_gap(gap: &str) -> bool {
    gap.bytes().all(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'>'))
}

/// The quote markers and indentation before `start`, with other characters as spaces.
pub fn continuation_prefix(content: &str, start: usize) -> String {
    let line_start = content[..start].rfind('\n').map_or(0, |newline| newline + 1);
    content[line_start..start]
        .chars()
        .map(|ch| if ch == '>' || ch.is_whitespace() { ch } else { ' ' })
        .collect()
}

/// Indented code with the prefix its lines continue.
pub struct IndentedCode {
    pub range: Range<usize>,
    pub prefix: String,
    pub value: String,
    pub padding: usize,
    pub newline: bool,
}

impl IndentedCode {
    /// The code at `range`, starting at `column`, or `None` when a line does not continue it.
    pub fn new(content: &str, range: Range<usize>, column: usize) -> Option<Self> {
        let line_start = content[..range.start].rfind('\n').map_or(0, |newline| newline + 1);
        let physical = columns(&content[line_start..range.start]);
        let padding = column.checked_sub(physical)?;
        let mut prefix = continuation_prefix(content, range.start);
        prefix.extend(std::iter::repeat_n(' ', padding));
        let depth = prefix.bytes().filter(|byte| *byte == b'>').count();
        let quoted = prefix.len() - quote_content(&prefix).len();
        let indent = column - columns(&prefix[..quoted]);
        let mut value = String::new();
        for (index, source) in content[range.clone()].split_terminator('\n').enumerate() {
            let mut line = source;
            if index > 0 {
                value.push('\n');
                for _ in 0..depth {
                    let rest = line.trim_start_matches([' ', '\t']).strip_prefix('>')?;
                    line = rest.strip_prefix(' ').unwrap_or(rest);
                }
            }
            let initial =
                if index == 0 { physical } else { columns(&source[..source.len() - line.len()]) };
            let width = if index == 0 { column + 4 } else { initial + indent + 4 };
            match indentation(line, initial..width, initial) {
                Some((range, extra)) => {
                    value.extend(std::iter::repeat_n(' ', extra));
                    value.push_str(&line[range.end..]);
                }
                None if line.trim_matches([' ', '\t']).is_empty() => {}
                None => return None,
            }
        }
        while let Some((before, last)) = value.rsplit_once('\n') {
            if !last.trim_matches([' ', '\t']).is_empty() {
                break;
            }
            value.truncate(before.len());
        }
        let newline = block_end(content, &range) < range.end;
        Some(Self { range, prefix, value, padding, newline })
    }
}

/// The closing fence of the fenced code at `range`, without its indentation and trailing
/// whitespace, or `None` when the block runs to its container's end.
pub fn closing_fence(content: &str, range: &Range<usize>) -> Option<Range<usize>> {
    let source = &content[range.clone()];
    let marker = source.as_bytes()[0];
    let opening = source.bytes().take_while(|byte| *byte == marker).count();
    let start = range.start + source.rfind('\n')? + 1;
    let line = &content[start..range.end];
    let unquoted = quote_content(line);
    let depth = |text: &str| text.bytes().filter(|byte| *byte == b'>').count();
    if depth(&line[..line.len() - unquoted.len()])
        != depth(&continuation_prefix(content, range.start))
    {
        return None;
    }
    let trimmed = unquoted.trim();
    let start = start + line.len() - unquoted.trim_start().len();
    (trimmed.len() >= opening && trimmed.bytes().all(|byte| byte == marker))
        .then_some(start..start + trimmed.len())
}

/// The lines holding the text of the code block at `range`, without its fences or the blank
/// lines that end indented code.
pub fn code_text(content: &str, range: &Range<usize>) -> Option<Range<usize>> {
    let end = block_end(content, range);
    if content[range.start..].starts_with(['`', '~']) {
        let start = range.start + content[range.start..end].find('\n')? + 1;
        let end = match closing_fence(content, range) {
            Some(closing) => content[..closing.start].rfind('\n')?,
            None => end,
        };
        return (start <= end).then_some(start..end);
    }
    let text_end =
        range.start + content[range.start..end].trim_end_matches([' ', '\t', '\n']).len();
    Some(range.start..content[text_end..end].find('\n').map_or(end, |newline| text_end + newline))
}

/// Skip quote markers while retaining indentation within the innermost quote.
pub fn quote_content(mut line: &str) -> &str {
    loop {
        let Some(rest) = line.trim_start_matches([' ', '\t']).strip_prefix('>') else {
            return line;
        };
        line = rest.strip_prefix(' ').unwrap_or(rest);
    }
}

/// The byte range of `node`.
pub fn span(node: &Node) -> Option<Range<usize>> {
    node.position().map(|position| position.start.offset..position.end.offset)
}

/// The end of the block at `range`, before the line ending its span holds after its last
/// line, as when a container closes the block.
pub fn block_end(content: &str, range: &Range<usize>) -> usize {
    range.end - usize::from(content[..range.end].ends_with('\n'))
}

/// The column after `byte`, with a tab stop every four columns.
pub fn next_column(column: usize, byte: u8) -> usize {
    column + if byte == b'\t' { 4 - column % 4 } else { 1 }
}

/// The column after `text`.
pub fn columns(text: &str) -> usize {
    text.bytes().fold(0, next_column)
}

/// The whitespace spanning these columns, including partial tabs at either edge.
pub fn indentation(
    line: &str,
    columns: Range<usize>,
    initial: usize,
) -> Option<(Range<usize>, usize)> {
    let mut column = initial;
    let mut start = None;
    for (index, byte) in line.bytes().enumerate() {
        if !(matches!(byte, b' ' | b'\t') || byte == b'>' && column < columns.start) {
            return None;
        }
        let next = next_column(column, byte);
        if start.is_none() && next > columns.start {
            start = Some((index, columns.start - column));
        }
        if next >= columns.end {
            let (start, before) = start?;
            return Some((start..index + 1, before + next - columns.end));
        }
        column = next;
    }
    None
}

#[cfg(test)]
mod tests;
