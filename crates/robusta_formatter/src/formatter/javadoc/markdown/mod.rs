use std::borrow::Cow;
use std::fmt::Write as _;
use std::ops::Range;

use markdown::mdast::{AlignKind, Heading, Node, Table, TableCell};

use robusta_formatter_core::IndentWidth;
use robusta_javadoc::markdown::{code_gap, columns, indented_range, parse, quote_content, span};
use robusta_javadoc::{Structure, TagForm};

use super::normalize::capitalize_first;

mod code;
mod lists;

/// The structure of a Markdown comment's `content` after normalization.
pub fn normalized_structure(content: &str, capitalize: bool) -> Structure {
    let structure = Structure::from_markdown(content);
    match normalize(content, &structure, capitalize) {
        Some(normalized) => Structure::from_markdown(&normalized),
        None => structure,
    }
}

/// Normalize headings, emphasis, lists, tables, and code blocks, or return
/// `None` when nothing changes.
fn normalize(content: &str, structure: &Structure, capitalize: bool) -> Option<String> {
    if !has_markup(content) {
        return None;
    }
    // javadoc reads the description and each block tag as separate Markdown.
    // Tags are not Markdown, so no rule edits inside one.
    let mut starts = vec![0];
    let mut protected = structure
        .literal_ranges()
        .iter()
        .filter(|range| content[range.start..].starts_with("{@"))
        .cloned()
        .collect::<Vec<_>>();
    for tag in structure.tags() {
        let range = tag.range();
        if tag.form() == TagForm::Block {
            let line_start = content[..range.start].rfind('\n').map_or(0, |newline| newline + 1);
            starts.push(line_start);
            protected.push(line_start..range.end);
        } else {
            protected.push(range);
        }
    }
    starts.push(content.len());
    let mut edits = Vec::new();
    for (index, bounds) in starts.windows(2).enumerate() {
        // javadoc reads each section to the end of its last non-empty line, so a block open
        // there ends with that line, whitespace included.
        let start = bounds[0];
        let end = start + content[start..bounds[1]].trim_end_matches('\n').len();
        let protected = protected
            .iter()
            .filter(|range| start <= range.start && range.start < end)
            .map(|range| range.start - start..range.end.min(end) - start)
            .collect::<Vec<_>>();
        // Block tag descriptions keep their case.
        let capitalize = capitalize && index == 0;
        let section = section_edits(&content[start..end], &protected, capitalize);
        edits.extend(
            section.into_iter().map(|edit| Edit {
                range: edit.range.start + start..edit.range.end + start,
                ..edit
            }),
        );
    }
    apply(content, &mut edits).filter(|normalized| normalized != content)
}

/// Whether any rule could apply. Plain prose skips the Markdown parser.
fn has_markup(content: &str) -> bool {
    if content.contains(['*', '|']) || content.contains("__") {
        return true;
    }
    let mut block_start = true;
    content.split('\n').any(|line| {
        let line = quote_content(line);
        let text = line.trim_start_matches([' ', '\t']);
        // Indented code cannot interrupt a paragraph, but can follow a heading.
        let code = block_start && columns(&line[..line.len() - text.len()]) >= 4;
        let number = text.trim_start_matches(|ch: char| ch.is_ascii_digit());
        let marker = text.starts_with(['-', '+', '=', '`', '~'])
            || (number.len() < text.len() && number.starts_with(['.', ')']));
        block_start = text.is_empty() || text.starts_with('#');
        code || marker
    })
}

/// The edits for one Markdown `section`, outside its `protected` ranges.
fn section_edits(section: &str, protected: &[Range<usize>], capitalize: bool) -> Vec<Edit> {
    let mut content = Cow::Borrowed(section);
    let mut protected = protected.to_vec();
    loop {
        let Some(mut root) = parse(&content) else {
            return Vec::new();
        };
        let mut normalizer = Normalizer {
            content: &content,
            capitalize,
            groups: Vec::new(),
            lists: Vec::new(),
            capitals: Vec::new(),
        };
        normalizer.children(&root, Scope { root: true, ..Scope::default() });
        let outside = |edit: &Edit| !protected.iter().any(|range| edit.touches(range));
        let mut lists = normalizer
            .lists
            .into_iter()
            .filter(|group| group.iter().all(outside))
            .flatten()
            .collect::<Vec<_>>();
        if !lists.is_empty()
            && let Some(edited) = apply(&content, &mut lists).filter(|edited| edited != &*content)
            && same_tree(&mut root, &edited)
        {
            // Reparse before block edits, whose generated prefixes use the new indentation.
            for range in &mut protected {
                for point in [&mut range.start, &mut range.end] {
                    let original = *point;
                    for edit in &lists {
                        if edit.range.end <= original {
                            *point = *point + edit.text.len() - edit.range.len();
                        }
                    }
                }
            }
            content = Cow::Owned(edited);
            continue;
        }
        let mut edits = lists;
        edits.extend(
            normalizer.groups.into_iter().filter(|group| group.iter().all(outside)).flatten(),
        );
        // Keep the section as written if the edits would change what it renders.
        if !edits.is_empty()
            && !apply(&content, &mut edits).is_some_and(|edited| same_tree(&mut root, &edited))
        {
            edits.clear();
        }
        edits.extend(normalizer.capitals.into_iter().filter(outside));
        return if matches!(content, Cow::Borrowed(_)) {
            edits
        } else {
            apply(&content, &mut edits)
                .map_or_default(|edited| vec![Edit::new(0..section.len(), edited)])
        };
    }
}

/// Whether `edited` parses to the tree of the original section, `root`.
fn same_tree(root: &mut Node, edited: &str) -> bool {
    let Some(mut edited) = parse(edited) else {
        return false;
    };
    strip(root);
    strip(&mut edited);
    *root == edited
}

/// Drop what the edits may change without changing the rendering: positions,
/// line breaks in text, and a short table row's missing cells.
fn strip(node: &mut Node) {
    node.position_set(None);
    match node {
        Node::Text(text) if text.value.contains('\n') => {
            let mut bytes = std::mem::take(&mut text.value).into_bytes();
            for byte in bytes.iter_mut().filter(|byte| **byte == b'\n') {
                *byte = b' ';
            }
            text.value = String::from_utf8(bytes).expect("an ASCII swap keeps UTF-8 valid");
        }
        Node::Table(table) => {
            let columns = table.align.len();
            for row in &mut table.children {
                if let Some(cells) = row.children_mut()
                    && cells.len() < columns
                {
                    cells.resize_with(columns, || {
                        Node::TableCell(TableCell { children: Vec::new(), position: None })
                    });
                }
            }
        }
        _ => {}
    }
    for child in node.children_mut().into_iter().flatten() {
        strip(child);
    }
}

/// A replacement of `range` in the Markdown content.
struct Edit {
    range: Range<usize>,
    text: Cow<'static, str>,
}

impl Edit {
    fn new(range: Range<usize>, text: impl Into<Cow<'static, str>>) -> Self {
        Self { range, text: text.into() }
    }

    /// Whether the edit replaces or inserts text inside `range`.
    fn touches(&self, range: &Range<usize>) -> bool {
        range.start < self.range.end.max(self.range.start + 1) && self.range.start < range.end
    }
}

/// Apply `edits`, or return `None` if two of them overlap.
fn apply(content: &str, edits: &mut [Edit]) -> Option<String> {
    edits.sort_by_key(|edit| (edit.range.start, edit.range.end));
    let mut output = String::with_capacity(content.len() + 16);
    let mut cursor = 0;
    for edit in edits.iter() {
        output.push_str(content.get(cursor..edit.range.start)?);
        output.push_str(&edit.text);
        cursor = edit.range.end;
    }
    output.push_str(&content[cursor..]);
    Some(output)
}

#[derive(Clone, Copy, Default)]
struct Scope {
    /// Directly in the document rather than in a container.
    root: bool,
    /// In a footnote, whose definition marker is not a continuation prefix.
    footnote: bool,
    /// The preceding paragraph, if a table could interrupt it.
    paragraph_end: Option<usize>,
    /// In emphasis, where nested emphasis keeps `*`.
    emphasized: bool,
    /// An ancestor item changed width and needs fresh child positions.
    reindented: bool,
}

struct Normalizer<'a> {
    content: &'a str,
    capitalize: bool,
    /// Edits, grouped by the rule application that needs all of them.
    groups: Vec<Vec<Edit>>,
    lists: Vec<Vec<Edit>>,
    /// Capitalized list items, kept apart because they change the text.
    capitals: Vec<Edit>,
}

impl Normalizer<'_> {
    fn children(&mut self, parent: &Node, scope: Scope) {
        let Some(children) = parent.children() else {
            return;
        };
        for (index, child) in children.iter().enumerate() {
            let scope = if matches!(child, Node::List(_)) {
                self.list(children, index, scope)
            } else {
                scope
            };
            // markdown-rs can split indented container code into paragraphs and code siblings.
            if indented_range(child, self.content).is_some_and(|range| {
                matches!(child, Node::Paragraph(_))
                    || [index.checked_sub(1), index.checked_add(1)].into_iter().flatten().any(
                        |index| {
                            let Some(other) = children
                                .get(index)
                                .and_then(|node| indented_range(node, self.content))
                            else {
                                return false;
                            };
                            let gap = range.end.min(other.end)..range.start.max(other.start);
                            code_gap(&self.content[gap])
                        },
                    )
            }) {
                continue;
            }
            let paragraph_end = index.checked_sub(1).and_then(|index| match &children[index] {
                node @ Node::Paragraph(_) => span(node).map(|range| range.end),
                _ => None,
            });
            self.node(child, Scope { paragraph_end, ..scope });
        }
    }

    fn node(&mut self, node: &Node, scope: Scope) {
        let Some(range) = span(node) else {
            return;
        };
        let mut inner = Scope { root: false, ..scope };
        match node {
            Node::Code(code) if !scope.footnote => self.code_block(code, range),
            Node::Heading(heading) if !scope.footnote => self.setext_heading(heading, range),
            Node::Emphasis(_) => {
                if !scope.emphasized {
                    self.delimiters(node, range, "*", "_");
                }
                inner.emphasized = true;
            }
            Node::Strong(_) => self.delimiters(node, range, "__", "**"),
            Node::Table(table) if !scope.footnote => self.table(table, scope.paragraph_end),
            Node::FootnoteDefinition(_) => inner.footnote = true,
            _ => {}
        }
        self.children(node, inner);
    }

    fn push(&mut self, mut group: Vec<Edit>) {
        group.retain(|edit| self.content[edit.range.clone()] != *edit.text);
        if !group.is_empty() {
            self.groups.push(group);
        }
    }

    /// `Title\n===` -> `# Title`.
    fn setext_heading(&mut self, heading: &Heading, range: Range<usize>) {
        let (Some(first), Some(last)) =
            (heading.children.first().and_then(span), heading.children.last().and_then(span))
        else {
            return;
        };
        let text = &self.content[first.start..last.end];
        // One line is ATX already. A trailing `#` would become a closing sequence.
        if !self.content[range.clone()].contains('\n')
            || text.ends_with('#')
            || heading.children.iter().any(|child| matches!(child, Node::Break(_)))
        {
            return;
        }
        let mut group = vec![Edit::new(
            range.start..first.start,
            format!("{} ", "#".repeat(usize::from(heading.depth))),
        )];
        for (offset, _) in text.match_indices('\n') {
            let newline = first.start + offset;
            let start = self.content[..newline].trim_end_matches([' ', '\t']).len();
            let next = &self.content[newline + 1..];
            let end = newline + 1 + next.len()
                - quote_content(next).trim_start_matches([' ', '\t']).len();
            group.push(Edit::new(start..end, " "));
        }
        group.push(Edit::new(last.end..range.end, ""));
        self.push(group);
    }

    /// `*text*` -> `_text_` and `__text__` -> `**text**`, unless a neighbor,
    /// a word, or text inside could pair with the new delimiters.
    fn delimiters(&mut self, node: &Node, range: Range<usize>, from: &str, to: &'static str) {
        let content = self.content;
        let width = from.len();
        let source = &content[range.clone()];
        if source.len() <= 2 * width || !source.starts_with(from) || !source.ends_with(from) {
            return;
        }
        let inside = &source[width..source.len() - width];
        let before = content[..range.start].chars().next_back();
        let after = content[range.end..].chars().next();
        let edges = [before, inside.chars().next(), inside.chars().next_back(), after];
        let in_word = |ch: Option<char>| {
            ch.is_some_and(|ch| !ch.is_whitespace() && !ch.is_ascii_punctuation())
        };
        let underscore = to == "_";
        if edges.iter().any(|ch| matches!(ch, Some('*' | '_')))
            || (underscore && (in_word(before) || in_word(after)))
            || text_contains(node, content, if underscore { '_' } else { '*' })
        {
            return;
        }
        self.push(vec![
            Edit::new(range.start..range.start + width, to),
            Edit::new(range.end - width..range.end, to),
        ]);
    }

    /// Pad cells to a common column width: `|a|bb|` -> `| a   | bb  |`.
    fn table(&mut self, table: &Table, paragraph_end: Option<usize>) {
        let content = self.content;
        let rows = table
            .children
            .iter()
            .map(|row| Some((span(row)?, self.cell_texts(row, table.align.len())?)))
            .collect::<Option<Vec<_>>>();
        let Some(rows) = rows.filter(|rows| !rows.is_empty()) else {
            return;
        };
        // javadoc reads a table right after a line of text as part of that paragraph.
        if paragraph_end.is_some_and(|end| {
            content[end..rows[0].0.start].bytes().filter(|byte| *byte == b'\n').count() < 2
        }) {
            return;
        }
        let mut widths = vec![3; table.align.len()];
        for (_, texts) in &rows {
            for (width, text) in widths.iter_mut().zip(texts) {
                *width = (*width).max(text_width(&content[text.clone()]));
            }
        }

        let mut group = Vec::new();
        for (row, texts) in &rows {
            let trailing = &content[row.end..];
            let row_end = row.end + trailing.len() - trailing.trim_start_matches([' ', '\t']).len();
            let mut gap_start = row.start;
            let mut separator = String::from("| ");
            for (index, (text, width)) in texts.iter().zip(&widths).enumerate() {
                if !is_cell_gap(&content[gap_start..text.start], index > 0) {
                    return;
                }
                group.push(Edit::new(gap_start..text.start, separator));
                separator = format!("{:1$} | ", "", width - text_width(&content[text.clone()]));
                gap_start = text.end;
            }
            let mut last = separator.trim_end().to_string();
            for width in &widths[texts.len()..] {
                let _ = write!(last, " {:width$} |", "");
            }
            if !is_cell_gap(&content[gap_start..row_end], false) {
                return;
            }
            group.push(Edit::new(gap_start..row_end, last));
        }
        let Some(delimiter) = self.delimiter_row(rows[0].0.end, &table.align, &widths) else {
            return;
        };
        group.push(delimiter);
        self.push(group);
    }

    /// The content range of each cell in `row`, or `None` for a row with more
    /// cells than the table has columns, which javadoc drops.
    fn cell_texts(&self, row: &Node, columns: usize) -> Option<Vec<Range<usize>>> {
        let cells = row.children().filter(|cells| cells.len() <= columns)?;
        cells
            .iter()
            .map(|cell| {
                let children = cell.children()?;
                if let (Some(first), Some(last)) =
                    (children.first().and_then(span), children.last().and_then(span))
                {
                    return Some(first.start..last.end);
                }
                // An empty cell sits after its `|` and one space.
                let cell_range = span(cell)?;
                let cell = &self.content[cell_range.clone()];
                let pipe = usize::from(cell.starts_with('|'));
                let start = cell_range.start + pipe + usize::from(cell[pipe..].starts_with(' '));
                Some(start..start)
            })
            .collect()
    }

    /// Rewrite the delimiter row, the line after the header, which the tree
    /// leaves out.
    fn delimiter_row(
        &self,
        header_end: usize,
        align: &[AlignKind],
        widths: &[usize],
    ) -> Option<Edit> {
        let content = self.content;
        let line_start = header_end + content[header_end..].find('\n')? + 1;
        let line = content[line_start..].split('\n').next().unwrap_or_default();
        let start = line_start + line.len() - quote_content(line).trim_start().len();
        let end = line_start + line.trim_end().len();
        if !content[start..end]
            .bytes()
            .all(|byte| matches!(byte, b'|' | b':' | b'-' | b' ' | b'\t'))
        {
            return None;
        }
        let mut delimiter = String::from("|");
        for (align, width) in align.iter().zip(widths) {
            let (left, right) = match align {
                AlignKind::Left => (":", ""),
                AlignKind::Right => ("", ":"),
                AlignKind::Center => (":", ":"),
                AlignKind::None => ("", ""),
            };
            let dashes = "-".repeat(width - left.len() - right.len());
            let _ = write!(delimiter, " {left}{dashes}{right} |");
        }
        Some(Edit::new(start..end, delimiter))
    }
}

/// Whether text in `node`, outside code and nested delimiters, contains `ch`.
fn text_contains(node: &Node, content: &str, ch: char) -> bool {
    node.children().is_some_and(|children| {
        children.iter().any(|child| match child {
            Node::Text(_) => span(child).is_some_and(|range| content[range].contains(ch)),
            _ => text_contains(child, content, ch),
        })
    })
}

/// Whether `gap` holds only spaces around one `|`, which only a row's edge
/// may leave out.
fn is_cell_gap(gap: &str, between_cells: bool) -> bool {
    let pipes = gap.bytes().filter(|byte| *byte == b'|').count();
    gap.bytes().all(|byte| matches!(byte, b'|' | b' ' | b'\t'))
        && (pipes == 1 || (pipes == 0 && !between_cells))
}

fn text_width(text: &str) -> usize {
    robusta_formatter_core::text_width(text, IndentWidth::default()) as usize
}

#[cfg(test)]
mod tests;
