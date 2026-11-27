use std::{borrow::Cow, ops::Range};

use markdown::mdast::Node;
use robusta_syntax::identifier::{identifier_name, is_horizontal_whitespace_char};

use crate::html::{HtmlBlock, block_tag_at, find_block_tag, preformatted_end, starts_preformatted};
use crate::markdown::code_text;
use crate::parser::javadoc_tag::{Tag, parse_tags, type_parameter_tag_len};
use crate::{Javadoc, JavadocKind};

/// Whether a flow contains leading prose or a block tag and its continuation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlowKind {
    Prose,
    BlockTag,
}

/// Atoms separated by breakable spaces; spaces inside an atom stay intact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flow {
    kind: FlowKind,
    document_kind: JavadocKind,
    atoms: Vec<String>,
    /// Indices of the atoms that began a source line, after the first.
    line_starts: Vec<usize>,
    opens_tag: bool,
    html_start: Option<HtmlBlock>,
    html_end: Option<HtmlBlock>,
    starts_verbatim: bool,
    ends_verbatim: bool,
    pending_space: bool,
    pending_line: bool,
}

impl Flow {
    fn new(kind: FlowKind, document_kind: JavadocKind) -> Self {
        Self {
            kind,
            document_kind,
            atoms: Vec::new(),
            line_starts: Vec::new(),
            opens_tag: false,
            html_start: None,
            html_end: None,
            starts_verbatim: false,
            ends_verbatim: false,
            pending_space: false,
            pending_line: false,
        }
    }

    pub fn kind(&self) -> FlowKind {
        self.kind
    }

    pub fn atoms(&self) -> &[String] {
        &self.atoms
    }

    pub fn atoms_mut(&mut self) -> &mut [String] {
        &mut self.atoms
    }

    pub fn into_atoms(self) -> Vec<String> {
        self.atoms
    }

    /// Indices of the atoms that began a source line, after the first.
    pub fn line_starts(&self) -> &[usize] {
        &self.line_starts
    }

    /// The name of the block tag this flow opens, such as `param`.
    pub fn tag_name(&self) -> Option<&str> {
        let atom = self.atoms.first().filter(|_| self.opens_tag)?;
        atom[1..].split(|ch: char| ch.is_whitespace() || ch == '{').next()
    }

    /// The block-level HTML tag that starts this flow.
    pub fn html_start(&self) -> Option<HtmlBlock> {
        self.html_start
    }

    /// The block-level HTML tag that ends this flow.
    pub fn html_end(&self) -> Option<HtmlBlock> {
        self.html_end
    }

    /// Whether the first atom begins with code or literal text.
    pub fn starts_verbatim(&self) -> bool {
        self.starts_verbatim
    }

    /// Whether the last atom ends with code or literal text.
    pub fn ends_verbatim(&self) -> bool {
        self.ends_verbatim
    }

    fn append_normal(&mut self, text: &str) {
        let mut start = 0usize;
        for (index, ch) in text.char_indices() {
            if is_horizontal_whitespace_char(ch) {
                self.append_fragment(&text[start..index], false);
                self.pending_space = true;
                start = index + ch.len_utf8();
            }
        }
        self.append_fragment(&text[start..], false);
    }

    fn append_verbatim(&mut self, text: &str) {
        self.append_fragment(text, true);
    }

    fn append_fragment(&mut self, text: &str, verbatim: bool) {
        if text.is_empty() {
            return;
        }
        let after_start_tag = self.atoms.len() == 1 && self.html_start.is_some();
        if self.atoms.is_empty() {
            self.starts_verbatim = verbatim;
            self.atoms.push(text.to_string());
        } else if (self.pending_space || after_start_tag)
            && !is_line_start_hazard(text, self.document_kind)
            && !ends_with_inline_tag_name(self.atoms.last().expect("non-empty atom list"))
        {
            if self.pending_line {
                self.line_starts.push(self.atoms.len());
            }
            self.atoms.push(text.to_string());
        } else {
            let previous = self.atoms.last_mut().expect("non-empty atom list");
            if self.pending_space {
                previous.push(' ');
            }
            previous.push_str(text);
        }
        self.ends_verbatim = verbatim;
        self.html_end = None;
        self.pending_space = false;
        self.pending_line = false;
    }
}

/// Keep an inline tag's name, such as `{@link`, with the reference after it.
fn ends_with_inline_tag_name(atom: &str) -> bool {
    atom.rfind("{@").is_some_and(|start| {
        let name = &atom[start + 2..];
        !name.is_empty() && name.bytes().all(|byte| byte.is_ascii_alphabetic())
    })
}

/// Keep prose from becoming a tag or Markdown block after wrapping. Prefix
/// checks deliberately include ambiguous markers rather than risk changing meaning.
fn is_line_start_hazard(text: &str, kind: JavadocKind) -> bool {
    if text.starts_with('@') {
        return true;
    }
    if kind != JavadocKind::Markdown {
        return false;
    }
    text.starts_with(['#', '-', '+', '*', '>', '`', '~', '_', '=', '<'])
        || text.strip_suffix(['.', ')']).is_some_and(|number| {
            !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
        })
}

/// A blank line or a flow of structured Javadoc content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Blank,
    Flow(Flow),
    /// One physical line whose spacing and line boundary must survive formatting.
    PreservedLine(String),
}

/// Parsed Javadoc content shared by formatting and linting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Structure {
    blocks: Vec<Block>,
    tags: Vec<Tag>,
    literals: Vec<Range<usize>>,
    has_ignorable_names: bool,
}

impl Structure {
    pub(super) fn parse(document: Javadoc<'_>) -> Self {
        let (content, has_interior_line) = normalized_content(document);
        if document.kind() == JavadocKind::Markdown {
            return Self::from_markdown(&content);
        }
        Self::traditional(&content, has_interior_line)
    }

    /// Parse traditional comment content after its margins have been removed.
    pub fn from_traditional(content: &str) -> Self {
        Self::traditional(content, !content.is_empty())
    }

    fn traditional(content: &str, has_interior_line: bool) -> Self {
        let mut content = Cow::Borrowed(content);
        let ranges = verbatim_ranges(&mut content, JavadocKind::Traditional, &[]);
        let tags = parse_tags(&content, &ranges);
        let multiline_ranges = ranges
            .iter()
            .filter(|range| content[(*range).clone()].contains('\n'))
            .cloned()
            .collect::<Vec<_>>();
        let mut builder = Builder::new(JavadocKind::Traditional);
        let mut line_start = 0usize;

        if has_interior_line {
            for line in content.split('\n') {
                let line_end = line_start + line.len();
                let blank = line.trim_matches(is_horizontal_whitespace_char).is_empty()
                    && !ranges.iter().any(|range| intersects(range, line_start..line_end));
                if blank {
                    builder.blank();
                } else {
                    let mut segments_start = line_start;
                    if let Some(start) = block_tag_start(&content, line_start, line_end, &ranges) {
                        builder.begin_tag();
                        if let Some(len) = type_parameter_tag_len(&content[start..line_end]) {
                            builder.append_prose(&content[start..start + len]);
                            segments_start = start + len;
                        }
                    }
                    append_line_segments(&mut builder, &content, segments_start, line_end, &ranges);

                    let verbatim_continues = multiline_ranges
                        .iter()
                        .any(|range| range.start <= line_end && range.end > line_end);
                    if verbatim_continues {
                        builder.flush();
                    } else {
                        builder.physical_space();
                    }
                }

                line_start = line_end.saturating_add(1);
            }
        }
        builder.flush();

        Self {
            blocks: builder.blocks,
            tags,
            literals: ranges,
            has_ignorable_names: matches!(identifier_name(&content), Cow::Owned(_)),
        }
    }

    /// Parse the content of a Markdown comment, its lines joined after the
    /// `///` margins are removed.
    pub fn from_markdown(content: &str) -> Self {
        let (markdown, ranges) = parse_markdown_ranges(content);
        let tags = parse_tags(content, &ranges);
        Self {
            blocks: markdown_blocks(content, markdown.as_ref()),
            tags,
            literals: ranges,
            has_ignorable_names: matches!(identifier_name(content), Cow::Owned(_)),
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn into_blocks(self) -> Vec<Block> {
        self.blocks
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    /// Code and literal regions of the comment content, which stay whole.
    pub fn literal_ranges(&self) -> &[Range<usize>] {
        &self.literals
    }

    /// Whether the document mentions `name` as a tag subject or a whole name in
    /// structured content. The content fallback deliberately remains
    /// conservative: keeping an import is safer than deleting one referenced
    /// only in prose.
    pub fn mentions(&self, name: &str) -> bool {
        let name = identifier_name(name);
        let contains = |text: &str| {
            let text =
                if self.has_ignorable_names { identifier_name(text) } else { Cow::Borrowed(text) };
            contains_name(&text, &name)
        };
        self.tags.iter().filter_map(Tag::referenced_name).any(contains)
            || self.blocks.iter().any(|block| match block {
                Block::Blank => false,
                Block::Flow(flow) => flow.atoms.iter().any(|atom| contains(atom)),
                Block::PreservedLine(line) => contains(line),
            })
    }
}

pub fn literal_regions(document: Javadoc<'_>) -> Vec<String> {
    let (content, _) = normalized_content(document);
    let mut content = Cow::Owned(content);
    let ranges = match document.kind() {
        JavadocKind::Traditional => verbatim_ranges(&mut content, document.kind(), &[]),
        JavadocKind::Markdown => parse_markdown_ranges(&content).1,
    };
    ranges.into_iter().map(|range| content[range].to_string()).collect()
}

/// Parse Markdown `content` with its code and literal regions.
fn parse_markdown_ranges(content: &str) -> (Option<MarkdownParse>, Vec<Range<usize>>) {
    let markdown = parse_markdown(content);
    let code = markdown.as_ref().map_or(&[][..], |markdown| &markdown.literal_ranges);
    let mut ranges = verbatim_ranges(&mut Cow::Borrowed(content), JavadocKind::Markdown, code);
    // The scan steps over `@param <T>` even inside code.
    ranges.extend_from_slice(code);
    merge_ranges(&mut ranges);
    (markdown, ranges)
}

fn normalized_content(document: Javadoc<'_>) -> (String, bool) {
    let mut lines = document.lines();
    if document.kind() == JavadocKind::Traditional {
        if lines.first().is_some_and(|line| line.is_empty()) {
            lines.remove(0);
        }
        if lines.last().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }
    }
    let has_interior_line = !lines.is_empty();
    (lines.join("\n"), has_interior_line)
}

/// Build Markdown blocks from the reflowable and preserved source ranges.
fn markdown_blocks(content: &str, markdown: Option<&MarkdownParse>) -> Vec<Block> {
    let Some(markdown) = markdown else {
        return content.split('\n').map(|line| Block::PreservedLine(line.to_string())).collect();
    };

    let mut builder = Builder::new(JavadocKind::Markdown);
    // The blank lines after the last text are outside every block but code.
    let text_end =
        content.trim_end_matches(|ch| ch == '\n' || is_horizontal_whitespace_char(ch)).len();
    // javadoc reads the description and each block tag as separate Markdown, in which
    // whitespace after the last line is text only in code.
    let mut section_end = None;
    let mut line_start = 0usize;
    for line in content.split('\n') {
        let line_end = line_start + line.len();
        if range_contains_line(&markdown.ranges.reflowable, line_start, line_end) {
            if block_tag_start(content, line_start, line_end, &[]).is_some() {
                trim_trailing_whitespace(&mut builder.blocks, section_end);
                builder.begin_tag();
            }
            section_end = None;
            builder.append_normal(line);
            builder.physical_space();
        } else if line.trim_matches(is_horizontal_whitespace_char).is_empty()
            && !range_contains_line(&markdown.ranges.preserved, line_start, line_end)
            || line_start > text_end
                && !range_contains_line(&markdown.literal_ranges, line_start, line_end)
        {
            builder.blank();
        } else {
            builder.flush();
            section_end = (!markdown.holds_code(line_end)).then_some(builder.blocks.len());
            builder.blocks.push(Block::PreservedLine(line.to_string()));
        }
        line_start = line_end.saturating_add(1);
    }
    builder.flush();
    trim_trailing_whitespace(&mut builder.blocks, section_end);
    builder.blocks
}

/// Drop the trailing spaces and tabs of the preserved line at `index`.
fn trim_trailing_whitespace(blocks: &mut [Block], index: Option<usize>) {
    if let Some(Block::PreservedLine(line)) = index.map(|index| &mut blocks[index]) {
        line.truncate(line.trim_end_matches([' ', '\t']).len());
    }
}

/// Whether the last line of Markdown `content` is code, so that its trailing
/// whitespace is code text.
pub fn ends_in_code(content: &str) -> bool {
    parse_markdown(content).is_some_and(|markdown| markdown.holds_code(content.len()))
}

#[derive(Default)]
struct MarkdownRanges {
    reflowable: Vec<Range<usize>>,
    preserved: Vec<Range<usize>>,
}

struct MarkdownParse {
    ranges: MarkdownRanges,
    literal_ranges: Vec<Range<usize>>,
    /// The lines holding each code block's text.
    code: Vec<Range<usize>>,
}

impl MarkdownParse {
    /// Whether the line ending at `line_end` holds code text.
    fn holds_code(&self, line_end: usize) -> bool {
        self.code.iter().any(|range| range.start <= line_end && line_end <= range.end)
    }
}

fn parse_markdown(content: &str) -> Option<MarkdownParse> {
    let Node::Root(root) = crate::markdown::parse(content)? else {
        return None;
    };
    let mut ranges = MarkdownRanges::default();
    let (mut literal_ranges, mut code) = (Vec::new(), Vec::new());
    for node in &root.children {
        let position = node.position()?;
        let range = position.start.offset..position.end.offset;
        if let Some(reflow_start) = markdown_reflow_start(node, &content[range.clone()]) {
            let reflow_start = range.start + reflow_start;
            if reflow_start > range.start {
                ranges.preserved.push(range.start..reflow_start);
            }
            if reflow_start < range.end {
                ranges.reflowable.push(reflow_start..range.end);
            }
        } else {
            ranges.preserved.push(range);
        }
        collect_markdown_literal_ranges(node, &mut literal_ranges, &mut code)?;
    }
    let code = code.iter().filter_map(|range| code_text(content, range)).collect();
    Some(MarkdownParse { ranges, literal_ranges, code })
}

/// Where the text of the top-level Markdown `node`, spanning `source`, starts to reflow,
/// dropping the indentation of its lines, or `None` when every line keeps its spacing.
pub fn markdown_reflow_start(node: &Node, source: &str) -> Option<usize> {
    let Node::Paragraph(paragraph) = node else {
        return None;
    };
    (paragraph.children.iter().all(|child| matches!(child, Node::Text(_)))
        && !source.contains("{@"))
    .then(|| indented_marker_prefix_len(source))
}

fn indented_marker_prefix_len(source: &str) -> usize {
    if !source.starts_with(is_horizontal_whitespace_char)
        || source
            .split_ascii_whitespace()
            .next()
            .is_none_or(|word| !is_line_start_hazard(word, JavadocKind::Markdown))
    {
        return 0;
    }
    // Preserve the literal marker and its paragraph up to the first Javadoc tag.
    source
        .split_inclusive('\n')
        .take_while(|line| block_tag_start(line, 0, line.len(), &[]).is_none())
        .map(str::len)
        .sum()
}

fn collect_markdown_literal_ranges(
    node: &Node,
    ranges: &mut Vec<Range<usize>>,
    blocks: &mut Vec<Range<usize>>,
) -> Option<()> {
    if matches!(node, Node::Code(_) | Node::InlineCode(_)) {
        let position = node.position()?;
        let range = position.start.offset..position.end.offset;
        if matches!(node, Node::Code(_)) {
            blocks.push(range.clone());
        }
        ranges.push(range);
        return Some(());
    }
    if let Some(children) = node.children() {
        for child in children {
            collect_markdown_literal_ranges(child, ranges, blocks)?;
        }
    }
    Some(())
}

fn range_contains_line(ranges: &[Range<usize>], line_start: usize, line_end: usize) -> bool {
    ranges.iter().any(|range| {
        if line_start == line_end {
            range.start <= line_start && line_start < range.end
        } else {
            range.start <= line_start && line_end <= range.end
        }
    })
}

fn merge_ranges(ranges: &mut Vec<Range<usize>>) {
    ranges.sort_unstable_by_key(|range| (range.start, range.end));
    let mut write = 0usize;
    for read in 0..ranges.len() {
        if write > 0 && ranges[read].start < ranges[write - 1].end {
            ranges[write - 1].end = ranges[write - 1].end.max(ranges[read].end);
        } else {
            ranges[write] = ranges[read].clone();
            write += 1;
        }
    }
    ranges.truncate(write);
}

struct Builder {
    blocks: Vec<Block>,
    current: Option<Flow>,
    active_kind: FlowKind,
    document_kind: JavadocKind,
}

impl Builder {
    fn new(document_kind: JavadocKind) -> Self {
        Self { blocks: Vec::new(), current: None, active_kind: FlowKind::Prose, document_kind }
    }

    fn begin_tag(&mut self) {
        self.flush();
        self.active_kind = FlowKind::BlockTag;
        let mut flow = Flow::new(FlowKind::BlockTag, self.document_kind);
        flow.opens_tag = true;
        self.current = Some(flow);
    }

    fn ensure_flow(&mut self) -> &mut Flow {
        self.current.get_or_insert_with(|| Flow::new(self.active_kind, self.document_kind))
    }

    fn append_normal(&mut self, text: &str) {
        let mut rest = text;
        while self.document_kind == JavadocKind::Traditional
            && let Some((block, start, end)) = find_block_tag(rest)
        {
            self.append_prose(&rest[..start]);
            if block.starts_flow() {
                self.begin_html(block);
            }
            self.ensure_flow().append_fragment(&rest[start..end], false);
            if block.ends_flow() {
                self.end_html(block);
            }
            rest = &rest[end..];
        }
        self.append_prose(rest);
    }

    fn begin_html(&mut self, block: HtmlBlock) {
        self.flush();
        let mut flow = Flow::new(self.active_kind, self.document_kind);
        flow.html_start = Some(block);
        self.current = Some(flow);
    }

    fn end_html(&mut self, block: HtmlBlock) {
        if let Some(flow) = &mut self.current {
            flow.html_end = Some(block);
        }
        self.flush();
    }

    fn append_prose(&mut self, text: &str) {
        // A split at block HTML or a literal must not expose a new tag.
        // Source block tags already have their own flow from begin_tag().
        if self.current.is_none()
            && text.trim_start_matches(is_horizontal_whitespace_char).starts_with('@')
            && matches!(self.blocks.last(), Some(Block::Flow(_)))
            && let Some(Block::Flow(mut previous)) = self.blocks.pop()
        {
            previous.pending_space = true;
            self.current = Some(previous);
        }
        self.ensure_flow().append_normal(text);
    }

    fn append_verbatim(&mut self, text: &str) {
        self.ensure_flow().append_verbatim(text);
    }

    fn physical_space(&mut self) {
        if let Some(flow) = &mut self.current {
            flow.pending_space = true;
            flow.pending_line = true;
        }
    }

    fn blank(&mut self) {
        // `<p>` belongs to the paragraph after it.
        if self.bare_start_tag() != Some(HtmlBlock::Paragraph) {
            self.flush();
            self.blocks.push(Block::Blank);
        }
    }

    /// The block HTML tag that is the only content of the current flow.
    fn bare_start_tag(&self) -> Option<HtmlBlock> {
        self.current.as_ref().filter(|flow| flow.atoms.len() == 1)?.html_start
    }

    fn flush(&mut self) {
        let Some(flow) = self.current.take() else {
            return;
        };
        if !flow.atoms.is_empty() {
            self.blocks.push(Block::Flow(flow));
        }
    }
}

fn append_line_segments(
    builder: &mut Builder,
    content: &str,
    line_start: usize,
    line_end: usize,
    ranges: &[Range<usize>],
) {
    let mut cursor = line_start;
    for range in ranges.iter().filter(|range| intersects(range, line_start..line_end)) {
        let start = range.start.max(line_start);
        let end = range.end.min(line_end);
        if range.start < line_start {
            // Keep a tag-like word after the literal on the same line.
            let rest = content[end..line_end].trim_start_matches(is_horizontal_whitespace_char);
            let keep = if rest.starts_with('@') { line_end } else { end };
            builder.flush();
            builder.blocks.push(Block::PreservedLine(content[line_start..keep].to_string()));
            cursor = keep;
            if keep == line_end {
                break;
            }
            continue;
        }
        if cursor < start {
            builder.append_normal(&content[cursor..start]);
        }
        let multiline = content[range.clone()].contains('\n');
        let preformatted = starts_preformatted(&content[range.clone()]);
        if preformatted {
            builder.begin_html(HtmlBlock::Preformatted);
        } else if multiline && builder.bare_start_tag().is_none() {
            builder.flush();
        }
        builder.append_verbatim(&content[start..end]);
        if preformatted && !multiline {
            builder.end_html(HtmlBlock::Preformatted);
        }
        cursor = end;
    }
    if cursor < line_end {
        builder.append_normal(&content[cursor..line_end]);
    }
}

pub fn block_tag_start(
    content: &str,
    line_start: usize,
    line_end: usize,
    ranges: &[Range<usize>],
) -> Option<usize> {
    let line = &content[line_start..line_end];
    let leading = line.len() - line.trim_start_matches(is_horizontal_whitespace_char).len();
    let start = line_start + leading;
    (start < line_end
        && content.as_bytes()[start] == b'@'
        && !ranges.iter().any(|range| range.contains(&start)))
    .then_some(start)
}

fn intersects(left: &Range<usize>, right: Range<usize>) -> bool {
    // An unclosed literal includes its final empty line at EOF.
    left.start < right.end
        && (right.start < left.end || (right.is_empty() && right.start == left.end))
}

/// Locate code and literal regions whose bytes must remain indivisible.
/// Markdown takes its sorted `code` ranges from the parsed tree.
fn verbatim_ranges(
    content: &mut Cow<'_, str>,
    kind: JavadocKind,
    code: &[Range<usize>],
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut position = 0usize;
    while position < content.len() {
        if line_start(content, position)
            && let Some(len) = type_parameter_tag_len(&content[position..])
        {
            if kind == JavadocKind::Traditional {
                join_lines(content, position..position + len);
            }
            position += len;
            continue;
        }
        let code = match kind {
            JavadocKind::Traditional => {
                // Block HTML can expose a fence at the start of an output flow.
                let fence_start = if content.as_bytes()[position] == b'<' {
                    block_tag_at(content, position).map(|(_, end)| {
                        join_lines(content, position..end);
                        let rest = &content[end..];
                        content.len() - rest.trim_start_matches(is_horizontal_whitespace_char).len()
                    })
                } else {
                    line_start(content, position).then_some(position)
                };
                fence_start.and_then(|start| fenced_block_range(content, start))
            }
            JavadocKind::Markdown => {
                let index = code.partition_point(|range| range.end <= position);
                code.get(index).filter(|range| range.start <= position).cloned()
            }
        };
        if let Some(range) = code {
            position = range.end;
            ranges.push(range);
            continue;
        }
        if let Some(end) = preformatted_end(content, position) {
            ranges.push(position..end);
            position = end;
            continue;
        }
        if inline_literal_starts(content, position) {
            let end = balanced_inline_end(content, position).unwrap_or(content.len());
            ranges.push(position..end);
            position = end;
            continue;
        }
        position += content[position..].chars().next().expect("position is in bounds").len_utf8();
    }
    ranges
}

fn line_start(content: &str, position: usize) -> bool {
    position == 0 || content.as_bytes().get(position.wrapping_sub(1)) == Some(&b'\n')
}

/// Join the lines of `range` before line-based flow construction, keeping
/// every offset.
fn join_lines(content: &mut Cow<'_, str>, range: Range<usize>) {
    let mut cursor = range.start;
    while let Some(offset) = content[cursor..range.end].find('\n') {
        let newline = cursor + offset;
        content.to_mut().replace_range(newline..=newline, " ");
        cursor = newline + 1;
    }
}

fn fenced_block_range(content: &str, line_start: usize) -> Option<Range<usize>> {
    let opening_end =
        content[line_start..].find('\n').map_or(content.len(), |offset| line_start + offset);
    let opening =
        content[line_start..opening_end].trim_start_matches(is_horizontal_whitespace_char);
    let marker = *opening.as_bytes().first()?;
    if !matches!(marker, b'`' | b'~') {
        return None;
    }
    let marker_len = opening.bytes().take_while(|byte| *byte == marker).count();
    if marker_len < 3 {
        return None;
    }

    let mut next_start = opening_end.checked_add(1)?;
    while next_start <= content.len() {
        let next_end =
            content[next_start..].find('\n').map_or(content.len(), |offset| next_start + offset);
        let candidate = content[next_start..next_end].trim_matches(is_horizontal_whitespace_char);
        let run = candidate.bytes().take_while(|byte| *byte == marker).count();
        if run >= marker_len && candidate[run..].is_empty() {
            return Some(line_start..next_end);
        }
        if next_end == content.len() {
            break;
        }
        next_start = next_end + 1;
    }
    Some(line_start..content.len())
}

fn inline_literal_starts(content: &str, position: usize) -> bool {
    ["{@code", "{@literal", "{@snippet"].into_iter().any(|marker| {
        ascii_starts_with_case_insensitive(&content[position..], marker)
            && content
                .as_bytes()
                .get(position + marker.len())
                .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'}')
    })
}

pub fn balanced_inline_end(content: &str, position: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, byte) in content.as_bytes()[position..].iter().copied().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(position + offset + 1);
                }
            }
            _ => {}
        }
    }
    None
}

fn ascii_starts_with_case_insensitive(text: &str, prefix: &str) -> bool {
    text.as_bytes()
        .get(..prefix.len())
        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(prefix.as_bytes()))
}

pub fn contains_name(haystack: &str, name: &str) -> bool {
    !name.is_empty()
        && haystack.match_indices(name).any(|(start, matched)| {
            let before = haystack[..start].chars().next_back();
            let after = haystack[start + matched.len()..].chars().next();
            before.is_none_or(|character| !identifier_character(character))
                && after.is_none_or(|character| !identifier_character(character))
        })
}

fn identifier_character(character: char) -> bool {
    character.is_alphanumeric()
        || character == '_'
        || character == '$'
        || (!character.is_ascii()
            && !character.is_whitespace()
            && !character.is_ascii_punctuation())
}

#[cfg(test)]
mod tests {
    use robusta_ast::CommentKind;

    use crate::parser::tests::comment;

    use super::*;

    fn structure(source: &str) -> Structure {
        Javadoc::from_comment(source, &comment(source, CommentKind::MultiLineBlock))
            .expect("test source is Javadoc")
            .structure()
    }

    fn markdown_structure(source: &str) -> Structure {
        Javadoc::from_comment(source, &comment(source, CommentKind::Line))
            .expect("test source is Markdown Javadoc")
            .structure()
    }

    #[test]
    fn prose_keeps_non_breaking_spaces_inside_atoms() {
        for kind in [JavadocKind::Traditional, JavadocKind::Markdown] {
            let mut flow = Flow::new(FlowKind::Prose, kind);
            flow.append_normal("a\u{a0}b c");
            assert_eq!(flow.atoms(), ["a\u{a0}b", "c"]);
        }
    }

    #[test]
    fn prose_breaks_only_on_java_horizontal_whitespace() {
        for space in [
            '\u{b}', '\u{85}', '\u{a0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}',
            '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}',
            '\u{200a}', '\u{2028}', '\u{2029}', '\u{202f}', '\u{205f}', '\u{3000}',
        ] {
            let atom = format!("{space}a{space}b{space}");
            let mut flow = Flow::new(FlowKind::Prose, JavadocKind::Traditional);
            flow.append_normal(&format!(" \t\u{c}{atom}\t\u{c} c "));
            assert_eq!(flow.atoms(), [atom, "c".to_string()]);
        }
    }

    #[test]
    fn paragraph_boundaries_preserve_unicode_spaces() {
        let structure = structure("/**\n * Before.\u{3000}<p>\u{a0}@support\u{202f}\n */");
        let atoms = structure
            .blocks()
            .iter()
            .map(|block| match block {
                Block::Flow(flow) => flow.atoms(),
                _ => panic!("expected prose flows"),
            })
            .collect::<Vec<_>>();

        assert_eq!(atoms, [&["Before.\u{3000}"][..], &["<p>", "\u{a0}@support\u{202f}"]]);
    }

    #[test]
    fn splits_prose_from_each_block_tag() {
        let structure = structure(
            "/**\n * Summary text.\n *\n * @param value first value\n * continued\n * @return result\n */",
        );

        assert_eq!(
            structure
                .blocks
                .iter()
                .map(|block| match block {
                    Block::Blank => None,
                    Block::Flow(flow) => Some(flow.kind),
                    Block::PreservedLine(_) =>
                        unreachable!("traditional Javadoc has no preserved blocks"),
                })
                .collect::<Vec<_>>(),
            [Some(FlowKind::Prose), None, Some(FlowKind::BlockTag), Some(FlowKind::BlockTag)]
        );
    }

    #[test]
    fn flows_record_source_lines_tags_and_verbatim_edges() {
        let structure = structure(
            "/**\n * {@code first} line\n * second {@link Map#get(Object) the\n * getter}\n * @param value text\n * ends {@code x}\n */",
        );
        let [Block::Flow(prose), Block::Flow(tag)] = structure.blocks() else {
            panic!("expected a prose flow and a tag flow: {:?}", structure.blocks());
        };

        assert_eq!(
            prose.atoms(),
            ["{@code first}", "line", "second", "{@link Map#get(Object)", "the", "getter}"]
        );
        assert_eq!(prose.line_starts(), [2, 5]);
        assert_eq!(prose.tag_name(), None);
        assert!(prose.starts_verbatim());
        assert!(!prose.ends_verbatim());
        assert_eq!(tag.tag_name(), Some("param"));
        assert_eq!(tag.line_starts(), [3]);
        assert!(tag.ends_verbatim());
    }

    #[test]
    fn detects_nested_inline_literal_and_case_insensitive_pre() {
        let content = "before {@code Map<{A}, B>} after\n<PRE>  x { y }  </Pre> end";
        let ranges = verbatim_ranges(&mut Cow::Borrowed(content), JavadocKind::Traditional, &[]);

        assert_eq!(
            ranges.iter().map(|range| &content[range.clone()]).collect::<Vec<_>>(),
            ["{@code Map<{A}, B>}", "<PRE>  x { y }  </Pre>"]
        );
    }

    #[test]
    fn keeps_a_fenced_block_together() {
        let content = "before\n```java\n  call(  x );\n```\nafter";
        let ranges = verbatim_ranges(&mut Cow::Borrowed(content), JavadocKind::Traditional, &[]);

        assert_eq!(
            ranges.iter().map(|range| &content[range.clone()]).collect::<Vec<_>>(),
            ["```java\n  call(  x );\n```"]
        );
    }

    #[test]
    fn html_prefixed_fences_keep_block_tags_inside_the_literal() {
        for marker in ["```", "~~~"] {
            for tag in ["<p>", "<p class=\"note\">", "<li>", "<h2>", "<blockquote>"] {
                for prefix in ["Before.\n", "Before. "] {
                    let literal = format!("{marker}other\n  @return literal\n{marker}");
                    let content = format!("{prefix}{tag}{literal}\n@param value data");
                    let ranges = verbatim_ranges(
                        &mut Cow::Borrowed(&content),
                        JavadocKind::Traditional,
                        &[],
                    );
                    assert_eq!(
                        ranges.iter().map(|range| &content[range.clone()]).collect::<Vec<_>>(),
                        [&literal]
                    );
                    assert_eq!(
                        parse_tags(&content, &ranges).iter().map(Tag::name).collect::<Vec<_>>(),
                        ["param"]
                    );
                    let (_, markdown_ranges) = parse_markdown_ranges(&content);
                    assert!(markdown_ranges.iter().all(|range| range.start > ranges[0].start));
                }
            }
        }
    }

    #[test]
    fn html_prefixed_fences_require_complete_attributes() {
        for marker in ["```", "~~~"] {
            for tag in ["<p", "<p class=\"note\"", "<blockquote"] {
                let content = format!("Before. {tag}\nprose>{marker}other\n@return a tag");
                let ranges =
                    verbatim_ranges(&mut Cow::Borrowed(&content), JavadocKind::Traditional, &[]);
                assert!(ranges.is_empty(), "{content}");
                assert_eq!(
                    parse_tags(&content, &ranges).iter().map(Tag::name).collect::<Vec<_>>(),
                    ["return"]
                );
            }
        }
    }

    #[test]
    fn multiline_html_separators_leave_literal_bytes_unchanged() {
        let literal = "```other\n  @return literal\n<p class=\"literal\"\n>\n```";
        let original = format!("Before. <p class=\"note\"\n>{literal}\n@param value data");
        let mut content = Cow::Borrowed(original.as_str());
        let ranges = verbatim_ranges(&mut content, JavadocKind::Traditional, &[]);
        assert_eq!(content.len(), original.len());
        assert!(content.starts_with("Before. <p class=\"note\" >"));
        assert_eq!(&content[ranges[0].clone()], literal);
        assert_eq!(
            parse_tags(&content, &ranges).iter().map(Tag::name).collect::<Vec<_>>(),
            ["param"]
        );

        for literal in [
            "{@code <p class=\"literal\"\n>}",
            "<pre><p class=\"literal\"\n></pre>",
            "<table><p class=\"literal\"\n></table>",
        ] {
            let mut content = Cow::Borrowed(literal);
            let ranges = verbatim_ranges(&mut content, JavadocKind::Traditional, &[]);
            assert!(matches!(content, Cow::Borrowed(_)));
            assert_eq!(&content[ranges[0].clone()], literal);
        }
        let mut content = Cow::Borrowed(original.as_str());
        verbatim_ranges(&mut content, JavadocKind::Markdown, &[]);
        assert!(matches!(content, Cow::Borrowed(_)));
        assert_eq!(content, original);
    }

    #[test]
    fn preserves_a_malformed_literal_to_the_comment_end() {
        let content = "before {@literal unclosed\n  still literal";
        let ranges = verbatim_ranges(&mut Cow::Borrowed(content), JavadocKind::Traditional, &[]);

        assert_eq!(&content[ranges[0].clone()], "{@literal unclosed\n  still literal");
    }

    #[test]
    fn name_boundaries_conservatively_include_unicode_identifier_parts() {
        assert!(!contains_name("A\u{301}", "A"));
        assert!(!contains_name("$List", "List"));
        assert!(contains_name("See List.", "List"));
    }

    #[test]
    fn markdown_reflows_only_plain_paragraphs() {
        let plain =
            markdown_structure("/// Élégant plain words on one line.\n/// More plain words.");
        assert!(matches!(plain.blocks.as_slice(), [Block::Flow(_)]));

        for source in [
            "/// # Heading",
            "/// - a list item\n/// lazy continuation without indentation",
            "/// | first | second |\n/// | --- | --- |\n/// | a | b |",
            "/// A [link](https://example.com) in prose.",
            "/// An `inline code span` in prose.",
            "/// a hard break\\\n/// continues here",
            "/// > a block quote\n/// lazy continuation",
        ] {
            let structure = markdown_structure(source);
            assert!(
                structure.blocks.iter().all(|block| matches!(block, Block::PreservedLine(_))),
                "Markdown structure was made reflowable: {source:?}"
            );
        }
    }

    #[test]
    fn tags_and_literals_record_where_they_are() {
        let content = "See {@link List} and {@code x}.\n  @param value `{@link Map}`";
        let structure = Structure::from_markdown(content);
        let text = |range: Range<usize>| &content[range];

        assert_eq!(
            structure.tags().iter().map(|tag| text(tag.range())).collect::<Vec<_>>(),
            ["{@link List}", "@param"]
        );
        assert_eq!(
            structure.literal_ranges().iter().cloned().map(text).collect::<Vec<_>>(),
            ["{@code x}", "`{@link Map}`"]
        );
    }

    #[test]
    fn container_code_holds_no_tags() {
        let content = "- Outer\n  -     {@link A} {@code B}";
        let structure = Structure::from_markdown(content);
        let text = |range: Range<usize>| &content[range];

        assert_eq!(structure.tags(), []);
        assert_eq!(
            structure.literal_ranges().iter().cloned().map(text).collect::<Vec<_>>(),
            ["    {@link A} {@code B}"]
        );
    }

    #[test]
    fn unclosed_code_ends_with_its_list_item() {
        let content = "- ```\n\n1. {@link List}";
        let structure = Structure::from_markdown(content);
        let text = |range: Range<usize>| &content[range];

        assert_eq!(
            structure.tags().iter().map(|tag| text(tag.range())).collect::<Vec<_>>(),
            ["{@link List}"]
        );
    }

    #[test]
    fn markdown_code_is_where_commonmark_finds_it() {
        for content in [
            "Text\n    ~~~\n{@link List}",
            "1. ```\n   x\n   ```\n\n{@link List}",
            "`{@code x` and {@link List}",
            "Text\n\n    @param <T>\n\n{@link List}",
        ] {
            let structure = Structure::from_markdown(content);
            assert_eq!(
                structure.tags().iter().map(Tag::name).collect::<Vec<_>>(),
                ["link"],
                "{content}"
            );
        }
    }

    #[test]
    fn markdown_sections_end_with_trailing_whitespace_only_in_code() {
        let preserved = |content| {
            Structure::from_markdown(content)
                .blocks
                .into_iter()
                .filter_map(|block| match block {
                    Block::PreservedLine(line) => Some(line),
                    Block::Blank | Block::Flow(_) => None,
                })
                .collect::<Vec<_>>()
        };

        assert_eq!(preserved("a\n\n    code  "), ["    code  "]);
        assert_eq!(preserved("- a\n\n      code\t"), ["- a", "", "      code\t"]);
        assert_eq!(preserved("> ```\n> b  "), ["> ```", "> b  "]);
        assert_eq!(preserved("```\nb  \n```  "), ["```", "b  ", "```"]);
        assert_eq!(preserved("a\n\n    code\n      "), ["    code", ""]);
        assert_eq!(preserved("# h  \n@param x y\n\n    code  "), ["# h", "    code  "]);
        assert!(!ends_in_code("```\nb\n```"));
        assert!(ends_in_code("a\n\n    code"));
    }

    #[test]
    fn markdown_raw_html_keeps_the_complete_document_physical() {
        let structure = markdown_structure(
            "/// <script>\n/// call(  one );\n///\n/// call(  two );\n/// </script>",
        );

        assert!(structure.blocks.iter().all(|block| matches!(block, Block::PreservedLine(_))));
    }

    #[test]
    fn markdown_plain_block_tags_keep_their_javadoc_flow_kind() {
        let structure = markdown_structure(
            "/// Summary.\n/// @param value plain description\n/// @return plain result",
        );

        assert_eq!(
            structure
                .blocks
                .iter()
                .map(|block| match block {
                    Block::Flow(flow) => flow.kind(),
                    Block::Blank | Block::PreservedLine(_) => {
                        panic!("plain Markdown Javadoc should remain reflowable")
                    }
                })
                .collect::<Vec<_>>(),
            [FlowKind::Prose, FlowKind::BlockTag, FlowKind::BlockTag]
        );
    }
}
