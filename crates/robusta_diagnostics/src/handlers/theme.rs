// Adapted from https://github.com/oxc-project/oxc/blob/ac4785a13909f6db5c986b0a2bc11fc4d8c3cc3c/crates/oxc_diagnostics/src/handlers/theme.rs

use std::fmt;

/// Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler).
///
/// Use one of the predefined constructors below.
#[derive(Debug, Clone)]
pub struct GraphicalTheme {
    pub(super) characters: ThemeCharacters,
    pub(super) styles: ThemeStyles,
}

// The caller resolves color once per output stream.
impl GraphicalTheme {
    /// Graphical theme that draws using both RGB ANSI colors and unicode
    /// characters.
    #[must_use]
    pub fn unicode() -> Self {
        Self { characters: ThemeCharacters::unicode(), styles: ThemeStyles::rgb() }
    }

    /// A "basic" graphical theme that skips colors and unicode characters and
    /// just does monochrome ASCII art.
    #[must_use]
    pub fn none() -> Self {
        Self { characters: ThemeCharacters::ascii(), styles: ThemeStyles::none() }
    }
}

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_PREFIXES: [&str; 9] = [
    "",
    "\x1b[38;2;225;80;80;1m",
    "\x1b[38;2;244;191;117;1m",
    "\x1b[38;2;106;159;181m",
    "\x1b[38;2;92;157;255;1m",
    "\x1b[2m",
    "\x1b[38;2;246;87;248m",
    "\x1b[38;2;30;201;212m",
    "\x1b[38;2;145;246;111m",
];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub(super) enum Style {
    Plain,
    Error,
    Warning,
    Info,
    Link,
    LineNumber,
    Highlight1,
    Highlight2,
    Highlight3,
}

impl Style {
    const fn prefix(self) -> &'static str {
        ANSI_PREFIXES[self as usize]
    }

    pub(super) const fn is_plain(self) -> bool {
        matches!(self, Self::Plain)
    }
}

pub(super) struct Styled<T> {
    target: T,
    style: Style,
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self.style.prefix();
        f.write_str(prefix)?;
        self.target.fmt(f)?;
        if self.style.is_plain() { Ok(()) } else { f.write_str(ANSI_RESET) }
    }
}

pub(super) trait DiagnosticColorize: Sized {
    fn style(self, style: Style) -> Styled<Self> {
        Styled { target: self, style }
    }
}

impl<T> DiagnosticColorize for T {}

#[derive(Debug, Clone)]
pub(super) struct ThemeStyles {
    pub(super) error: Style,
    pub(super) warning: Style,
    pub(super) advice: Style,
    pub(super) help: Style,
    pub(super) note: Style,
    pub(super) link: Style,
    pub(super) linum: Style,
    pub(super) highlights: [Style; 3],
}

impl ThemeStyles {
    fn rgb() -> Self {
        Self {
            error: Style::Error,
            warning: Style::Warning,
            advice: Style::Info,
            help: Style::Info,
            note: Style::Info,
            link: Style::Link,
            linum: Style::LineNumber,
            highlights: [Style::Highlight1, Style::Highlight2, Style::Highlight3],
        }
    }

    fn none() -> Self {
        Self {
            error: Style::Plain,
            warning: Style::Plain,
            advice: Style::Plain,
            help: Style::Plain,
            note: Style::Plain,
            link: Style::Plain,
            linum: Style::Plain,
            highlights: [Style::Plain; 3],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct ThemeCharacters {
    pub(super) hbar: char,
    pub(super) vbar: char,
    pub(super) vbar_break: char,

    pub(super) uarrow: char,
    pub(super) rarrow: char,

    pub(super) ltop: char,
    pub(super) lbot: char,

    pub(super) lcross: char,
    pub(super) rcross: char,

    pub(super) underbar: char,
    pub(super) underline: char,

    pub(super) error: &'static str,
    pub(super) warning: &'static str,
    pub(super) advice: &'static str,
}

impl ThemeCharacters {
    const fn unicode() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            lbot: '╰',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            error: "×",
            warning: "⚠",
            advice: "☞",
        }
    }

    const fn ascii() -> Self {
        Self {
            hbar: '-',
            vbar: '|',
            vbar_break: ':',
            uarrow: '^',
            rarrow: '>',
            ltop: ',',
            lbot: '`',
            lcross: '|',
            rcross: '|',
            underbar: '|',
            underline: '^',
            error: "x",
            warning: "!",
            advice: ">",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagnosticColorize, Style};

    #[test]
    fn styled_keeps_padding_inside_escapes() {
        assert_eq!(format!("{:3}", 7.style(Style::LineNumber)), "\x1b[2m  7\x1b[0m");
        assert_eq!(format!("{:<4}", "×".style(Style::Error)), "\x1b[38;2;225;80;80;1m×   \x1b[0m");
        assert_eq!(format!("{:3}", 7.style(Style::Plain)), "  7");
    }
}
