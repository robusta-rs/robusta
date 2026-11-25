use std::{cell::Cell, num::NonZeroU8};

use crate::GroupId;

use super::PrintMode;

/// A Tag marking the start and end of some content to which some special formatting should be applied.
///
/// Tags always come in pairs of a start and an end tag and the styling defined by this tag
/// will be applied to all elements in between the start/end tags.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tag {
    /// Indents the content one level deeper, see [crate::builders::indent] for documentation and examples.
    StartIndent,
    EndIndent,

    /// Variant of `Indent` that indents content by a number of spaces. For example, `Align(2)`
    /// indents any content following a line break by an additional two spaces.
    ///
    /// Nesting Aligns has the effect that all except the most inner align are handled as `Indent`.
    StartAlign(Align),
    EndAlign,

    /// Variant of `Align` that starts any content following a line break at the column where the
    /// content starts. See [crate::builders::align_to_column].
    StartAlignToColumn,
    EndAlignToColumn,

    /// Variant of `Indent` that indents any content following a line break one level deeper than
    /// the line where the content starts, or to the content's column when it starts that line.
    /// See [crate::builders::indent_from_line].
    StartIndentFromLine,
    EndIndentFromLine,

    /// Reduces the indention of the specified content either by one level or to the root, depending on the mode.
    /// Reverse operation of `Indent` and can be used to *undo* an `Align` for nested content.
    StartDedent(DedentMode),
    EndDedent(DedentMode),

    /// Creates a logical group where its content is either consistently printed:
    /// * on a single line: Omitting `LineMode::Soft` line breaks and printing spaces for `LineMode::SoftOrSpace`
    /// * on multiple lines: Printing all line breaks
    ///
    /// See [crate::builders::group] for documentation and examples.
    StartGroup(Group),
    EndGroup,

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [crate::builders::if_group_breaks] for examples.
    StartConditionalContent(Condition),
    EndConditionalContent,

    /// Optimized version of [Tag::StartConditionalContent] for the case where some content
    /// should be indented if the specified group breaks. A count aligns by that many
    /// columns instead of taking one indent level.
    StartIndentIfGroupBreaks(GroupId, Option<NonZeroU8>),
    EndIndentIfGroupBreaks(GroupId),

    /// Concatenates multiple elements together with a given separator printed in either
    /// flat or expanded mode to fill the print width. Expect that the content is a list of alternating
    /// [element, separator] See [crate::Formatter::fill].
    StartFill,
    EndFill,

    /// Entry inside of a [Tag::StartFill]
    StartEntry,
    EndEntry,

    /// Delay the printing of its content until the next line break
    StartLineSuffix,
    EndLineSuffix,

    /// Marks content without affecting its printed layout.
    StartLabelled(LabelId),
    EndLabelled,

    /// Marks the current indention as the root that [LineMode::Literal](super::LineMode::Literal)
    /// line breaks and [DedentMode::Root] dedents return to.
    /// See [crate::builders::mark_as_root] for documentation.
    StartMarkAsRoot,
    EndMarkAsRoot,
}

// `Tag` must fit next to the tag of [crate::FormatElement] (see the size assertion in `format_element/mod.rs`),
// so its payloads keep it 4-byte aligned: no `u64` or plain pointer.
const _: () = assert!(
    size_of::<Tag>() <= 12 && align_of::<Tag>() <= 4,
    "`Tag` exceeds 12 bytes or 4-byte alignment, `FormatElement` would exceed 16 bytes"
);

impl Tag {
    /// Returns `true` if `self` is any start tag.
    pub const fn is_start(&self) -> bool {
        matches!(
            self,
            Tag::StartIndent
                | Tag::StartAlign(_)
                | Tag::StartAlignToColumn
                | Tag::StartIndentFromLine
                | Tag::StartDedent(_)
                | Tag::StartGroup { .. }
                | Tag::StartConditionalContent(_)
                | Tag::StartIndentIfGroupBreaks(..)
                | Tag::StartFill
                | Tag::StartEntry
                | Tag::StartLineSuffix
                | Tag::StartLabelled(_)
                | Tag::StartMarkAsRoot
        )
    }

    /// Returns `true` if `self` is any end tag.
    pub const fn is_end(&self) -> bool {
        !self.is_start()
    }

    pub const fn kind(&self) -> TagKind {
        use Tag::{
            EndAlign, EndAlignToColumn, EndConditionalContent, EndDedent, EndEntry, EndFill,
            EndGroup, EndIndent, EndIndentFromLine, EndIndentIfGroupBreaks, EndLineSuffix,
            EndMarkAsRoot, StartAlign, StartAlignToColumn, StartConditionalContent, StartDedent,
            StartEntry, StartFill, StartGroup, StartIndent, StartIndentFromLine,
            StartIndentIfGroupBreaks, StartLineSuffix, StartMarkAsRoot,
        };

        match self {
            StartIndent | EndIndent => TagKind::Indent,
            StartAlign(_) | EndAlign => TagKind::Align,
            StartAlignToColumn | EndAlignToColumn => TagKind::AlignToColumn,
            StartIndentFromLine | EndIndentFromLine => TagKind::IndentFromLine,
            StartDedent(_) | EndDedent(_) => TagKind::Dedent,
            StartGroup(_) | EndGroup => TagKind::Group,
            StartConditionalContent(_) | EndConditionalContent => TagKind::ConditionalContent,
            StartIndentIfGroupBreaks(..) | EndIndentIfGroupBreaks(_) => {
                TagKind::IndentIfGroupBreaks
            }
            StartFill | EndFill => TagKind::Fill,
            StartEntry | EndEntry => TagKind::Entry,
            StartLineSuffix | EndLineSuffix => TagKind::LineSuffix,
            Tag::StartLabelled(_) | Tag::EndLabelled => TagKind::Labelled,
            StartMarkAsRoot | EndMarkAsRoot => TagKind::MarkAsRoot,
        }
    }
}

/// The kind of a [Tag].
///
/// Each start end tag pair has its own [tag kind](TagKind).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TagKind {
    Indent,
    Align,
    AlignToColumn,
    IndentFromLine,
    Dedent,
    Group,
    ConditionalContent,
    IndentIfGroupBreaks,
    Fill,
    Entry,
    LineSuffix,
    Labelled,
    MarkAsRoot,
}

#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub enum GroupMode {
    /// Print group in flat mode.
    #[default]
    Flat,

    /// Keep the group flat when measuring a layout other than a fallback.
    MeasureFlat,

    /// The group should be printed in expanded mode
    Expand,

    /// Expand mode has been propagated from an enclosing group to this group.
    Propagated,
}

impl GroupMode {
    pub const fn is_flat(self) -> bool {
        matches!(self, GroupMode::Flat | GroupMode::MeasureFlat)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Group {
    id: Option<GroupId>,
    mode: Cell<GroupMode>,
    fits_all_lines: bool,
    fits_last_line: bool,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: None,
            mode: Cell::new(GroupMode::Flat),
            fits_all_lines: false,
            fits_last_line: false,
        }
    }

    /// Print the group flat only when every line of its flat content fits,
    /// rather than only the first. Its own hard line breaks do not expand it.
    #[must_use]
    pub fn with_fits_all_lines(mut self, fits_all_lines: bool) -> Self {
        self.fits_all_lines = fits_all_lines;
        self
    }

    pub fn fits_all_lines(&self) -> bool {
        self.fits_all_lines
    }

    /// Measured expanded, the group fits only when its last line fits too.
    /// The lines between its first and last are not measured.
    #[must_use]
    pub fn with_fits_last_line(mut self, fits_last_line: bool) -> Self {
        self.fits_last_line = fits_last_line;
        self
    }

    pub fn fits_last_line(&self) -> bool {
        self.fits_last_line
    }

    #[must_use]
    pub fn with_id(mut self, id: Option<GroupId>) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    pub fn with_mode(mut self, mode: GroupMode) -> Self {
        self.mode = Cell::new(mode);
        self
    }

    pub fn mode(&self) -> GroupMode {
        self.mode.get()
    }

    pub fn propagate_expand(&self) {
        if self.mode.get().is_flat() {
            self.mode.set(GroupMode::Propagated);
        }
    }

    pub fn id(&self) -> Option<GroupId> {
        self.id
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DedentMode {
    /// Reduces the indent by a level (if the current indent is > 0)
    Level,

    /// Reduces the indent to the root
    Root,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Condition {
    /// * Flat -> Omitted if the enclosing group is a multiline group, printed for groups fitting on a single line
    /// * Multiline -> Omitted if the enclosing group fits on a single line, printed if the group breaks over multiple lines.
    pub(crate) mode: PrintMode,

    /// The id of the group for which it should check if it breaks or not. The group must appear in the document
    /// before the conditional group (but doesn't have to be in the ancestor chain).
    pub(crate) group_id: Option<GroupId>,
}

impl Condition {
    pub fn new(mode: PrintMode) -> Self {
        Self { mode, group_id: None }
    }

    #[must_use]
    pub fn with_group_id(mut self, id: Option<GroupId>) -> Self {
        self.group_id = id;
        self
    }

    pub fn mode(&self) -> PrintMode {
        self.mode
    }

    pub fn group_id(&self) -> Option<GroupId> {
        self.group_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Align(pub(crate) NonZeroU8);

impl Align {
    pub fn new(count: NonZeroU8) -> Self {
        Self(count)
    }

    pub fn count(&self) -> NonZeroU8 {
        self.0
    }
}

/// Identifies a label applied via [crate::builders::labelled].
///
/// Debug names live in a side table so IR elements have the same layout in every build.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct LabelId {
    value: u32,
}

impl std::fmt::Debug for LabelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(debug_assertions)]
        if let Some(name) = debug_names::lookup(self.value) {
            return f.write_str(name);
        }
        write!(f, "#{}", self.value)
    }
}

impl LabelId {
    #[expect(clippy::needless_pass_by_value)]
    pub fn of<T: Label>(label: T) -> Self {
        let value = label.id();
        #[cfg(debug_assertions)]
        debug_names::record(value, label.debug_name());
        Self { value }
    }
}

#[cfg(debug_assertions)]
mod debug_names {
    use std::sync::Mutex;

    /// Labels are few, so a linear scan suffices.
    static NAMES: Mutex<Vec<(u32, &'static str)>> = Mutex::new(Vec::new());

    pub(super) fn record(value: u32, name: &'static str) {
        let mut names = NAMES.lock().unwrap();
        if let Some((_, existing_name)) = names.iter().find(|(existing, _)| *existing == value) {
            assert_eq!(
                *existing_name, name,
                "Two labels with different names have the same value ({value})"
            );
        } else {
            names.push((value, name));
        }
    }

    pub(super) fn lookup(value: u32) -> Option<&'static str> {
        NAMES
            .lock()
            .unwrap()
            .iter()
            .find_map(|&(existing, name)| (existing == value).then_some(name))
    }
}

/// Defines a language formatter's labels.
pub trait Label {
    /// Returns the `u32` uniquely identifying this specific label.
    fn id(&self) -> u32;

    /// Returns the name of the label that is shown in debug builds.
    fn debug_name(&self) -> &'static str;
}
