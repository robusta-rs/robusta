use std::ops::Range;

use markdown::mdast::Node;
use robusta_javadoc::markdown::{
    columns, indentation, indented_range, next_column, quote_content, span,
};

use super::{Edit, Normalizer, Scope, capitalize_first};

/// The largest number CommonMark reads as an ordered list marker.
const MAXIMUM_ORDERED_LIST_MARKER: u32 = 999_999_999;

impl Normalizer<'_> {
    /// Mark bullets with `-` and numbers with `.`, renumber from the first
    /// number, and put a blank line between a paragraph and the list after it.
    /// Adjacent lists of the same kind alternate with `*` and `)`, and also get
    /// a blank line between them.
    pub(super) fn list(&mut self, siblings: &[Node], index: usize, scope: Scope) -> Scope {
        if scope.reindented {
            return scope;
        }
        let (Node::List(list), Some(range)) = (&siblings[index], span(&siblings[index])) else {
            return scope;
        };
        let content = self.content;
        // The start of the list's line when no blank line comes before it.
        let line_start = content[..range.start].rfind('\n').and_then(|newline| {
            let previous = content[..newline].rsplit('\n').next().unwrap_or_default();
            (index > 0 && !quote_content(previous).trim().is_empty()).then_some(newline + 1)
        });
        let interrupts = line_start.is_some() && matches!(siblings[index - 1], Node::Paragraph(_));
        // A different marker is what separates adjacent lists of the same kind.
        let same_kind =
            |node: &Node| matches!(node, Node::List(other) if other.ordered == list.ordered);
        let mut first = index;
        while first > 0 && same_kind(&siblings[first - 1]) {
            first -= 1;
        }
        let (bullet, delimiter) = if (index - first) % 2 == 1 { ("*", ")") } else { ("-", ".") };
        let mut group = Vec::new();
        let mut numbers = Vec::new();
        let mut renumber = true;
        for (index, item) in list.children.iter().enumerate() {
            let Some(item_range) = span(item) else {
                return scope;
            };
            // An item starts at its indentation, before the marker.
            let indented = &content[item_range.start..];
            let start =
                item_range.start + indented.len() - indented.trim_start_matches([' ', '\t']).len();
            let marker = &content[start..];
            if list.ordered {
                let digits = marker.bytes().take_while(u8::is_ascii_digit).count();
                if digits == 0 {
                    renumber = false;
                } else {
                    if !marker[digits..].starts_with(delimiter) {
                        group.push(Edit::new(start + digits..start + digits + 1, delimiter));
                    }
                    let number = u32::try_from(index)
                        .map_or(MAXIMUM_ORDERED_LIST_MARKER, |index| {
                            list.start.unwrap_or(1).saturating_add(index)
                        })
                        .min(MAXIMUM_ORDERED_LIST_MARKER)
                        .to_string();
                    if number.len() != digits {
                        numbers.extend(self.item_indentation(
                            item_range,
                            start..start + digits,
                            number.len(),
                        ));
                    }
                    numbers.push(Edit::new(start..start + digits, number));
                }
            } else if marker.starts_with(['-', '*', '+']) && !marker.starts_with(bullet) {
                group.push(Edit::new(start..start + 1, bullet));
            }
            if self.capitalize {
                self.capitalize_item(item);
            }
        }
        let reindented = renumber
            && numbers.iter().any(|edit| {
                self.content[edit.range.clone()].bytes().all(|byte| matches!(byte, b' ' | b'\t'))
            });
        if renumber {
            group.append(&mut numbers);
        }
        group.retain(|edit| self.content[edit.range.clone()] != *edit.text);
        self.lists.push(group);

        if scope.root
            && (interrupts || index > first)
            && let Some(line_start) = line_start
        {
            self.push(vec![Edit::new(line_start..line_start, "\n")]);
        }
        Scope { reindented, ..scope }
    }

    fn capitalize_item(&mut self, item: &Node) {
        let Some(node @ Node::Paragraph(paragraph)) =
            item.children().and_then(|children| children.first())
        else {
            return;
        };
        if indented_range(node, self.content).is_some() {
            return;
        }
        let Some(text) = paragraph.children.first().filter(|child| matches!(child, Node::Text(_)))
        else {
            return;
        };
        let Some(range) = span(text) else {
            return;
        };
        let source = &self.content[range.clone()];
        // Text can start with Unicode whitespace, such as a no-break space.
        let mut word = source.split(char::is_whitespace).next().unwrap_or_default().to_string();
        capitalize_first(&mut word);
        if let Some(first) = word.get(..1)
            && !source.starts_with(first)
        {
            self.capitals.push(Edit::new(range.start..range.start + 1, first.to_string()));
        }
    }

    /// Move the item's continuation prefix by the change in marker width.
    fn item_indentation(
        &self,
        range: Range<usize>,
        marker: Range<usize>,
        width: usize,
    ) -> Vec<Edit> {
        let digits = marker.len();
        let start = marker.start;
        let line_start = self.content[..start].rfind('\n').map_or(0, |newline| newline + 1);
        let column = columns(&self.content[line_start..start]);
        let after = column + digits + 1;
        let padding = self.content[marker.end + 1..]
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t'))
            .fold(after, next_column)
            - after;
        let padding = if padding > 4 { 1 } else { padding.max(1) };
        let end_column = after + padding;
        let mut edits = Vec::new();
        let mut offset = range.start;
        for (index, line) in self.content[range].split('\n').enumerate() {
            if index > 0
                && let Some((range, extra)) = indentation(line, column..end_column, 0)
            {
                let spaces = end_column - column + width - digits + extra;
                edits.push(Edit::new(offset + range.start..offset + range.end, " ".repeat(spaces)));
            }
            offset += line.len() + 1;
        }
        edits
    }
}
