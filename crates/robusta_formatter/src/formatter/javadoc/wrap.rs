use robusta_formatter_core::write;

use crate::formatter::prelude::*;

/// Fill `atoms` to the line width, starting each wrapped line with
/// `break_prefix`.
pub(super) fn fill<'ast>(
    atoms: Vec<String>,
    break_prefix: &'ast str,
    f: &mut JavaFormatter<'_, 'ast>,
) {
    if atoms.is_empty() {
        return;
    }
    let atoms = alloc_atoms(atoms, f);
    write_fill(&atoms, break_prefix, f);
}

/// Keep the source lines, split at `line_starts`, when every one fits;
/// otherwise fill as [`fill`] does.
pub(super) fn kept_lines_or_fill<'ast>(
    atoms: Vec<String>,
    line_starts: &[usize],
    break_prefix: &'ast str,
    f: &mut JavaFormatter<'_, 'ast>,
) {
    let atoms = alloc_atoms(atoms, f);
    let mut lines = Vec::with_capacity(line_starts.len() + 1);
    let mut start = 0;
    for &end in line_starts.iter().chain(std::iter::once(&atoms.len())) {
        lines.push(&*f.allocator().alloc_str(&atoms[start..end].join(" ")));
        start = end;
    }
    let group_id = f.state().group_id("javadoc_kept_lines");
    let kept = format_with(|f| {
        for (index, &line) in lines.iter().enumerate() {
            if index > 0 {
                write!(f, [hard_line_break(), text(break_prefix)]);
            }
            text(line).fmt(f);
        }
    });
    let filled = format_with(|f| write_fill(&atoms, break_prefix, f));
    let content = format_with(|f| {
        write!(
            f,
            [
                if_group_fits_on_line(&kept).with_group_id(Some(group_id)),
                if_group_breaks(&filled).with_group_id(Some(group_id)),
            ]
        );
    });
    write!(f, [group(&content).with_group_id(Some(group_id)).fits_all_lines(true)]);
}

/// Write `/** Content. */` when it fits on one line, and otherwise the
/// multi-line form with the content filled.
pub(super) fn single_line_or_block(atoms: Vec<String>, f: &mut JavaFormatter<'_, '_>) {
    let atoms = alloc_atoms(atoms, f);
    let filled = format_with(|f| write_fill(&atoms, " * ", f));
    let content = format_with(|f| {
        write!(
            f,
            [
                text("/**"),
                soft_line_break_or_space(),
                if_group_breaks(&text(" * ")),
                filled,
                soft_line_break_or_space(),
                if_group_breaks(&text(" ")),
                text("*/"),
            ]
        );
    });
    write!(f, [group(&content)]);
}

fn alloc_atoms<'ast>(atoms: Vec<String>, f: &JavaFormatter<'_, 'ast>) -> Vec<&'ast str> {
    atoms.into_iter().map(|atom| &*f.allocator().alloc_str(&atom)).collect()
}

fn write_fill<'ast>(atoms: &[&'ast str], break_prefix: &'ast str, f: &mut JavaFormatter<'_, 'ast>) {
    let separator = format_with(|f| {
        soft_line_break_or_space().fmt(f);
        if !break_prefix.is_empty() {
            write!(f, [if_group_breaks(&text(break_prefix))]);
        }
    });
    let mut fill = f.fill();
    for atom in atoms {
        fill.entry(&separator, &text(atom));
    }
    fill.finish();
}
