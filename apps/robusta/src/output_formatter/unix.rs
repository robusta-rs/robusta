use robusta_diagnostics::{FileReport, Severity};

use super::Info;

pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut out = String::new();
    let mut total = 0;
    for info in Info::of_all(reports) {
        unix_finding(&mut out, &info);
        total += 1;
    }
    if total > 0 {
        out.push('\n');
        out.push_str(&total.to_string());
        out.push_str(if total == 1 { " problem\n" } else { " problems\n" });
    }
    out
}

fn unix_finding(out: &mut String, info: &Info<'_>) {
    let Info { start, filename, message, severity, rule, .. } = info;
    out.push_str(filename);
    out.push(':');
    out.push_str(&start.line.to_string());
    out.push(':');
    out.push_str(&start.column.to_string());
    out.push_str(": ");
    unix_escape_message(message, out);
    out.push_str(" [");
    out.push_str(match severity {
        Severity::Error => "Error",
        Severity::Warning => "Warning",
        Severity::Advice => "Advice",
    });
    if !rule.id.is_empty() {
        out.push('/');
        out.push_str(rule.id);
    }
    out.push_str("]\n");
}

/// Keep every finding on one physical line so diagnostic text cannot forge
/// another Unix-format record.
fn unix_escape_message(value: &str, out: &mut String) {
    for ch in value.chars() {
        match ch {
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, RULELESS, finding, report_with};
    use super::*;

    #[test]
    fn emits_one_line_per_finding_and_pluralized_summary() {
        let report = report_with(
            "src/C.java",
            Some(false),
            vec![
                finding(DEMO_RULE, Severity::Warning, "Expected ':' but found ]\nnext", 2, 5),
                finding(RULELESS, Severity::Error, "bare", 4, 9),
            ],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"
        src/C.java:1:1: file is not formatted; run `robusta format --fix` [Warning/format]
        src/C.java:2:5: Expected ':' but found ]\nnext [Warning/demo]
        src/C.java:4:9: bare [Error]

        3 problems
        "###);
    }

    #[test]
    fn clean_report_is_empty_and_single_problem_is_singular() {
        let clean = report_with("src/Clean.java", Some(true), Vec::new());
        assert_eq!(render(&[&clean]), "");

        let dirty = report_with("src/Dirty.java", Some(false), Vec::new());
        assert!(render(&[&dirty]).ends_with("\n1 problem\n"));
    }

    /// This format's total counts emitted records rather than severities, so
    /// advice joins it — unlike the run summary in `report.rs`, which counts
    /// only the two severities its closing line names.
    #[test]
    fn advice_prints_its_own_label_and_counts_as_an_emitted_record() {
        let report = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Advice, "consider this", 2, 5)],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"
        src/C.java:2:5: consider this [Advice/demo]

        1 problem
        "###);
    }
}
