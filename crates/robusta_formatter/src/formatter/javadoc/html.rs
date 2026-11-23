use robusta_javadoc::{Block, Flow, FlowKind, HtmlBlock, block_tag_at};
use robusta_syntax::identifier::is_horizontal_whitespace_char;

/// The line break before a unit.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Break {
    None,
    Line,
    Blank,
}

/// A flow or preserved line placed on the page.
pub struct Unit {
    pub break_before: Break,
    pub content: UnitContent,
}

pub enum UnitContent {
    /// Atoms filled from column `indent`, with wrapped lines at column `hang`.
    Flow {
        kind: FlowKind,
        atoms: Vec<String>,
        line_starts: Vec<usize>,
        indent: usize,
        hang: usize,
    },
    Preserved(String),
}

/// Lay out `blocks` with blank lines around block HTML and list contents
/// indented under their list. With `paragraph_html_tags`, every paragraph
/// after the first starts with `<p>`, as Google style and google-java-format
/// write it.
pub fn lay_out(blocks: Vec<Block>, paragraph_html_tags: bool) -> Vec<Unit> {
    let mut layout = Layout { paragraph_html_tags, ..Layout::default() };
    for block in blocks {
        match block {
            Block::Blank => layout.blank = true,
            Block::PreservedLine(line) => layout.push_preserved(line),
            Block::Flow(flow) => layout.push_flow(flow),
        }
    }
    if layout.units.is_empty() || !paragraph_html_tags {
        layout.emit_empty_paragraph();
    }
    layout.units
}

#[derive(Default)]
struct Layout {
    paragraph_html_tags: bool,
    units: Vec<Unit>,
    /// Open lists, each holding the width of its current item's tag.
    lists: Vec<ListFrame>,
    in_tag: bool,
    /// A blank source line since the last unit.
    blank: bool,
    /// The break the last flow asks for after it.
    after: Option<Break>,
    /// Whether the last unit is paragraph text that a blank line continues
    /// with a new paragraph.
    paragraph_text: bool,
    /// An empty `<p>` not yet placed.
    empty_paragraph: Option<Vec<String>>,
}

struct ListFrame {
    indent: usize,
    item: usize,
}

impl Layout {
    fn indent(&self) -> usize {
        let tag = if self.in_tag { 4 } else { 0 };
        tag + self.lists.iter().map(|list| list.indent + list.item).sum::<usize>()
    }

    fn push_preserved(&mut self, line: String) {
        self.units.push(Unit { break_before: Break::Line, content: UnitContent::Preserved(line) });
    }

    fn push_flow(&mut self, flow: Flow) {
        let (kind, start, end) = (flow.kind(), flow.html_start(), flow.html_end());
        let starts_verbatim = flow.starts_verbatim();
        let opens_tag = flow.tag_name().is_some();
        let mut line_starts = flow.line_starts().to_vec();
        let mut atoms = flow.into_atoms();
        if !self.paragraph_html_tags && (opens_tag || start.is_some()) {
            self.emit_empty_paragraph();
        }
        if opens_tag {
            self.lists.clear();
            self.in_tag = true;
            let before = if self.blank { Break::Blank } else { Break::Line };
            let content = UnitContent::Flow { kind, atoms, line_starts, indent: 0, hang: 4 };
            self.emit(before, false, content);
            self.after = None;
            self.paragraph_text = true;
            return;
        }

        let mut nested = self.in_tag || !self.lists.is_empty();
        let before = match start {
            Some(HtmlBlock::ListEnd | HtmlBlock::ListItem) => Break::Line,
            Some(_) => Break::Blank,
            None if self.blank => Break::Blank,
            None => Break::Line,
        };
        if start == Some(HtmlBlock::Paragraph) {
            let (tag, rest) = split_tag(&atoms[0]);
            let (tag, rest) = (tag.to_string(), rest.to_string());
            if rest.is_empty() && atoms.len() == 1 {
                // An empty paragraph, such as `<p>` before a list.
                self.empty_paragraph.get_or_insert(atoms);
                return;
            }
            if self.paragraph_html_tags && self.units.is_empty() && !rest.starts_with('@') {
                if rest.is_empty() {
                    atoms.remove(0);
                    shift_line_starts(&mut line_starts);
                } else {
                    atoms[0] = rest;
                }
            } else {
                let tag = if is_simple_tag(&tag) { "<p>" } else { &tag };
                atoms[0] = format!("{tag}{rest}");
                glue_tag(&mut atoms, &mut line_starts);
            }
        } else if matches!(start, Some(HtmlBlock::ListItem | HtmlBlock::Heading)) {
            glue_tag(&mut atoms, &mut line_starts);
        } else if start.is_none() && !self.paragraph_html_tags {
            match self.empty_paragraph.take() {
                Some(empty) => {
                    let tag = if is_simple_tag(&empty[0]) { "<p>" } else { &empty[0] };
                    prepend_tag(&mut atoms[0], tag);
                }
                // Without `<p>`, only the blank line keeps a nested paragraph apart.
                None => nested &= !self.blank,
            }
        } else if start.is_none() && self.blank && self.paragraph_text {
            prepend_tag(&mut atoms[0], "<p>");
        }

        let (mut indent, hang) = match start {
            Some(HtmlBlock::ListEnd) => {
                self.lists.pop();
                (self.indent(), self.indent())
            }
            Some(HtmlBlock::ListItem) => {
                let width = split_tag(&atoms[0]).0.chars().count();
                if self.lists.is_empty() {
                    self.lists.push(ListFrame { indent: 0, item: 0 });
                }
                self.lists.last_mut().expect("an open list").item = 0;
                let indent = self.indent();
                self.lists.last_mut().expect("an open list").item = width;
                (indent, indent + width)
            }
            _ => (self.indent(), self.indent()),
        };
        if starts_verbatim && start.is_none() {
            // A literal opener already carries its source indentation.
            let margin =
                atoms[0].chars().take_while(|&ch| is_horizontal_whitespace_char(ch)).count();
            indent = indent.saturating_sub(margin);
        }
        self.emit(before, nested, UnitContent::Flow { kind, atoms, line_starts, indent, hang });
        if start == Some(HtmlBlock::List) {
            self.lists.push(ListFrame { indent: 2, item: 0 });
        }

        self.after = match end {
            Some(HtmlBlock::List) => Some(Break::Line),
            Some(_) => Some(Break::Blank),
            None if start == Some(HtmlBlock::Preformatted) => Some(Break::Blank),
            None => None,
        };
        self.paragraph_text = end.is_none() && start != Some(HtmlBlock::Preformatted);
    }

    /// Put a held empty `<p>` on its own line.
    fn emit_empty_paragraph(&mut self) {
        let Some(atoms) = self.empty_paragraph.take() else { return };
        let (indent, nested, blank) =
            (self.indent(), self.in_tag || !self.lists.is_empty(), self.blank);
        let content = UnitContent::Flow {
            kind: FlowKind::Prose,
            atoms,
            line_starts: Vec::new(),
            indent,
            hang: indent,
        };
        self.emit(Break::Blank, nested, content);
        // The blank line after the `<p>` still separates the next unit.
        self.blank = blank;
    }

    /// Push a unit. Inside a list or block tag (`nested`), a blank line
    /// becomes a line break.
    fn emit(&mut self, before: Break, nested: bool, content: UnitContent) {
        let break_before = if self.units.is_empty() {
            Break::None
        } else {
            let requested = before.max(self.after.take().unwrap_or(Break::None));
            if nested && requested == Break::Blank { Break::Line } else { requested }
        };
        self.units.push(Unit { break_before, content });
        self.blank = false;
    }
}

/// Split a leading tag, such as `<p>` in `<p>Text`, from the text after it.
fn split_tag(atom: &str) -> (&str, &str) {
    let end = block_tag_at(atom, 0).map_or(atom.len(), |(_, end)| end);
    (&atom[..end], atom[end..].trim_start())
}

/// Whether `tag` has no attributes, such as `<P>` or `<p/>`.
fn is_simple_tag(tag: &str) -> bool {
    let name = tag[1..].trim_end_matches('>').trim_end().trim_end_matches('/').trim_end();
    name.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

/// Join a tag atom such as `<li>` to the word after it.
fn glue_tag(atoms: &mut Vec<String>, line_starts: &mut Vec<usize>) {
    if atoms.len() > 1 && atoms[0].ends_with('>') && split_tag(&atoms[0]).1.is_empty() {
        let tag = atoms.remove(0);
        prepend_tag(&mut atoms[0], &tag);
        shift_line_starts(line_starts);
    }
}

fn prepend_tag(atom: &mut String, tag: &str) {
    let leading = atom.len() - atom.trim_start_matches(is_horizontal_whitespace_char).len();
    atom.replace_range(..leading, tag);
}

fn shift_line_starts(line_starts: &mut Vec<usize>) {
    line_starts.retain(|&start| start != 1);
    for start in line_starts {
        *start -= 1;
    }
}
