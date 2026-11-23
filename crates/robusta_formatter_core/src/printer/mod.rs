// Directory adapted from https://github.com/oxc-project/oxc/tree/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter_core/src/printer

mod call_stack;
pub mod error;
mod line_suffixes;
mod options;
mod queue;
mod stack;

use std::num::{NonZeroU8, NonZeroU16};

pub use options::*;
use robusta_data_structures::code_buffer::{self, CodeBuffer};
use unicode_width::UnicodeWidthChar;

use crate::{
    ActualStart, BestFittingElement, Condition, DedentMode, FormatElement, GroupId, GroupMode,
    IndentStyle, InvalidDocumentError, LineMode, PrintError, PrintMode, Tag, TagKind,
    format_element::TextWidth,
};

use self::call_stack::{
    CallStack, FitsCallStack, FitsIndentStack, IndentStack, PrintCallStack, PrintElementArgs,
    PrintIndentStack, StackFrame, SuffixStack,
};
use self::line_suffixes::{LineSuffixEntry, LineSuffixes};
use self::queue::{
    AllPredicate, FitsEndPredicate, FitsQueue, PrintQueue, Queue, SingleEntryPredicate,
};

pub type PrintResult<T> = Result<T, PrintError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Printed {
    code: String,
}

impl Printed {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    /// Access the resulting code, borrowing the result
    pub fn as_code(&self) -> &str {
        &self.code
    }

    /// Access the resulting code, consuming the result
    pub fn into_code(self) -> String {
        self.code
    }
}

/// Prints the format elements into a string
#[derive(Debug, Default)]
pub struct Printer<'a> {
    options: PrinterOptions,
    state: PrinterState<'a>,
}

impl<'a> Printer<'a> {
    pub fn new(options: PrinterOptions) -> Self {
        Self::with_capacity(0, options)
    }

    /// Construct a [`Printer`] with the underlying output buffer pre-sized to
    /// at least `capacity` bytes.
    ///
    /// Use the source-text length as the hint when known — formatted output is
    /// typically within a small factor of the input size, so this eliminates
    /// the doubling-growth reallocations of a multi-megabyte buffer (each of
    /// which copies the full buffer's bytes).
    pub fn with_capacity(capacity: usize, options: PrinterOptions) -> Self {
        let (indent_char, indent_width) = match options.indent_style() {
            IndentStyle::Tab => (code_buffer::IndentChar::Tab, 1),
            IndentStyle::Space => {
                (code_buffer::IndentChar::Space, options.indent_width().value() as usize)
            }
        };
        let buffer = CodeBuffer::with_capacity_and_indent(capacity, indent_char, indent_width);
        Self { options, state: PrinterState::new(buffer) }
    }

    /// Prints the passed in element as well as all its content
    ///
    /// # Errors
    ///
    /// Returns a [`PrintError`] if the document contains invalid or unbalanced elements.
    pub fn print(self, document: &'a [FormatElement<'a>]) -> PrintResult<Printed> {
        self.print_with_indent(document, 0)
    }

    /// Prints the passed in element as well as all its content,
    /// starting at the specified indentation level
    ///
    /// # Errors
    ///
    /// Returns a [`PrintError`] if the document contains invalid or unbalanced elements.
    pub fn print_with_indent(
        mut self,
        document: &'a [FormatElement<'a>],
        indent: u16,
    ) -> PrintResult<Printed> {
        let mut stack = PrintCallStack::new(PrintElementArgs::new());
        let mut queue: PrintQueue<'a> = PrintQueue::new(document);
        let mut indent_stack = PrintIndentStack::new(Indention::Level(indent));

        while let Some(element) = queue.pop() {
            self.print_element(&mut stack, &mut indent_stack, &mut queue, element)?;

            if queue.is_empty() {
                self.flush_line_suffixes(&mut queue, &mut stack, &mut indent_stack, None);
            }
        }

        Ok(Printed::new(self.state.buffer.into_string()))
    }

    /// Prints a single element and push the following elements to queue
    fn print_element(
        &mut self,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
        queue: &mut PrintQueue<'a>,
        element: &'a FormatElement,
    ) -> PrintResult<()> {
        use Tag::{
            EndAlign, EndAlignToColumn, EndConditionalContent, EndDedent, EndEntry, EndFill,
            EndGroup, EndIndent, EndIndentIfGroupBreaks, EndLineSuffix, EndMarkAsRoot, StartAlign,
            StartAlignToColumn, StartConditionalContent, StartDedent, StartEntry, StartFill,
            StartGroup, StartIndent, StartIndentIfGroupBreaks, StartLineSuffix, StartMarkAsRoot,
        };

        let args = stack.top();
        match element {
            FormatElement::Space => {
                if self.state.line_width > 0 {
                    self.state.pending_space = true;
                }
            }

            FormatElement::Token { text } => self.print_text(Text::Token(text)),
            FormatElement::Text { text, width } => {
                self.print_text(Text::Text { text, width: *width });
            }
            FormatElement::Line(line_mode) => {
                if args.mode().is_flat() {
                    match line_mode {
                        LineMode::Soft | LineMode::SoftOrSpace => {
                            if line_mode == &LineMode::SoftOrSpace && self.state.line_width > 0 {
                                self.state.pending_space = true;
                            }
                            return Ok(());
                        }
                        LineMode::Hard | LineMode::Empty | LineMode::Literal => {
                            self.state.measured_group_fits = false;
                        }
                    }
                }

                if self.state.line_suffixes.has_pending() {
                    self.flush_line_suffixes(queue, stack, indent_stack, Some(element));
                    return Ok(());
                }

                if line_mode == &LineMode::Literal {
                    // Prettier's literal line (`doc.literal` in `printDocToString`):
                    // pending indention/space materialize as-is, the newline always prints
                    // (even on an empty line), and the next line starts at the marked root
                    // indention, which stays pending until content claims it.
                    self.flush_pending_indention_and_space();
                    self.print_char('\n');
                    self.state.line_indent = LineIndention::new(indent_stack.root());
                    self.state.has_empty_line = false;
                    return Ok(());
                }

                // Only print a newline if the current line isn't already empty.
                //
                // No end-of-line trimming happens here: pending space and indent never
                // materialize at a line end, and keeping text trimmed is its emitter's job.
                if self.state.line_width > 0 {
                    self.print_char('\n');
                    self.state.has_empty_line = false;
                }

                // Print a second line break if this is an empty line
                if line_mode == &LineMode::Empty && !self.state.has_empty_line {
                    self.print_char('\n');
                    self.state.has_empty_line = true;
                }

                self.state.pending_space = false;
                self.state.line_indent = LineIndention::new(indent_stack.indention());
            }

            FormatElement::ExpandParent => {
                // Handled in `Document::propagate_expands()
            }

            FormatElement::LineSuffixBoundary => {
                const HARD_BREAK: &FormatElement = &FormatElement::Line(LineMode::Hard);
                self.flush_line_suffixes(queue, stack, indent_stack, Some(HARD_BREAK));
            }

            FormatElement::BestFitting(best_fitting) => {
                self.print_best_fitting(best_fitting, queue, stack, indent_stack)?;
            }

            FormatElement::Interned(content) => {
                queue.extend_back(content);
            }

            FormatElement::Tag(StartGroup(group)) => {
                let group_mode = if group.mode().is_flat() {
                    match args.mode() {
                        PrintMode::Flat if self.state.measured_group_fits => {
                            // A parent group has already verified that this group fits on a single line
                            // Thus, just continue in flat mode
                            PrintMode::Flat
                        }
                        // The printer is either in expanded mode or it's necessary to re-measure if the group fits
                        // because the printer printed a line break
                        _ => {
                            self.state.measured_group_fits = true;

                            if let Some(id) = group.id() {
                                self.state.group_modes.insert_print_mode(id, PrintMode::Flat);
                            }

                            // Measure to see if the group fits up on a single line. If that's the case,
                            // print the group in "flat" mode, otherwise continue in expanded mode
                            stack.push(
                                TagKind::Group,
                                args.with_print_mode(PrintMode::Flat)
                                    .with_fits_all_lines(group.fits_all_lines()),
                            );
                            let fits = self.fits(
                                queue,
                                stack,
                                indent_stack,
                                group.mode() == GroupMode::MeasureFlat,
                            )?;
                            stack.pop(TagKind::Group)?;

                            if fits { PrintMode::Flat } else { PrintMode::Expanded }
                        }
                    }
                } else {
                    PrintMode::Expanded
                };

                stack.push(TagKind::Group, args.with_print_mode(group_mode));

                if let Some(id) = group.id() {
                    self.state.group_modes.insert_print_mode(id, group_mode);
                }
            }

            FormatElement::Tag(StartFill) => {
                self.print_fill_entries(queue, stack, indent_stack)?;
            }

            FormatElement::Tag(StartIndent) => {
                indent_stack.indent();
                stack.push(TagKind::Indent, args);
            }

            FormatElement::Tag(StartDedent(mode)) => {
                match mode {
                    DedentMode::Level => indent_stack.start_dedent(),
                    DedentMode::Root => indent_stack.reset_indent(),
                }
                stack.push(TagKind::Dedent, args);
            }

            FormatElement::Tag(StartAlign(align)) => {
                indent_stack.align(align.count());
                stack.push(TagKind::Align, args);
            }

            FormatElement::Tag(StartAlignToColumn) => {
                indent_stack.push(self.state.line_indent.align_to_column(
                    self.state.line_width,
                    self.state.pending_space,
                    self.options.indent_width().value() as usize,
                ));
                stack.push(TagKind::AlignToColumn, args);
            }

            FormatElement::Tag(StartMarkAsRoot) => {
                indent_stack.mark_root();
                stack.push(TagKind::MarkAsRoot, args);
            }

            FormatElement::Tag(tag @ EndMarkAsRoot) => {
                stack.pop(tag.kind())?;
                indent_stack.end_mark_root();
            }

            FormatElement::Tag(StartConditionalContent(Condition { mode, group_id })) => {
                let group_mode = match group_id {
                    None => args.mode(),
                    Some(id) => self.state.group_modes.unwrap_print_mode(*id, element),
                };

                if group_mode == *mode {
                    stack.push(TagKind::ConditionalContent, args);
                } else {
                    queue.skip_content(TagKind::ConditionalContent);
                }
            }

            FormatElement::Tag(StartIndentIfGroupBreaks(group_id, align)) => {
                let group_mode = self.state.group_modes.unwrap_print_mode(*group_id, element);

                if group_mode == PrintMode::Expanded {
                    match align {
                        Some(count) => indent_stack.align(*count),
                        None => indent_stack.indent(),
                    }
                }

                stack.push(TagKind::IndentIfGroupBreaks, args);
            }

            FormatElement::Tag(StartLineSuffix) => {
                indent_stack.push_suffix(indent_stack.indention());
                self.state.line_suffixes.extend(args, queue.iter_content(TagKind::LineSuffix));
            }
            FormatElement::Tag(tag @ (Tag::StartLabelled(_) | StartEntry)) => {
                stack.push(tag.kind(), args);
            }
            FormatElement::Tag(
                tag @ (Tag::EndLabelled | EndEntry | EndGroup | EndConditionalContent | EndFill),
            ) => {
                stack.pop(tag.kind())?;
            }
            FormatElement::Tag(tag @ EndIndentIfGroupBreaks(group_id)) => {
                if self.state.group_modes.unwrap_print_mode(*group_id, element)
                    == PrintMode::Expanded
                {
                    indent_stack.pop();
                }
                stack.pop(tag.kind())?;
            }
            FormatElement::Tag(tag @ (EndIndent | EndAlign | EndAlignToColumn | EndLineSuffix)) => {
                stack.pop(tag.kind())?;
                indent_stack.pop();
            }
            FormatElement::Tag(tag @ EndDedent(mode)) => {
                match mode {
                    DedentMode::Level => indent_stack.end_dedent(),
                    DedentMode::Root => indent_stack.pop(),
                }
                stack.pop(tag.kind())?;
            }
        }

        Ok(())
    }

    fn fits(
        &mut self,
        queue: &PrintQueue<'a>,
        stack: &PrintCallStack,
        indent_stack: &PrintIndentStack,
        allow_fallback_breaks: bool,
    ) -> PrintResult<bool> {
        let mut measure = FitsMeasurer::new(queue, stack, indent_stack, self);
        measure.allow_fallback_breaks = allow_fallback_breaks;
        let result = measure.fits(&mut AllPredicate);
        measure.finish();
        result
    }

    fn flush_line_suffixes(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
        line_break: Option<&'a FormatElement>,
    ) {
        let suffixes = self.state.line_suffixes.take_pending();

        if suffixes.len() > 0 {
            // Print this line break element again once all the line suffixes have been flushed
            if let Some(line_break) = line_break {
                queue.push(line_break);
            }
            indent_stack.flush_suffixes();
            for entry in suffixes.rev() {
                match entry {
                    LineSuffixEntry::Suffix(suffix) => {
                        queue.push(suffix);
                    }
                    LineSuffixEntry::Args(args) => {
                        const LINE_SUFFIX_END: &FormatElement =
                            &FormatElement::Tag(Tag::EndLineSuffix);
                        stack.push(TagKind::LineSuffix, args);
                        queue.push(LINE_SUFFIX_END);
                    }
                }
            }
        }
    }

    fn print_best_fitting(
        &mut self,
        best_fitting: &'a BestFittingElement,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
    ) -> PrintResult<()> {
        let args = stack.top();

        if args.mode().is_flat() && self.state.measured_group_fits {
            queue.extend_back(best_fitting.most_flat());
            self.print_entry(queue, stack, indent_stack, args)
        } else {
            self.state.measured_group_fits = true;

            let (most_expanded, remaining_variants) =
                best_fitting.split_to_most_expanded_and_flat_variants();

            for variant in remaining_variants {
                // Test if this variant fits and if so, use it. Otherwise try the next
                // variant.

                // Try to fit only the first variant on a single line
                if !matches!(variant.first(), Some(&FormatElement::Tag(Tag::StartEntry))) {
                    return invalid_start_tag(TagKind::Entry, variant.first());
                }

                let entry_args = args.with_print_mode(PrintMode::Flat);

                // Skip the first element because we want to override the args for the entry and the
                // args must be popped from the stack as soon as it sees the matching end entry.
                let content = &variant[1..];

                queue.extend_back(content);
                stack.push(TagKind::Entry, entry_args);
                let variant_fits = self.fits(queue, stack, indent_stack, false)?;
                stack.pop(TagKind::Entry)?;

                // Remove the content slice because printing needs the variant WITH the start entry
                let popped_slice = queue.pop_slice();
                debug_assert_eq!(popped_slice, Some(content));

                if variant_fits {
                    queue.extend_back(variant);
                    return self.print_entry(queue, stack, indent_stack, entry_args);
                }
            }

            // No variant fits, take the last (most expanded) as fallback
            queue.extend_back(most_expanded);
            self.print_entry(queue, stack, indent_stack, args.with_print_mode(PrintMode::Expanded))
        }
    }

    /// Tries to fit as much content as possible on a single line.
    ///
    /// `Fill` is a sequence of *item*, *separator*, *item*, *separator*, *item*, ... entries.
    /// The goal is to fit as many items (with their separators) on a single line as possible and
    /// first expand the *separator* if the content exceeds the print width and only fallback to expanding
    /// the *item*s if the *item* or the *item* and the expanded *separator* don't fit on the line.
    ///
    /// The implementation handles the following 5 cases:
    ///
    /// * The *item*, *separator*, and the *next item* fit on the same line.
    ///   Print the *item* and *separator* in flat mode.
    /// * The *item* and *separator* fit on the line but there's not enough space for the *next item*.
    ///   Print the *item* in flat mode and the *separator* in expanded mode.
    /// * The *item* fits on the line but the *separator* does not in flat mode.
    ///   Print the *item* in flat mode and the *separator* in expanded mode.
    /// * The *item* fits on the line but the *separator* does not in flat **NOR** expanded mode.
    ///   Print the *item* and *separator* in expanded mode.
    /// * The *item* does not fit on the line.
    ///   Print the *item* and *separator* in expanded mode.
    fn print_fill_entries(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
    ) -> PrintResult<()> {
        let args = stack.top();

        // It's already known that the content fit, print all items in flat mode.
        if self.state.measured_group_fits && args.mode().is_flat() {
            stack.push(TagKind::Fill, args.with_print_mode(PrintMode::Flat));
            return Ok(());
        }

        stack.push(TagKind::Fill, args);

        while matches!(queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
            let mut measurer = FitsMeasurer::new_flat(queue, stack, indent_stack, self);

            // The number of item/separator pairs that fit on the same line.
            let mut flat_pairs = 0usize;
            let mut item_fits = measurer.fill_item_fits()?;

            let last_pair_layout = if item_fits {
                // Measure the remaining pairs until the first item or separator that does not fit (or the end of the fill element).
                // Optimisation to avoid re-measuring the next-item twice:
                // * Once when measuring if the *item*, *separator*, *next-item* fit
                // * A second time when measuring if *next-item*, *separator*, *next-next-item* fit.
                loop {
                    // Item that fits without a following separator.
                    if !matches!(measurer.queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
                        // Attached closing syntax shares the last item's line.
                        break if measurer.queue.top() != Some(&FormatElement::Tag(Tag::EndFill))
                            || measurer.fill_tail_fits()?
                        {
                            FillPairLayout::Flat
                        } else {
                            FillPairLayout::Expanded
                        };
                    }

                    let separator_fits = measurer.fill_separator_fits(PrintMode::Flat)?;

                    // Item fits but the flat separator does not.
                    if !separator_fits {
                        break FillPairLayout::ItemMaybeFlat;
                    }

                    // Last item/separator pair that both fit
                    if !matches!(measurer.queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
                        break FillPairLayout::Flat;
                    }

                    item_fits = measurer.fill_item_fits()?;

                    if item_fits && measurer.queue.top() == Some(&FormatElement::Tag(Tag::EndFill))
                    {
                        if measurer.fill_tail_fits()? {
                            flat_pairs += 1;
                            break FillPairLayout::Flat;
                        }
                        break FillPairLayout::ItemFlatSeparatorExpanded;
                    }

                    if item_fits {
                        flat_pairs += 1;
                    } else {
                        // Item and separator both fit, but the next element doesn't.
                        // Print the separator in expanded mode and then re-measure if the item now
                        // fits in the next iteration of the outer loop.
                        break FillPairLayout::ItemFlatSeparatorExpanded;
                    }
                }
            } else {
                // Neither item nor separator fit, print both in expanded mode.
                FillPairLayout::Expanded
            };

            measurer.finish();

            // Print all pairs that fit in flat mode.
            for _ in 0..flat_pairs {
                self.print_fill_item(
                    queue,
                    stack,
                    indent_stack,
                    args.with_print_mode(PrintMode::Flat),
                )?;
                self.print_fill_separator(
                    queue,
                    stack,
                    indent_stack,
                    args.with_print_mode(PrintMode::Flat),
                )?;
            }

            let item_mode = match last_pair_layout {
                FillPairLayout::Flat | FillPairLayout::ItemFlatSeparatorExpanded => PrintMode::Flat,
                FillPairLayout::Expanded => PrintMode::Expanded,
                FillPairLayout::ItemMaybeFlat => {
                    let mut measurer = FitsMeasurer::new_flat(queue, stack, indent_stack, self);
                    // SAFETY: That the item fits is guaranteed by `ItemMaybeFlat`.
                    // Re-measuring is required to get the measurer in the correct state for measuring the separator.
                    assert!(measurer.fill_item_fits()?);
                    let separator_fits = measurer.fill_separator_fits(PrintMode::Expanded)?;
                    measurer.finish();

                    if separator_fits { PrintMode::Flat } else { PrintMode::Expanded }
                }
            };

            self.print_fill_item(queue, stack, indent_stack, args.with_print_mode(item_mode))?;

            if matches!(queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
                let separator_mode = match last_pair_layout {
                    FillPairLayout::Flat => PrintMode::Flat,
                    FillPairLayout::ItemFlatSeparatorExpanded
                    | FillPairLayout::Expanded
                    | FillPairLayout::ItemMaybeFlat => PrintMode::Expanded,
                };

                // Push a new stack frame with print mode `Flat` for the case where the separator gets printed in expanded mode
                // but does contain a group to ensure that the group will measure "fits" with the "flat" versions of the next item/separator.
                stack.push(TagKind::Fill, args.with_print_mode(PrintMode::Flat));
                self.print_fill_separator(
                    queue,
                    stack,
                    indent_stack,
                    args.with_print_mode(separator_mode),
                )?;
                stack.pop(TagKind::Fill)?;
            }
        }

        if queue.top() == Some(&FormatElement::Tag(Tag::EndFill)) {
            Ok(())
        } else {
            invalid_end_tag(TagKind::Fill, stack.top_kind())
        }
    }

    /// Semantic alias for [Self::print_entry] for fill items.
    fn print_fill_item(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
        args: PrintElementArgs,
    ) -> PrintResult<()> {
        self.print_entry(queue, stack, indent_stack, args)
    }

    /// Semantic alias for [Self::print_entry] for fill separators.
    fn print_fill_separator(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
        args: PrintElementArgs,
    ) -> PrintResult<()> {
        self.print_entry(queue, stack, indent_stack, args)
    }

    /// Fully print an element (print the element itself and all its descendants)
    ///
    /// Unlike [`Self::print_element`], this function ensures the entire element has
    /// been printed when it returns and the queue is back to its original state
    fn print_entry(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        indent_stack: &mut PrintIndentStack,
        args: PrintElementArgs,
    ) -> PrintResult<()> {
        let start_entry = queue.top();

        if !matches!(start_entry, Some(&FormatElement::Tag(Tag::StartEntry))) {
            return invalid_start_tag(TagKind::Entry, start_entry);
        }

        let mut depth = 0;

        while let Some(element) = queue.pop() {
            match element {
                FormatElement::Tag(Tag::StartEntry) => {
                    // Handle the start of the first element by pushing the args on the stack.
                    if depth == 0 {
                        depth = 1;
                        stack.push(TagKind::Entry, args);
                        continue;
                    }

                    depth += 1;
                }
                FormatElement::Tag(Tag::EndEntry) => {
                    depth -= 1;
                    // Reached the end entry, pop the entry from the stack and return.
                    if depth == 0 {
                        stack.pop(TagKind::Entry)?;
                        return Ok(());
                    }
                }
                _ => {
                    // Fall through
                }
            }

            self.print_element(stack, indent_stack, queue, element)?;
        }

        invalid_end_tag(TagKind::Entry, stack.top_kind())
    }

    /// Prints `indention` (indent levels + align spaces) and adds its width to the current line.
    #[inline]
    fn print_indention(&mut self, indention: Indention) {
        if indention.is_empty() {
            return;
        }

        let level = indention.level() as usize;
        self.state.buffer.print_indent(level);
        self.state.line_width += level * self.options.indent_width().value() as usize;

        let align_count = indention.align() as usize;
        for _ in 0..align_count {
            // SAFETY: `' '` is an valid ASCII character
            unsafe {
                self.state.buffer.print_byte_unchecked(b' ');
            }
        }
        self.state.line_width += align_count;
    }

    /// Materializes the pending indention and pending space before printing visible content.
    #[inline]
    fn flush_pending_indention_and_space(&mut self) {
        let pending_indent = std::mem::take(&mut self.state.line_indent.pending);
        self.print_indention(pending_indent);

        if self.state.pending_space {
            // SAFETY: `' '` is an valid ASCII character
            unsafe {
                self.state.buffer.print_byte_unchecked(b' ');
            }
            self.state.pending_space = false;
            self.state.line_width += 1;
        }
    }

    fn print_text(&mut self, text: Text) {
        self.flush_pending_indention_and_space();

        match text {
            Text::Token(text) => {
                self.state.buffer.print_str_short(text);
                self.state.line_width += text.len();
            }
            Text::Text { text, width } => {
                if width.is_multiline() {
                    let line_break_position = text.find('\n').unwrap_or(text.len());
                    let (first_line, remaining) = text.split_at(line_break_position);
                    self.state.buffer.print_str_short(first_line);
                    self.state.line_width += width.value() as usize;
                    // Print the remaining lines
                    for char in remaining.chars() {
                        self.print_char(char);
                    }
                } else {
                    self.state.buffer.print_str_short(text);
                    self.state.line_width += width.value() as usize;
                }
            }
        }

        self.state.has_empty_line = false;
    }

    fn print_char(&mut self, char: char) {
        if char == '\n' {
            // SAFETY: `line_ending` is one of `\n`, `\r\n` or `\r`, all valid ASCII sequences
            unsafe {
                self.state.buffer.print_bytes_unchecked(self.options.line_ending.as_bytes());
            }

            self.state.line_width = 0;
            self.state.line_indent = LineIndention::default();
        } else {
            let char_width = if char == '\t' {
                // SAFETY: `'\t'` is an valid ASCII character
                unsafe {
                    self.state.buffer.print_byte_unchecked(b'\t');
                }
                self.options.indent_width().value() as usize
            } else {
                self.state.buffer.print_char(char);
                char.width().unwrap_or(0)
            };

            self.state.line_width += char_width;
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum FillPairLayout {
    /// The item, separator, and next item fit. Print the first item and the separator in flat mode.
    Flat,

    /// The item and separator fit but the next element does not. Print the item in flat mode and
    /// the separator in expanded mode.
    ItemFlatSeparatorExpanded,

    /// The item does not fit. Print the item and any potential separator in expanded mode.
    Expanded,

    /// The item fits but the separator does not in flat mode. If the separator fits in expanded mode then
    /// print the item in flat and the separator in expanded mode, otherwise print both in expanded mode.
    ItemMaybeFlat,
}

/// Printer state that is global to all elements.
/// Stores the result of the print operation (buffer and mappings) and at what
/// position the printer currently is.
#[derive(Default, Debug)]
struct PrinterState<'a> {
    buffer: CodeBuffer,
    line_indent: LineIndention,
    pending_space: bool,
    measured_group_fits: bool,
    line_width: usize,
    has_empty_line: bool,
    line_suffixes: LineSuffixes<'a>,
    group_modes: GroupModes,
    // Re-used queue to measure if a group fits. Optimisation to avoid re-allocating a new
    // vec everytime a group gets measured
    fits_stack: Vec<StackFrame>,
    fits_indent_stack: Vec<Indention>,
    fits_stack_tem_indent: Vec<Indention>,
    fits_root_indent_stack: Vec<Indention>,
    fits_queue: Vec<&'a [FormatElement<'a>]>,
}

impl PrinterState<'_> {
    pub fn new(buffer: CodeBuffer) -> Self {
        Self {
            buffer,
            // Initialize `measured_group_fits` to true to indicate that groups are initially assumed to fit.
            measured_group_fits: true,
            ..Default::default()
        }
    }
}
/// Tracks the mode in which groups with ids are printed. Stores the groups at `group.id()` index.
/// This is based on the assumption that the group ids for a single document are dense.
#[derive(Debug, Default)]
struct GroupModes(Vec<Option<PrintMode>>);

impl GroupModes {
    fn insert_print_mode(&mut self, group_id: GroupId, mode: PrintMode) {
        let index = u32::from(group_id) as usize;

        if self.0.len() <= index {
            self.0.resize(index + 1, None);
        }

        self.0[index] = Some(mode);
    }

    fn get_print_mode(&self, group_id: GroupId) -> Option<PrintMode> {
        let index = u32::from(group_id) as usize;
        self.0.get(index).and_then(|option| option.as_ref().copied())
    }

    fn unwrap_print_mode(&self, group_id: GroupId, next_element: &FormatElement) -> PrintMode {
        self.get_print_mode(group_id).unwrap_or_else(|| {
            panic!("Expected group with id {group_id:?} to exist but it wasn't present in the document. Ensure that a group with such a document appears in the document before the element {next_element:?}.")
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Indention {
    /// Indent the content by `count` levels by using the indention sequence specified by the printer options.
    Level(u16),

    /// Indent the content by n-`level`s using the indention sequence specified by the printer options and `align` spaces.
    Align { level: u16, align: NonZeroU16 },
}

impl Indention {
    const fn is_empty(self) -> bool {
        matches!(self, Indention::Level(0))
    }

    /// Creates a new indention level with a zero-indent.
    const fn new() -> Self {
        Indention::Level(0)
    }

    /// Returns the indention level
    fn level(self) -> u16 {
        match self {
            Indention::Level(count) => count,
            Indention::Align { level: indent, .. } => indent,
        }
    }

    /// Returns the number of trailing align spaces or 0 if none
    fn align(self) -> u16 {
        match self {
            Indention::Level(_) => 0,
            Indention::Align { align, .. } => align.into(),
        }
    }

    /// The number of columns the indention takes, counting an indent level as `indent_width`.
    fn width(self, indent_width: usize) -> usize {
        self.level() as usize * indent_width + self.align() as usize
    }

    /// Increments the level by one and keeps the alignment, which prints as spaces in tab mode too.
    fn increment_level(self) -> Self {
        match self {
            Indention::Level(count) => Indention::Level(count + 1),
            Indention::Align { level, align } => Indention::Align { level: level + 1, align },
        }
    }

    /// Adds an `align` of `count` spaces to the current indention.
    ///
    /// If the current value is [`Indention::Align`], this preserves the level and adds `count` to
    /// the trailing alignment.
    fn set_align(self, count: NonZeroU8) -> Self {
        match self {
            Indention::Level(indent_count) => {
                Indention::Align { level: indent_count, align: count.into() }
            }

            Indention::Align { level: indent, align } => {
                Indention::Align { level: indent, align: align.saturating_add(count.get().into()) }
            }
        }
    }
}

impl Default for Indention {
    fn default() -> Self {
        Indention::new()
    }
}

/// Keeps the line's structural indent after the pending whitespace has been consumed.
#[derive(Copy, Clone, Debug, Default)]
struct LineIndention {
    pending: Indention,
    level: u16,
}

impl LineIndention {
    fn new(pending: Indention) -> Self {
        Self { pending, level: pending.level() }
    }

    fn align_to_column(
        self,
        line_width: usize,
        pending_space: bool,
        indent_width: usize,
    ) -> Indention {
        let column = line_width + self.pending.width(indent_width) + usize::from(pending_space);
        let spaces = column - usize::from(self.level) * indent_width;
        match NonZeroU16::new(u16::try_from(spaces).unwrap_or(u16::MAX)) {
            Some(align) => Indention::Align { level: self.level, align },
            None => Indention::Level(self.level),
        }
    }
}

#[must_use = "FitsMeasurer must be finished."]
struct FitsMeasurer<'a, 'print> {
    state: FitsState,
    queue: FitsQueue<'a, 'print>,
    stack: FitsCallStack<'print>,
    indent_stack: FitsIndentStack<'print>,
    printer: &'print mut Printer<'a>,
    must_be_flat: bool,
    allow_fallback_breaks: bool,
}

impl<'a, 'print> FitsMeasurer<'a, 'print> {
    fn new_flat(
        print_queue: &'print PrintQueue<'a>,
        print_stack: &'print PrintCallStack,
        print_indent_stack: &'print PrintIndentStack,
        printer: &'print mut Printer<'a>,
    ) -> Self {
        let mut measurer = Self::new(print_queue, print_stack, print_indent_stack, printer);
        measurer.must_be_flat = true;
        measurer
    }

    fn new(
        print_queue: &'print PrintQueue<'a>,
        print_stack: &'print PrintCallStack,
        print_indent_stack: &'print PrintIndentStack,
        printer: &'print mut Printer<'a>,
    ) -> Self {
        let saved_stack = std::mem::take(&mut printer.state.fits_stack);
        let saved_queue = std::mem::take(&mut printer.state.fits_queue);
        let saved_indent_stack = std::mem::take(&mut printer.state.fits_indent_stack);
        let saved_stack_tem_indent = std::mem::take(&mut printer.state.fits_stack_tem_indent);
        let saved_root_indent_stack = std::mem::take(&mut printer.state.fits_root_indent_stack);
        debug_assert_eq!(saved_stack.len(), 0);
        debug_assert_eq!(saved_queue.len(), 0);
        debug_assert_eq!(saved_indent_stack.len(), 0);
        debug_assert_eq!(saved_stack_tem_indent.len(), 0);
        debug_assert_eq!(saved_root_indent_stack.len(), 0);

        let fits_queue = FitsQueue::new(print_queue, saved_queue);
        let fits_stack = FitsCallStack::new(print_stack, saved_stack);

        let fits_indent_stack = FitsIndentStack::new(
            print_indent_stack,
            saved_indent_stack,
            saved_stack_tem_indent,
            saved_root_indent_stack,
        );

        let fits_state = FitsState {
            line_indent: printer.state.line_indent,
            pending_space: printer.state.pending_space,
            line_width: printer.state.line_width,
            has_line_suffix: printer.state.line_suffixes.has_pending(),
            line_suffix_after_content: false,
            skips_lines: false,
        };

        Self {
            state: fits_state,
            queue: fits_queue,
            stack: fits_stack,
            indent_stack: fits_indent_stack,
            must_be_flat: false,
            allow_fallback_breaks: false,
            printer,
        }
    }

    /// Tests if it's possible to print the content of the queue up to the first hard line break
    /// or the end of the document on a single line without exceeding the line width.
    fn fits<P>(&mut self, predicate: &mut P) -> PrintResult<bool>
    where
        P: FitsEndPredicate,
    {
        while let Some(element) = self.queue.pop() {
            match self.fits_element(element)? {
                Fits::Yes => return Ok(true),
                Fits::No => return Ok(false),
                Fits::Maybe => {
                    if predicate.is_end(element)? {
                        break;
                    }
                }
            }
        }

        Ok(true)
    }

    /// Tests if the content of a `Fill` item fits in [PrintMode::Flat].
    ///
    /// Returns `Err` if the top element of the queue is not a [Tag::StartEntry]
    /// or if the document has any mismatching start/end tags.
    fn fill_item_fits(&mut self) -> PrintResult<bool> {
        self.fill_entry_fits(PrintMode::Flat)
    }

    /// Tests if the content of a `Fill` separator fits with `mode`.
    ///
    /// Returns `Err` if the top element of the queue is not a [Tag::StartEntry]
    /// or if the document has any mismatching start/end tags.
    fn fill_separator_fits(&mut self, mode: PrintMode) -> PrintResult<bool> {
        self.fill_entry_fits(mode)
    }

    fn fill_tail_fits(&mut self) -> PrintResult<bool> {
        // Measure attached syntax in its own mode once the last entry fits.
        self.must_be_flat = false;
        self.fits(&mut AllPredicate)
    }

    /// Tests if the elements between the [Tag::StartEntry] and [Tag::EndEntry]
    /// of a fill item or separator fits with `mode`.
    ///
    /// Returns `Err` if the queue isn't positioned at a [Tag::StartEntry] or if
    /// the matching [Tag::EndEntry] is missing.
    fn fill_entry_fits(&mut self, mode: PrintMode) -> PrintResult<bool> {
        let start_entry = self.queue.top();

        if !matches!(start_entry, Some(&FormatElement::Tag(Tag::StartEntry))) {
            return invalid_start_tag(TagKind::Entry, start_entry);
        }

        // Measure complete entries so the next entry starts at its printed column.
        self.stack
            .push(TagKind::Fill, self.stack.top().with_print_mode(mode).with_fits_all_lines(true));
        let mut predicate = SingleEntryPredicate::default();
        let fits = self.fits(&mut predicate)?;

        if predicate.is_done() {
            self.stack.pop(TagKind::Fill)?;
        }

        Ok(fits)
    }

    /// Tests if the passed element fits on the current line or not.
    // LLVM considers this dispatcher too large to inline heuristically, but the fitting loop calls
    // it for every visited format element.
    #[inline(always)]
    fn fits_element(&mut self, element: &'a FormatElement) -> PrintResult<Fits> {
        use Tag::{
            EndAlign, EndAlignToColumn, EndConditionalContent, EndDedent, EndEntry, EndFill,
            EndGroup, EndIndent, EndIndentIfGroupBreaks, EndLineSuffix, EndMarkAsRoot, StartAlign,
            StartAlignToColumn, StartConditionalContent, StartDedent, StartEntry, StartFill,
            StartGroup, StartIndent, StartIndentIfGroupBreaks, StartLineSuffix, StartMarkAsRoot,
        };

        let args = self.stack.top();

        match element {
            FormatElement::Space => {
                if self.state.line_width > 0 {
                    self.state.pending_space = true;
                }
            }
            FormatElement::Line(line_mode) => {
                if args.mode().is_flat() {
                    match line_mode {
                        LineMode::SoftOrSpace => {
                            self.state.pending_space = true;
                        }
                        LineMode::Soft => {}
                        LineMode::Hard | LineMode::Empty if args.fits_all_lines() => {
                            self.state.line_width = 0;
                            self.state.pending_space = false;
                            self.state.line_indent =
                                LineIndention::new(self.indent_stack.indention());
                        }
                        LineMode::Literal if args.fits_all_lines() => {
                            // A literal line keeps pending whitespace, and the next line starts at
                            // the root indent.
                            if self.fits_text(Text::Token("")) == Fits::No {
                                return Ok(Fits::No);
                            }
                            self.state.line_width = 0;
                            self.state.line_indent = LineIndention::new(self.indent_stack.root());
                            return Ok(self.fits_text(Text::Token("")));
                        }
                        LineMode::Hard | LineMode::Empty | LineMode::Literal => {
                            // Even in flat mode, content that _directly_ contains a hard or empty
                            // line is considered to fit when a hard break is reached, since that
                            // break is always going to exist, regardless of the print mode.
                            // This is particularly important for `Fill` entries, where _ungrouped_
                            // content that contains hard breaks shouldn't force the surrounding
                            // elements to also expand. Example:
                            //   [
                            //     -1, -2, 3
                            //     // leading comment
                            //     -4, -5, -6
                            //   ]
                            // Here, `-4` contains a hardline because of the leading comment, but that
                            // doesn't cause the element (`-4`) nor the separator (`,`) to print in
                            // expanded mode, allowing the rest of the elements to fill in. If this
                            // _did_ respect `must_be_flat` and returned `Fits::No` instead, the result
                            // would put the `-4` on its own line, which is not preferable (at least,
                            // it doesn't match Prettier):
                            //   [
                            //     -1, -2, 3
                            //     // leading comment
                            //     -4,
                            //     -5, -6
                            //   ]
                            // The perception here is that most comments inline for fills are used to
                            // separate _groups_ rather than to single out an individual element.
                            //
                            // The alternative case is when the fill entry is grouped, in which case
                            // this fit returns `Yes`, but the parent group is already known to
                            // expand _because_ of this hard line, and so the fill entry and separator
                            // are automatically printed in expanded mode anyway, and this fit check
                            // has no bearing on that (so it doesn't need to care about flat or not):
                            //   <div>
                            //     <span a b>
                            //       <Foo />
                            //     </span>{" "}
                            //     ({variable})
                            //   </div>
                            // Here the `<span>...</span>` _is_ grouped and contains a hardline, so it
                            // is known to break and _not_ fit already because the check is performed
                            // on the group. But within the group itself, the content with hardlines
                            // (the `\n<Foo />\n`) _does_ fit, for the same logic in the first case.
                            return Ok(Fits::Yes);
                        }
                    }
                } else {
                    // Reachable if the restQueue contains an element with mode expanded because Expanded
                    // is what the mode's initialized to by default
                    // This means, the printer is outside of the current element at this point and any
                    // line break should be printed as regular line break -> Fits
                    //
                    // Before returning, resolve any pending space so the width check is accurate.
                    // Without this, a deferred `Space` before a group boundary can be lost when
                    // the fits measurer exits early via an expanded-mode line break, causing the
                    // measured width to be off by one.
                    if self.state.pending_space {
                        self.state.line_width += 1;
                        if self.exceeds_width() {
                            return Ok(Fits::No);
                        }
                    }
                    if !args.fits_last_line() {
                        return Ok(Fits::Yes);
                    }
                    self.state.has_line_suffix = false;
                    self.state.line_suffix_after_content = false;
                    let indent = if *line_mode == LineMode::Literal {
                        self.indent_stack.root()
                    } else {
                        self.indent_stack.indention()
                    };
                    self.skip_line(indent);
                }
            }

            FormatElement::Token { text } => {
                return Ok(self.fits_text(Text::Token(text)));
            }
            FormatElement::Text { text, width } => {
                return Ok(self.fits_text(Text::Text { text, width: *width }));
            }

            FormatElement::LineSuffixBoundary => {
                if self.state.has_line_suffix {
                    if !args.mode().is_flat() && self.state.line_suffix_after_content {
                        // No break in the measured content can print the suffix sooner, and it
                        // ends the line here as a hard break does.
                        return self.fits_element(&FormatElement::Line(LineMode::Hard));
                    }
                    if !self.state.skips_lines {
                        // The boundary will end the current line even without a fallback break.
                        return Ok(if self.allow_fallback_breaks { Fits::Yes } else { Fits::No });
                    }
                    // The pending suffix prints here, followed by a line break.
                    self.state.has_line_suffix = false;
                    self.state.line_suffix_after_content = false;
                    self.skip_line(self.indent_stack.indention());
                }
            }

            FormatElement::ExpandParent => {
                if self.must_be_flat {
                    return Ok(Fits::No);
                }
            }

            FormatElement::BestFitting(best_fitting) => {
                let slice = match args.mode() {
                    PrintMode::Flat => best_fitting.most_flat(),
                    PrintMode::Expanded => best_fitting.most_expanded(),
                };

                if !matches!(slice.first(), Some(FormatElement::Tag(Tag::StartEntry))) {
                    return invalid_start_tag(TagKind::Entry, slice.first());
                }

                self.queue.extend_back(slice);
            }

            FormatElement::Interned(content) => self.queue.extend_back(content),

            FormatElement::Tag(StartIndent) => {
                self.indent_stack.indent();
                self.stack.push(TagKind::Indent, args);
            }

            FormatElement::Tag(StartDedent(mode)) => {
                match mode {
                    DedentMode::Level => self.indent_stack.start_dedent(),
                    DedentMode::Root => self.indent_stack.reset_indent(),
                }
                self.stack.push(TagKind::Dedent, args);
            }

            FormatElement::Tag(StartAlign(align)) => {
                self.indent_stack.align(align.count());
                self.stack.push(TagKind::Align, args);
            }

            FormatElement::Tag(StartAlignToColumn) => {
                self.indent_stack.push(self.state.line_indent.align_to_column(
                    self.state.line_width,
                    self.state.pending_space,
                    self.options().indent_width().value() as usize,
                ));
                self.stack.push(TagKind::AlignToColumn, args);
            }

            FormatElement::Tag(StartGroup(group)) => {
                if self.must_be_flat && !group.mode().is_flat() {
                    return Ok(Fits::No);
                }

                let group_mode = match group.mode() {
                    GroupMode::MeasureFlat if !self.allow_fallback_breaks => PrintMode::Flat,
                    GroupMode::Flat | GroupMode::MeasureFlat => args.mode(),
                    GroupMode::Expand | GroupMode::Propagated => PrintMode::Expanded,
                };

                self.stack.push(
                    TagKind::Group,
                    args.with_print_mode(group_mode)
                        .with_fits_all_lines(group.fits_all_lines())
                        .with_fits_last_line(group.fits_last_line()),
                );

                if let Some(id) = group.id() {
                    self.group_modes_mut().insert_print_mode(id, group_mode);
                }
            }

            FormatElement::Tag(StartConditionalContent(condition)) => {
                let group_mode = match condition.group_id {
                    None => args.mode(),
                    Some(group_id) => {
                        self.group_modes().get_print_mode(group_id).unwrap_or_else(|| args.mode())
                    }
                };

                if group_mode == condition.mode {
                    self.stack.push(TagKind::ConditionalContent, args);
                } else {
                    self.queue.skip_content(TagKind::ConditionalContent);
                }
            }

            FormatElement::Tag(StartIndentIfGroupBreaks(id, align)) => {
                let group_mode =
                    self.group_modes().get_print_mode(*id).unwrap_or_else(|| args.mode());

                match group_mode {
                    PrintMode::Flat => {
                        self.stack.push(TagKind::IndentIfGroupBreaks, args);
                    }
                    PrintMode::Expanded => {
                        match align {
                            Some(count) => self.indent_stack.align(*count),
                            None => self.indent_stack.indent(),
                        }
                        self.stack.push(TagKind::IndentIfGroupBreaks, args);
                    }
                }
            }

            FormatElement::Tag(StartLineSuffix) => {
                self.queue.skip_content(TagKind::LineSuffix);
                if !self.state.has_line_suffix {
                    self.state.line_suffix_after_content = !args.mode().is_flat();
                }
                self.state.has_line_suffix = true;
            }

            FormatElement::Tag(tag @ (Tag::StartLabelled(_) | StartFill | StartEntry)) => {
                self.stack.push(tag.kind(), args);
            }
            FormatElement::Tag(EndGroup) => {
                let group_args = self.stack.pop(TagKind::Group)?;
                // The group's last line continues after it.
                if group_args.fits_last_line() && !self.stack.top().fits_last_line() {
                    self.state.skips_lines = false;
                    if self.exceeds_width() {
                        return Ok(Fits::No);
                    }
                }
            }
            FormatElement::Tag(
                tag @ (Tag::EndLabelled | EndEntry | EndConditionalContent | EndFill),
            ) => {
                self.stack.pop(tag.kind())?;
            }
            FormatElement::Tag(tag @ EndIndentIfGroupBreaks(group_id)) => {
                let group_mode =
                    self.group_modes().get_print_mode(*group_id).unwrap_or_else(|| args.mode());
                if group_mode == PrintMode::Expanded {
                    self.indent_stack.pop();
                }
                self.stack.pop(tag.kind())?;
            }
            // A skipped suffix hides its end, so `EndLineSuffix` closes a flushed
            // suffix whose content holds a group.
            FormatElement::Tag(tag @ (EndIndent | EndAlign | EndAlignToColumn | EndLineSuffix)) => {
                self.stack.pop(tag.kind())?;
                self.indent_stack.pop();
            }
            FormatElement::Tag(StartMarkAsRoot) => {
                self.indent_stack.mark_root();
                self.stack.push(TagKind::MarkAsRoot, args);
            }
            FormatElement::Tag(tag @ EndMarkAsRoot) => {
                self.stack.pop(tag.kind())?;
                self.indent_stack.end_mark_root();
            }
            FormatElement::Tag(tag @ EndDedent(mode)) => {
                if matches!(mode, DedentMode::Level) {
                    self.indent_stack.end_dedent();
                }
                self.stack.pop(tag.kind())?;
            }
        }

        Ok(Fits::Maybe)
    }

    fn fits_text(&mut self, text: Text) -> Fits {
        let indent = std::mem::take(&mut self.state.line_indent.pending);
        self.state.line_width += indent.width(self.options().indent_width().value() as usize);

        if self.state.pending_space {
            self.state.line_width += 1;
        }

        match text {
            Text::Token(text) => {
                self.state.line_width += text.len();
            }
            Text::Text { text, width } => {
                if width.is_multiline() {
                    if self.state.skips_lines {
                        let last_line = &text[text.rfind('\n').map_or(0, |end| end + 1)..];
                        self.state.line_width =
                            TextWidth::from_text(last_line, self.options().indent_width()).value()
                                as usize;
                        self.state.line_indent = LineIndention::default();
                        self.state.pending_space = false;
                        return Fits::Maybe;
                    }
                    return if self.must_be_flat
                        || self.state.line_width + width.value() as usize
                            > usize::from(self.options().print_width)
                    {
                        Fits::No
                    } else {
                        Fits::Yes
                    };
                }

                self.state.line_width += width.value() as usize;
            }
        }

        if self.exceeds_width() {
            return Fits::No;
        }

        self.state.pending_space = false;

        Fits::Maybe
    }

    /// Whether the current line is over the width and is measured.
    fn exceeds_width(&self) -> bool {
        !self.state.skips_lines && self.state.line_width > usize::from(self.options().print_width)
    }

    /// Starts a line that counts only if the `fits_last_line` group ends on it.
    fn skip_line(&mut self, indent: Indention) {
        self.state.skips_lines = true;
        self.state.line_width = 0;
        self.state.pending_space = false;
        self.state.line_indent = LineIndention::new(indent);
    }

    fn finish(self) {
        let mut queue = self.queue.finish();
        queue.clear();
        self.printer.state.fits_queue = queue;

        let mut stack = self.stack.finish();
        stack.clear();
        self.printer.state.fits_stack = stack;

        let (mut indent_stack, mut history_stack, mut root_indent_stack) =
            self.indent_stack.finish();
        indent_stack.clear();
        self.printer.state.fits_indent_stack = indent_stack;
        history_stack.clear();
        self.printer.state.fits_stack_tem_indent = history_stack;
        root_indent_stack.clear();
        self.printer.state.fits_root_indent_stack = root_indent_stack;
    }

    fn options(&self) -> &PrinterOptions {
        &self.printer.options
    }

    fn group_modes(&self) -> &GroupModes {
        &self.printer.state.group_modes
    }

    fn group_modes_mut(&mut self) -> &mut GroupModes {
        &mut self.printer.state.group_modes
    }
}

#[cold]
fn invalid_end_tag<R>(end_tag: TagKind, start_tag: Option<TagKind>) -> PrintResult<R> {
    Err(PrintError::InvalidDocument(match start_tag {
        None => InvalidDocumentError::StartTagMissing { kind: end_tag },
        Some(kind) => {
            InvalidDocumentError::StartEndTagMismatch { start_kind: end_tag, end_kind: kind }
        }
    }))
}

#[cold]
fn invalid_start_tag<R>(expected: TagKind, actual: Option<&FormatElement>) -> PrintResult<R> {
    let start = match actual {
        None => ActualStart::EndOfDocument,
        Some(FormatElement::Tag(tag)) => {
            if tag.is_start() {
                ActualStart::Start(tag.kind())
            } else {
                ActualStart::End(tag.kind())
            }
        }
        Some(_) => ActualStart::Content,
    };

    Err(PrintError::InvalidDocument(InvalidDocumentError::ExpectedStart {
        actual: start,
        expected_start: expected,
    }))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Fits {
    // Element fits
    Yes,
    // Element doesn't fit
    No,
    // Element may fit, depends on the elements following it
    Maybe,
}

impl From<bool> for Fits {
    fn from(value: bool) -> Self {
        if value { Fits::Yes } else { Fits::No }
    }
}

/// State used when measuring if a group fits on a single line
#[derive(Debug)]
struct FitsState {
    line_indent: LineIndention,
    pending_space: bool,
    has_line_suffix: bool,
    /// Whether the pending line suffix was queued past the measured content, in expanded mode.
    line_suffix_after_content: bool,
    line_width: usize,
    /// Past the first line of a `fits_last_line` group, where only its last line is measured.
    skips_lines: bool,
}

#[derive(Copy, Clone, Debug)]
enum Text<'a> {
    /// ASCII only text that contains no line breaks or tab characters.
    Token(&'a str),
    /// Arbitrary text. May contain `\n` line breaks, tab characters, or unicode characters.
    Text { text: &'a str, width: TextWidth },
}

#[cfg(test)]
mod tests {
    use robusta_allocator::{Allocator, Vec as ArenaVec};

    use crate::{
        BestFittingElement, Document, Format, FormatElement, FormatState, Formatter, IndentStyle,
        LineEnding, Printed, Printer, PrinterOptions, SimpleFormatContext,
        builders::{
            align, align_to_column, best_fitting_variant, block_indent, dedent_to_root, empty_line,
            expand_parent, group, hard_line_break, if_group_breaks, if_group_fits_on_line, indent,
            line_suffix, literal_line_break, mark_as_root, soft_block_indent, soft_line_break,
            soft_line_break_or_space, space, text, token,
        },
        format_args,
        printer::PrintWidth,
        write,
    };

    /// Test-only context type alias.
    type TestContext<'a> = SimpleFormatContext<'a>;

    fn format_simple<'a, F>(allocator: &'a Allocator, root: &F) -> Printed
    where
        F: Format<'a, TestContext<'a>>,
    {
        format_simple_with_options(
            allocator,
            root,
            PrinterOptions {
                indent_style: IndentStyle::Space,
                indent_width: 2.try_into().unwrap(),
                line_ending: LineEnding::Lf,
                ..PrinterOptions::default()
            },
        )
    }

    fn format_simple_with_options<'a, F>(
        allocator: &'a Allocator,
        root: &F,
        options: PrinterOptions,
    ) -> Printed
    where
        F: Format<'a, TestContext<'a>>,
    {
        let context = SimpleFormatContext::default();
        let mut state = FormatState::new(context, allocator);
        let mut buffer = Formatter::new(&mut state);
        root.fmt(&mut buffer);

        let elements = buffer.into_vec();
        Document::new(elements).print(0, options).expect("Document to be valid")
    }

    #[test]
    fn it_prints_a_group_on_a_single_line_if_it_fits() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &FormatArrayElements {
                items: vec![&token("\"a\""), &token("\"b\""), &token("\"c\""), &token("\"d\"")],
            },
        );

        assert_eq!(r#"["a", "b", "c", "d"]"#, result.as_code());
    }

    #[test]
    fn labels_preserve_group_layout_and_line_suffixes() {
        use crate::{Label, LabelId, builders::labelled};

        #[derive(Copy, Clone)]
        struct TestLabel;
        impl Label for TestLabel {
            fn id(&self) -> u32 {
                u32::MAX
            }

            fn debug_name(&self) -> &'static str {
                "TestLabel"
            }
        }

        struct Content;
        impl<'a> Format<'a, TestContext<'a>> for Content {
            fn fmt(&self, f: &mut crate::Formatter<'_, 'a, TestContext<'a>>) {
                let label = LabelId::of(TestLabel);
                write!(
                    f,
                    [
                        group(&format_args!(
                            token("alpha"),
                            soft_line_break_or_space(),
                            labelled(label, &token("beta"))
                        )),
                        line_suffix(&token(" // tail")),
                        hard_line_break()
                    ]
                );
            }
        }

        let allocator = Allocator::new();
        let label = LabelId::of(TestLabel);
        for width in [5, 80] {
            let options =
                PrinterOptions { print_width: PrintWidth::new(width), ..PrinterOptions::default() };
            let content = Content;
            let plain = format_simple_with_options(&allocator, &content, options.clone());
            let marked =
                format_simple_with_options(&allocator, &labelled(label, &content), options);
            assert_eq!(marked.as_code(), plain.as_code());
            assert!(marked.as_code().ends_with("beta // tail\n"));
        }

        let mut state = FormatState::new(SimpleFormatContext::default(), &allocator);
        let mut buffer = Formatter::new(&mut state);
        write!(buffer, [labelled(label, &token("x"))]);
        let document = Document::new(buffer.into_vec());
        assert!(document.display("").to_string().contains("TestLabel"));
    }

    #[test]
    fn it_tracks_the_indent_for_each_token() {
        let allocator = Allocator::default();
        let formatted = format_simple(
            &allocator,
            &format_args!(
                token("a"),
                soft_block_indent(&format_args!(
                    token("b"),
                    soft_block_indent(&format_args!(
                        token("c"),
                        soft_block_indent(
                            &format_args!(token("d"), soft_line_break(), token("d"),)
                        ),
                        token("c"),
                    )),
                    token("b"),
                )),
                token("a")
            ),
        );

        assert_eq!(
            r"a
  b
    c
      d
      d
    c
  b
a",
            formatted.as_code()
        );
    }

    #[test]
    fn hard_line_break_keeps_the_spaces_a_text_ends_with() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(text("a  "), hard_line_break(), text("b\t"), empty_line(), token("c")),
        );

        assert_eq!("a  \nb\t\n\nc", result.as_code());
    }

    // The expected outputs of the `literal_line_break` / `mark_as_root` tests below are
    // verified against Prettier's doc printer (`printDocToString`) with the equivalent
    // `literallineWithoutBreakParent` / `markAsRoot` / `dedentToRoot` documents.

    #[test]
    fn literal_line_break_prints_at_column_zero_without_trimming() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("a"),
                indent(&format_args!(
                    hard_line_break(),
                    token("b "),
                    literal_line_break(),
                    token("c")
                )),
                hard_line_break(),
                token("d")
            ),
        );

        // The trailing space after `b` is preserved and `c` starts at column 0.
        assert_eq!("a\n  b \nc\nd", result.as_code());
    }

    #[test]
    fn literal_line_break_always_prints_the_newline() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(token("a"), literal_line_break(), literal_line_break(), token("b")),
        );

        // Unlike `hard_line_break`, consecutive literal lines are not collapsed.
        assert_eq!("a\n\nb", result.as_code());
    }

    #[test]
    fn literal_line_break_returns_to_marked_root() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("k:"),
                align(
                    2,
                    &format_args!(
                        hard_line_break(),
                        token("line1"),
                        mark_as_root(&literal_line_break()),
                        token("line2")
                    )
                ),
                hard_line_break(),
                token("z")
            ),
        );

        // Without `mark_as_root`, `line2` would start at column 0.
        assert_eq!("k:\n  line1\n  line2\nz", result.as_code());
    }

    #[test]
    fn literal_line_break_expands_the_enclosing_group() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &group(&format_args!(
                token("<"),
                soft_line_break(),
                token("a"),
                literal_line_break(),
                token("b"),
                soft_line_break(),
                token(">")
            )),
        );

        assert_eq!("<\na\nb\n>", result.as_code());
    }

    #[test]
    fn literal_line_break_flushes_line_suffixes() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("a"),
                line_suffix(&token(" // c")),
                literal_line_break(),
                token("b")
            ),
        );

        assert_eq!("a // c\nb", result.as_code());
    }

    #[test]
    fn line_suffix_boundary_breaks_only_groups_that_could_print_the_suffix_sooner() {
        use crate::builders::line_suffix_boundary;

        let allocator = Allocator::default();
        let print = |suffix_first| {
            format_simple(
                &allocator,
                &test_format_with(|f| {
                    if suffix_first {
                        write!(f, [token("x"), line_suffix(&token(" // c"))]);
                    }
                    write!(
                        f,
                        [group(&format_args!(token("a"), soft_line_break_or_space(), token("b")))]
                    );
                    if !suffix_first {
                        write!(f, [token(";"), line_suffix(&token(" // c"))]);
                    }
                    write!(f, [line_suffix_boundary(), token("/* d */")]);
                }),
            )
            .as_code()
            .to_string()
        };

        // Queued after the group, the suffix prints at the boundary however the group breaks.
        assert_eq!(print(false), "a b; // c\n/* d */");
        // Pending before the group, the suffix prints sooner at the group's line break.
        assert_eq!(print(true), "xa // c\nb/* d */");
    }

    #[test]
    fn dedent_to_root_returns_to_marked_root() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("k:"),
                align(
                    2,
                    &mark_as_root(&format_args!(
                        hard_line_break(),
                        token("a"),
                        indent(&format_args!(
                            hard_line_break(),
                            token("deep"),
                            dedent_to_root(&format_args!(hard_line_break(), token("back")))
                        ))
                    ))
                )
            ),
        );

        // `dedent_to_root` returns to the `mark_as_root` indention (column 2), not column 0.
        assert_eq!("k:\n  a\n    deep\n  back", result.as_code());
    }

    /// Composes the document shape of Prettier's YAML block scalar printing
    /// (`language-yaml/print/block.js`): content indented via `align`,
    /// line boundaries as `mark_as_root(&literal_line_break())` so continuation lines
    /// keep the block's base indention, an empty content line as an empty line,
    /// and the keep-chomping (`|+`) trailing newline as `dedent_to_root(&literal_line_break())`.
    #[test]
    fn yaml_block_scalar_shaped_document() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("k: |+"),
                align(
                    2,
                    &format_args!(
                        hard_line_break(),
                        token("hello world"),
                        mark_as_root(&literal_line_break()),
                        empty_line(),
                        token("next"),
                        dedent_to_root(&literal_line_break())
                    )
                )
            ),
        );

        assert_eq!("k: |+\n  hello world\n\n  next\n", result.as_code());
    }

    /// Known divergence from Prettier: a hard line directly after a column-0 literal line
    /// is absorbed by the "only print a newline if the line isn't already empty" rule
    /// (Prettier prints both newlines). When a consumer needs the extra structural newline
    /// after a trailing literal line (e.g. after a keep-chomping YAML block scalar),
    /// use `empty_line()` which prints exactly one newline in this state.
    #[test]
    fn hard_line_after_column_zero_literal_line_is_absorbed() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(token("a"), literal_line_break(), hard_line_break(), token("b")),
        );
        assert_eq!("a\nb", result.as_code());

        let result = format_simple(
            &allocator,
            &format_args!(token("a"), literal_line_break(), empty_line(), token("b")),
        );
        assert_eq!("a\n\nb", result.as_code());
    }

    #[test]
    fn it_converts_line_endings() {
        let allocator = Allocator::default();
        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            line_ending: LineEnding::Crlf,
            ..PrinterOptions::default()
        };

        let result = format_simple_with_options(
            &allocator,
            &format_args!(
                token("function main() {"),
                block_indent(&text("let x = `This is a multiline\nstring`;")),
                token("}"),
                hard_line_break()
            ),
            options,
        );

        assert_eq!(
            "function main() {\r\n\tlet x = `This is a multiline\r\nstring`;\r\n}\r\n",
            result.as_code()
        );
    }

    #[test]
    fn it_converts_line_endings_to_cr() {
        let allocator = Allocator::default();
        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            line_ending: LineEnding::Cr,
            ..PrinterOptions::default()
        };

        let result = format_simple_with_options(
            &allocator,
            &format_args!(
                token("function main() {"),
                block_indent(&text("let x = `This is a multiline\nstring`;")),
                token("}"),
                hard_line_break()
            ),
            options,
        );

        assert_eq!(
            "function main() {\r\tlet x = `This is a multiline\rstring`;\r}\r",
            result.as_code()
        );
    }

    #[test]
    fn it_breaks_a_group_if_a_string_contains_a_newline() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &FormatArrayElements {
                items: vec![&text("`This is a string spanning\ntwo lines`"), &token("\"b\"")],
            },
        );

        assert_eq!(
            r#"[
  `This is a string spanning
two lines`,
  "b",
]"#,
            result.as_code()
        );
    }

    #[test]
    fn it_breaks_a_group_if_it_contains_a_hard_line_break() {
        let allocator = Allocator::default();
        let result =
            format_simple(&allocator, &group(&format_args!(token("a"), block_indent(&token("b")))));

        assert_eq!("a\n  b\n", result.as_code());
    }

    #[test]
    fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &FormatArrayElements {
                items: vec![
                    &token("\"a\""),
                    &token("\"b\""),
                    &token("\"c\""),
                    &token("\"d\""),
                    &FormatArrayElements {
                        items: vec![
                            &token("\"0123456789\""),
                            &token("\"0123456789\""),
                            &token("\"0123456789\""),
                            &token("\"0123456789\""),
                            &token("\"0123456789\""),
                            &token("\"0123456789\""),
                        ],
                    },
                ],
            },
        );

        assert_eq!(
            r#"[
  "a",
  "b",
  "c",
  "d",
  ["0123456789", "0123456789", "0123456789", "0123456789", "0123456789", "0123456789"],
]"#,
            result.as_code()
        );
    }

    #[test]
    fn it_use_the_indent_character_specified_in_the_options() {
        let allocator = Allocator::default();
        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            indent_width: 2.try_into().unwrap(),
            print_width: PrintWidth::new(19),
            ..PrinterOptions::default()
        };

        let result = format_simple_with_options(
            &allocator,
            &FormatArrayElements {
                items: vec![&token("'a'"), &token("'b'"), &token("'c'"), &token("'d'")],
            },
            options,
        );

        assert_eq!("[\n\t'a',\n\t\'b',\n\t\'c',\n\t'd',\n]", result.as_code());
    }

    #[test]
    fn tab_style_keeps_alignment_as_spaces_inside_an_indent() {
        let allocator = Allocator::default();
        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            indent_width: 4.try_into().unwrap(),
            ..PrinterOptions::default()
        };

        let result = format_simple_with_options(
            &allocator,
            &indent(&format_args!(
                hard_line_break(),
                token("a"),
                align(
                    8,
                    &format_args!(
                        hard_line_break(),
                        token("? {"),
                        block_indent(&token("b")),
                        token("}")
                    )
                )
            )),
            options,
        );

        assert_eq!("\ta\n\t        ? {\n\t\t        b\n\t        }", result.as_code());
    }

    #[test]
    fn align_to_column_starts_later_lines_where_its_content_starts() {
        let allocator = Allocator::default();
        let comment = test_format_with(|f| {
            write!(
                f,
                [
                    token("x;"),
                    space(),
                    align_to_column(&format_args!(
                        token("/* a"),
                        hard_line_break(),
                        token(" * b */")
                    ))
                ]
            );
        });

        let result = format_simple(&allocator, &indent(&comment));
        assert_eq!("x; /* a\n    * b */", result.as_code());

        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            indent_width: 2.try_into().unwrap(),
            ..PrinterOptions::default()
        };
        let result = format_simple_with_options(
            &allocator,
            &indent(&format_args!(hard_line_break(), indent(&comment))),
            options,
        );
        assert_eq!("\tx; /* a\n\t    * b */", result.as_code());
    }

    #[test]
    fn align_to_column_reaches_a_wide_column_after_multiline_text() {
        let allocator = Allocator::default();
        let padding = " ".repeat(260);
        let multiline = format!("first line\n{padding}");
        let result = format_simple(
            &allocator,
            &format_args!(
                text(&multiline),
                align_to_column(&format_args!(token("/* a"), hard_line_break(), token(" * b */")))
            ),
        );
        assert_eq!(format!("{multiline}/* a\n{padding} * b */"), result.as_code());
    }

    #[test]
    fn it_prints_consecutive_hard_lines_as_one() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("a"),
                hard_line_break(),
                hard_line_break(),
                hard_line_break(),
                token("b"),
            ),
        );

        assert_eq!("a\nb", result.as_code());
    }

    #[test]
    fn it_prints_consecutive_empty_lines_as_one() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(token("a"), empty_line(), empty_line(), empty_line(), token("b"),),
        );

        assert_eq!("a\n\nb", result.as_code());
    }

    #[test]
    fn it_prints_consecutive_mixed_lines_as_one() {
        let allocator = Allocator::default();
        let result = format_simple(
            &allocator,
            &format_args!(
                token("a"),
                empty_line(),
                hard_line_break(),
                empty_line(),
                hard_line_break(),
                token("b"),
            ),
        );

        assert_eq!("a\n\nb", result.as_code());
    }

    #[test]
    fn test_fill_breaks() {
        let allocator = Allocator::default();
        let context = SimpleFormatContext::default();
        let mut state = FormatState::new(context, &allocator);
        let mut formatter = Formatter::new(&mut state);

        formatter
            .fill()
            // These all fit on the same line together
            .entry(&soft_line_break_or_space(), &format_args!(token("1"), token(",")))
            .entry(&soft_line_break_or_space(), &format_args!(token("2"), token(",")))
            .entry(&soft_line_break_or_space(), &format_args!(token("3"), token(",")))
            // This one fits on a line by itself,
            .entry(&soft_line_break_or_space(), &format_args!(token("723493294"), token(",")))
            // fits without breaking
            .entry(
                &soft_line_break_or_space(),
                &group(&format_args!(token("["), soft_block_indent(&token("5")), token("],"))),
            )
            // this one must be printed in expanded mode to fit
            .entry(
                &soft_line_break_or_space(),
                &group(&format_args!(
                    token("["),
                    soft_block_indent(&token("123456789")),
                    token("]"),
                )),
            )
            .finish();

        let document = Document::new(formatter.into_vec());

        let printed = Printer::new(
            PrinterOptions::default()
                .with_indent_style(IndentStyle::Tab)
                .with_print_width(PrintWidth::new(10)),
        )
        .print(document.elements())
        .unwrap();

        assert_eq!(printed.as_code(), "1, 2, 3,\n723493294,\n[5],\n[\n\t123456789\n]");
    }

    #[test]
    fn fill_measures_entries_after_line_breaks() {
        for (line_break, expected) in [
            (hard_line_break(), "    aaaa,\n    // x\n    bbbb,\n    cccc, d"),
            (empty_line(), "    aaaa,\n    // x\n\n    bbbb,\n    cccc, d"),
            (literal_line_break(), "    aaaa,\n    // x\n    bbbb,\n    cccc, d"),
        ] {
            let allocator = Allocator::default();
            let printed = format_simple_with_options(
                &allocator,
                &group(&indent(&format_args!(
                    soft_line_break(),
                    test_format_with(|f| {
                        f.fill()
                            .entry(&soft_line_break_or_space(), &token("aaaa,"))
                            .entry(
                                &soft_line_break_or_space(),
                                &format_args!(
                                    token("// x"),
                                    mark_as_root(&line_break),
                                    token("bbbb,")
                                ),
                            )
                            .entry(&soft_line_break_or_space(), &token("cccc,"))
                            .entry(&soft_line_break_or_space(), &token("d"))
                            .finish();
                    })
                ))),
                PrinterOptions {
                    print_width: PrintWidth::new(12),
                    indent_width: 4.try_into().unwrap(),
                    ..PrinterOptions::default()
                },
            );

            assert_eq!(printed.as_code(), expected);
        }
    }

    #[test]
    fn fill_literal_line_measures_from_the_root_indent() {
        for (marked_root, expected) in [
            (true, "    aaaa, // x\n    bbbb,\n    cccc, d"),
            (false, "    aaaa, // x\nbbbb, cccc, d"),
        ] {
            let allocator = Allocator::default();
            let printed = format_simple_with_options(
                &allocator,
                &group(&indent(&format_args!(
                    soft_line_break(),
                    test_format_with(|f| {
                        f.fill()
                            .entry(&soft_line_break_or_space(), &token("aaaa,"))
                            .entry(
                                &soft_line_break_or_space(),
                                &format_args!(
                                    token("// x"),
                                    test_format_with(|f| {
                                        if marked_root {
                                            mark_as_root(&literal_line_break()).fmt(f);
                                        } else {
                                            literal_line_break().fmt(f);
                                        }
                                    }),
                                    space(),
                                    token("bbbb,")
                                ),
                            )
                            .entry(&soft_line_break_or_space(), &token("cccc,"))
                            .entry(&soft_line_break_or_space(), &token("d"))
                            .finish();
                    })
                ))),
                PrinterOptions {
                    print_width: PrintWidth::new(15),
                    indent_width: 4.try_into().unwrap(),
                    ..PrinterOptions::default()
                },
            );

            assert_eq!(printed.as_code(), expected);
        }
    }

    #[test]
    fn fill_last_item_measures_attached_syntax_after_its_group() {
        let allocator = Allocator::default();
        let printed = format_simple_with_options(
            &allocator,
            &format_args!(
                group(&format_args!(
                    token("Map<"),
                    indent(&format_args!(
                        soft_line_break(),
                        test_format_with(|f| {
                            f.fill()
                                .entry(
                                    &soft_line_break_or_space(),
                                    &format_args!(token("foo"), token(",")),
                                )
                                .entry(&soft_line_break_or_space(), &token("bar"))
                                .finish();
                        })
                    )),
                    token(">")
                )),
                space(),
                token("name")
            ),
            PrinterOptions {
                print_width: PrintWidth::new(11),
                indent_width: 2.try_into().unwrap(),
                ..PrinterOptions::default()
            },
        );

        assert_eq!(printed.as_code(), "Map<\n  foo,\n  bar> name");
    }

    #[test]
    fn line_suffix_after_fill_does_not_break_its_entries() {
        let allocator = Allocator::default();
        let printed = format_simple(
            &allocator,
            &format_args!(
                test_format_with(|f| {
                    f.fill()
                        .entry(&soft_line_break_or_space(), &token("a"))
                        .entry(&soft_line_break_or_space(), &token("b"))
                        .finish();
                }),
                token(";"),
                line_suffix(&token(" // c")),
                expand_parent(),
            ),
        );

        assert_eq!(printed.as_code(), "a b; // c");
    }

    #[test]
    fn line_suffix_printed_at_end() {
        let allocator = Allocator::default();
        let printed = format_simple(
            &allocator,
            &format_args!(
                group(&format_args!(
                    token("["),
                    soft_block_indent(&test_format_with(|f| {
                        f.fill()
                            .entry(
                                &soft_line_break_or_space(),
                                &format_args!(token("1"), token(",")),
                            )
                            .entry(
                                &soft_line_break_or_space(),
                                &format_args!(token("2"), token(",")),
                            )
                            .entry(
                                &soft_line_break_or_space(),
                                &format_args!(token("3"), if_group_breaks(&token(","))),
                            )
                            .finish();
                    })),
                    token("]")
                )),
                token(";"),
                &line_suffix(&format_args!(space(), token("// trailing"), space()))
            ),
        );

        assert_eq!(printed.as_code(), "[1, 2, 3]; // trailing");
    }

    #[test]
    fn line_suffix_group_measures_to_the_end_of_its_line() {
        let allocator = Allocator::default();
        let options =
            || PrinterOptions { print_width: PrintWidth::new(10), ..PrinterOptions::default() };
        let suffix = |content: &'static str| {
            test_format_with(move |f| {
                write!(
                    f,
                    [line_suffix(&format_args!(
                        hard_line_break(),
                        group(&format_args!(
                            token("["),
                            soft_block_indent(&token(content)),
                            token("]")
                        ))
                    ))]
                );
            })
        };

        let flushed_at_break = format_simple_with_options(
            &allocator,
            &format_args!(token("a"), suffix("b"), token(";"), hard_line_break()),
            options(),
        );
        assert_eq!(flushed_at_break.as_code(), "a;\n[b]\n");

        let flushed_at_end = format_simple_with_options(
            &allocator,
            &format_args!(token("a"), suffix("b"), token(";")),
            options(),
        );
        assert_eq!(flushed_at_end.as_code(), "a;\n[b]");

        let expanded_by_next_suffix = format_simple_with_options(
            &allocator,
            &format_args!(
                token("a"),
                suffix("b"),
                line_suffix(&token("cccccccc")),
                hard_line_break()
            ),
            options(),
        );
        assert_eq!(expanded_by_next_suffix.as_code(), "a\n[\n  b\n]cccccccc\n");
    }

    #[test]
    fn conditional_with_group_id_in_fits() {
        let allocator = Allocator::default();
        let content = test_format_with(|f| {
            let group_id = f.state().group_id("test");
            write!(
                f,
                [
                    group(&format_args!(
                        token("The referenced group breaks."),
                        hard_line_break()
                    ))
                    .with_group_id(Some(group_id)),
                    group(&format_args!(
                        token("This group breaks because:"),
                        soft_line_break_or_space(),
                        if_group_fits_on_line(&token("This content fits but should not be printed.")).with_group_id(Some(group_id)),
                        if_group_breaks(&token("It measures with the 'if_group_breaks' variant because the referenced group breaks and that's just way too much text.")).with_group_id(Some(group_id)),
                    ))
                ]
            );
        });

        let printed = format_simple(&allocator, &content);

        assert_eq!(
            printed.as_code(),
            "The referenced group breaks.\nThis group breaks because:\nIt measures with the 'if_group_breaks' variant because the referenced group breaks and that's just way too much text."
        );
    }

    #[test]
    fn out_of_order_group_ids() {
        let allocator = Allocator::default();
        let content = test_format_with(|f| {
            let id_1 = f.state().group_id("id-1");
            let id_2 = f.state().group_id("id-2");

            write!(
                f,
                [group(&token("Group with id-2")).with_group_id(Some(id_2)), hard_line_break()]
            );

            write!(f,
            [
                group(&token("Group with id-1 does not fit on the line because it exceeds the line width of 100 characters by..........")).with_group_id(Some(id_1)),
                hard_line_break()
            ]);

            write!(
                f,
                [
                    if_group_fits_on_line(&token("Group 2 fits")).with_group_id(Some(id_2)),
                    hard_line_break(),
                    if_group_breaks(&token("Group 1 breaks")).with_group_id(Some(id_1))
                ]
            );
        });

        let printed = format_simple(&allocator, &content);

        assert_eq!(
            printed.as_code(),
            r"Group with id-2
Group with id-1 does not fit on the line because it exceeds the line width of 100 characters by..........
Group 2 fits
Group 1 breaks"
        );
    }

    #[test]
    fn break_group_if_partial_string_exceeds_print_width() {
        let allocator = Allocator::default();
        let options =
            PrinterOptions { print_width: PrintWidth::new(10), ..PrinterOptions::default() };

        let result = format_simple_with_options(
            &allocator,
            &format_args!(group(&format_args!(
                token("("),
                soft_line_break(),
                text("This is a string\n containing a newline"),
                soft_line_break(),
                token(")")
            ))),
            options,
        );

        assert_eq!("(\nThis is a string\n containing a newline\n)", result.as_code());
    }

    #[test]
    fn fits_all_lines_group_prints_flat_only_when_every_line_fits() {
        let allocator = Allocator::default();
        let options =
            PrinterOptions { print_width: PrintWidth::new(10), ..PrinterOptions::default() };
        let print = |last_line| {
            format_simple_with_options(
                &allocator,
                &group(&format_args!(
                    if_group_fits_on_line(&format_args!(
                        token("short"),
                        hard_line_break(),
                        token(last_line)
                    )),
                    if_group_breaks(&token("broken")),
                ))
                .fits_all_lines(true),
                options.clone(),
            )
            .as_code()
            .to_string()
        };

        assert_eq!(print("fits"), "short\nfits");
        assert_eq!(print("much too long"), "broken");
    }

    /// Prints `content` in an expanded `fits_last_line` group followed by `rest`,
    /// or `expanded` when that does not fit in 10 columns.
    fn print_fits_last_line<'a>(
        allocator: &'a Allocator,
        content: &dyn Format<'a, TestContext<'a>>,
        rest: &dyn Format<'a, TestContext<'a>>,
    ) -> String {
        let options = PrinterOptions {
            print_width: PrintWidth::new(10),
            indent_style: IndentStyle::Space,
            indent_width: 2.try_into().unwrap(),
            ..PrinterOptions::default()
        };
        let root = test_format_with(|f| {
            let middle = best_fitting_variant(f, |f| {
                write!(f, [group(&content).should_expand(true).fits_last_line(true), rest]);
            });
            let expanded = best_fitting_variant(f, |f| token("expanded").fmt(f));
            let variants = ArenaVec::from_iter_in([middle, expanded], f);
            // SAFETY: There are two variants.
            f.write_element(FormatElement::BestFitting(unsafe {
                BestFittingElement::from_vec_unchecked(variants)
            }));
        });
        format_simple_with_options(allocator, &root, options).as_code().to_string()
    }

    #[test]
    fn fits_last_line_group_measures_the_line_it_ends_on() {
        let allocator = Allocator::default();
        let print = |body, rest| {
            print_fits_last_line(
                &allocator,
                &format_args!(token("{"), block_indent(&token(body)), token("}")),
                &token(rest),
            )
        };

        assert_eq!(print("body", ", fits"), "{\n  body\n}, fits");
        assert_eq!(print("a body too long", ", fits"), "{\n  a body too long\n}, fits");
        assert_eq!(print("body", ", too long"), "expanded");
    }

    #[test]
    fn fits_last_line_group_measures_past_a_boundary_that_prints_a_suffix() {
        use crate::builders::line_suffix_boundary;

        let allocator = Allocator::default();
        let print = |measure_flat| {
            print_fits_last_line(
                &allocator,
                &test_format_with(|f| {
                    write!(
                        f,
                        [token("{"), hard_line_break(), token("a"), line_suffix(&token(" // c"))]
                    );
                    let tail =
                        test_format_with(|f| write!(f, [line_suffix_boundary(), token("b")]));
                    if measure_flat {
                        write!(f, [group(&tail).measure_flat()]);
                    } else {
                        write!(f, [tail]);
                    }
                    write!(f, [hard_line_break(), token("}")]);
                }),
                &token(", x"),
            )
        };

        // On a line the measurement skips, the boundary ends that line as the suffix prints,
        // whether the boundary is measured expanded or flat.
        assert_eq!(print(false), "{\na // c\nb\n}, x");
        assert_eq!(print(true), "{\na // c\nb\n}, x");
    }

    struct FormatArrayElements<'a> {
        items: Vec<&'a dyn Format<'a, TestContext<'a>>>,
    }

    impl<'a> Format<'a, TestContext<'a>> for FormatArrayElements<'a> {
        fn fmt(&self, f: &mut crate::Formatter<'_, 'a, TestContext<'a>>) {
            write!(
                f,
                [group(&format_args!(
                    token("["),
                    soft_block_indent(&format_args!(
                        test_format_with(|f| {
                            f.join_with(format_args!(token(","), soft_line_break_or_space()))
                                .entries(&self.items);
                        }),
                        if_group_breaks(&token(",")),
                    )),
                    token("]")
                ))]
            );
        }
    }

    /// Test-only `format_with` closure adapter.
    fn test_format_with<'a, T>(closure: T) -> TestFormatWith<T>
    where
        T: Fn(&mut crate::Formatter<'_, 'a, TestContext<'a>>),
    {
        TestFormatWith { closure }
    }

    struct TestFormatWith<T> {
        closure: T,
    }

    impl<'a, T> Format<'a, TestContext<'a>> for TestFormatWith<T>
    where
        T: Fn(&mut crate::Formatter<'_, 'a, TestContext<'a>>),
    {
        fn fmt(&self, f: &mut crate::Formatter<'_, 'a, TestContext<'a>>) {
            (self.closure)(f);
        }
    }
}
