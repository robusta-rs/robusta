//! Small formatter contract fixture. External-corpus conformance belongs to
//! the `robusta_coverage` task.

use std::{fs, path::Path};

mod fixtures;

use fixtures::check_formatter_contract;

const SMOKE_CORPUS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/corpus_smoke");

#[test]
fn corpus_smoke_round_trip() {
    check_corpus(Path::new(SMOKE_CORPUS));
}

fn check_corpus(root: &Path) {
    let mut files: Vec<_> = fs::read_dir(root)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", root.display()))
        .map(|entry| entry.expect("fixture entry is readable").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "java"))
        .collect();
    files.sort();

    assert!(!files.is_empty(), "{} did not contain any .java files", root.display());

    let mut failures = Vec::new();
    for path in &files {
        if let Err(err) = check_file(path) {
            failures.push(format!("{}: {err}", path.display()));
        }
    }

    assert!(
        failures.is_empty(),
        "corpus round-trip failed for {}/{} files:\n\n{}",
        failures.len(),
        files.len(),
        failures.join("\n\n")
    );
}

fn check_file(path: &Path) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|err| format!("failed to read source: {err}"))?;
    check_formatter_contract(&source)
}
