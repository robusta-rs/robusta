use std::{fmt, str::FromStr};

// Language-agnostic format options shared by all formatter implementations.
//
// Part of [`crate::FormatOptions`] because the printing phase consumes these options.
// NOTE: Do NOT define language-specific options here, even if they look like they could be shared.

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
pub enum IndentStyle {
    /// Tab
    Tab,
    /// Space
    #[default]
    Space,
}

impl IndentStyle {
    pub const DEFAULT_SPACES: u8 = 2;

    /// Returns `true` if this is an [IndentStyle::Tab].
    pub const fn is_tab(self) -> bool {
        matches!(self, IndentStyle::Tab)
    }

    /// Returns `true` if this is an [IndentStyle::Space].
    pub const fn is_space(self) -> bool {
        matches!(self, IndentStyle::Space)
    }
}

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IndentStyle::Tab => "Tab",
            IndentStyle::Space => "Space",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum LineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    #[default]
    Lf,
    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    Crlf,
    /// Carriage Return character only (\r), used very rarely
    Cr,
}

impl LineEnding {
    #[inline]
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            LineEnding::Lf => b"\n",
            LineEnding::Crlf => b"\r\n",
            LineEnding::Cr => b"\r",
        }
    }

    /// Returns `true` if this is a [LineEnding::Lf].
    pub const fn is_line_feed(self) -> bool {
        matches!(self, LineEnding::Lf)
    }

    /// Returns `true` if this is a [LineEnding::Crlf].
    pub const fn is_carriage_return_line_feed(self) -> bool {
        matches!(self, LineEnding::Crlf)
    }

    /// Returns `true` if this is a [LineEnding::Cr].
    pub const fn is_carriage_return(self) -> bool {
        matches!(self, LineEnding::Cr)
    }
}

impl FromStr for LineEnding {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lf" => Ok(Self::Lf),
            "crlf" => Ok(Self::Crlf),
            "cr" => Ok(Self::Cr),
            _ => Err("Value not supported for LineEnding"),
        }
    }
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LineEnding::Lf => "LF",
            LineEnding::Crlf => "CRLF",
            LineEnding::Cr => "CR",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct IndentWidth(u8);

impl IndentWidth {
    pub const MAX: u8 = 24;
    pub const MIN: u8 = 0;

    /// Return the numeric value for this [IndentWidth]
    pub fn value(self) -> u8 {
        self.0
    }
}

impl Default for IndentWidth {
    fn default() -> Self {
        Self(2)
    }
}

impl TryFrom<u8> for IndentWidth {
    type Error = IndentWidthFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(IndentWidthFromIntError(value))
        }
    }
}

impl fmt::Display for IndentWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value();
        f.write_str(&std::format!("{value}"))
    }
}

impl fmt::Debug for IndentWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Validated value for the `line_width` formatter options
///
/// The allowed range of values is 1..=320
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct LineWidth(u16);

impl LineWidth {
    /// Maximum allowed value for a valid [LineWidth]
    pub const MAX: u16 = 320;
    /// Minimum allowed value for a valid [LineWidth]
    pub const MIN: u16 = 1;

    /// Return the numeric value for this [LineWidth]
    pub fn value(self) -> u16 {
        self.0
    }
}

impl Default for LineWidth {
    fn default() -> Self {
        Self(100)
    }
}

impl fmt::Display for LineWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value();
        f.write_str(&std::format!("{value}"))
    }
}

impl fmt::Debug for LineWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl TryFrom<u16> for LineWidth {
    type Error = LineWidthFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(LineWidthFromIntError(value))
        }
    }
}

/// Error type returned when converting a `u8` to an [`IndentWidth`] fails.
#[derive(Clone, Copy, Debug)]
pub struct IndentWidthFromIntError(pub u8);

impl fmt::Display for IndentWidthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The indent width should be between {} and {}",
            IndentWidth::MIN,
            IndentWidth::MAX
        )
    }
}

/// Error type returned when converting a `u16` to a [`LineWidth`] fails.
#[derive(Clone, Copy, Debug)]
pub struct LineWidthFromIntError(pub u16);

impl fmt::Display for LineWidthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The line width should be between {} and {}", LineWidth::MIN, LineWidth::MAX)
    }
}

impl From<LineWidth> for u16 {
    fn from(value: LineWidth) -> Self {
        value.0
    }
}

/// The four fields shared by every formatter, i.e. exactly the getters of
/// [`crate::FormatOptions`].
///
/// This is the minimal [`crate::FormatOptions`] on its own, and is what the
/// simple context uses for tests and debug rendering. A language formatter
/// defines its own options type instead, adding its language-specific fields
/// beside these four.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct CoreFormatOptions {
    pub indent_style: IndentStyle,
    pub indent_width: IndentWidth,
    pub line_width: LineWidth,
    pub line_ending: LineEnding,
}

impl crate::FormatOptions for CoreFormatOptions {
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

#[cfg(test)]
mod tests {
    use super::{IndentWidth, LineWidth};

    #[test]
    fn width_errors_report_their_own_ranges() {
        assert_eq!(
            IndentWidth::try_from(25).unwrap_err().to_string(),
            "The indent width should be between 0 and 24"
        );
        assert_eq!(
            LineWidth::try_from(321).unwrap_err().to_string(),
            "The line width should be between 1 and 320"
        );
    }
}
