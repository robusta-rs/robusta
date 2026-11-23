// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/tasks/coverage/src/tools.rs

use std::{
    any::Any,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    slice,
};

use rayon::prelude::*;
use robusta_allocator::Allocator;
use robusta_diagnostics::{
    Diagnostic, GraphicalReportHandler, GraphicalTheme, NamedSource, RobustaDiagnostic,
};
use robusta_formatter::{LineWidthReport, Options};
use robusta_parser::ParseStatus;

use crate::{CoverageResult, SourceFile, TestResult, display_path};

// ================================
// Parser
// ================================

fn run_parser(path: &Path, source: &str) -> TestResult {
    let allocator = Allocator::new();
    let parsed = robusta_parser::parse(&allocator, source);
    if parsed.errors.is_empty() {
        TestResult::Passed
    } else {
        let errors = render_errors(path, source, &parsed.errors);
        TestResult::ParseError(errors, parsed.status == ParseStatus::Fatal)
    }
}

fn evaluate_result(result: TestResult, should_fail: bool) -> TestResult {
    match (result, should_fail) {
        (TestResult::ParseError(err, is_fatal), true) => TestResult::CorrectError(err, is_fatal),
        (TestResult::Passed, true) => TestResult::IncorrectlyPassed,
        (result, _) => result,
    }
}

pub fn run_parser_corpus(files: &[SourceFile]) -> Vec<CoverageResult> {
    files
        .par_iter()
        .map(|f| {
            let result = catch_panic(|| run_parser(&f.path, &f.source)).unwrap_or_else(|r| r);
            let result = evaluate_result(result, f.should_fail);
            CoverageResult { path: f.path.clone(), should_fail: f.should_fail, result }
        })
        .collect()
}

// ================================
// Formatter
// ================================

/// What formatting one file measured without judging it.
pub struct Layout {
    pub path: PathBuf,
    /// Output lines wider than the configured budget, classified. Most are
    /// lines no layout could have shortened, and the external corpora are not
    /// committed, so the formatter's fixture suites judge this instead.
    pub line_width: LineWidthReport,
}

fn run_formatter(path: &Path, source: &str) -> (TestResult, Option<Layout>) {
    let options = Options::default();
    let Ok(formatted) = robusta_formatter::format(source, &options) else {
        return (TestResult::Passed, None); // Skip if parse error
    };
    let layout = Layout {
        path: path.to_path_buf(),
        line_width: robusta_formatter::check_line_width(&formatted, &options),
    };

    let result = if let Err(message) = robusta_formatter::detect_code_removal(source, &formatted) {
        TestResult::GenericError("Code Removal", message)
    } else {
        run_second_pass(path, formatted, &options)
    };
    (result, Some(layout))
}

fn run_second_pass(path: &Path, formatted: String, options: &Options) -> TestResult {
    // Re-format the output: a parse error on the second pass is a real formatter bug.
    let report = match robusta_formatter::check_idempotency(&formatted, options) {
        Ok(report) => report,
        Err(error) => {
            let errors = render_errors(path, &formatted, slice::from_ref(&error));
            return TestResult::ParseError(errors, false);
        }
    };
    if report.is_stable() {
        TestResult::Passed
    } else {
        TestResult::Mismatch("Mismatch", formatted, report.reformatted().to_string())
    }
}

pub fn run_formatter_corpus(files: &[SourceFile]) -> (Vec<CoverageResult>, Vec<Layout>) {
    let (results, layouts): (Vec<_>, Vec<_>) = files
        .par_iter()
        .filter(|f| !f.should_fail)
        .map(|f| {
            let (result, layout) = catch_panic(|| run_formatter(&f.path, &f.source))
                .unwrap_or_else(|result| (result, None));
            (CoverageResult { path: f.path.clone(), should_fail: false, result }, layout)
        })
        .unzip();
    (results, layouts.into_iter().flatten().collect())
}

// ================================
// Shared
// ================================

fn render_errors(path: &Path, source: &str, errors: &[RobustaDiagnostic]) -> String {
    let mut output = String::new();
    GraphicalReportHandler::new_themed(GraphicalTheme::none())
        .render_reports(
            &mut output,
            &NamedSource::new(display_path(path), source),
            errors.iter().map(|error| error as &dyn Diagnostic),
        )
        .expect("rendering diagnostics to a string cannot fail");
    output
}

/// Run one file so that its crash becomes its result instead of ending the run.
fn catch_panic<T>(run: impl FnOnce() -> T) -> Result<T, TestResult> {
    catch_unwind(AssertUnwindSafe(run))
        .map_err(|payload| TestResult::GenericError("Panic", panic_message(payload.as_ref())))
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    payload.downcast_ref::<String>().map_or_else(
        || payload.downcast_ref::<&str>().copied().unwrap_or("unknown panic").to_string(),
        String::clone,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_file(path: &str, source: &str, should_fail: bool) -> SourceFile {
        SourceFile { path: PathBuf::from(path), source: source.to_string(), should_fail }
    }

    #[test]
    fn clean_source_passes_every_formatter_check() {
        let (results, layouts) =
            run_formatter_corpus(&[source_file("Clean.java", "class Clean { int value; }", false)]);

        assert_eq!(results[0].result, TestResult::Passed);
        assert!(layouts[0].line_width.is_clean());
    }

    #[test]
    fn a_comment_that_survives_both_passes_passes() {
        let source = "class A {\n  // leading\n  int x = 1; // trailing\n}\n";
        let (results, _) = run_formatter_corpus(&[source_file("A.java", source, false)]);

        assert_eq!(results[0].result, TestResult::Passed);
    }

    #[test]
    fn invalid_source_fails_the_parser_and_skips_the_formatter() {
        let files = [source_file("Invalid.java", "class {", false)];

        let parsed = run_parser_corpus(&files);
        let (formatted, layouts) = run_formatter_corpus(&files);

        assert!(matches!(&parsed[0].result, TestResult::ParseError(error, _)
            if error.contains("[Invalid.java:1:7]")));
        assert_eq!(formatted[0].result, TestResult::Passed);
        assert!(layouts.is_empty());
    }

    #[test]
    fn should_fail_case_renders_its_syntax_errors() {
        let files = [source_file("misc/fail/a.java", "class A { void f() }", true)];

        let results = run_parser_corpus(&files);

        let TestResult::CorrectError(error, false) = &results[0].result else {
            panic!("{:?}", results[0].result);
        };
        assert!(error.contains("expected a method body"), "{error}");
        assert!(error.contains("[misc/fail/a.java:1:20]"), "{error}");
        assert!(run_formatter_corpus(&files).0.is_empty());
    }

    #[test]
    fn should_fail_case_fails_when_the_source_parses() {
        let results = run_parser_corpus(&[source_file("misc/fail/a.java", "class A {}", true)]);

        assert_eq!(results[0].result, TestResult::IncorrectlyPassed);
    }

    #[test]
    fn a_panic_becomes_the_file_result() {
        let result = catch_panic::<()>(|| panic!("boom")).unwrap_err();

        assert_eq!(result, TestResult::GenericError("Panic", "boom".to_string()));
    }
}
