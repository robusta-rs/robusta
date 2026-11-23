use robusta_diagnostics::FileReport;

use super::xml_utils::escape as xml_escape;
use super::{Info, severity_label};

/// Render the Checkstyle XML document: one line, one `<file>` element per
/// reported file, and `version="4.3"`. That version is the marker ESLint's
/// checkstyle formatter established for this element set, and every emitter of
/// it writes: it names the document shape a consumer should expect, not a tool
/// release that produced the document.
pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut files = String::new();
    for report in reports {
        // A file with nothing to say gets no element, so the document never
        // carries an empty `<file>`.
        if !report.is_reportable() {
            continue;
        }
        if !files.is_empty() {
            files.push(' ');
        }
        files.push_str("<file name=\"");
        xml_escape(report.path(), &mut files);
        files.push_str("\">");
        for info in Info::of(report) {
            checkstyle_error(&mut files, &info);
        }
        files.push_str("</file>");
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><checkstyle version=\"4.3\">{files}</checkstyle>\n"
    )
}

fn checkstyle_error(out: &mut String, info: &Info<'_>) {
    let Info { start, message, severity, rule, .. } = info;
    out.push_str("<error line=\"");
    out.push_str(&start.line.to_string());
    out.push_str("\" column=\"");
    out.push_str(&start.column.to_string());
    out.push_str("\" severity=\"");
    out.push_str(severity_label(*severity));
    out.push_str("\" message=\"");
    xml_escape(message, out);
    out.push_str("\" source=\"");
    xml_escape(rule.id, out);
    out.push_str("\" />");
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{DEMO_RULE, finding, report_with};
    use super::*;
    use robusta_diagnostics::Severity;

    #[test]
    fn synthesizes_a_formatting_error_and_escapes_text() {
        let report = report_with(
            "src/C.java",
            Some(false),
            vec![finding(DEMO_RULE, Severity::Error, "use <T> & \"quotes\"", 3, 7)],
        );
        insta::assert_snapshot!(render(&[&report]), @r###"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"><file name="src/C.java"><error line="1" column="1" severity="warning" message="file is not formatted; run `robusta format --fix`" source="format" /><error line="3" column="7" severity="error" message="use &lt;T&gt; &amp; &quot;quotes&quot;" source="demo" /></file></checkstyle>"###);
    }

    /// The whole document is one line, and only the `<file>` elements are
    /// separated — by a single space.
    #[test]
    fn one_space_separates_the_files_and_nothing_else() {
        let first = report_with(
            "src/First.java",
            None,
            vec![finding(DEMO_RULE, Severity::Warning, "first", 2, 5)],
        );
        let second = report_with("src/Second.java", Some(false), Vec::new());

        insta::assert_snapshot!(render(&[&first, &second]), @r###"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"><file name="src/First.java"><error line="2" column="5" severity="warning" message="first" source="demo" /></file> <file name="src/Second.java"><error line="1" column="1" severity="warning" message="file is not formatted; run `robusta format --fix`" source="format" /></file></checkstyle>"###);
    }

    #[test]
    fn a_clean_run_is_a_complete_empty_document() {
        let clean = report_with("src/Clean.java", Some(true), Vec::new());
        insta::assert_snapshot!(
            render(&[&clean]),
            @r###"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"></checkstyle>"###
        );
    }
}
