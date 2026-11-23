use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use robusta_linter::{FixKind, RuleId};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{
    CEILINGS, HarnessError, TestRunner,
    javac::{Javac, group_errors},
    pipeline,
};

/// The release the self-test compiles at.
const SELF_TEST_RELEASE: u16 = 11;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    sources: Vec<SourceEntry>,
}

/// A source directory, compiled by one `javac` run.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceEntry {
    directory: PathBuf,
    #[serde(default)]
    classpath: Vec<PathBuf>,
    release: u16,
}

/// The outcome of one corpus run.
#[derive(Debug)]
pub struct CorpusReport {
    pub javac_version: String,
    pub self_test: SelfTest,
    pub units: Vec<UnitReport>,
    /// Set when `--keep-temp` preserved the compile workspace for inspection.
    pub kept_workspace: Option<PathBuf>,
}

impl CorpusReport {
    pub fn passed(&self) -> bool {
        self.self_test.passed && self.units.iter().all(UnitReport::passed)
    }
}

/// Whether the check reports a fix that breaks compilation.
#[derive(Debug)]
pub struct SelfTest {
    pub passed: bool,
    pub message: String,
}

/// One source directory: what the rules found and how each tree compiled.
#[derive(Debug)]
pub struct UnitReport {
    pub directory: PathBuf,
    pub release: u16,
    pub files: usize,
    /// Findings per rule, fixable or not.
    pub findings: BTreeMap<&'static str, usize>,
    /// Files the pipeline could not read, lint, or fix, with the reason.
    pub unfixed: Vec<String>,
    pub trees: Vec<TreeReport>,
    /// A problem that stopped the check before anything compiled.
    pub error: Option<String>,
    pub elapsed: Duration,
}

impl UnitReport {
    pub fn passed(&self) -> bool {
        self.error.is_none() && self.unfixed.is_empty() && self.trees.iter().all(TreeReport::passed)
    }
}

/// The original tree, or the tree with every fix up to one ceiling applied.
#[derive(Debug)]
pub struct TreeReport {
    /// `original`, or a ceiling name.
    pub variant: &'static str,
    /// Whether the tree's fixes may break the code.
    pub may_break: bool,
    pub applied: usize,
    pub outcome: TreeOutcome,
}

impl TreeReport {
    pub fn passed(&self) -> bool {
        match self.outcome {
            TreeOutcome::Compiled | TreeOutcome::SameAs(_) => true,
            TreeOutcome::Failed(_) => self.may_break,
            TreeOutcome::Error(_) => false,
        }
    }
}

#[derive(Debug)]
pub enum TreeOutcome {
    Compiled,
    /// Identical to an earlier tree, so not compiled again.
    SameAs(&'static str),
    Failed(Vec<FileErrors>),
    /// The tree could not be written or `javac` could not run.
    Error(String),
}

/// The `javac` errors in one file, with the fixes the tree applied to it.
#[derive(Debug)]
pub struct FileErrors {
    /// The file as `javac` names it, or empty when no file was named.
    pub file: String,
    /// Fix counts per rule and kind.
    pub fixes: BTreeMap<(&'static str, &'static str), usize>,
    pub errors: String,
}

/// One Java file, linted once and fixed at each of [`CEILINGS`].
#[derive(Debug)]
struct SourceFile {
    /// The path relative to the source directory.
    path: String,
    source: String,
    /// The rule and kind of every fix offered for the file.
    fixes: Vec<(RuleId, FixKind)>,
    /// The fixed text at each ceiling, or none when unchanged.
    fixed: [Option<String>; CEILINGS.len()],
    applied: [usize; CEILINGS.len()],
}

/// A source directory's files, ready to compile.
struct Unit<'a> {
    javac: &'a Javac,
    entry: &'a SourceEntry,
    files: Vec<SourceFile>,
}

impl TestRunner {
    /// Fix and compile every source directory the `--corpus` manifests list.
    ///
    /// # Errors
    ///
    /// Returns a [`HarnessError`] when a manifest cannot be read. Compile
    /// failures are reported in the returned [`CorpusReport`].
    pub fn run_corpus(self) -> Result<CorpusReport, HarnessError> {
        let mut entries = Vec::new();
        for path in &self.options.corpus {
            entries.extend(load(path)?);
        }

        let workspace = self.workspace.path();
        let self_test = self_test(&self.javac, &workspace.join("self-test"));
        let units = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                check_unit(&self.javac, entry, &workspace.join(format!("unit-{index}")))
            })
            .collect();

        let kept_workspace = self.options.keep_temp.then(|| self.workspace.keep());
        Ok(CorpusReport { javac_version: self.javac_version, self_test, units, kept_workspace })
    }
}

fn load(path: &Path) -> Result<Vec<SourceEntry>, HarnessError> {
    let error = |message: String| HarnessError(format!("{}: {message}", path.display()));
    let text = fs::read_to_string(path).map_err(|e| error(e.to_string()))?;
    let entries = parse_manifest(&text, path.parent().unwrap_or(Path::new(""))).map_err(error)?;
    for entry in &entries {
        if !entry.directory.is_dir() {
            return Err(error(format!("{} is not a directory", entry.directory.display())));
        }
        if let Some(missing) = entry.classpath.iter().find(|path| !path.exists()) {
            return Err(error(format!("classpath entry {} does not exist", missing.display())));
        }
    }
    Ok(entries)
}

/// Parse a manifest, resolving its paths against `base`. They are made absolute
/// because `javac` runs inside the temporary tree.
fn parse_manifest(text: &str, base: &Path) -> Result<Vec<SourceEntry>, String> {
    let manifest: Manifest = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let resolve = |path: &Path| std::path::absolute(base.join(path)).map_err(|e| e.to_string());
    let mut entries = manifest.sources;
    for entry in &mut entries {
        entry.directory = resolve(&entry.directory)?;
        for path in &mut entry.classpath {
            *path = resolve(path)?;
        }
    }
    Ok(entries)
}

fn check_unit(javac: &Javac, entry: &SourceEntry, dir: &Path) -> UnitReport {
    let started = Instant::now();
    let mut report = UnitReport {
        directory: entry.directory.clone(),
        release: entry.release,
        files: 0,
        findings: BTreeMap::new(),
        unfixed: Vec::new(),
        trees: Vec::new(),
        error: None,
        elapsed: Duration::ZERO,
    };
    match fix_files(&entry.directory, &mut report) {
        Ok(files) => report.trees = Unit { javac, entry, files }.compile_trees(dir),
        Err(error) => report.error = Some(error),
    }
    report.elapsed = started.elapsed();
    report
}

/// Read and fix every Java file under `directory`, recording findings and the
/// files it cannot read or fix. A file that reads but cannot be fixed still
/// compiles unchanged.
fn fix_files(directory: &Path, report: &mut UnitReport) -> Result<Vec<SourceFile>, String> {
    let paths = java_files(directory)
        .map_err(|error| format!("could not read {}: {error}", directory.display()))?;
    if paths.is_empty() {
        return Err("no Java files found".to_owned());
    }
    report.files = paths.len();

    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let name = path.to_string_lossy().into_owned();
        let source = match fs::read_to_string(directory.join(&path)) {
            Ok(source) => source,
            Err(error) => {
                report.unfixed.push(format!("{name}: {error}"));
                continue;
            }
        };
        let mut file = SourceFile {
            path: name,
            source,
            fixes: Vec::new(),
            fixed: Default::default(),
            applied: [0; CEILINGS.len()],
        };
        if let Err(message) = fix_file(&mut file, &mut report.findings) {
            report.unfixed.push(format!("{}: {message}", file.path));
        }
        files.push(file);
    }
    Ok(files)
}

fn fix_file(
    file: &mut SourceFile,
    findings: &mut BTreeMap<&'static str, usize>,
) -> Result<(), String> {
    let diagnostics = pipeline::lint(&file.source)?;
    for diagnostic in &diagnostics {
        *findings.entry(diagnostic.rule_id().as_str()).or_default() += 1;
        if let Some(fix) = diagnostic.fix() {
            file.fixes.push((diagnostic.rule_id(), fix.kind));
        }
    }

    let mut fixed: [Option<String>; CEILINGS.len()] = Default::default();
    let mut applied = [0; CEILINGS.len()];
    for (index, (_, ceiling)) in CEILINGS.into_iter().enumerate() {
        let output = pipeline::fix(&file.source, &diagnostics, ceiling)?;
        applied[index] = output.applied;
        fixed[index] = (output.code != file.source).then_some(output.code);
    }
    file.fixed = fixed;
    file.applied = applied;
    Ok(())
}

/// The `.java` files under `directory`, relative and sorted. `module-info.java`
/// is left out because the check compiles on the class path.
fn java_files(directory: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(directory).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type().is_dir()
            && path.extension().is_some_and(|extension| extension == "java")
            && entry.file_name() != "module-info.java"
        {
            files.push(path.strip_prefix(directory).map_err(io::Error::other)?.to_path_buf());
        }
    }
    Ok(files)
}

impl Unit<'_> {
    /// Compile the original, then each ceiling's tree unless it matches an
    /// earlier one.
    fn compile_trees(&self, dir: &Path) -> Vec<TreeReport> {
        let original = self.compile(None, &dir.join("original"));
        let original_compiled = matches!(original, TreeOutcome::Compiled);
        let mut trees = vec![TreeReport {
            variant: "original",
            may_break: false,
            applied: 0,
            outcome: original,
        }];
        if !original_compiled {
            return trees;
        }

        for (index, (label, ceiling)) in CEILINGS.into_iter().enumerate() {
            let unchanged = self.files.iter().all(|file| file.fixed[index].is_none());
            let same_as = if unchanged {
                Some("original")
            } else {
                (0..index)
                    .rev()
                    .find(|&earlier| {
                        self.files.iter().all(|file| file.fixed[earlier] == file.fixed[index])
                    })
                    .map(|earlier| CEILINGS[earlier].0)
            };
            let outcome = match same_as {
                Some(variant) => TreeOutcome::SameAs(variant),
                None => self.compile(Some(index), &dir.join(label)),
            };
            trees.push(TreeReport {
                variant: label,
                may_break: ceiling.is_unsafe(),
                applied: self.files.iter().map(|file| file.applied[index]).sum(),
                outcome,
            });
        }
        trees
    }

    /// Write the tree at the ceiling with index `ceiling`, or the original,
    /// under `dir/src` and compile it.
    fn compile(&self, ceiling: Option<usize>, dir: &Path) -> TreeOutcome {
        let written = self.files.iter().try_for_each(|file| {
            let text = ceiling.and_then(|index| file.fixed[index].as_deref());
            let path = dir.join("src").join(&file.path);
            fs::create_dir_all(path.parent().unwrap_or(dir))?;
            fs::write(path, text.unwrap_or(&file.source))
        });
        if let Err(error) = written {
            return TreeOutcome::Error(format!("could not write the tree: {error}"));
        }

        let names = self.files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>();
        match self.javac.compile_tree(dir, &names, self.entry.release, &self.entry.classpath) {
            Ok(result) if result.success => TreeOutcome::Compiled,
            Ok(result) => TreeOutcome::Failed(self.attribute(&result.diagnostics, ceiling)),
            Err(error) => TreeOutcome::Error(format!("could not run javac: {error}")),
        }
    }

    /// Pair each file's `javac` errors with the fixes the tree applied to it.
    fn attribute(&self, diagnostics: &str, ceiling: Option<usize>) -> Vec<FileErrors> {
        group_errors(diagnostics)
            .into_iter()
            .map(|(file, errors)| {
                let mut fixes = BTreeMap::new();
                let source = self.files.iter().find(|source| source.path == file);
                if let (Some(index), Some(source)) = (ceiling, source) {
                    for &(rule, kind) in &source.fixes {
                        if CEILINGS[index].1.can_apply(kind) {
                            *fixes.entry((rule.as_str(), kind.as_str())).or_default() += 1;
                        }
                    }
                }
                FileErrors { file, fixes, errors }
            })
            .collect()
    }
}

/// Prove a fix that breaks compilation is reported: a safe fix that deletes a
/// used import must fail the safe tree, attributed to its file and fix.
fn self_test(javac: &Javac, dir: &Path) -> SelfTest {
    const SOURCE: &str =
        "package p;\n\nimport java.util.List;\n\nclass Use {\n    List<String> names;\n}\n";
    const BROKEN: &str = "package p;\n\n\nclass Use {\n    List<String> names;\n}\n";

    let path = Path::new("p").join("Use.java").to_string_lossy().into_owned();
    let entry = SourceEntry {
        directory: PathBuf::new(),
        classpath: Vec::new(),
        release: SELF_TEST_RELEASE,
    };
    let file = SourceFile {
        path: path.clone(),
        source: SOURCE.to_owned(),
        fixes: vec![(RuleId::new("self-test"), FixKind::SAFE_FIX)],
        fixed: std::array::from_fn(|_| Some(BROKEN.to_owned())),
        applied: [1; CEILINGS.len()],
    };
    let trees = Unit { javac, entry: &entry, files: vec![file] }.compile_trees(dir);

    let caught = trees.iter().any(|tree| match &tree.outcome {
        TreeOutcome::Failed(errors) => {
            tree.variant == CEILINGS[0].0
                && !tree.passed()
                && errors.iter().any(|error| {
                    error.file == path && error.fixes.contains_key(&("self-test", "fix"))
                })
        }
        _ => false,
    });
    let (passed, message) = match trees.first().map(|tree| &tree.outcome) {
        Some(TreeOutcome::Compiled) if caught => (
            true,
            "a safe fix that breaks compilation is reported with its file and rule".to_owned(),
        ),
        Some(TreeOutcome::Compiled) => {
            (false, "a safe fix that breaks compilation was not reported".to_owned())
        }
        _ => (
            false,
            format!("the self-test original did not compile at --release {SELF_TEST_RELEASE}"),
        ),
    };
    SelfTest { passed, message }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_paths_resolve_against_its_directory() {
        let absolute = std::env::temp_dir().join("b.jar");
        let text = format!(
            r#"{{ "sources": [
                {{ "directory": "src", "classpath": ["lib/a.jar", {absolute:?}], "release": 17 }},
                {{ "directory": {absolute:?}, "release": 8 }}
            ] }}"#
        );
        let entries = parse_manifest(&text, Path::new("corpus")).unwrap();

        let base = std::path::absolute("corpus").unwrap();
        assert_eq!(entries[0].directory, base.join("src"));
        assert_eq!(entries[0].classpath, [base.join("lib").join("a.jar"), absolute.clone()]);
        assert_eq!(entries[0].release, 17);
        assert_eq!(entries[1].directory, absolute);
        assert_eq!(entries[1].classpath.len(), 0);
    }

    #[test]
    fn manifest_rejects_unknown_fields_and_a_missing_release() {
        assert!(
            parse_manifest(r#"{ "sources": [{ "directory": "src" }] }"#, Path::new("")).is_err()
        );
        let unknown = r#"{ "sources": [{ "directory": "src", "release": 8, "jdk": "/opt" }] }"#;
        assert!(parse_manifest(unknown, Path::new("")).is_err());
    }

    #[test]
    fn java_files_are_relative_sorted_and_skip_module_info() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("b/c")).unwrap();
        for file in ["module-info.java", "b/c/Z.java", "A.java", "b/notes.txt"] {
            fs::write(root.join(file), "").unwrap();
        }

        let files = java_files(root).unwrap();
        assert_eq!(files, [PathBuf::from("A.java"), Path::new("b").join("c").join("Z.java")]);
    }
}
