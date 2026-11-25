//! Human-readable debug rendering for [`Document`].
//!
//! This module provides:
//! * The `Format` implementation for `&[FormatElement]` that produces a readable
//!   textual representation of the IR (used by `DisplayDocument`).
//! * The [`DisplayDocument`] wrapper exposed via [`Document::display`].
//!
//! The rendering is language-agnostic and works with any `C: FormatContext`.

use cow_utils::CowUtils;
use robusta_allocator::Allocator;

use crate::{
    Document, Format, FormatContext, FormatElement, FormatState, Formatter, PrintMode, Printer,
    PrinterOptions, SimpleFormatContext,
    builders::{hard_line_break, soft_line_break_or_space, space, text, token},
    format_element::{
        LineMode,
        tag::{self, DedentMode, GroupMode, Tag},
    },
    write as w,
};

impl<'a> Document<'a> {
    /// Returns a wrapper that formats the document for human-readable debug display.
    ///
    /// The `source_text` is passed to the temporary `SimpleFormatContext` used
    /// while rendering and is only required so that `Format` implementations that
    /// look up the source text behave sensibly. The debug renderer itself does not
    /// use the source text.
    pub fn display<'src>(&'a self, source_text: &'src str) -> DisplayDocument<'a, 'src> {
        DisplayDocument { elements: self.elements(), source_text }
    }
}

/// `Display` wrapper that renders a [`Document`] as a human-readable debug string.
///
/// Obtain one via [`Document::display`].
pub struct DisplayDocument<'a, 'src> {
    elements: &'a [FormatElement<'a>],
    source_text: &'src str,
}

impl std::fmt::Display for DisplayDocument<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let allocator = Allocator::default();
        let context = SimpleFormatContext::default().with_source_code(self.source_text);

        let mut state: FormatState<'_, SimpleFormatContext<'_>> =
            FormatState::new(context, &allocator);
        let mut buffer = Formatter::new(&mut state);

        let elements = FormatElementSlice(self.elements);
        elements.fmt(&mut buffer);

        let elements = buffer.into_vec();
        let document = Document::new(elements);

        let printer_options = PrinterOptions::default();
        let printed = Printer::new(printer_options)
            .print(document.elements())
            .expect("Expected a valid document");

        f.write_str(printed.as_code())
    }
}

/// Wrapper around a [`FormatElement`] slice so we can provide a `Format`
/// implementation without conflicting with the blanket `Format` impl for `&T`.
struct FormatElementSlice<'a>(&'a [FormatElement<'a>]);

impl<'a, C> Format<'a, C> for FormatElementSlice<'a>
where
    C: FormatContext,
{
    fn fmt(&self, f: &mut Formatter<'_, 'a, C>) {
        use Tag::{
            EndAlign, EndAlignToColumn, EndConditionalContent, EndDedent, EndEntry, EndFill,
            EndGroup, EndIndent, EndIndentFromLine, EndIndentIfGroupBreaks, EndLineSuffix,
            EndMarkAsRoot, StartAlign, StartAlignToColumn, StartConditionalContent, StartDedent,
            StartEntry, StartFill, StartGroup, StartIndent, StartIndentFromLine,
            StartIndentIfGroupBreaks, StartLineSuffix, StartMarkAsRoot,
        };

        w!(f, [ContentArrayStart]);

        let mut tag_stack = Vec::new();
        let mut first_element = true;
        let mut in_text = false;

        let mut iter = self.0.iter().peekable();

        while let Some(element) = iter.next() {
            if !first_element
                && !in_text
                && !matches!(element, FormatElement::Tag(tag) if tag.is_end())
            {
                // Write a separator between every two elements
                w!(f, [token(","), soft_line_break_or_space()]);
            }

            first_element = false;

            match element {
                element @ (FormatElement::Space
                | FormatElement::Token { .. }
                | FormatElement::Text { .. }) => {
                    if !in_text {
                        w!(f, [token("\"")]);
                    }

                    in_text = true;

                    match element {
                        FormatElement::Space => {
                            w!(f, [token(" ")]);
                        }
                        // Tokens are written as is, without escaping quotes
                        FormatElement::Token { .. } => f.write_element(element.clone()),
                        // escape quotes
                        FormatElement::Text { text: content, .. } => {
                            let escaped = content.cow_replace('"', "\\\"");
                            w!(f, [text(f.allocator().alloc_str(&escaped))]);
                        }
                        _ => unreachable!(),
                    }

                    let is_next_text = matches!(
                        iter.peek(),
                        Some(
                            FormatElement::Space
                                | FormatElement::Token { .. }
                                | FormatElement::Text { .. }
                        )
                    );

                    if !is_next_text {
                        w!(f, [token("\"")]);
                        in_text = false;
                    }
                }

                FormatElement::Line(mode) => match mode {
                    LineMode::SoftOrSpace => {
                        w!(f, [token("soft_line_break_or_space")]);
                    }
                    LineMode::Soft => {
                        w!(f, [token("soft_line_break")]);
                    }
                    LineMode::Hard => {
                        w!(f, [token("hard_line_break")]);
                    }
                    LineMode::Empty => {
                        w!(f, [token("empty_line")]);
                    }
                    LineMode::Literal => {
                        w!(f, [token("literal_line_break")]);
                    }
                },
                FormatElement::ExpandParent => {
                    w!(f, [token("expand_parent")]);
                }

                FormatElement::LineSuffixBoundary => {
                    w!(f, [token("line_suffix_boundary")]);
                }

                FormatElement::BestFitting(best_fitting) => {
                    w!(f, [token("best_fitting([")]);
                    f.write_elements([
                        FormatElement::Tag(StartIndent),
                        FormatElement::Line(LineMode::Hard),
                    ]);

                    for variant in best_fitting.variants() {
                        w!(f, [FormatElementSlice(variant), hard_line_break()]);
                    }

                    f.write_elements([
                        FormatElement::Tag(EndIndent),
                        FormatElement::Line(LineMode::Hard),
                    ]);

                    w!(f, [token("])")]);
                }

                FormatElement::Interned(interned) => {
                    let interned_elements = f.state_mut().printed_interned_elements();
                    match interned_elements.get(interned).copied() {
                        None => {
                            let index = interned_elements.len();
                            interned_elements.insert(interned.clone(), index);

                            w!(
                                f,
                                [
                                    text(
                                        f.allocator()
                                            .alloc_str(&std::format!("<interned {index}>"))
                                    ),
                                    space(),
                                    FormatElementSlice(interned),
                                ]
                            );
                        }
                        Some(reference) => {
                            w!(
                                f,
                                [text(
                                    f.allocator()
                                        .alloc_str(&std::format!("<ref interned *{reference}>"))
                                )]
                            );
                        }
                    }
                }

                FormatElement::Tag(tag) => {
                    if tag.is_start() {
                        first_element = true;
                        tag_stack.push(tag.kind());
                    }
                    // Handle documents with mismatching start/end or superfluous end tags
                    else {
                        match tag_stack.pop() {
                            None => {
                                // Only write the end tag without any indent to ensure the output document is valid.
                                w!(
                                    f,
                                    [
                                        token("<END_TAG_WITHOUT_START<"),
                                        text(
                                            f.allocator()
                                                .alloc_str(&std::format!("{:?}", tag.kind()))
                                        ),
                                        token(">>"),
                                    ]
                                );
                                first_element = false;
                                continue;
                            }
                            Some(start_kind) if start_kind != tag.kind() => {
                                w!(
                                    f,
                                    [
                                        ContentArrayEnd,
                                        token(")"),
                                        soft_line_break_or_space(),
                                        token("ERROR<START_END_TAG_MISMATCH<start: "),
                                        text(
                                            f.allocator()
                                                .alloc_str(&std::format!("{start_kind:?}"))
                                        ),
                                        token(", end: "),
                                        text(
                                            f.allocator()
                                                .alloc_str(&std::format!("{:?}", tag.kind()))
                                        ),
                                        token(">>")
                                    ]
                                );
                                first_element = false;
                                continue;
                            }
                            _ => {
                                // all ok
                            }
                        }
                    }

                    match tag {
                        StartIndent => {
                            w!(f, [token("indent(")]);
                        }

                        StartDedent(mode) => {
                            let label = match mode {
                                DedentMode::Level => "dedent",
                                DedentMode::Root => "dedentRoot",
                            };

                            w!(f, [token(label), token("(")]);
                        }

                        StartAlign(align) => {
                            w!(
                                f,
                                [
                                    token("align("),
                                    text(f.allocator().alloc_str(&align.count().to_string()),),
                                    token(","),
                                    space(),
                                ]
                            );
                        }

                        StartAlignToColumn => {
                            w!(f, [token("align_to_column(")]);
                        }

                        StartIndentFromLine => {
                            w!(f, [token("indent_from_line(")]);
                        }

                        StartLineSuffix => {
                            w!(f, [token("line_suffix(")]);
                        }

                        StartGroup(group) => {
                            w!(f, [token("group(")]);

                            if let Some(group_id) = group.id() {
                                w!(
                                    f,
                                    [
                                        text(
                                            f.allocator()
                                                .alloc_str(&std::format!("\"{group_id:?}\""))
                                        ),
                                        token(","),
                                        space(),
                                    ]
                                );
                            }

                            match group.mode() {
                                GroupMode::Flat => {}
                                GroupMode::MeasureFlat => {
                                    w!(f, [token("measure_flat: true,"), space()]);
                                }
                                GroupMode::Expand => {
                                    w!(f, [token("expand: true,"), space()]);
                                }
                                GroupMode::Propagated => {
                                    w!(f, [token("expand: propagated,"), space()]);
                                }
                            }
                        }

                        StartIndentIfGroupBreaks(id, align) => {
                            let (name, argument) = match align {
                                Some(count) => {
                                    ("align_if_group_breaks(", std::format!("{count}, \"{id:?}\""))
                                }
                                None => ("indent_if_group_breaks(", std::format!("\"{id:?}\"")),
                            };
                            w!(
                                f,
                                [
                                    token(name),
                                    text(f.allocator().alloc_str(&argument)),
                                    token(","),
                                    space(),
                                ]
                            );
                        }

                        StartConditionalContent(condition) => {
                            match condition.mode() {
                                PrintMode::Flat => {
                                    w!(f, [token("if_group_fits_on_line(")]);
                                }
                                PrintMode::Expanded => {
                                    w!(f, [token("if_group_breaks(")]);
                                }
                            }

                            if let Some(group_id) = condition.group_id() {
                                w!(
                                    f,
                                    [
                                        text(
                                            f.allocator()
                                                .alloc_str(&std::format!("\"{group_id:?}\"")),
                                        ),
                                        token(","),
                                        space(),
                                    ]
                                );
                            }
                        }

                        Tag::StartLabelled(label_id) => {
                            w!(
                                f,
                                [
                                    token("label("),
                                    text(
                                        f.allocator().alloc_str(&std::format!("\"{label_id:?}\""))
                                    ),
                                    token(","),
                                    space()
                                ]
                            );
                        }

                        StartFill => {
                            w!(f, [token("fill(")]);
                        }

                        StartMarkAsRoot => {
                            w!(f, [token("mark_as_root(")]);
                        }

                        StartEntry => {
                            // handled after the match for all start tags
                        }
                        EndEntry => w!(f, [ContentArrayEnd]),

                        EndFill
                        | Tag::EndLabelled
                        | EndConditionalContent
                        | EndIndentIfGroupBreaks(_)
                        | EndAlign
                        | EndAlignToColumn
                        | EndIndentFromLine
                        | EndIndent
                        | EndGroup
                        | EndLineSuffix
                        | EndMarkAsRoot
                        | EndDedent(_) => {
                            w!(f, [ContentArrayEnd, token(")")]);
                        }
                    }

                    if tag.is_start() {
                        w!(f, [ContentArrayStart]);
                    }
                }
            }
        }

        while let Some(top) = tag_stack.pop() {
            w!(
                f,
                [
                    ContentArrayEnd,
                    token(")"),
                    soft_line_break_or_space(),
                    text(f.allocator().alloc_str(&std::format!("<START_WITHOUT_END<{top:?}>>"))),
                ]
            );
        }

        w!(f, [ContentArrayEnd]);
    }
}

struct ContentArrayStart;

impl<'a, C> Format<'a, C> for ContentArrayStart {
    fn fmt(&self, f: &mut Formatter<'_, 'a, C>) {
        use Tag::{StartGroup, StartIndent};

        w!(f, [token("[")]);

        f.write_elements([
            FormatElement::Tag(StartGroup(tag::Group::new())),
            FormatElement::Tag(StartIndent),
            FormatElement::Line(LineMode::Soft),
        ]);
    }
}

struct ContentArrayEnd;

impl<'a, C> Format<'a, C> for ContentArrayEnd {
    fn fmt(&self, f: &mut Formatter<'_, 'a, C>) {
        use Tag::{EndGroup, EndIndent};
        f.write_elements([
            FormatElement::Tag(EndIndent),
            FormatElement::Line(LineMode::Soft),
            FormatElement::Tag(EndGroup),
        ]);

        w!(f, [token("]")]);
    }
}
