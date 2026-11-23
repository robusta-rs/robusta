use std::ops::Deref;

use robusta_allocator::Vec as ArenaVec;
use rustc_hash::FxHashMap;

use crate::{PrintResult, Printed, Printer, PrinterOptions};

use super::{
    FormatElement, FormatElements, Interned,
    tag::{self, GroupMode, LabelId, Tag, TagKind},
};

/// A formatted document.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Document<'a> {
    elements: &'a [FormatElement<'a>],
}

impl<'a> Document<'a> {
    /// Creates a new document from the given elements.
    pub fn new(elements: ArenaVec<'a, FormatElement<'a>>) -> Self {
        Self { elements: elements.into_arena_slice() }
    }

    /// Returns the format elements that make up this document.
    pub fn elements(&self) -> &'a [FormatElement<'a>] {
        self.elements
    }

    /// Finalizes and prints the document: propagates group expansion once
    /// (`Self::propagate_expand`) and hands the elements to the `Printer`.
    ///
    /// The single home of the finalize-once-then-print sequence:
    /// `Formatted::print` delegates here.
    ///
    /// # Errors
    /// Returns `PrintError` if the document contains invalid structure.
    pub fn print(self, source_size_hint: usize, options: PrinterOptions) -> PrintResult<Printed> {
        self.propagate_expand();
        Printer::with_capacity(source_size_hint, options).print(self.elements())
    }

    /// Like [`Self::print`], but starts at the given indentation level.
    ///
    /// # Errors
    /// Returns `PrintError` if the document contains invalid structure.
    pub fn print_with_indent(
        self,
        source_size_hint: usize,
        options: PrinterOptions,
        indent: u16,
    ) -> PrintResult<Printed> {
        self.propagate_expand();
        Printer::with_capacity(source_size_hint, options).print_with_indent(self.elements(), indent)
    }
}

impl Document<'_> {
    /// Sets the group's [`mode`](tag::Group::mode) to [`Propagated`](tag::GroupMode::Propagated)
    /// if the group contains any of:
    /// * a group whose mode is [`Propagated`](tag::GroupMode::Propagated) or
    ///   [`Expand`](tag::GroupMode::Expand).
    /// * a non-soft [line break](FormatElement::Line) whose [`LineMode::will_break()`](super::LineMode::will_break) is true.
    /// * a multiline [FormatElement::Text] whose `TextWidth` is not marked `without_expand_parent`.
    /// * a [FormatElement::ExpandParent]
    ///
    /// [`BestFitting`] elements act as expand boundaries, meaning that the fact that a
    /// [`BestFitting`]'s content expands is not propagated past the [`BestFitting`] element.
    ///
    /// [`BestFitting`]: FormatElement::BestFitting
    pub(crate) fn propagate_expand(&self) {
        #[derive(Debug)]
        enum Enclosing<'a> {
            Group(&'a tag::Group),
            BestFitting,
        }

        fn expand_parent(enclosing: &[Enclosing]) {
            if let Some(Enclosing::Group(group)) = enclosing.last() {
                group.propagate_expand();
            }
        }

        fn propagate_expands<'a>(
            elements: &'a [FormatElement<'a>],
            enclosing: &mut Vec<Enclosing<'a>>,
            checked_interned: &mut FxHashMap<&'a Interned<'a>, bool>,
        ) -> bool {
            let mut expands = false;
            for element in elements {
                let element_expands = match element {
                    FormatElement::Tag(Tag::StartGroup(group)) => {
                        enclosing.push(Enclosing::Group(group));
                        false
                    }
                    FormatElement::Tag(Tag::EndGroup) => match enclosing.pop() {
                        Some(Enclosing::Group(group)) => !group.mode().is_flat(),
                        _ => false,
                    },
                    FormatElement::Interned(interned) => {
                        if let Some(interned_expands) = checked_interned.get(interned) {
                            *interned_expands
                        } else {
                            let interned_expands =
                                propagate_expands(interned, enclosing, checked_interned);
                            checked_interned.insert(interned, interned_expands);
                            interned_expands
                        }
                    }
                    FormatElement::BestFitting(best_fitting) => {
                        enclosing.push(Enclosing::BestFitting);

                        for variant in best_fitting.variants() {
                            propagate_expands(variant, enclosing, checked_interned);
                        }

                        enclosing.pop();
                        // Not `expands = false`: an interned list caches whether it expands, so
                        // resetting it here would hide an earlier expanding sibling from every
                        // later reference to the same interned list.
                        false
                    }
                    // `FormatElement::Token` cannot contain line breaks
                    FormatElement::Text { text: _, width } => width.propagates_expand(),
                    FormatElement::ExpandParent => true,
                    FormatElement::Line(mode) => {
                        mode.will_break()
                            && !matches!(
                                enclosing.last(),
                                Some(Enclosing::Group(group)) if group.fits_all_lines()
                            )
                    }
                    _ => false,
                };

                if element_expands {
                    expands = true;
                    expand_parent(enclosing);
                }
            }

            expands
        }

        let mut enclosing: Vec<Enclosing> = Vec::new();
        let mut interned = FxHashMap::default();
        propagate_expands(self, &mut enclosing, &mut interned);
    }
}

impl<'a> Deref for Document<'a> {
    type Target = [FormatElement<'a>];

    fn deref(&self) -> &Self::Target {
        self.elements
    }
}

impl FormatElements for [FormatElement<'_>] {
    fn will_break(&self) -> bool {
        use Tag::{EndLineSuffix, StartLineSuffix};
        let mut ignore_depth = 0usize;

        for element in self {
            match element {
                // Line suffix
                // Ignore if any of its content breaks
                FormatElement::Tag(StartLineSuffix) => {
                    ignore_depth += 1;
                }
                FormatElement::Tag(EndLineSuffix) => {
                    ignore_depth -= 1;
                }
                FormatElement::Line(line) if line.will_break() => {
                    return true;
                }
                element if ignore_depth == 0 && element.will_break() => {
                    return true;
                }
                _ => {}
            }
        }

        debug_assert_eq!(ignore_depth, 0, "Unclosed start container");

        false
    }

    fn may_directly_break(&self) -> bool {
        fn may_break(elements: &[FormatElement<'_>], measure_flat: bool) -> bool {
            use Tag::{EndGroup, EndLineSuffix, StartGroup, StartLineSuffix};
            let mut suffix_depth = 0usize;
            let mut flat_depth = usize::from(measure_flat);

            for element in elements {
                match element {
                    FormatElement::Tag(StartLineSuffix) => suffix_depth += 1,
                    FormatElement::Tag(EndLineSuffix) => suffix_depth -= 1,
                    // A fallback cannot contribute a soft break to a surrounding layout.
                    FormatElement::Tag(StartGroup(group))
                        if flat_depth > 0 || group.mode() == GroupMode::MeasureFlat =>
                    {
                        flat_depth += 1;
                    }
                    FormatElement::Tag(EndGroup) if flat_depth > 0 => flat_depth -= 1,
                    FormatElement::Line(line)
                        if suffix_depth == 0 && (flat_depth == 0 || line.will_break()) =>
                    {
                        return true;
                    }
                    FormatElement::Interned(interned)
                        if suffix_depth == 0 && may_break(interned, flat_depth > 0) =>
                    {
                        return true;
                    }
                    _ => {}
                }
            }

            debug_assert_eq!(suffix_depth, 0, "Unclosed line suffix");
            debug_assert_eq!(flat_depth, usize::from(measure_flat), "Unclosed group");
            false
        }

        may_break(self, false)
    }

    fn has_label(&self, expected: LabelId) -> bool {
        self.first().is_some_and(|element| element.has_label(expected))
    }

    fn start_tag(&self, kind: TagKind) -> Option<&Tag> {
        fn traverse_slice<'a>(
            slice: &'a [FormatElement],
            kind: TagKind,
            depth: &mut usize,
        ) -> Option<&'a Tag> {
            for element in slice.iter().rev() {
                match element {
                    FormatElement::Tag(tag) if tag.kind() == kind => {
                        if tag.is_start() {
                            if *depth == 0 {
                                // Invalid document
                                return None;
                            } else if *depth == 1 {
                                return Some(tag);
                            }
                            *depth -= 1;
                        } else {
                            *depth += 1;
                        }
                    }
                    FormatElement::Interned(interned) => {
                        match traverse_slice(interned, kind, depth) {
                            Some(start) => {
                                return Some(start);
                            }
                            // Reached end or invalid document
                            None if *depth == 0 => {
                                return None;
                            }
                            _ => {
                                // continue with other elements
                            }
                        }
                    }
                    _ => {}
                }
            }

            None
        }

        // Assert that the document ends at a tag with the specified kind;
        let _ = self.end_tag(kind);

        let mut depth = 0usize;

        traverse_slice(self, kind, &mut depth)
    }

    fn end_tag(&self, kind: TagKind) -> Option<&Tag> {
        self.last().and_then(|element| element.end_tag(kind))
    }
}

#[cfg(test)]
mod tests {
    use robusta_allocator::{Allocator, Vec as ArenaVec};

    use crate::{FormatElement, FormatElements, LineMode, format_element::Interned};

    #[test]
    fn will_break_deeply_nested_interned() {
        let allocator = &Allocator::new();

        for (line, expected) in [(LineMode::Soft, false), (LineMode::Hard, true)] {
            let mut elements = ArenaVec::from_iter_in([FormatElement::Line(line)], &allocator);

            // The non-breaking path must traverse each interned slice only once.
            // Repeating it after a failed match guard makes this exponential in depth.
            for _ in 0..64 {
                elements = ArenaVec::from_iter_in(
                    [FormatElement::Interned(Interned::new(elements))],
                    &allocator,
                );
            }

            assert_eq!(elements.will_break(), expected);
        }
    }
}
