use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A temp directory holding one `Example.java`. The [`tempfile::TempDir`]
/// guard removes the directory when the fixture is dropped.
pub struct TempJava {
    _root: tempfile::TempDir,
    pub dir: PathBuf,
    pub file: PathBuf,
}

/// A temp project directory populated through [`TempProject::write`]. The
/// [`tempfile::TempDir`] guard removes the directory when the fixture is
/// dropped.
pub struct TempProject {
    _root: tempfile::TempDir,
    pub dir: PathBuf,
}

impl TempJava {
    pub fn new(source: &str) -> Self {
        let root = tempfile::tempdir().expect("failed to create temp dir");
        let dir = root.path().to_path_buf();
        let file = dir.join("Example.java");
        std::fs::write(&file, source).expect("failed to write temp Java source");
        Self { _root: root, dir, file }
    }
}

/// A suppression for a rule the configuration enables. The rule finds
/// nothing, so an audit reports the suppression unused.
pub const STALE_SUPPRESSION: &str =
    "class Example {\n  @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n  int clean;\n}\n";

impl TempJava {
    /// [`STALE_SUPPRESSION`] beside a configuration whose `lint` section holds
    /// `lint` and enables its rule.
    pub fn stale_suppression(lint: &str) -> Self {
        let temp = Self::new(STALE_SUPPRESSION);
        std::fs::write(
            temp.dir.join("robusta.yaml"),
            format!("lint:\n{lint}  rules:\n    uppercase-long-suffix: warn\n"),
        )
        .expect("failed to write config");
        temp
    }
}

impl TempProject {
    pub fn new() -> Self {
        let root = tempfile::tempdir().expect("failed to create temp dir");
        let dir = root.path().to_path_buf();
        Self { _root: root, dir }
    }

    pub fn write(&self, relative: &str, source: &str) -> PathBuf {
        let path = self.dir.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create parent dir");
        }
        std::fs::write(&path, source).expect("failed to write temp file");
        path
    }
}

pub fn robusta_in_dir(args: &[&str], dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_robusta"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run robusta")
}

pub fn robusta_in_dir_with_env(args: &[&str], dir: &Path, key: &str, value: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_robusta"))
        .current_dir(dir)
        .args(args)
        .env(key, value)
        .output()
        .expect("failed to run robusta")
}

pub fn robusta_with_path(args: &[&str], path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_robusta"))
        .args(args)
        .arg(path)
        .output()
        .expect("failed to run robusta")
}

pub fn robusta_with_stdin(args: &[&str], dir: &Path, source: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_robusta"))
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run robusta");
    // Argument-validation paths make robusta exit before reading stdin; the
    // write to the now-closed pipe then fails with `BrokenPipe`, which is
    // expected and must not fail the test. Any other error is a real fault.
    let mut stdin = child.stdin.take().expect("stdin should be piped");
    if let Err(error) = stdin.write_all(source.as_bytes()) {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::BrokenPipe,
            "failed to write robusta stdin: {error}",
        );
    }
    drop(stdin); // close the pipe so stdin-consuming runs see EOF
    child.wait_with_output().expect("failed to wait for robusta")
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A run's stdout without the summary the human reporter closes every file run
/// with, and without the blank line separating them. Assertions whose subject
/// is the diagnostics use this so they do not restate the summary; the tests
/// that own the summary assert it through [`run_summary`].
pub fn stdout_without_summary(output: &Output) -> String {
    without_summary(stdout(output))
}

/// [`stdout_without_summary`] for a run whose stdout carries a document, which
/// moves the report to stderr.
pub fn stderr_without_summary(output: &Output) -> String {
    without_summary(stderr(output))
}

fn without_summary(mut report: String) -> String {
    if let Some(start) = summary_start(&report) {
        report.truncate(start);
    }
    report
}

/// The summary a run closes its stdout report with, from the diagnostic totals
/// onward, with the elapsed duration replaced so a snapshot pins the line's
/// shape and never a measurement.
pub fn run_summary(output: &Output) -> String {
    summary(&stdout(output))
}

fn summary(report: &str) -> String {
    let summary = summary_start(report).map_or("", |start| &report[start..]);
    let Some((before, after)) = summary.split_once("Finished in ") else {
        return summary.to_owned();
    };
    let rest = after.split_once(" on ").expect("the execution summary states a file count").1;
    format!("{before}Finished in <time> on {rest}")
}

/// Where the closing summary begins: the last totals line at the left margin,
/// plus the blank line the reporter puts in front of it when the run reported
/// findings. Rendered diagnostics are always indented, so no finding can open
/// a totals line itself. A run that processed no file has no totals line.
fn summary_start(report: &str) -> Option<usize> {
    const CLEAN: &str = "Found 0 warnings and 0 errors.";
    let mut start = None;
    let mut offset = 0;
    for line in report.split_inclusive('\n') {
        if line.starts_with("Found ") {
            start = Some(if line.starts_with(CLEAN) { offset } else { offset - 1 });
        } else if start.is_none() && line.starts_with("Finished in ") {
            start = Some(offset);
        }
        offset += line.len();
    }
    start
}

pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Render a path as the CLI presents it: with forward-slash separators on
/// every platform.
#[expect(clippy::disallowed_methods, reason = "the path is returned as an owned string")]
pub fn presentation_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// `dir` spelled the way a process started in it reports its working
/// directory. Unix resolves symlinks there, as on macOS, whose temp directory
/// is reached through `/var`. Windows keeps the spelling the process was given;
/// canonicalizing would add a `\\?\` prefix that the working directory lacks.
pub fn working_dir_spelling(dir: &Path) -> PathBuf {
    if cfg!(windows) {
        dir.to_path_buf()
    } else {
        dir.canonicalize().expect("canonicalize working directory")
    }
}

/// Replaces the temp directory with a stable placeholder so path-bearing
/// output can be pinned with inline snapshots.
pub fn redact_dir(text: &str, dir: &Path) -> String {
    redact_dir_with_separator(text, &presentation_path(dir), '/')
}

pub fn candidate_from_diff(diff: &str) -> String {
    let mut candidate = String::new();
    let mut in_hunk = false;
    let mut last_line_was_candidate = false;
    // `str::lines` would drop the `\r` of a CRLF line.
    for line in diff.split('\n') {
        if line.starts_with("@@ ") {
            in_hunk = true;
            last_line_was_candidate = false;
        } else if in_hunk {
            if let Some(content) = line.strip_prefix('+').or_else(|| line.strip_prefix(' ')) {
                candidate.push_str(content);
                candidate.push('\n');
                last_line_was_candidate = true;
            } else if line == "\\ No newline at end of file" && last_line_was_candidate {
                candidate.pop();
            } else {
                last_line_was_candidate = false;
            }
        }
    }
    candidate
}

#[expect(clippy::disallowed_methods, reason = "the chained replacements need an owned string")]
pub fn redact_dir_with_separator(text: &str, dir: &str, separator: char) -> String {
    text.replace(&format!("{dir}{separator}"), "<temp>/").replace(dir, "<temp>")
}

/// Run robusta with stdout piped to a reader that is closed immediately,
/// so writes beyond the OS pipe buffer fail with a broken pipe.
pub fn robusta_with_closed_stdout(args: &[&str], dir: &Path) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_robusta"))
        .current_dir(dir)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run robusta");
    drop(child.stdout.take().expect("stdout should be piped"));
    child.wait_with_output().expect("failed to wait for robusta")
}

/// A valid compilation unit whose formatted output far exceeds the OS pipe
/// buffer, guaranteeing the closed-stdout tests actually hit a failing write.
pub fn oversized_java_source() -> String {
    use std::fmt::Write as _;

    let mut source = String::from("class Example {\n");
    for index in 0..8000 {
        writeln!(source, "  private int field{index};").expect("writing to a String");
    }
    source.push_str("}\n");
    source
}
