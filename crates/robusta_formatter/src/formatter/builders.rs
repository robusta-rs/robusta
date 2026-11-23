use robusta_formatter_core::Format;
use robusta_formatter_core::builders::{FormatOnce, FormatWith};
use robusta_span::{GetSpan, Span};

use crate::formatter::trivia::trailing_whitespace;
use crate::formatter::{JavaFormatContext, JavaFormatter, JavaFormatterExt};

// Keep Java syntax on the language-agnostic construction API while pinning
// closure inference to the Java format context.
pub use robusta_formatter_core::builders::*;

/// Wraps a re-entrant Java formatting closure in a [`FormatWith`].
#[inline]
pub const fn format_with<'ast, T>(formatter: T) -> FormatWith<T>
where
    T: Fn(&mut JavaFormatter<'_, 'ast>),
{
    FormatWith::new(formatter)
}

/// Wraps a single-use Java formatting closure in a [`FormatOnce`].
#[inline]
pub const fn format_once<'ast, T>(formatter: T) -> FormatOnce<T>
where
    T: FnOnce(&mut JavaFormatter<'_, 'ast>),
{
    FormatOnce::new(formatter)
}

/// Indents wrapped lines one level, as Oxc does, or two where they follow dart
/// format or google-java-format, whose continuation indent is twice their block
/// indent.
#[inline]
pub const fn wrapped_indent<Content>(
    double: bool,
    content: &Content,
) -> WrappedIndent<'_, Content> {
    WrappedIndent { double, content }
}

pub struct WrappedIndent<'a, Content> {
    double: bool,
    content: &'a Content,
}

impl<'ast, Content> Format<'ast, JavaFormatContext<'ast>> for WrappedIndent<'_, Content>
where
    Content: Format<'ast, JavaFormatContext<'ast>>,
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'ast>) {
        if self.double {
            indent(&indent(self.content)).fmt(f);
        } else {
            indent(self.content).fmt(f);
        }
    }
}

/// Builder to join together nodes that ensures that nodes separated by empty lines continue
/// to be separated by empty lines in the formatted output.
///
/// It writes separators only between entries. The break before the first entry
/// belongs to the frame the nodes sit in, such as a braced body.
pub struct JoinNodesBuilder<'fmt, 'buf, 'ast, Separator> {
    /// The separator to insert between nodes. Either a soft or hard line break
    separator: Separator,
    fmt: &'fmt mut JavaFormatter<'buf, 'ast>,
    has_elements: bool,
}

impl<'fmt, 'buf, 'ast, Separator> JoinNodesBuilder<'fmt, 'buf, 'ast, Separator>
where
    Separator: Format<'ast, JavaFormatContext<'ast>>,
{
    pub(super) const fn new(
        separator: Separator,
        fmt: &'fmt mut JavaFormatter<'buf, 'ast>,
    ) -> Self {
        Self { separator, fmt, has_elements: false }
    }

    /// Adds a new node with the specified formatted content to the output, respecting any new lines
    /// that appear before the node in the input source.
    pub fn entry(&mut self, span: Span, content: &dyn Format<'ast, JavaFormatContext<'ast>>) {
        self.separator_no_entry(span);
        self.write_entry(content);
    }

    /// Adds a new node after an empty line when `empty_line` holds, or after the
    /// separator otherwise, whatever the input source has before it.
    pub fn entry_with_empty_line(
        &mut self,
        empty_line: bool,
        content: &dyn Format<'ast, JavaFormatContext<'ast>>,
    ) {
        if self.has_elements {
            self.write_separator(empty_line);
        }
        self.write_entry(content);
    }

    /// Adds an iterator of entries to the output.
    pub fn entries<F, I>(&mut self, entries: I) -> &mut Self
    where
        F: Format<'ast, JavaFormatContext<'ast>> + GetSpan,
        I: IntoIterator<Item = F>,
    {
        for content in entries {
            self.entry(content.span(), &content);
        }
        self
    }

    fn write_entry(&mut self, content: &dyn Format<'ast, JavaFormatContext<'ast>>) {
        self.has_elements = true;
        content.fmt(self.fmt);
    }

    /// Writes a separator for before `span`, without adding an entry.
    fn separator_no_entry(&mut self, span: Span) {
        if self.has_elements {
            let empty_line = self.has_lines_before(span);
            self.write_separator(empty_line);
        }
    }

    /// Whether an empty line precedes `span` or the comments leading it.
    fn has_lines_before(&self, span: Span) -> bool {
        self.fmt.lines_before(span) > 1
    }

    /// A `///` comment that continues past the last entry's line stays whole:
    /// no empty line splits it, and its line keeps its hard-break spaces.
    fn write_separator(&mut self, blank: bool) {
        if let Some(comment) = self.fmt.comments().continued_markdown_line() {
            // The comment rides a line suffix, so its spaces follow it there.
            let spaces = trailing_whitespace(self.fmt.source_text().text_for(&comment.span));
            line_suffix(&text(spaces)).fmt(self.fmt);
            hard_line_break().fmt(self.fmt);
        } else if blank {
            empty_line().fmt(self.fmt);
        } else {
            self.separator.fmt(self.fmt);
        }
    }
}
