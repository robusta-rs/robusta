use markdown::mdast::Node;

use super::{Repair, break_before};
use crate::markdown::{setext, span};

/// Breaks before the lines of `=` or `-` that markdown-rs reads as a setext underline after a
/// setext heading or a definition, and can panic on: those right after another such line, or
/// after lines that hold the `]:` of a definition. In order of lines, each break is settled in
/// the reading with it: kept after a setext heading or a definition, which leave no paragraph
/// open, and dropped after anything else.
pub struct Underlines<'a> {
    /// The kept breaks, then the ones not yet settled.
    breaks: Vec<Repair<'a>>,
    kept: usize,
    dropped: Vec<Repair<'a>>,
}

impl<'a> Underlines<'a> {
    pub fn new(content: &'a str) -> Self {
        let mut breaks = Vec::new();
        let mut line = 0;
        let (mut after_marks, mut after_definition) = (false, false);
        for text in content.split('\n') {
            let marks = text.trim_start_matches([' ', '\t', '>']);
            let sequence = marks.trim_end_matches([' ', '\t']);
            let underline = !sequence.is_empty()
                && (sequence.bytes().all(|byte| byte == b'=')
                    || sequence.bytes().all(|byte| byte == b'-'));
            if underline && (after_marks || after_definition) {
                breaks.push(break_before(content, line, line + text.len() - marks.len()));
            }
            after_marks = underline;
            after_definition = (after_definition || text.contains("]:"))
                && !text.trim_matches([' ', '\t']).is_empty();
            line += text.len() + 1;
        }
        Self { breaks, kept: 0, dropped: Vec::new() }
    }

    /// The breaks to insert.
    pub fn breaks(&self) -> &[Repair<'a>] {
        &self.breaks
    }

    /// Whether a break is kept.
    pub fn any_kept(&self) -> bool {
        self.kept > 0
    }

    /// Settle the breaks in order in the reading `root`. The first that follows anything but a
    /// setext heading or a definition, or that shares its offset with one of `repairs`, is
    /// dropped. Return whether one was, which changes how the lines after it read.
    pub fn settle(&mut self, root: &Node, content: &str, repairs: &[Repair<'_>]) -> bool {
        while let Some(&(at, _)) = self.breaks.get(self.kept) {
            if repairs.iter().any(|repair| repair.0 == at)
                || !follows_no_paragraph(root, content, at)
            {
                self.dropped.push(self.breaks.remove(self.kept));
                return true;
            }
            self.kept += 1;
        }
        false
    }

    /// Settle every break again, after a repair or a tab changed how the lines read.
    pub fn unsettle(&mut self) {
        self.breaks.append(&mut self.dropped);
        self.breaks.sort_unstable();
        self.kept = 0;
    }
}

/// Whether the break that a repair inserted at `at` follows a setext heading or a definition.
fn follows_no_paragraph(node: &Node, content: &str, at: usize) -> bool {
    let Some(children) = node.children() else { return false };
    children.iter().enumerate().any(|(index, child)| match span(child) {
        Some(range) if range.is_empty() && range.start == at => {
            matches!(child, Node::ThematicBreak(_))
                && index > 0
                && (setext(&children[index - 1], content)
                    || matches!(children[index - 1], Node::Definition(_)))
        }
        Some(range) if range.contains(&at) => follows_no_paragraph(child, content, at),
        _ => false,
    })
}
