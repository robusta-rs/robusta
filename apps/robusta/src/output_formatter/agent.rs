use std::fmt::Write as _;

use robusta_diagnostics::{FileReport, Severity};

use super::Info;

/// Render one compact physical line per finding, with no source excerpt or
/// completion summary. The enclosing [`super::BufferedReporter`] holds these
/// lines until the run succeeds so operational failures cannot leak a partial
/// report.
pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut out = String::new();
    for info in Info::of_all(reports) {
        agent_finding(&mut out, &info);
    }
    out
}

/// This is the one format that reads an unresolved position as absence rather
/// than printing its zero coordinates.
fn agent_finding(out: &mut String, info: &Info<'_>) {
    let Info { start, filename, message, severity, rule, help, .. } = info;
    out.push_str(filename);
    if info.has_position() {
        let _ = write!(out, ":{}:{}", start.line, start.column);
    }
    out.push_str(": ");
    out.push_str(match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Advice => "advice",
    });
    if !rule.id.is_empty() {
        out.push(' ');
        out.push_str(rule.id);
    }
    out.push_str(": ");
    compact_message(message, out);
    if let Some(help) = help {
        out.push_str(" help: ");
        compact_message(help, out);
    }
    out.push('\n');
}

fn compact_message(message: &str, out: &mut String) {
    let mut first = true;
    for word in message.split_whitespace() {
        if !first {
            out.push(' ');
        }
        out.push_str(word);
        first = false;
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{
        DEMO_RULE, RULELESS, finding, finding_with_help, report_with, unpositioned_finding,
    };
    use super::*;

    #[test]
    fn renders_severities_rules_positions_and_compacted_messages() {
        let first = report_with(
            "src/First.java",
            None,
            vec![
                finding_with_help(
                    DEMO_RULE,
                    Severity::Warning,
                    "warning with\n  collapsed\twhitespace",
                    "replace it with\n something clearer",
                    2,
                    5,
                ),
                finding(RULELESS, Severity::Error, "Expected ':' but found ]", 4, 9),
            ],
        );
        let second = report_with(
            "src/Second.java",
            None,
            vec![unpositioned_finding(RULELESS, Severity::Error, "configuration\nfailed")],
        );

        insta::assert_snapshot!(render(&[&first, &second]), @r###"
        src/First.java:2:5: warning demo: warning with collapsed whitespace help: replace it with something clearer
        src/First.java:4:9: error: Expected ':' but found ]
        src/Second.java: error: configuration failed
        "###);
    }

    #[test]
    fn renders_formatting_only_findings_and_no_summary() {
        let report = report_with("src/Dirty.java", Some(false), Vec::new());
        insta::assert_snapshot!(render(&[&report]), @"src/Dirty.java:1:1: warning format: file is not formatted; run `robusta format --fix`");
    }

    #[test]
    fn clean_run_emits_nothing() {
        let report = report_with("src/Clean.java", Some(true), Vec::new());
        assert_eq!(render(&[&report]), "");
    }

    #[test]
    fn advice_renders_as_its_own_severity_word() {
        let report = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Advice, "consider this", 2, 5)],
        );
        insta::assert_snapshot!(render(&[&report]), @"src/C.java:2:5: advice demo: consider this");
    }
}
