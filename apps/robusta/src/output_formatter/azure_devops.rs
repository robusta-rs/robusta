use robusta_diagnostics::FileReport;

use super::{Info, severity_label};

pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut out = String::new();
    for info in Info::of_all(reports) {
        azure_devops_annotation(&mut out, &info);
    }
    out
}

fn azure_devops_annotation(out: &mut String, info: &Info<'_>) {
    let Info { start, filename, message, severity, rule, .. } = info;
    out.push_str("##vso[task.logissue type=");
    out.push_str(severity_label(*severity));
    out.push_str(";sourcepath=");
    azure_devops_escape(filename, out);
    out.push_str(";linenumber=");
    out.push_str(&start.line.to_string());
    out.push_str(";columnnumber=");
    out.push_str(&start.column.to_string());
    out.push_str(";code=");
    azure_devops_escape(rule.id, out);
    out.push_str(";]");
    azure_devops_escape(message, out);
    out.push('\n');
}

/// Escape Azure Pipelines logging-command values. Percent is encoded to the
/// agent-specific `%AZP25` token before the escape sequences for command
/// delimiters and physical newlines are introduced.
fn azure_devops_escape(value: &str, out: &mut String) {
    for ch in value.chars() {
        match ch {
            '%' => out.push_str("%AZP25"),
            ';' => out.push_str("%3B"),
            '\r' => out.push_str("%0D"),
            '\n' => out.push_str("%0A"),
            ']' => out.push_str("%5D"),
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, finding, report_with};
    use super::*;
    use robusta_diagnostics::{RuleInfo, Severity};

    const AZURE_ESCAPE_RULE: RuleInfo = RuleInfo { id: "rule%;]\r\n规则", summary: None };

    #[test]
    fn emits_annotations_and_escapes_command_values() {
        let report = report_with(
            "src/A%;]\r\n雪.java",
            Some(false),
            vec![
                finding(DEMO_RULE, Severity::Warning, "warning", 2, 5),
                finding(AZURE_ESCAPE_RULE, Severity::Error, "use 100%;]\r\n然后 stop", 3, 7),
            ],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"
        ##vso[task.logissue type=warning;sourcepath=src/A%AZP25%3B%5D%0D%0A雪.java;linenumber=1;columnnumber=1;code=format;]file is not formatted%3B run `robusta format --fix`
        ##vso[task.logissue type=warning;sourcepath=src/A%AZP25%3B%5D%0D%0A雪.java;linenumber=2;columnnumber=5;code=demo;]warning
        ##vso[task.logissue type=error;sourcepath=src/A%AZP25%3B%5D%0D%0A雪.java;linenumber=3;columnnumber=7;code=rule%AZP25%3B%5D%0D%0A规则;]use 100%AZP25%3B%5D%0D%0A然后 stop
        "###);
    }

    #[test]
    fn clean_report_is_empty() {
        let report = report_with("src/Clean.java", Some(true), Vec::new());
        assert_eq!(render(&[&report]), "");
    }
}
