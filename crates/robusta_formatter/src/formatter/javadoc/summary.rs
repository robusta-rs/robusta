use robusta_javadoc::{Block, FlowKind, HtmlBlock, JavadocKind, balanced_inline_end};

use super::is_empty_paragraph;
use super::normalize::append_trailing_dot;

/// Add a period to the summary, the text javadoc shows for the comment in
/// summary tables, when it ends in a letter or digit.
pub fn punctuate_summary(blocks: &mut [Block], kind: JavadocKind, paragraph_html_tags: bool) {
    if let Some((index, atom, end)) = summary_end(blocks, kind, paragraph_html_tags)
        && let Block::Flow(flow) = &mut blocks[index]
    {
        append_trailing_dot(&mut flow.atoms_mut()[atom], end);
    }
}

/// Where the summary ends without a period, as a block, atom, and byte
/// offset, or `None` when it ends with one or in text that takes none.
/// Javadoc ends it in text, at a block tag, at a break tag after other
/// content, and in Markdown with the first paragraph.
fn summary_end(
    blocks: &[Block],
    kind: JavadocKind,
    paragraph_html_tags: bool,
) -> Option<(usize, usize, usize)> {
    let markdown = kind == JavadocKind::Markdown;
    let mut end = None;
    let (mut seen, mut after_blank, mut after_text) = (false, false, false);
    for (index, block) in blocks.iter().enumerate() {
        let flow = match block {
            Block::Blank if markdown && seen => return end,
            Block::Blank => {
                after_blank = true;
                continue;
            }
            Block::PreservedLine(line) if markdown && seen && ends_markdown_paragraph(line) => {
                return end;
            }
            // The summary runs on into literal text kept as written.
            Block::PreservedLine(_) => return None,
            Block::Flow(flow) => flow,
        };
        let start = flow.html_start();
        if flow.tag_name().is_some() {
            return end;
        }
        if paragraph_html_tags {
            // Layout drops an empty `<p>` and starts a paragraph after a blank line with one.
            if is_empty_paragraph(flow) {
                continue;
            }
            if seen && start.is_none() && after_blank && after_text {
                return end;
            }
        }
        let editable =
            flow.kind() == FlowKind::Prose && matches!(start, None | Some(HtmlBlock::Paragraph));
        let atoms = flow.atoms();
        for (atom_index, atom) in atoms.iter().enumerate() {
            let mut from = 0;
            loop {
                let tag = break_tag(atom, from);
                let text_end = tag.map_or(atom.len(), |(tag_start, _)| tag_start);
                if text_end > from {
                    if ends_summary(&atom[from..text_end]) {
                        return None;
                    }
                    let verbatim = text_end == atom.len()
                        && atom_index + 1 == atoms.len()
                        && flow.ends_verbatim();
                    seen = true;
                    end = (editable && !verbatim).then_some((index, atom_index, text_end));
                }
                let Some((_, tag_end)) = tag else { break };
                if seen {
                    return end;
                }
                from = tag_end;
            }
        }
        after_text = flow.html_end().is_none() && start != Some(HtmlBlock::Preformatted);
        after_blank = false;
    }
    end
}

/// Whether javadoc ends the summary in `text`: at a period before white space
/// or at the end, or at `{@return` or `{@summary`. Other inline tags hold no
/// break.
fn ends_summary(text: &str) -> bool {
    let mut from = 0;
    while let Some(offset) = text[from..].find(['.', '{']) {
        let start = from + offset;
        if let Some(after) = text[start..].strip_prefix('.') {
            if after.bytes().next().is_none_or(|byte| byte.is_ascii_whitespace()) {
                return true;
            }
        } else if let Some(name) = text[start..].strip_prefix("{@") {
            let name = name.split(|ch: char| ch.is_whitespace() || ch == '}').next();
            if matches!(name, Some("return" | "summary")) {
                return true;
            }
            from = balanced_inline_end(text, start).unwrap_or(text.len());
            continue;
        }
        from = start + 1;
    }
    false
}

/// The next complete `<p>`, `<pre>`, or heading tag in `atom` from byte
/// `from`, opening or closing, as its start and end offsets.
fn break_tag(atom: &str, from: usize) -> Option<(usize, usize)> {
    let mut search = from;
    while let Some(offset) = atom[search..].find('<') {
        let start = search + offset;
        let rest = &atom[start + 1..];
        let rest = rest.strip_prefix('/').unwrap_or(rest);
        let name_len = rest.bytes().take_while(u8::is_ascii_alphanumeric).count();
        let name = &rest[..name_len];
        let named = ["p", "pre", "h1", "h2", "h3", "h4", "h5", "h6"]
            .iter()
            .any(|tag| name.eq_ignore_ascii_case(tag));
        let after_name = &rest[name_len..];
        if named
            && after_name.starts_with(|ch: char| ch == '>' || ch == '/' || ch.is_whitespace())
            && let Some(close) = after_name.find('>')
        {
            return Some((start, atom.len() - after_name.len() + close + 1));
        }
        search = start + 1;
    }
    None
}

/// Whether javadoc ends a Markdown paragraph before `line`: it starts, after
/// at most three spaces, with a list, heading, thematic break, or block quote
/// marker.
fn ends_markdown_paragraph(line: &str) -> bool {
    let marker = line.trim_start_matches(' ');
    line.len() - marker.len() <= 3 && marker.starts_with(['-', '+', '*', '#', '=', '_', '>'])
}
