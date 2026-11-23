// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/tasks/coverage/src/lib.rs

pub mod args;
mod load;
mod tools;

use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use robusta_diagnostics::Output;
use robusta_formatter::WideLineCause;
use similar::TextDiff;

use tools::Layout;

// ================================
// Data Structures
// ================================

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TestResult {
    Passed,
    IncorrectlyPassed,
    Mismatch(&'static str, String, String),
    ParseError(String, bool),
    CorrectError(String, bool),
    GenericError(&'static str, String),
}

#[derive(Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub source: String,
    /// A `misc/fail` fixture, which must report a syntax error.
    pub should_fail: bool,
}

/// A set of source files reported as one suite per tool.
pub struct Corpus {
    pub name: String,
    /// The directory failure paths are printed under.
    pub root: PathBuf,
    pub files: Vec<SourceFile>,
    /// Whether the results are committed under `snapshots/`.
    pub snapshot: bool,
}

/// The misc fixtures, then each caller-given corpus.
pub struct TestData {
    pub corpora: Vec<Corpus>,
}

// ================================
// Paths
// ================================

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn snap_root() -> PathBuf {
    workspace_root().join("snapshots")
}

#[expect(clippy::disallowed_methods, reason = "the path is returned as an owned string")]
fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

// ================================
// Result & Reporting
// ================================

pub struct CoverageResult {
    pub path: PathBuf,
    pub should_fail: bool,
    pub result: TestResult,
}

impl CoverageResult {
    fn passed(&self) -> bool {
        match &self.result {
            TestResult::Passed if !self.should_fail => true,
            TestResult::CorrectError(_, _) => true,
            _ => false,
        }
    }

    fn parsed(&self) -> bool {
        match &self.result {
            TestResult::ParseError(_, is_fatal) | TestResult::CorrectError(_, is_fatal) => {
                !is_fatal
            }
            _ => true,
        }
    }
}

struct CoverageStats<'a> {
    positives: Vec<&'a CoverageResult>,
    negatives: Vec<&'a CoverageResult>,
    all_positives: usize,
    parsed_positives: usize,
    passed_positives: usize,
    all_negatives: usize,
    passed_negatives: usize,
}

impl<'a> CoverageStats<'a> {
    fn new(results: &'a [CoverageResult]) -> Self {
        let (positives, negatives): (Vec<_>, Vec<_>) = results.iter().partition(|r| !r.should_fail);
        let all_positives = positives.len();
        let parsed_positives = positives.iter().filter(|r| r.parsed()).count();
        let passed_positives = positives.iter().filter(|r| r.passed()).count();
        let all_negatives = negatives.len();
        let passed_negatives = negatives.iter().filter(|r| r.passed()).count();
        Self {
            positives,
            negatives,
            all_positives,
            parsed_positives,
            passed_positives,
            all_negatives,
            passed_negatives,
        }
    }

    fn write_summary(&self, name: &str, out: &mut String) {
        let _ = writeln!(out, "{name} Summary:");
        let _ =
            writeln!(out, "AST Parsed     : {}", score(self.parsed_positives, self.all_positives));
        let _ =
            writeln!(out, "Positive Passed: {}", score(self.passed_positives, self.all_positives));
        if self.all_negatives > 0 {
            let _ = writeln!(
                out,
                "Negative Passed: {}",
                score(self.passed_negatives, self.all_negatives)
            );
        }
    }
}

fn write_coverage(
    name: &str,
    root: &Path,
    results: &[CoverageResult],
    detail: bool,
    out: &mut String,
) {
    CoverageStats::new(results).write_summary(name, out);

    if detail {
        for r in results {
            write_result(root, r, out);
        }
    }
}

fn write_result(root: &Path, r: &CoverageResult, out: &mut String) {
    let path = display_path(&root.join(&r.path));
    match &r.result {
        TestResult::ParseError(error, _) => {
            let _ = writeln!(out, "Expect to Parse: {path}\n{error}");
        }
        TestResult::Mismatch(case, actual, expected) => {
            let _ = writeln!(out, "{case}: {path}");
            let _ = write!(out, "{}", TextDiff::from_lines(expected, actual).unified_diff());
        }
        TestResult::GenericError(case, error) => {
            let _ = writeln!(out, "{case} Error: {path}\n{error}");
        }
        TestResult::IncorrectlyPassed => {
            let _ = writeln!(out, "Expect Syntax Error: {path}");
        }
        _ => {}
    }
}

fn snapshot_results(name: &str, root: &Path, results: &[CoverageResult]) {
    let mut out = String::new();
    let stats = CoverageStats::new(results);
    stats.write_summary(name, &mut out);

    let failures = stats.negatives.into_iter().chain(stats.positives).filter(|r| !r.passed());
    for r in failures {
        let path = display_path(&root.join(&r.path));
        match &r.result {
            TestResult::IncorrectlyPassed => {
                let _ = writeln!(out, "Expect Syntax Error: {path}\n");
            }
            TestResult::ParseError(error, is_fatal) => {
                let label = if *is_fatal { "Fatal error" } else { "Expect to Parse" };
                let _ = writeln!(out, "{label}: {path}\n{error}");
            }
            TestResult::Mismatch(case, _, _) => {
                let _ = writeln!(out, "{case}: {path}\n");
            }
            TestResult::GenericError(case, error) => {
                let _ = writeln!(out, "{case} Error: {path}\n{error}\n");
            }
            _ => {}
        }
    }

    for r in results {
        if let TestResult::CorrectError(error, _) = &r.result {
            out.push_str(error);
        }
    }

    let path = snap_root().join(format!("{name}.snap"));
    fs::write(&path, out)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

/// The formatter's line widths, reported but not judged.
fn write_layout(name: &str, layouts: &[Layout], out: &mut String) {
    let clean = layouts.iter().filter(|layout| layout.line_width.is_clean()).count();
    let wide: usize = layouts.iter().map(|layout| layout.line_width.wide_lines().len()).sum();
    let misses: usize = layouts.iter().map(|layout| layout.line_width.misses().count()).sum();
    let _ = writeln!(
        out,
        "Line Width     : {}, {misses} miss(es) in {wide} wide line(s)",
        score(clean, layouts.len())
    );
    write_widest(name, layouts, out);
}

/// The files with the most line-width misses, capped.
fn write_widest(name: &str, layouts: &[Layout], out: &mut String) {
    const LISTED: usize = 10;

    let mut worst: Vec<_> = layouts
        .iter()
        .filter_map(|layout| {
            let first = layout.line_width.misses().next()?;
            Some((layout.line_width.misses().count(), &layout.path, *first))
        })
        .collect();
    if worst.is_empty() {
        return;
    }
    worst.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(right.1)));

    let _ = writeln!(out, "\n{name} widest files:");
    for (misses, path, first) in worst.iter().take(LISTED) {
        let _ = writeln!(
            out,
            "  {misses:>4} miss(es)  {}:{} ({} columns)",
            display_path(path),
            first.line,
            first.width
        );
    }
    if worst.len() > LISTED {
        let _ = writeln!(out, "  … and {} more file(s)", worst.len() - LISTED);
    }

    let excused = [
        (WideLineCause::Declaration, "import or package declaration"),
        (WideLineCause::Comment, "comment text"),
        (WideLineCause::IndivisibleRun, "no break position inside the wide part"),
        (WideLineCause::KeptTogether, "only breaks the layout keeps closed"),
    ];
    let _ = writeln!(out, "  excused:");
    for (cause, label) in excused {
        let count: usize = layouts.iter().map(|layout| layout.line_width.count(cause)).sum();
        let _ = writeln!(out, "    {count:>6}  {label}");
    }
}

fn score(passed: usize, total: usize) -> String {
    format!("{passed}/{total} ({})", percentage(passed, total))
}

#[expect(clippy::cast_precision_loss)]
fn percentage(passed: usize, total: usize) -> String {
    format!("{:.2}%", passed as f64 / total as f64 * 100.0)
}

// ================================
// App Args & Main Entry
// ================================

#[derive(Debug, Default)]
pub struct AppArgs {
    /// Use one worker thread to make crash isolation deterministic.
    pub debug: bool,
    /// Select files whose corpus-relative path contains this text.
    pub filter: Option<String>,
    /// Print every failure after its suite's summary.
    pub detail: bool,
    /// Corpus root directories, scanned recursively for .java files.
    pub corpora: Vec<PathBuf>,
}

impl AppArgs {
    pub fn run_all(&self, data: &TestData, output: &mut Output) -> bool {
        self.run_parser(data, output) & self.run_formatter(data, output)
    }

    /// Whether every result passed. Writes the misc snapshot unless filtered.
    fn run_tool(
        &self,
        name: &str,
        corpus: &Corpus,
        results: &[CoverageResult],
        out: &mut String,
    ) -> bool {
        let filter = self.filter.is_some();
        write_coverage(name, &corpus.root, results, self.detail || filter, out);
        if corpus.snapshot && !filter {
            snapshot_results(name, &corpus.root, results);
        }
        results.iter().all(CoverageResult::passed)
    }

    pub fn run_parser(&self, data: &TestData, output: &mut Output) -> bool {
        let mut passed = true;
        for corpus in &data.corpora {
            let results = tools::run_parser_corpus(&corpus.files);
            if results.is_empty() {
                continue;
            }
            let mut out = String::new();
            passed &= self.run_tool(&format!("parser_{}", corpus.name), corpus, &results, &mut out);
            output.stdout(&out);
        }
        passed
    }

    pub fn run_formatter(&self, data: &TestData, output: &mut Output) -> bool {
        let mut passed = true;
        for corpus in &data.corpora {
            let (results, layouts) = tools::run_formatter_corpus(&corpus.files);
            if results.is_empty() {
                continue;
            }
            let name = format!("formatter_{}", corpus.name);
            let mut out = String::new();
            passed &= self.run_tool(&name, corpus, &results, &mut out);
            write_layout(&name, &layouts, &mut out);
            output.stdout(&out);
        }
        passed
    }
}

#[cfg(test)]
mod tests {
    use robusta_formatter::LineWidthReport;

    use super::*;

    fn result(path: &str, should_fail: bool, result: TestResult) -> CoverageResult {
        CoverageResult { path: PathBuf::from(path), should_fail, result }
    }

    #[test]
    fn reports_scores_failures_and_final_status() {
        let corpus = Corpus {
            name: "fixture".to_string(),
            root: PathBuf::from("fixture"),
            files: Vec::new(),
            snapshot: false,
        };
        let results = [
            result("Pass.java", false, TestResult::Passed),
            result(
                "nested/Fail.java",
                false,
                TestResult::ParseError("expected identifier".to_string(), true),
            ),
            result("Rejected.java", true, TestResult::CorrectError(String::new(), false)),
        ];
        let args = AppArgs { detail: true, ..AppArgs::default() };
        let mut out = String::new();

        let passed = args.run_tool("parser_fixture", &corpus, &results, &mut out);

        assert!(!passed);
        assert_eq!(
            out,
            "parser_fixture Summary:\n\
             AST Parsed     : 1/2 (50.00%)\n\
             Positive Passed: 1/2 (50.00%)\n\
             Negative Passed: 1/1 (100.00%)\n\
             Expect to Parse: fixture/nested/Fail.java\n\
             expected identifier\n"
        );
    }

    #[test]
    fn mismatch_prints_the_diff_from_expected_to_actual() {
        let mismatch = TestResult::Mismatch(
            "Mismatch",
            "class B {}\n".to_string(),
            "class A {}\n".to_string(),
        );
        let mut out = String::new();

        write_result(Path::new("fixture"), &result("A.java", false, mismatch), &mut out);

        assert_eq!(out, "Mismatch: fixture/A.java\n@@ -1 +1 @@\n-class A {}\n+class B {}\n");
    }

    #[test]
    fn a_clean_layout_reports_no_widest_files() {
        let layouts =
            [Layout { path: PathBuf::from("A.java"), line_width: LineWidthReport::default() }];
        let mut out = String::new();

        write_layout("formatter_fixture", &layouts, &mut out);

        assert_eq!(out, "Line Width     : 1/1 (100.00%), 0 miss(es) in 0 wide line(s)\n");
    }
}
