use crate::{ImportOrder, IndentStyle, IndentWidth, LineEnding, LineWidth};

/// Layout options loaded from `robusta.yaml`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    /// Maximum line width the printer aims for, validated to `1..=320`.
    pub line_width: LineWidth,
    /// Whether structural indentation uses spaces or tabs.
    pub indent_style: IndentStyle,
    /// Display columns per indentation level (and per tab in tab mode),
    /// validated to `0..=24`.
    pub indent_width: IndentWidth,
    /// Where `else`, `catch`, and `finally` go after a closing brace.
    pub after_closing_brace: AfterClosingBrace,
    /// Import-kind section order plus package-prefix ordering within type
    /// and static sections. Imports are sorted by dotted name within each
    /// matched group; non-empty groups are separated by one blank line.
    pub import_order: ImportOrder,
    /// Javadoc formatting settings. `None` keeps Javadoc comments as written.
    pub javadoc: Option<JavadocOptions>,
    /// Whether each text block's opening `"""` starts a wrapped line or follows
    /// an operator that starts one, except after `return`, `yield`, or `case`, and
    /// a closing `"""` after text moves to the next line after a `\`.
    pub text_blocks: bool,
    /// The line terminator emitted at every output line break.
    pub line_ending: LineEnding,
    /// Whether non-empty output ends with one line terminator.
    pub insert_final_newline: bool,
}

/// Settings for formatting traditional (`/** ... */`) and Markdown (`///`)
/// Javadoc comments.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JavadocOptions {
    /// Capitalize the first word of each description paragraph.
    pub capitalize_descriptions: bool,
    pub description_with_dot: DescriptionWithDot,
    pub comment_line_strategy: CommentLineStrategy,
    pub line_wrapping_style: LineWrappingStyle,
    pub separate_tag_groups: SeparateTagGroups,
    /// Start each paragraph after the first of a traditional comment with
    /// `<p>`, and remove one from the first.
    pub paragraph_html_tags: bool,
}

/// Which descriptions that end in a letter or digit get a period.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DescriptionWithDot {
    #[default]
    Never,
    /// The summary, the text javadoc shows for the comment in summary tables.
    Summary,
    /// Each description paragraph.
    Always,
}

/// When a traditional comment uses the single-line `/** Content. */` form.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CommentLineStrategy {
    /// When the comment has no block tags and fits on one line.
    #[default]
    SingleLine,
    Multiline,
    /// Only for a comment already on one line.
    Keep,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineWrappingStyle {
    /// Fill each line to the line width.
    #[default]
    Greedy,
    /// Keep a paragraph's line breaks when every line fits.
    Keep,
}

/// Where blank lines separate block tags.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SeparateTagGroups {
    #[default]
    Never,
    /// Before `@return`.
    BeforeReturn,
    /// Between block tags of different kinds.
    Always,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AfterClosingBrace {
    #[default]
    SameLine,
    NewLine,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            line_width: LineWidth::default(),
            indent_style: IndentStyle::Space,
            indent_width: IndentWidth::default(),
            after_closing_brace: AfterClosingBrace::SameLine,
            import_order: ImportOrder::default(),
            javadoc: None,
            text_blocks: false,
            line_ending: LineEnding::Lf,
            insert_final_newline: true,
        }
    }
}

impl Options {
    pub(crate) fn import_order(&self) -> &ImportOrder {
        &self.import_order
    }
}

impl robusta_formatter_core::FormatOptions for Options {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }
}
