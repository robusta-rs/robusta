use serde::Serialize;

use robusta_diagnostics::FileReport;
use robusta_linter::table::RuleTable;

use super::{severity_label, to_pretty_document};

/// Return the metadata for all registered rules as JSON, sorted by name.
pub fn get_all_rules_json() -> String {
    #[derive(Serialize)]
    struct RuleData {
        value: &'static str,
        category: &'static str,
        fix: &'static str,
        default: bool,
    }

    let mut rules: Vec<_> = RuleTable::default()
        .into_rows()
        .map(|rule| RuleData {
            value: rule.name,
            category: rule.category.as_str(),
            fix: rule.autofix.as_str(),
            default: rule.turned_on_by_default,
        })
        .collect();
    rules.sort_unstable_by_key(|rule| rule.value);
    to_pretty_document(&rules)
}

#[derive(Serialize)]
struct JsonDocument<'a> {
    files: Vec<JsonFile<'a>>,
    summary: JsonSummary,
}

#[derive(Serialize)]
struct JsonFile<'a> {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<bool>,
    diagnostics: Vec<JsonFinding<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonFinding<'a> {
    rule_id: &'a str,
    severity: &'static str,
    message: &'a str,
    line: usize,
    column: usize,
}

#[derive(Serialize)]
struct JsonSummary {
    diagnostics: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    unformatted: Option<usize>,
}

pub(super) fn render(reports: &[&FileReport], checks_formatting: bool) -> String {
    let files = reports
        .iter()
        .map(|report| JsonFile {
            path: report.path().to_owned(),
            // A file the run could not process has no formatting state, so the
            // field is omitted rather than claiming the file was in order.
            formatted: checks_formatting.then(|| report.formatted()).flatten(),
            diagnostics: report
                .findings()
                .iter()
                .map(|finding| JsonFinding {
                    rule_id: finding.rule_id(),
                    severity: severity_label(finding.severity()),
                    message: finding.message(),
                    line: finding.range().start.line,
                    column: finding.range().start.column,
                })
                .collect(),
        })
        .collect();

    let diagnostics: usize = reports.iter().map(|report| report.findings().len()).sum();
    let unformatted =
        checks_formatting.then(|| reports.iter().filter(|report| report.would_reformat()).count());

    to_pretty_document(&JsonDocument { files, summary: JsonSummary { diagnostics, unformatted } })
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, finding, report_with};
    use super::*;
    use robusta_diagnostics::Severity;

    #[test]
    fn omits_formatting_for_lint_and_reports_it_for_check() {
        let check = report_with(
            "src/C.java",
            Some(false),
            vec![finding(DEMO_RULE, Severity::Warning, "msg", 2, 5)],
        );
        let lint = report_with(
            "src/D.java",
            None,
            vec![finding(DEMO_RULE, Severity::Error, "boom", 1, 1)],
        );

        insta::assert_snapshot!(render(&[&check], true), @r#"
        {
          "files": [
            {
              "path": "src/C.java",
              "formatted": false,
              "diagnostics": [
                {
                  "ruleId": "demo",
                  "severity": "warning",
                  "message": "msg",
                  "line": 2,
                  "column": 5
                }
              ]
            }
          ],
          "summary": {
            "diagnostics": 1,
            "unformatted": 1
          }
        }
        "#);

        insta::assert_snapshot!(render(&[&lint], false), @r#"
        {
          "files": [
            {
              "path": "src/D.java",
              "diagnostics": [
                {
                  "ruleId": "demo",
                  "severity": "error",
                  "message": "boom",
                  "line": 1,
                  "column": 1
                }
              ]
            }
          ],
          "summary": {
            "diagnostics": 1
          }
        }
        "#);
    }

    #[test]
    fn clean_files_are_excluded_from_the_document() {
        let clean = report_with("src/Clean.java", Some(true), Vec::new());
        insta::assert_snapshot!(super::super::render(
            super::super::BufferedFormat::Json,
            &[clean],
            true,
            false,
        ), @r#"
        {
          "files": [],
          "summary": {
            "diagnostics": 0,
            "unformatted": 0
          }
        }
        "#);
    }
}
