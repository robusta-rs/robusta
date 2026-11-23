//! Diagnostic-level rendering: everything except the source snippets.
//!
//! [`render_report`](GraphicalReportHandler::render_report) renders one
//! diagnostic; [`render_reports`](GraphicalReportHandler::render_reports)
//! renders a source-sharing batch with one line index. Both render the title, hand off to
//! [`render_snippets`](GraphicalReportHandler::render_snippets), then renders
//! the help/note footer. Each block of prose is wrapped to the terminal width
//! using the shared [`wrap_options`](GraphicalReportHandler::wrap_options)
//! helper.

use std::fmt::{self, Write as _};

use smallvec::SmallVec;

use super::{handler::GraphicalReportHandler, text::escape_controls};
use crate::{
    Diagnostic, Severity, SourceCode, handlers::theme::DiagnosticColorize,
    source_impls::SpanScanner,
};

struct TitleBuffer(SmallVec<[u8; 128]>);

impl TitleBuffer {
    fn new() -> Self {
        Self(SmallVec::new())
    }

    fn as_str(&self) -> &str {
        // SAFETY: writes only append valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl fmt::Write for TitleBuffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.0.extend_from_slice(text.as_bytes());
        Ok(())
    }
}

impl GraphicalReportHandler {
    /// Render a [`Diagnostic`].
    ///
    /// # Errors
    ///
    /// Returns an error when writing the rendered report fails.
    pub fn render_report(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> fmt::Result {
        let source = diagnostic.source_code();
        let mut scanner = source.map(|source| SpanScanner::new(source.data(), 1, 1));
        let source_name = source.and_then(|source| source.name());
        self.render_report_with_scanner(f, diagnostic, scanner.as_mut(), source_name)
    }

    /// Render [`Diagnostic`]s in order against one source, reusing its line index.
    ///
    /// # Errors
    ///
    /// Returns an error when writing the rendered reports fails.
    pub fn render_reports<'a>(
        &self,
        f: &mut impl fmt::Write,
        source: &dyn SourceCode,
        diagnostics: impl IntoIterator<Item = &'a dyn Diagnostic>,
    ) -> fmt::Result {
        let mut scanner = SpanScanner::new(source.data(), 1, 1);
        let source_name = source.name();
        for diagnostic in diagnostics {
            self.render_report_with_scanner(f, diagnostic, Some(&mut scanner), source_name)?;
        }
        Ok(())
    }

    fn render_report_with_scanner(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &dyn Diagnostic,
        scanner: Option<&mut SpanScanner<'_>>,
        source_name: Option<&str>,
    ) -> fmt::Result {
        writeln!(f)?;
        self.render_title(f, diagnostic)?;
        self.render_snippets(f, diagnostic, scanner, source_name)?;
        self.render_footer(f, diagnostic)?;
        Ok(())
    }

    fn render_title(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result {
        let (severity_style, severity_icon) = match diagnostic.severity() {
            Some(Severity::Error) | None => (self.theme.styles.error, &self.theme.characters.error),
            Some(Severity::Warning) => (self.theme.styles.warning, &self.theme.characters.warning),
            Some(Severity::Advice) => (self.theme.styles.advice, &self.theme.characters.advice),
        };

        let width = self.termwidth.saturating_sub(2);

        let mut title = TitleBuffer::new();
        match diagnostic.code() {
            Some(code) => write!(title, "{code}: {diagnostic}")?,
            None => write!(title, "{diagnostic}")?,
        }
        let escaped = escape_controls(title.as_str());
        let mut styled = TitleBuffer::new();
        let title = if severity_style.is_plain() {
            escaped.as_ref()
        } else {
            write!(styled, "{}", escaped.style(severity_style))?;
            styled.as_str()
        };
        if !title.contains('\n')
            && severity_icon.len().saturating_add(title.len()).saturating_add(3) <= width
        {
            f.write_str("  ")?;
            if severity_style.is_plain() {
                f.write_str(severity_icon)?;
            } else {
                write!(f, "{}", severity_icon.style(severity_style))?;
            }
            f.write_char(' ')?;
            f.write_str(title.trim_end_matches(' '))?;
        } else {
            // No-color themes skip the styling path entirely.
            let (initial_indent, rest_indent) = if severity_style.is_plain() {
                (format!("  {severity_icon} "), format!("  {} ", self.theme.characters.vbar))
            } else {
                (
                    format!("  {} ", severity_icon.style(severity_style)),
                    format!("  {} ", self.theme.characters.vbar.style(severity_style)),
                )
            };
            let opts = Self::wrap_options(width, &initial_indent, &rest_indent);
            Self::write_fill(f, title, opts)?;
        }
        f.write_char('\n')?;

        Ok(())
    }

    fn render_footer(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result {
        if let Some(help) = diagnostic.help() {
            const PREFIX: &str = "  help: ";
            let help = escape_controls(&help);
            let width = self.termwidth.saturating_sub(4);
            if memchr::memchr(b'\n', help.as_bytes()).is_none()
                && PREFIX.len().saturating_add(help.len()) <= width
            {
                if self.theme.styles.help.is_plain() {
                    f.write_str(PREFIX)?;
                } else {
                    write!(f, "{}", PREFIX.style(self.theme.styles.help))?;
                }
                f.write_str(help.trim_end_matches(' '))?;
            } else {
                let initial_indent = PREFIX.style(self.theme.styles.help).to_string();
                let opts = Self::wrap_options(width, &initial_indent, "        ");
                Self::write_fill(f, &help, opts)?;
            }
            f.write_char('\n')?;
        }
        if let Some(note) = diagnostic.note() {
            let note = escape_controls(&note);
            // Renders as:
            //   note: This is a note about the error
            let width = self.termwidth.saturating_sub(4);
            let initial_indent = "  note: ".style(self.theme.styles.note).to_string();
            let opts = Self::wrap_options(width, &initial_indent, "           ");
            Self::write_fill(f, &note, opts)?;
            f.write_char('\n')?;
        }
        Ok(())
    }

    /// Builds the [`textwrap::Options`] shared by every wrapped block.
    fn wrap_options<'a>(
        width: usize,
        initial_indent: &'a str,
        subsequent_indent: &'a str,
    ) -> textwrap::Options<'a> {
        textwrap::Options::new(width)
            .initial_indent(initial_indent)
            .subsequent_indent(subsequent_indent)
    }

    fn write_fill(f: &mut impl fmt::Write, text: &str, opts: textwrap::Options<'_>) -> fmt::Result {
        if Self::fits_on_line(text, &opts) {
            f.write_str(opts.initial_indent)?;
            f.write_str(text.trim_end_matches(' '))
        } else {
            f.write_str(&textwrap::fill(text, opts))
        }
    }

    /// Skip word separation and optimal-fit layout when the text demonstrably
    /// fits on its first line. `textwrap` only provides this fast path without
    /// indentation, while every diagnostic block has an initial indent.
    #[cfg(test)]
    fn fill(text: &str, opts: textwrap::Options<'_>) -> String {
        if Self::fits_on_line(text, &opts) {
            let text = text.trim_end_matches(' ');
            let mut result = String::with_capacity(opts.initial_indent.len() + text.len());
            result.push_str(opts.initial_indent);
            result.push_str(text);
            return result;
        }
        textwrap::fill(text, opts)
    }

    fn fits_on_line(text: &str, opts: &textwrap::Options<'_>) -> bool {
        if memchr::memchr(b'\n', text.as_bytes()).is_some() {
            return false;
        }

        // UTF-8 byte length is an upper bound on terminal display width,
        // including for ANSI escape sequences. Avoid both width scans when
        // even that conservative bound fits.
        opts.initial_indent.len().saturating_add(text.len()) <= opts.width || {
            let available = opts.width.saturating_sub(Self::display_width(opts.initial_indent));
            Self::display_width(text) <= available
        }
    }

    /// Compute terminal width bytewise for ASCII, including the CSI and OSC
    /// escape sequences recognized by `textwrap`. Unicode retains its full
    /// width calculation.
    fn display_width(text: &str) -> usize {
        if !text.is_ascii() {
            return textwrap::core::display_width(text);
        }

        let bytes = text.as_bytes();
        let mut width = 0;
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] != b'\x1b' {
                width += usize::from((b' '..=b'~').contains(&bytes[i]));
                i += 1;
                continue;
            }

            i += 1;
            let Some(&kind) = bytes.get(i) else { break };
            i += 1;
            match kind {
                b'[' => {
                    while i < bytes.len() {
                        let byte = bytes[i];
                        i += 1;
                        if (b'@'..=b'~').contains(&byte) {
                            break;
                        }
                    }
                }
                b']' => {
                    while i < bytes.len() {
                        if bytes[i] == b'\x07' {
                            i += 1;
                            break;
                        }
                        if bytes[i] == b'\x1b' && bytes.get(i + 1) == Some(&b'\\') {
                            i += 2;
                            break;
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
        }
        width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GraphicalTheme, NamedSource, RobustaDiagnostic, primary_labeled_span};
    use robusta_span::Span;

    #[test]
    fn controls_are_visible_in_every_report_field() {
        const RAW: &str = "\u{1b}[2J\u{1b}]0;title\u{7}\u{8}\0\u{7f}\u{85}";
        const VISIBLE: &str = "␛[2J␛]0;title␇␈␀␡<U+0085>";
        let source = NamedSource::new(format!("{RAW}.java"), format!("{RAW}\nx"));
        let diagnostic = RobustaDiagnostic::error(RAW, Span::new(0, 1))
            .with_help(RAW)
            .with_note(RAW)
            .with_labels([
                primary_labeled_span(Span::new(0, 1), format!("{RAW}\n{RAW}")),
                primary_labeled_span(
                    Span::new(0, u32::try_from(source.data().len()).unwrap()),
                    format!("{RAW}\n{RAW}"),
                ),
            ])
            .with_source_code(source);
        for theme in [GraphicalTheme::none(), GraphicalTheme::unicode()] {
            let colored = !theme.styles.error.is_plain();
            let handler = GraphicalReportHandler::new_themed(theme);
            let mut output = String::new();
            handler.render_report(&mut output, diagnostic.as_ref()).unwrap();
            assert_eq!(output.matches(VISIBLE).count(), 9, "{output:?}");
            assert_eq!(output.contains('\u{1b}'), colored);
            // Only renderer-owned SGR styles may reach the terminal.
            let mut chars = output.chars();
            while let Some(c) = chars.next() {
                if c == '\u{1b}' {
                    assert_eq!(chars.next(), Some('['));
                    for c in chars.by_ref() {
                        if c == 'm' {
                            break;
                        }
                        assert!(c.is_ascii_digit() || c == ';');
                    }
                } else {
                    assert!(!c.is_control() || c == '\n', "{output:?}");
                }
            }
        }
    }

    #[test]
    fn source_controls_do_not_move_or_shorten_underlines() {
        for (text, start, end, expected) in [
            ("/* \u{8}\u{8}\u{8} */ int y = ;", 14, 15, "   :               ^"),
            ("import java.util.Li\u{1b}cst;", 17, 23, "   :                  ^^^^^^"),
            ("\u{85}\tX", 3, 4, "   :             ^"),
        ] {
            let diagnostic = RobustaDiagnostic::error("problem", Span::new(start, end))
                .with_source_code(NamedSource::new("Example.java", text.to_owned()));
            let mut output = String::new();
            GraphicalReportHandler::new_themed(GraphicalTheme::none())
                .render_report(&mut output, diagnostic.as_ref())
                .unwrap();
            assert!(output.lines().any(|line| line == expected), "{output:?}");
        }
    }

    #[test]
    fn visible_controls_are_wrapped_at_their_printed_width() {
        let diagnostic = RobustaDiagnostic::error("\u{85} \u{85} \u{85}", Span::new(0, 0))
            .with_help("\u{85} \u{85} \u{85}")
            .with_note("\u{85} \u{85} \u{85}");
        let mut output = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::none())
            .with_width(24)
            .render_report(&mut output, &diagnostic)
            .unwrap();
        assert_eq!(output.matches("<U+0085>").count(), 9);
        assert!(output.lines().all(|line| line.len() <= 24), "{output}");
        assert!(!output.contains('\u{85}'));
    }

    #[test]
    fn plain_line_numbers_and_locations_pad_across_digit_counts() {
        let text = "x\n".repeat(100);
        for (start, top, location, gutter) in [
            (
                16,
                "    ,-",
                "9:1]",
                "  8 |     x\n  9 | ,-> x\n 10 | `-> x\n 11 |     x\n    `----\n",
            ),
            (196, "     ,-", "99:1]", "  98 |     x\n  99 | ,-> x\n 100 | `-> x\n     `----\n"),
        ] {
            let diagnostic = RobustaDiagnostic::error("problem", Span::new(start, start + 3));
            for (source_name, header) in [(None, "["), (Some("Example.java"), "[Example.java:")] {
                let diagnostic = match source_name {
                    Some(name) => {
                        diagnostic.clone().with_source_code(NamedSource::new(name, text.clone()))
                    }
                    None => diagnostic.clone().with_source_code(text.clone()),
                };
                let mut output = String::new();
                GraphicalReportHandler::new_themed(GraphicalTheme::none())
                    .render_report(&mut output, diagnostic.as_ref())
                    .unwrap();
                let expected = format!("\n  x problem\n{top}{header}{location}\n{gutter}");
                assert_eq!(output, expected);
            }
        }
    }

    #[test]
    fn batch_render_matches_individual_reports() {
        let source = NamedSource::new(
            "Example.java",
            "class Example {\n    int first;\n    int second;\n}\n".to_string(),
        );
        let diagnostics = [
            RobustaDiagnostic::warning("later", Span::new(41, 47)),
            RobustaDiagnostic::error("multi-label", Span::new(6, 13)).with_labels([
                primary_labeled_span(Span::new(6, 13), "type name"),
                primary_labeled_span(Span::new(24, 29), "first field"),
            ]),
            RobustaDiagnostic::warning("earlier", Span::new(24, 29))
                .with_help("Review this field."),
        ];
        let handler = GraphicalReportHandler::new_themed(GraphicalTheme::none());

        let mut expected = String::new();
        for diagnostic in &diagnostics {
            let diagnostic = diagnostic.clone().with_source_code(source.clone());
            handler.render_report(&mut expected, diagnostic.as_ref()).unwrap();
        }

        let mut actual = String::new();
        handler
            .render_reports(
                &mut actual,
                &source,
                diagnostics.iter().map(|diagnostic| diagnostic as &dyn Diagnostic),
            )
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "exhaustive equivalence check over safe text wrapping code; interpreting every \
                  textwrap case under Miri takes more than 16 minutes"
    )]
    fn fill_fast_path_matches_textwrap() {
        let texts = [
            "",
            "short diagnostic",
            "trailing spaces   ",
            "  leading spaces",
            "two  inner  spaces",
            "Café 火",
            "combining e\u{301}",
            "emoji 🐂",
            "\u{1b}[31mstyled text\u{1b}[0m",
            "\u{1b}]8;;https://example.com\u{1b}\\linked\u{1b}]8;;\u{1b}\\",
            "\u{1b}]0;title\u{7}visible",
            "control\tcharacters\u{7}",
            "incomplete \u{1b}[31",
            "first\nsecond",
        ];
        for width in 0..32 {
            for initial_indent in ["", "  ", "  help: ", "\u{1b}[31m  × \u{1b}[0m"] {
                for text in texts {
                    let opts = textwrap::Options::new(width)
                        .initial_indent(initial_indent)
                        .subsequent_indent("    ");
                    assert_eq!(
                        GraphicalReportHandler::fill(text, opts.clone()),
                        textwrap::fill(text, opts.clone()),
                        "width={width}, indent={initial_indent:?}, text={text:?}"
                    );
                    let mut output = String::new();
                    GraphicalReportHandler::write_fill(&mut output, text, opts.clone()).unwrap();
                    assert_eq!(
                        output,
                        textwrap::fill(text, opts),
                        "streaming: width={width}, indent={initial_indent:?}, text={text:?}"
                    );
                }
            }
        }
    }
}
