use std::fmt::Write as _;

use robusta_diagnostics::{FileReport, Severity, SourcePosition};

use super::Info;

const UNDERLINE: &str = "\x1b[4m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

/// Render ESLint's `stylish` layout: a heading per file, one line per finding
/// with its position padded to the file's widest, and a closing problem count.
///
/// Files keep the run's selection order and findings are ordered by position
/// within each file, so the document is a function of the sources alone. The
/// counts are of the findings actually rendered, so `--quiet` shrinks them just
/// as it shrinks the block above; the run's unfiltered totals belong to the
/// exit status, not to this document.
pub(super) fn render(reports: &[&FileReport], color: bool) -> String {
    let mut out = String::new();
    let mut errors = 0usize;
    let mut warnings = 0usize;

    for report in reports {
        // The block is laid out only after its widest position is known, so
        // this format resolves a whole file's items before writing any of them.
        let mut entries: Vec<Info<'_>> = Info::of(report).collect();
        if entries.is_empty() {
            continue;
        }
        entries.sort_by_key(|entry| (entry.start.line, entry.start.column));

        let width = entries.iter().map(|entry| position(entry.start).len()).max().unwrap_or(0);
        out.push('\n');
        if color {
            let _ = writeln!(out, "{UNDERLINE}{}{RESET}", report.path());
        } else {
            let _ = writeln!(out, "{}", report.path());
        }

        for entry in &entries {
            match entry.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
                // The closing line counts only the two severities it names, so
                // advice prints its own entry without joining the problem total.
                Severity::Advice => {}
            }
            let position = position(entry.start);
            let severity = severity(entry.severity);
            let message = entry.message;
            if color {
                let color = severity_color(entry.severity);
                let _ = write!(
                    out,
                    "  {DIM}{position:width$}{RESET}  {color}{severity}{RESET}  {message}"
                );
            } else {
                let _ = write!(out, "  {position:width$}  {severity}  {message}");
            }
            if !entry.rule.id.is_empty() {
                if color {
                    let _ = write!(out, "  {DIM}{}{RESET}", entry.rule.id);
                } else {
                    let _ = write!(out, "  {}", entry.rule.id);
                }
            }
            out.push('\n');
        }
    }

    let total = errors + warnings;
    if total > 0 {
        let (open, close) =
            if color { (if errors > 0 { RED } else { YELLOW }, RESET) } else { ("", "") };
        let _ = write!(
            out,
            "\n{open}\u{2716} {total} problem{} ({errors} error{}, {warnings} warning{}){close}\n",
            plural(total),
            plural(errors),
            plural(warnings),
        );
    }
    out
}

fn position(position: SourcePosition) -> String {
    format!("{}:{}", position.line, position.column)
}

fn severity(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Advice => "advice",
    }
}

const fn severity_color(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => RED,
        _ => YELLOW,
    }
}

const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, RULELESS, finding, report_with};
    use super::*;

    #[test]
    fn aligns_each_file_block_and_closes_with_the_problem_count() {
        let first = report_with(
            "src/First.java",
            Some(false),
            vec![
                finding(DEMO_RULE, Severity::Error, "use uppercase `L`", 10, 5),
                finding(RULELESS, Severity::Warning, "bare finding", 2, 17),
            ],
        );
        let second = report_with(
            "src/Second.java",
            None,
            vec![finding(DEMO_RULE, Severity::Warning, "modifiers are out of order", 3, 9)],
        );

        insta::assert_snapshot!(render(&[&first, &second], false), @r###"

        src/First.java
          1:1   warning  file is not formatted; run `robusta format --fix`  format
          2:17  warning  bare finding
          10:5  error  use uppercase `L`  demo

        src/Second.java
          3:9  warning  modifiers are out of order  demo

        ✖ 4 problems (1 error, 3 warnings)
        "###);
    }

    #[test]
    fn a_single_warning_is_singular_and_uncounted_files_are_skipped() {
        let clean = report_with("src/Clean.java", Some(true), Vec::new());
        let dirty = report_with("src/Dirty.java", Some(false), Vec::new());

        insta::assert_snapshot!(render(&[&clean, &dirty], false), @r###"

        src/Dirty.java
          1:1  warning  file is not formatted; run `robusta format --fix`  format

        ✖ 1 problem (0 errors, 1 warning)
        "###);
        assert_eq!(render(&[&clean], false), "");
    }

    #[test]
    fn color_underlines_the_path_dims_the_position_and_matches_the_worst_severity() {
        let report = report_with(
            "src/C.java",
            None,
            vec![
                finding(DEMO_RULE, Severity::Warning, "a warning", 1, 1),
                finding(DEMO_RULE, Severity::Error, "an error", 2, 1),
            ],
        );

        let rendered = render(&[&report], true);
        assert!(rendered.contains(&format!("{UNDERLINE}src/C.java{RESET}")), "{rendered:?}");
        assert!(rendered.contains(&format!("{DIM}1:1{RESET}")), "{rendered:?}");
        assert!(rendered.contains(&format!("{YELLOW}warning{RESET}")), "{rendered:?}");
        assert!(rendered.contains(&format!("{RED}error{RESET}")), "{rendered:?}");
        assert!(rendered.contains(&format!("{RED}\u{2716} 2 problems")), "{rendered:?}");

        let warnings_only = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Warning, "a warning", 1, 1)],
        );
        assert!(
            render(&[&warnings_only], true).contains(&format!("{YELLOW}\u{2716} 1 problem")),
            "a run without errors summarizes in the warning color"
        );
    }

    #[test]
    fn advice_is_listed_but_counts_toward_no_total() {
        let report = report_with(
            "src/C.java",
            None,
            vec![
                finding(DEMO_RULE, Severity::Advice, "consider this", 2, 5),
                finding(DEMO_RULE, Severity::Warning, "a warning", 3, 1),
            ],
        );
        let rendered = render(&[&report], false);
        assert!(rendered.contains("advice  consider this"), "advice keeps its own entry");
        assert!(
            rendered.contains("1 problem (0 errors, 1 warning)"),
            "the closing line counts only the severities it names, got: {rendered}"
        );
    }
}
