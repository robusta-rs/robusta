mod declarations;
mod embedded;
mod html;
mod markdown;
mod normalize;
mod param_order;
mod summary;
mod wrap;

use robusta_ast::Comment;
use robusta_formatter_core::write;
use robusta_javadoc::{Block, Flow, FlowKind, HtmlBlock, Javadoc, JavadocKind};
use robusta_syntax::line_terminator::contains_line_terminator;

use crate::formatter::prelude::*;
use crate::{
    CommentLineStrategy, DescriptionWithDot, JavadocOptions, LineWrappingStyle, SeparateTagGroups,
};

pub use self::declarations::JavadocDeclarations;
pub use self::embedded::format_embedded;
use self::html::{Break, Unit, UnitContent, lay_out};
use self::markdown::normalized_structure;
use self::normalize::{append_trailing_dot, capitalize_first, normalize_tag_name};
pub use self::param_order::DeclaredParameters;
use self::param_order::reorder_param_tags;
use self::summary::punctuate_summary;
use self::wrap::{fill, kept_lines_or_fill, single_line_or_block};

/// The options `comment` is rewritten with, when policy formats it as Javadoc.
pub(super) fn javadoc_options(
    comment: Comment,
    own_line: bool,
    f: &JavaFormatter<'_, '_>,
) -> Option<crate::JavadocOptions> {
    let comments = f.comments();
    f.options().javadoc.filter(|_| comments.should_format_javadoc(comment, own_line))
}

/// Write `comment`, which policy formats as Javadoc.
pub(super) fn format_javadoc_comment(
    comment: Comment,
    options: JavadocOptions,
    f: &mut JavaFormatter<'_, '_>,
) {
    let document = Javadoc::from_comment(f.source_text().as_str(), &comment)
        .expect("formatting policy accepts only Javadoc comments");
    let blocks = content_blocks(document, options);
    let blocks = order_tags(blocks, f.comments().declared_parameters(document));
    let blocks = arrange_blocks(blocks, document.kind(), options);
    match document.kind() {
        JavadocKind::Traditional => {
            // Line up the stars under the first one, wherever the comment starts.
            let comment = format_once(|f| format_traditional(document, blocks, options, f));
            align_to_column(&comment).fmt(f);
        }
        JavadocKind::Markdown => {
            if blocks.is_empty() {
                text("///").fmt(f);
            }
            let last = blocks.len().saturating_sub(1);
            for (index, block) in blocks.into_iter().enumerate() {
                if index > 0 {
                    hard_line_break().fmt(f);
                }
                match block {
                    Block::Blank => text("///").fmt(f),
                    Block::Flow(flow) => {
                        text("/// ").fmt(f);
                        let prefix = if flow.kind() == FlowKind::Prose { "/// " } else { "///   " };
                        let line_starts = flow.line_starts().to_vec();
                        let style = options.line_wrapping_style;
                        write_atoms(flow.into_atoms(), &line_starts, prefix, style, f);
                    }
                    Block::PreservedLine(line) => {
                        text("///").fmt(f);
                        // Spaces after the last line mean nothing.
                        let line =
                            if index == last { line.trim_end_matches([' ', '\t']) } else { &line };
                        if !line.is_empty() {
                            write!(f, [text(" "), text(f.allocator().alloc_str(line))]);
                        }
                    }
                }
            }
        }
    }
}

/// Write the `/** ... */` form of `document`.
fn format_traditional(
    document: Javadoc<'_>,
    blocks: Vec<Block>,
    options: JavadocOptions,
    f: &mut JavaFormatter<'_, '_>,
) {
    let mut units = lay_out(blocks, options.paragraph_html_tags);
    if units.is_empty() && allows_single_line(document, options) {
        text("/** */").fmt(f);
        return;
    }
    if matches!(units.as_slice(), [unit] if uses_single_line(document, unit, options))
        && let Some(Unit { content: UnitContent::Flow { atoms, .. }, .. }) = units.pop()
    {
        single_line_or_block(atoms, f);
        return;
    }
    text("/**").fmt(f);
    for unit in units {
        hard_line_break().fmt(f);
        if unit.break_before == Break::Blank {
            write!(f, [text(" *"), hard_line_break()]);
        }
        match unit.content {
            UnitContent::Flow { atoms, line_starts, indent, hang, .. } => {
                let allocator = f.allocator();
                text(allocator.alloc_str(&format!(" * {:indent$}", ""))).fmt(f);
                let prefix = allocator.alloc_str(&format!(" * {:hang$}", ""));
                write_atoms(atoms, &line_starts, prefix, options.line_wrapping_style, f);
            }
            UnitContent::Preserved(line) if line.is_empty() => text(" *").fmt(f),
            UnitContent::Preserved(line) => {
                write!(f, [text(" * "), text(f.allocator().alloc_str(&line))]);
            }
        }
    }
    write!(f, [hard_line_break(), text(" */")]);
}

/// The comment's blocks, none when it has no content. An empty comment is
/// kept, as javadoc tells it apart from no comment.
fn content_blocks(document: Javadoc<'_>, options: JavadocOptions) -> Vec<Block> {
    let structure = match document.kind() {
        JavadocKind::Traditional => document.structure(),
        JavadocKind::Markdown => normalized_structure(
            &document.literal_lines().expect("literal Javadoc delimiters").join("\n"),
            options.capitalize_descriptions,
        ),
    };
    let mut blocks = structure.into_blocks();
    if blocks.iter().all(is_empty_block) {
        blocks.clear();
    }
    blocks
}

/// Whether `comment-line-strategy` lets `document` take one line.
fn allows_single_line(document: Javadoc<'_>, options: JavadocOptions) -> bool {
    match options.comment_line_strategy {
        CommentLineStrategy::SingleLine => true,
        CommentLineStrategy::Multiline => false,
        CommentLineStrategy::Keep => !contains_line_terminator(document.text()),
    }
}

fn uses_single_line(document: Javadoc<'_>, unit: &Unit, options: JavadocOptions) -> bool {
    let UnitContent::Flow { kind, atoms, line_starts, indent, .. } = &unit.content else {
        return false;
    };
    allows_single_line(document, options)
        && *kind == FlowKind::Prose
        && *indent == 0
        // Stars on the opening line would become a margin on the next pass.
        && !atoms.first().is_some_and(|atom| atom.starts_with('*'))
        && (options.line_wrapping_style == LineWrappingStyle::Greedy || line_starts.is_empty())
}

fn write_atoms<'ast>(
    atoms: Vec<String>,
    line_starts: &[usize],
    break_prefix: &'ast str,
    style: LineWrappingStyle,
    f: &mut JavaFormatter<'_, 'ast>,
) {
    if style == LineWrappingStyle::Keep && !line_starts.is_empty() {
        kept_lines_or_fill(atoms, line_starts, break_prefix, f);
    } else {
        fill(atoms, break_prefix, f);
    }
}

/// A block tag and the blocks that continue it.
type TagSection = Vec<Block>;

/// Order the block tags after the description and rename synonyms.
fn order_tags(mut blocks: Vec<Block>, declared: Option<DeclaredParameters<'_>>) -> Vec<Block> {
    let Some(first_tag) = blocks.iter().position(|block| block_tag_name(block).is_some()) else {
        return blocks;
    };
    let mut sections: Vec<(TagRank, TagSection)> = Vec::new();
    let mut nearest_listed = None;
    for mut block in blocks.drain(first_tag..) {
        if let Block::Flow(flow) = &mut block
            && flow.tag_name().is_some()
        {
            normalize_tag_name(flow);
            let listed = flow.tag_name().and_then(listed_tag_order);
            nearest_listed = listed.or(nearest_listed);
            sections
                .push((TagRank { group: nearest_listed, unlisted: listed.is_none() }, Vec::new()));
        }
        sections.last_mut().expect("the first block opens a tag").1.push(block);
    }
    sections.sort_by_key(|(rank, _)| *rank);
    let mut sections: Vec<TagSection> = sections.into_iter().map(|(_, section)| section).collect();
    if let Some(declared) = declared {
        let params = listed_range(&sections, "param");
        reorder_param_tags(&mut sections[params], declared);
    }
    let throws = listed_range(&sections, "throws");
    sections[throws].sort_by(|left, right| tag_subject(left).cmp(&tag_subject(right)));
    blocks.extend(sections.into_iter().flatten());
    blocks
}

/// A tag sorts with the nearest listed tag at or above it, after the listed
/// tags of that group when it is not listed itself.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct TagRank {
    group: Option<u8>,
    unlisted: bool,
}

fn listed_tag_order(name: &str) -> Option<u8> {
    Some(match name {
        "author" => 0,
        "version" => 1,
        "param" => 2,
        "return" => 3,
        "throws" => 4,
        "see" => 5,
        "since" => 6,
        "serial" | "serialField" | "serialData" => 7,
        "deprecated" => 8,
        _ => return None,
    })
}

/// The contiguous sections of the listed tag `name`.
fn listed_range(sections: &[TagSection], name: &str) -> std::ops::Range<usize> {
    let is_name = |section: &TagSection| section.first().and_then(block_tag_name) == Some(name);
    let start = sections.iter().position(is_name).unwrap_or(sections.len());
    let end = start + sections[start..].iter().take_while(|section| is_name(section)).count();
    start..end
}

fn block_tag_name(block: &Block) -> Option<&str> {
    match block {
        Block::Flow(flow) => flow.tag_name(),
        Block::Blank | Block::PreservedLine(_) => None,
    }
}

/// The word after the tag name, such as `x` in `@param x the value`.
fn tag_subject(section: &TagSection) -> Option<&str> {
    let Some(Block::Flow(flow)) = section.first() else {
        return None;
    };
    flow.atoms().iter().flat_map(|atom| atom.split_whitespace()).nth(1)
}

/// Apply the content rules: trim blank edges, space the block tags, and
/// capitalize or punctuate description paragraphs.
fn arrange_blocks(blocks: Vec<Block>, kind: JavadocKind, options: JavadocOptions) -> Vec<Block> {
    let mut arranged: Vec<Block> = Vec::with_capacity(blocks.len() + 1);
    let mut previous_tag: Option<String> = None;
    for block in blocks {
        let tag = match &block {
            Block::Flow(flow) => flow.tag_name().map(str::to_string),
            Block::Blank | Block::PreservedLine(_) => None,
        };
        if let Some(tag) = tag {
            // Drop the blank lines before the tag, also around the empty `<p>`s
            // that end the previous description, which layout places itself.
            let mut index = arranged.len();
            while index > 0 && is_empty_paragraph_or_blank(&arranged[index - 1]) {
                index -= 1;
                if matches!(arranged[index], Block::Blank) {
                    arranged.remove(index);
                }
            }
            let separate = match &previous_tag {
                None => !arranged.is_empty(),
                Some(previous) => separates_tags(options.separate_tag_groups, previous, &tag),
            };
            if separate {
                arranged.push(Block::Blank);
            }
            previous_tag = Some(tag);
        }
        arranged.push(block);
    }
    while matches!(arranged.last(), Some(Block::Blank)) {
        arranged.pop();
    }
    let leading_blanks = arranged.iter().take_while(|block| matches!(block, Block::Blank)).count();
    arranged.drain(..leading_blanks);
    let dot = options.description_with_dot;
    if options.capitalize_descriptions || dot == DescriptionWithDot::Always {
        edit_description_paragraphs(&mut arranged, kind, options);
    }
    if dot == DescriptionWithDot::Summary {
        punctuate_summary(&mut arranged, kind, options.paragraph_html_tags);
    }
    arranged
}

fn separates_tags(policy: SeparateTagGroups, previous: &str, tag: &str) -> bool {
    match policy {
        SeparateTagGroups::Never => false,
        SeparateTagGroups::BeforeReturn => tag == "return" && previous != "return",
        SeparateTagGroups::Always => tag != previous,
    }
}

fn is_empty_paragraph_or_blank(block: &Block) -> bool {
    match block {
        Block::Blank => true,
        Block::Flow(flow) => is_empty_paragraph(flow),
        Block::PreservedLine(_) => false,
    }
}

/// Whether `flow` is a `<p>` with no text, such as one before a list.
fn is_empty_paragraph(flow: &Flow) -> bool {
    flow.html_start() == Some(HtmlBlock::Paragraph) && flow.atoms().len() == 1
}

fn is_empty_block(block: &Block) -> bool {
    match block {
        Block::Blank => true,
        Block::Flow(flow) => flow.atoms().is_empty(),
        Block::PreservedLine(line) => line.trim().is_empty(),
    }
}

fn edit_description_paragraphs(blocks: &mut [Block], kind: JavadocKind, options: JavadocOptions) {
    let breaks = paragraph_breaks(blocks, kind, options);
    for index in 0..blocks.len() {
        let (starts, ends) = (breaks[index], breaks[index + 1]);
        let Block::Flow(flow) = &mut blocks[index] else {
            continue;
        };
        let paragraph = flow.html_start() == Some(HtmlBlock::Paragraph);
        if flow.kind() != FlowKind::Prose || (flow.html_start().is_some() && !paragraph) {
            continue;
        }
        // Skip the `<p>` atom; a `<p>` joined to its word keeps its case.
        let first = if paragraph { flow.atoms()[0].ends_with('>').then_some(1) } else { Some(0) };
        let starts_verbatim = flow.starts_verbatim();
        let ends_verbatim = flow.ends_verbatim();
        let atoms = flow.atoms_mut();
        if options.capitalize_descriptions
            && (starts || paragraph)
            && !starts_verbatim
            && let Some(first) = first.and_then(|first| atoms.get_mut(first))
        {
            capitalize_first(first);
        }
        if options.description_with_dot == DescriptionWithDot::Always
            && ends
            && !ends_verbatim
            && let Some(last) = atoms.last_mut()
        {
            let end = last.len();
            append_trailing_dot(last, end);
        }
    }
}

/// Whether a paragraph boundary lies before each block, with a final entry
/// for the end. A traditional paragraph starts at `<p>`, at a block tag, or at
/// text after a blank line that follows paragraph text. Text next to other
/// block HTML stays in the paragraph around it.
fn paragraph_breaks(blocks: &[Block], kind: JavadocKind, options: JavadocOptions) -> Vec<bool> {
    let mut breaks = vec![false; blocks.len() + 1];
    breaks[0] = true;
    breaks[blocks.len()] = true;
    if kind == JavadocKind::Markdown {
        let boundary = |block: &Block| match block {
            Block::Blank | Block::PreservedLine(_) => true,
            Block::Flow(flow) => flow.tag_name().is_some(),
        };
        for index in 1..blocks.len() {
            breaks[index] = boundary(&blocks[index - 1]) || boundary(&blocks[index]);
        }
        return breaks;
    }
    let (mut content_end, mut after_text) = (0, false);
    for (index, block) in blocks.iter().enumerate() {
        let flow = match block {
            Block::Blank => continue,
            // A preserved line continues the literal of the flow before it.
            Block::PreservedLine(_) => {
                content_end = index + 1;
                continue;
            }
            Block::Flow(flow) => flow,
        };
        let start = flow.html_start();
        if options.paragraph_html_tags && is_empty_paragraph(flow) {
            // Layout drops an empty paragraph, such as `<p>` before a list.
            continue;
        }
        let after_blank = content_end < index;
        if flow.tag_name().is_some()
            || start == Some(HtmlBlock::Paragraph)
            || (after_blank && after_text && start.is_none())
        {
            breaks[content_end..=index].fill(true);
        }
        after_text = flow.html_end().is_none() && start != Some(HtmlBlock::Preformatted);
        content_end = index + 1;
    }
    breaks
}
