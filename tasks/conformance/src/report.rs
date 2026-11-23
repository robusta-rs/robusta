use std::fmt::Write as _;

use cow_utils::CowUtils;

use crate::{
    Report,
    corpus::{CorpusReport, FileErrors, TreeOutcome, TreeReport},
};

/// Render a report as stable, human-readable text.
pub fn render(report: &Report) -> String {
    let mut out = String::new();

    let releases = report.releases.iter().map(u16::to_string).collect::<Vec<_>>().join(", ");
    let _ = writeln!(out, "javac: {}", report.javac_version.cow_replace('\n', " "));
    let _ = writeln!(out, "releases: {releases}");
    let _ = writeln!(out);

    for fixture in &report.fixtures {
        let status = if fixture.passed() { "ok" } else { "FAIL" };
        let _ = writeln!(out, "[{status}] {} (rule {})", fixture.name, fixture.rule);
        if let Some(error) = &fixture.error {
            let _ = writeln!(out, "    error: {error}");
        }
        for failure in &fixture.failures {
            let _ = writeln!(
                out,
                "    {} did not compile at --release {}:",
                failure.variant, failure.release
            );
            for line in failure.diagnostics.lines() {
                let _ = writeln!(out, "        {line}");
            }
        }
    }

    for gap in &report.coverage_gaps {
        let _ = writeln!(out, "[FAIL] coverage: {gap}");
    }
    for error in &report.load_errors {
        let _ = writeln!(out, "[FAIL] fixture load: {error}");
    }

    let negative_status = if report.negative.passed { "ok" } else { "FAIL" };
    let compiled = report
        .negative
        .compiled
        .iter()
        .map(|(release, success)| {
            format!("{release}:{}", if *success { "accepts" } else { "rejects" })
        })
        .collect::<Vec<_>>()
        .join(", ");
    let _ = writeln!(out, "[{negative_status}] negative self-test ({compiled})");
    let _ = writeln!(out, "    {}", report.negative.message);

    if let Some(path) = &report.kept_workspace {
        let _ = writeln!(out, "kept compile workspace at {}", path.display());
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "{}",
        if report.passed() { "conformance gate passed" } else { "conformance gate FAILED" }
    );

    out
}

/// Render a corpus run. Times vary between runs; everything else is stable.
pub fn render_corpus(report: &CorpusReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "javac: {}", report.javac_version.cow_replace('\n', " "));
    let _ = writeln!(out);

    let status = if report.self_test.passed { "ok" } else { "FAIL" };
    let _ = writeln!(out, "[{status}] self-test: {}", report.self_test.message);

    for unit in &report.units {
        let status = if unit.passed() { "ok" } else { "FAIL" };
        let _ = writeln!(
            out,
            "[{status}] {} (release {}, {} files, {:.1} s)",
            unit.directory.display(),
            unit.release,
            unit.files,
            unit.elapsed.as_secs_f64()
        );
        if let Some(error) = &unit.error {
            let _ = writeln!(out, "    error: {error}");
        }
        let findings = unit
            .findings
            .iter()
            .map(|(rule, count)| format!("{rule} {count}"))
            .collect::<Vec<_>>()
            .join(", ");
        if !findings.is_empty() {
            let _ = writeln!(out, "    findings: {findings}");
        }
        for file in &unit.unfixed {
            let _ = writeln!(out, "    not fixed: {file}");
        }
        let mut reported = Vec::new();
        for tree in &unit.trees {
            write_tree(&mut out, tree, &mut reported);
        }
    }

    if let Some(path) = &report.kept_workspace {
        let _ = writeln!(out, "kept compile workspace at {}", path.display());
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "{}",
        if report.passed() { "corpus check passed" } else { "corpus check FAILED" }
    );
    out
}

/// Write one tree's outcome. A file whose errors an earlier tree already showed
/// is named with that tree instead of repeating them.
fn write_tree<'a>(
    out: &mut String,
    tree: &'a TreeReport,
    reported: &mut Vec<(&'a FileErrors, &'static str)>,
) {
    let name = if tree.variant == "original" {
        tree.variant.to_owned()
    } else {
        format!("{} ({} fixes)", tree.variant, tree.applied)
    };
    match &tree.outcome {
        TreeOutcome::Compiled => {
            let _ = writeln!(out, "    [ok] {name} compiled");
        }
        TreeOutcome::SameAs(earlier) => {
            let _ = writeln!(out, "    [same] {name}: same tree as {earlier}");
        }
        TreeOutcome::Failed(errors) => {
            let status = if tree.may_break { "review" } else { "FAIL" };
            let _ = writeln!(out, "    [{status}] {name} did not compile:");
            for file in errors {
                let earlier = reported
                    .iter()
                    .find(|(seen, _)| seen.file == file.file && seen.errors == file.errors);
                if let Some((_, variant)) = earlier {
                    let _ = writeln!(out, "        {}: same errors as {variant}", file.file);
                } else {
                    write_file_errors(out, file);
                    reported.push((file, tree.variant));
                }
            }
        }
        TreeOutcome::Error(error) => {
            let _ = writeln!(out, "    [FAIL] {name}: {error}");
        }
    }
}

fn write_file_errors(out: &mut String, file: &FileErrors) {
    let fixes = file
        .fixes
        .iter()
        .map(|((rule, kind), count)| format!("{rule} {kind} {count}"))
        .collect::<Vec<_>>()
        .join(", ");
    let name = if file.file.is_empty() { "javac" } else { &file.file };
    if fixes.is_empty() {
        let _ = writeln!(out, "        {name} (no fix applied):");
    } else {
        let _ = writeln!(out, "        {name} (fixes: {fixes}):");
    }
    for line in file.errors.lines() {
        let _ = writeln!(out, "            {line}");
    }
}
