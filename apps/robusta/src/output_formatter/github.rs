use robusta_diagnostics::FileReport;

use super::{Info, severity_label};

/// Caption for an annotation raised by no rule: a parse error, or a file the
/// tool could not process. GitHub renders the title as the annotation's
/// heading, so the tool names itself rather than heading the panel with
/// nothing.
const NO_RULE_TITLE: &str = "robusta";

pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut out = String::new();
    for info in Info::of_all(reports) {
        github_annotation(&mut out, &info);
    }
    out
}

fn github_annotation(out: &mut String, info: &Info<'_>) {
    let Info { start, end, filename, message, severity, rule, .. } = info;
    let title = if rule.id.is_empty() { NO_RULE_TITLE } else { rule.id };
    out.push_str("::");
    out.push_str(severity_label(*severity));
    out.push_str(" file=");
    github_escape_property(filename, out);
    out.push_str(",line=");
    out.push_str(&start.line.to_string());
    out.push_str(",endLine=");
    out.push_str(&end.line.to_string());
    out.push_str(",col=");
    out.push_str(&start.column.to_string());
    out.push_str(",endColumn=");
    out.push_str(&end.column.to_string());
    out.push_str(",title=");
    github_escape_property(title, out);
    out.push_str("::");
    // The parameters before `::` feed the annotations panel only, not the log
    // stream, so the location is repeated in the message text a reader scrolling
    // raw CI output sees. The path is escaped as data here rather than as a
    // property: `,` and `:` delimit the command grammar, not the message.
    github_escape_data(filename, out);
    out.push(':');
    out.push_str(&start.line.to_string());
    out.push(':');
    out.push_str(&start.column.to_string());
    out.push_str(": ");
    github_escape_data(message, out);
    out.push('\n');
}

/// Escape GitHub workflow-command data. Percent must be encoded along with
/// physical newlines so one finding always occupies exactly one command line.
fn github_escape_data(value: &str, out: &mut String) {
    for ch in value.chars() {
        match ch {
            '\r' => out.push_str("%0D"),
            '\n' => out.push_str("%0A"),
            '%' => out.push_str("%25"),
            _ => out.push(ch),
        }
    }
}

/// Escape GitHub workflow-command properties. Colons and commas delimit the
/// command/property grammar and therefore need encoding in paths and titles.
fn github_escape_property(value: &str, out: &mut String) {
    for ch in value.chars() {
        match ch {
            '\r' => out.push_str("%0D"),
            '\n' => out.push_str("%0A"),
            ':' => out.push_str("%3A"),
            ',' => out.push_str("%2C"),
            '%' => out.push_str("%25"),
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{
        DEMO_RULE, RULELESS, finding, position, ranged_finding, report_with, unpositioned_finding,
    };
    use super::*;
    use robusta_diagnostics::Severity;

    /// The path is escaped as a property before `::` and as data after it, so
    /// `:` and `,` are encoded in the annotation parameters and left literal in
    /// the log text, while `%` is encoded in both.
    #[test]
    fn emits_annotations_and_escapes_command_values() {
        let report = report_with(
            "src/A:B,C%.java",
            Some(false),
            vec![finding(DEMO_RULE, Severity::Error, "use 100%\nthen stop\r", 3, 7)],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"
        ::warning file=src/A%3AB%2CC%25.java,line=1,endLine=1,col=1,endColumn=1,title=format::src/A:B,C%25.java:1:1: file is not formatted; run `robusta format --fix`
        ::error file=src/A%3AB%2CC%25.java,line=3,endLine=3,col=7,endColumn=7,title=demo::src/A:B,C%25.java:3:7: use 100%25%0Athen stop%0D
        "###);
    }

    /// A finding that spans a region reports where it ends, which is what
    /// GitHub draws the highlight across.
    #[test]
    fn a_spanned_finding_reports_both_ends() {
        let report = report_with(
            "src/Spanned.java",
            None,
            vec![ranged_finding(
                DEMO_RULE,
                Severity::Warning,
                "spans two lines",
                position(2, 5),
                position(3, 11),
            )],
        );
        insta::assert_snapshot!(render(&[&report]), @"::warning file=src/Spanned.java,line=2,endLine=3,col=5,endColumn=11,title=demo::src/Spanned.java:2:5: spans two lines");
    }

    /// A finding no rule raised is captioned with the tool's name, and one
    /// whose position the producer never resolved keeps the zero coordinates
    /// every line-oriented format prints.
    #[test]
    fn a_ruleless_finding_falls_back_to_the_tool_name() {
        let report = report_with(
            "src/Locked.java",
            None,
            vec![
                finding(RULELESS, Severity::Error, "expected a member name", 4, 9),
                unpositioned_finding(RULELESS, Severity::Error, "src/Locked.java: denied"),
            ],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"
        ::error file=src/Locked.java,line=4,endLine=4,col=9,endColumn=9,title=robusta::src/Locked.java:4:9: expected a member name
        ::error file=src/Locked.java,line=0,endLine=0,col=0,endColumn=0,title=robusta::src/Locked.java:0:0: src/Locked.java: denied
        "###);
    }
}
