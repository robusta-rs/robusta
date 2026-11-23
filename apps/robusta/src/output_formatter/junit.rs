use std::fmt::Write as _;

use robusta_diagnostics::{FileReport, Severity};

use super::Info;
use super::xml_utils::escape as xml_escape;

/// Render the JUnit XML document CI result consumers expect: one test suite per
/// reported file, one test case per finding, error findings as `<error>` and
/// warning findings as `<failure>`.
///
/// Suites keep the run's selection order, which is already sorted by path, so
/// the document is a function of the sources alone.
pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut suites = String::new();
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;

    for report in reports {
        let mut cases = String::new();
        let mut errors = 0usize;
        let mut warnings = 0usize;
        let mut tests = 0usize;

        for info in Info::of(report) {
            match info.severity {
                Severity::Error => errors += 1,
                // JUnit has no third outcome, so advice is counted with the
                // failures its element joins; leaving it out of both totals
                // would make `tests` disagree with `errors` plus `failures`.
                Severity::Warning | Severity::Advice => warnings += 1,
            }
            tests += 1;
            test_case(&mut cases, &info);
        }
        if tests == 0 {
            continue;
        }

        total_errors += errors;
        total_warnings += warnings;
        if !suites.is_empty() {
            suites.push('\n');
        }
        suites.push_str("    <testsuite name=\"");
        xml_escape(report.path(), &mut suites);
        let _ = write!(
            suites,
            "\" tests=\"{tests}\" disabled=\"0\" errors=\"{errors}\" failures=\"{warnings}\">"
        );
        suites.push_str(&cases);
        suites.push_str("\n    </testsuite>");
    }

    let total = total_errors + total_warnings;
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <testsuites name=\"Robusta\" tests=\"{total}\" failures=\"{total_warnings}\" errors=\"{total_errors}\">\n\
         {suites}\n\
         </testsuites>\n"
    )
}

/// A warning is a JUnit `failure` and an error is a JUnit `error`: the pair a
/// result consumer distinguishes, in the direction their names already imply.
///
/// The schema admits no third outcome, so advice joins `failure` — the weaker
/// of the two — rather than being dropped from a report that counted it.
fn element(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning | Severity::Advice => "failure",
    }
}

fn test_case(out: &mut String, info: &Info<'_>) {
    let Info { start, message, severity, rule, .. } = info;
    let element = element(*severity);
    let mut escaped = String::new();
    xml_escape(message, &mut escaped);

    out.push_str("\n        <testcase name=\"");
    xml_escape(rule.id, out);
    let _ = write!(
        out,
        "\">\n            <{element} message=\"{escaped}\">line {}, column {}, {escaped}</{element}>\n        </testcase>",
        start.line, start.column
    );
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, RULELESS, finding, report_with};
    use super::*;

    #[test]
    fn reports_one_suite_per_file_with_severity_split_counts_and_escaping() {
        let first = report_with(
            "src/First.java",
            Some(false),
            vec![finding(DEMO_RULE, Severity::Error, "use <T> & 'quotes' \"here\"", 3, 7)],
        );
        let second = report_with(
            "src/Second.java",
            None,
            vec![finding(RULELESS, Severity::Warning, "bare finding", 2, 5)],
        );

        insta::assert_snapshot!(render(&[&first, &second]), @r###"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites name="Robusta" tests="3" failures="2" errors="1">
            <testsuite name="src/First.java" tests="2" disabled="0" errors="1" failures="1">
                <testcase name="format">
                    <failure message="file is not formatted; run `robusta format --fix`">line 1, column 1, file is not formatted; run `robusta format --fix`</failure>
                </testcase>
                <testcase name="demo">
                    <error message="use &lt;T&gt; &amp; &apos;quotes&apos; &quot;here&quot;">line 3, column 7, use &lt;T&gt; &amp; &apos;quotes&apos; &quot;here&quot;</error>
                </testcase>
            </testsuite>
            <testsuite name="src/Second.java" tests="1" disabled="0" errors="0" failures="1">
                <testcase name="">
                    <failure message="bare finding">line 2, column 5, bare finding</failure>
                </testcase>
            </testsuite>
        </testsuites>
        "###);
    }

    #[test]
    fn advice_joins_failures_so_tests_matches_the_totals() {
        let report = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Advice, "consider this", 2, 5)],
        );
        let rendered = render(&[&report]);
        assert!(rendered.contains("tests=\"1\""), "advice is still a test case: {rendered}");
        assert!(rendered.contains("failures=\"1\""), "advice counts as a failure: {rendered}");
        assert!(rendered.contains("<failure"), "advice uses the failure element: {rendered}");
    }

    #[test]
    fn a_clean_run_is_a_complete_empty_document() {
        let clean = report_with("src/Clean.java", Some(true), Vec::new());
        insta::assert_snapshot!(render(&[&clean]), @r###"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites name="Robusta" tests="0" failures="0" errors="0">

        </testsuites>
        "###);
    }
}
