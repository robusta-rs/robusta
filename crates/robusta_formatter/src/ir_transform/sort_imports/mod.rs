// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/ir_transform/sort_imports/mod.rs

mod group_matcher;
pub mod options;
mod sortable_imports;

pub use group_matcher::import_sort_key;
pub use sortable_imports::{FormattedImports, SortableImport};

use crate::formatter::prelude::*;
use robusta_allocator::Vec as ArenaVec;
use robusta_formatter_core::{FormatElement, LabelId, LineMode, Tag};

use crate::JavaLabels;

pub fn sort_imports_chunk(f: &mut JavaFormatter<'_, '_>, imports: FormattedImports) {
    let FormattedImports { start: chunk_start, mut items } = imports;
    let mut detached = Vec::with_capacity(items.len());
    let mut tail = 0..0;
    let import_label = LabelId::of(JavaLabels::ImportDeclaration);
    for item in &mut items {
        let start = item.elements.start;
        let end = item.elements.end;
        let mut label_depth = 0;
        let mut seen_import = false;
        let mut detached_end = start;
        let mut tail_start = end;
        for (index, element) in f.elements()[start..end].iter().enumerate() {
            match element {
                FormatElement::Tag(Tag::StartLabelled(label)) => {
                    seen_import |= *label == import_label;
                    label_depth += 1;
                }
                FormatElement::Tag(Tag::EndLabelled) => label_depth -= 1,
                FormatElement::Line(LineMode::Empty) if label_depth == 0 && !seen_import => {
                    detached_end = start + index + 1;
                }
                FormatElement::Tag(Tag::StartLineSuffix)
                    if label_depth == 0
                        && seen_import
                        && matches!(
                            f.elements().get(start + index + 1),
                            Some(FormatElement::Line(LineMode::Hard | LineMode::Empty))
                        ) =>
                {
                    tail_start = start + index;
                    break;
                }
                _ => {}
            }
        }
        detached.push(start..detached_end);
        item.elements = detached_end..tail_start;
        if tail_start < end {
            tail = tail_start..end;
        }
    }
    items.sort_by(|left, right| left.key.cmp(&right.key));
    let mut sorted =
        ArenaVec::with_capacity_in(f.elements().len() - chunk_start + items.len(), &f.allocator());
    let mut previous_group = None;
    for (item, detached) in items.iter().zip(detached) {
        let group = (item.key.section, item.key.group);
        if let Some(previous) = previous_group {
            sorted.push(FormatElement::Line(if previous == group {
                LineMode::Hard
            } else {
                LineMode::Empty
            }));
        }
        sorted.extend(f.elements()[detached].iter().cloned());
        sorted.extend(f.elements()[item.elements.clone()].iter().cloned());
        previous_group = Some(group);
    }
    sorted.extend(f.elements()[tail].iter().cloned());
    f.replace_end(chunk_start, &sorted);
}
