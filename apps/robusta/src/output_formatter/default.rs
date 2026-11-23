use std::{fmt::Write, path::Path};

use robusta_diagnostics::{
    Diagnostic, DiagnosticSummary, FileReport, GraphicalReportHandler, GraphicalTheme, Reporter,
};
use robusta_linter::{RuleId, table::RuleTable};
use robusta_syntax::line_terminator::{is_line_terminator, split_lines_with_endings};
use rustc_hash::FxHashSet;
use similar::{ChangeTag, TextDiff};

use crate::parallel::Sink;

const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_RESET: &str = "\x1b[0m";

/// The streaming graphical reporter for human-readable diagnostics.
///
/// It renders each file's findings through the graphical report handler the
/// moment the file is processed, and owns the unified-diff renderer every
/// `--diff` preview uses.
/// The diff is the one piece of human output the shared finding model cannot
/// express (it needs the before/after source, not a finding), so it stays a
/// reporter-owned renderer rather than flowing through [`Reporter::report`].
#[derive(Debug)]
pub struct DefaultReporter {
    handler: GraphicalReportHandler,
}

impl DefaultReporter {
    // Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxlint/src/output_formatter/default.rs
    pub fn all_rules(enabled: &FxHashSet<RuleId>) -> String {
        let mut output = String::new();
        let table = RuleTable::default();
        for section in &table.sections {
            output.push_str(&section.render_markdown_table_cli(enabled));
            output.push('\n');
        }
        writeln!(output, "Default: {}", table.turned_on_by_default_count).unwrap();
        writeln!(output, "Total: {}", table.total).unwrap();
        output
    }

    pub fn new(color: bool) -> Self {
        let theme = if color { GraphicalTheme::unicode() } else { GraphicalTheme::none() };
        Self { handler: GraphicalReportHandler::new_themed(theme) }
    }

    /// Render the concise default verification message for one formatting
    /// change. Formatting is scope work rather than a lint warning, so
    /// `--quiet` deliberately does not filter it.
    pub fn print_would_reformat(out: &mut impl Sink, path: &Path) {
        out.stdout_line(&format!("would reformat: {}", path.display()));
    }

    /// Render a unified diff of a file's candidate source to `out` (stdout).
    ///
    /// Lines are split by Robusta's own line-terminator policy, so `\n`, `\r`,
    /// and `\r\n` all delimit a line and each line keeps the terminator it was
    /// written with. Only the change detection and hunk grouping come from
    /// `similar`: unchanged runs longer than twice the context are elided, so a
    /// file with several distant changes prints several small hunks instead of
    /// one hunk spanning everything between the first and last change.
    pub fn print_unified_diff(
        out: &mut impl Sink,
        path: &Path,
        before: &str,
        after: &str,
        color: bool,
    ) {
        const CONTEXT_LINES: usize = 3;
        let before_lines = split_lines_with_endings(before).collect::<Vec<_>>();
        let after_lines = split_lines_with_endings(after).collect::<Vec<_>>();
        let diff =
            TextDiff::configure().newline_terminated(true).diff_slices(&before_lines, &after_lines);

        print_styled_line(out, &format!("--- {}", path.display()), ANSI_RED, color);
        print_styled_line(out, &format!("+++ {}", path.display()), ANSI_GREEN, color);
        for hunk in diff.grouped_ops(CONTEXT_LINES) {
            let (Some(first), Some(last)) = (hunk.first(), hunk.last()) else {
                continue;
            };
            let before_range = first.old_range().start..last.old_range().end;
            let after_range = first.new_range().start..last.new_range().end;
            print_styled_line(
                out,
                &format!(
                    "@@ -{},{} +{},{} @@",
                    diff_range_start(before_range.start, before_range.len()),
                    before_range.len(),
                    diff_range_start(after_range.start, after_range.len()),
                    after_range.len()
                ),
                ANSI_CYAN,
                color,
            );
            for change in hunk.iter().flat_map(|op| diff.iter_changes(op)) {
                let (marker, style) = match change.tag() {
                    ChangeTag::Equal => (' ', None),
                    ChangeTag::Delete => ('-', Some(ANSI_RED)),
                    ChangeTag::Insert => ('+', Some(ANSI_GREEN)),
                };
                print_diff_line(out, marker, change.value(), style, color);
            }
        }
    }
}

impl Reporter for DefaultReporter {
    fn report(&mut self, report: FileReport) -> Option<String> {
        if report.findings().is_empty() {
            return None;
        }
        let mut rendered = String::new();
        self.handler
            .render_reports(
                &mut rendered,
                report.source(),
                report.findings().iter().map(|finding| finding as &dyn Diagnostic),
            )
            .expect("rendering diagnostics to a string cannot fail");
        Some(rendered)
    }

    /// Findings are already on the stream, so all that remains is the totals
    /// line. It is emitted even for a clean run; the blank line separates it
    /// from the last rendered finding.
    fn finish(&mut self, summary: DiagnosticSummary) -> Option<String> {
        let DiagnosticSummary { warnings, errors } = summary;
        let separator = if warnings + errors > 0 { "\n" } else { "" };
        Some(format!(
            "{separator}Found {warnings} warning{} and {errors} error{}.\n",
            plural(warnings),
            plural(errors)
        ))
    }
}

const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn diff_range_start(start_index: usize, count: usize) -> usize {
    if count == 0 { start_index } else { start_index + 1 }
}

fn print_styled_line(out: &mut impl Sink, line: &str, style: &str, color: bool) {
    if color {
        out.stdout_line(&format!("{style}{line}{ANSI_RESET}"));
    } else {
        out.stdout_line(line);
    }
}

fn print_diff_line(
    out: &mut impl Sink,
    prefix: char,
    line: &str,
    style: Option<&str>,
    color: bool,
) {
    let ending_start = line.bytes().position(is_line_terminator).unwrap_or(line.len());
    let (content, ending) = line.split_at(ending_start);
    if color && let Some(style) = style {
        out.stdout(&format!("{style}{prefix}{content}{ANSI_RESET}{ending}"));
    } else {
        out.stdout(&format!("{prefix}{line}"));
    }
    if ending.is_empty() {
        out.stdout_line("");
        out.stdout_line("\\ No newline at end of file");
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use std::path::Path;

    use robusta_diagnostics::{
        DiagnosticSummary, FileReport, Finding, Reporter, RobustaDiagnostic, RuleInfo, Severity,
        SourceLocator, labeled_span, primary_labeled_span,
    };
    use robusta_span::Span;

    use super::DefaultReporter;

    const RULE: RuleInfo = RuleInfo { id: "demo", summary: None };

    fn offset(source: &str, needle: &str) -> u32 {
        u32::try_from(source.find(needle).expect("test source should contain the span"))
            .expect("test source should fit in a u32 span")
    }

    fn render(source: &str, span: Span) -> String {
        let finding = Finding::new(
            RULE,
            Severity::Warning,
            "demonstration finding",
            span,
            SourceLocator::new(source).range(span),
        );
        let report = FileReport::new(Path::new("Example.java"), source, None, vec![finding]);
        DefaultReporter::new(false).report(report).expect("report should render")
    }

    #[test]
    fn totals_pluralize_each_count_and_lead_with_a_blank_line_only_after_findings() {
        let mut reporter = DefaultReporter::new(false);
        assert_eq!(
            reporter.finish(DiagnosticSummary::default()).as_deref(),
            Some("Found 0 warnings and 0 errors.\n")
        );
        assert_eq!(
            reporter.finish(DiagnosticSummary { warnings: 1, errors: 1 }).as_deref(),
            Some("\nFound 1 warning and 1 error.\n")
        );
        assert_eq!(
            reporter.finish(DiagnosticSummary { warnings: 6, errors: 4 }).as_deref(),
            Some("\nFound 6 warnings and 4 errors.\n")
        );
    }

    #[test]
    fn renders_multi_line_span() {
        let source = "class Example {\n    void call() {\n        first(\n            one,\n            two);\n    }\n}\n";
        let start = offset(source, "one");
        let end = offset(source, "two") + 3;

        insta::assert_snapshot!(render(source, Span::new(start, end)), @r###"

         ! demo: demonstration finding
          ,-[Example.java:4:13]
        3 |             first(
        4 | ,->             one,
        5 | `->             two);
        6 |         }
          `----
        "###);
    }

    #[test]
    fn renders_tabbed_source() {
        let source = "class Example {\n\tlong value = 10l;\n}\n";
        let start = offset(source, "10l");

        insta::assert_snapshot!(render(source, Span::new(start, start + 3)), @r###"

         ! demo: demonstration finding
          ,-[Example.java:2:15]
        1 | class Example {
        2 |     long value = 10l;
          :                  ^^^
        3 | }
          `----
        "###);
    }

    #[test]
    fn renders_wide_source_character() {
        let source = "class Example {\n    String value = \"文件\";\n}\n";
        let start = offset(source, "文件");

        insta::assert_snapshot!(render(source, Span::new(start, start + 6)), @r###"

         ! demo: demonstration finding
          ,-[Example.java:2:21]
        1 | class Example {
        2 |     String value = "文件";
          :                     ^^^^
        3 | }
          `----
        "###);
    }

    #[test]
    fn renders_multiple_labels_help_and_note_without_color() {
        let source = "public static final class Example {}\n";
        let public = Span::new(offset(source, "public"), offset(source, "public") + 6);
        let static_span = Span::new(offset(source, "static"), offset(source, "static") + 6);
        let diagnostic =
            RobustaDiagnostic::warning("demonstration finding", public.cover(static_span))
                .with_labels([
                    primary_labeled_span(static_span, "move this modifier"),
                    labeled_span(public, "before this modifier"),
                ])
                .with_help("Reorder the modifiers.")
                .with_note("Java uses a customary modifier order.");
        let finding = Finding::from_diagnostic(
            RULE,
            diagnostic,
            SourceLocator::new(source).range(static_span),
        );
        let report = FileReport::new(Path::new("Example.java"), source, None, vec![finding]);
        let rendered = DefaultReporter::new(false).report(report).expect("report should render");

        insta::assert_snapshot!(rendered, @r###"

         ! demo: demonstration finding
          ,-[Example.java:1:8]
        1 | public static final class Example {}
          : ^^^|^^ ^^^|^^
          :    |      `-- move this modifier
          :    `-- before this modifier
          `----
         help: Reorder the modifiers.
         note: Java uses a customary modifier order.
        "###);
    }
}
