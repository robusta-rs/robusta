use std::{
    fs, io,
    path::{Path, PathBuf},
};

use robusta_linter::{FixKind, rules};
use tempfile::TempDir;

pub mod args;
pub mod corpus;
mod javac;
mod pipeline;
mod report;
mod test_case;

use args::Args;
use javac::Javac;
use test_case::TestCase;

pub use report::{render, render_corpus};

/// The two CLI fix ceilings, widening: `--fix` and `--fix --unsafe-fixes`.
const CEILINGS: [(&str, FixKind); 2] = [("safe", FixKind::SAFE_FIX), ("unsafe", FixKind::ALL)];

/// A fatal condition that prevents the gate from producing a verdict at all,
/// as opposed to a fixture that simply fails.
#[derive(Debug)]
pub struct HarnessError(pub String);

impl std::fmt::Display for HarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The full outcome of one gate run.
#[derive(Debug)]
pub struct Report {
    pub javac_version: String,
    pub releases: Vec<u16>,
    pub fixtures: Vec<FixtureReport>,
    pub coverage_gaps: Vec<String>,
    pub load_errors: Vec<String>,
    pub negative: NegativeReport,
    /// Set when `--keep-temp` preserved the compile workspace for inspection.
    pub kept_workspace: Option<PathBuf>,
}

impl Report {
    /// Whether every fixture, the coverage requirement, and the self-test all
    /// passed.
    pub fn passed(&self) -> bool {
        self.load_errors.is_empty()
            && self.coverage_gaps.is_empty()
            && self.negative.passed
            && self.fixtures.iter().all(FixtureReport::passed)
    }
}

/// One compiled fixture and every way it could have failed.
#[derive(Debug)]
pub struct FixtureReport {
    pub name: String,
    pub rule: String,
    pub applied_at_unsafe: usize,
    /// A pipeline or authoring problem that stopped compilation.
    pub error: Option<String>,
    pub failures: Vec<CompileFailure>,
}

impl FixtureReport {
    pub fn passed(&self) -> bool {
        self.error.is_none() && self.failures.is_empty() && self.applied_at_unsafe > 0
    }
}

/// A single `--release` compilation that should have succeeded but did not.
#[derive(Debug)]
pub struct CompileFailure {
    /// Which emitted variant failed: `original`, or a ceiling name.
    pub variant: String,
    pub release: u16,
    pub diagnostics: String,
}

/// The harness self-test: a deliberately post-Java-8 rewrite the matrix must
/// reject.
#[derive(Debug)]
pub struct NegativeReport {
    pub compiled: Vec<(u16, bool)>,
    pub passed: bool,
    pub message: String,
}

/// Runs the gate with one `javac` and one temporary compile workspace.
pub struct TestRunner {
    options: Args,
    javac: Javac,
    javac_version: String,
    workspace: TempDir,
}

impl TestRunner {
    /// Resolve `javac`, prove it runs, and create the temporary compile workspace.
    ///
    /// # Errors
    ///
    /// Returns a [`HarnessError`] when `javac` cannot run or the workspace
    /// cannot be created.
    pub fn new(options: Args) -> Result<Self, HarnessError> {
        let javac = Javac::resolve();
        let javac_version = javac
            .version()
            .map_err(|error| HarnessError(format!("could not run javac: {error}")))?;
        let workspace = TempDir::new()
            .map_err(|error| HarnessError(format!("could not create temp workspace: {error}")))?;
        Ok(Self { options, javac, javac_version, workspace })
    }

    /// Run the conformance gate.
    ///
    /// # Errors
    ///
    /// Returns a [`HarnessError`] for conditions that make the gate unable to
    /// judge: a requested `--release` level is unsupported, or the fixture
    /// directory cannot be read. Fixture-level failures are reported in the
    /// returned [`Report`], not as errors.
    pub fn run(self) -> Result<Report, HarnessError> {
        self.ensure_releases_supported()?;

        let fixtures_dir = self.options.fixtures_dir();
        let (cases, load_errors) = self
            .load_test_cases(&fixtures_dir)
            .map_err(|error| HarnessError(format!("could not read fixtures: {error}")))?;
        if cases.is_empty() && load_errors.is_empty() {
            return Err(HarnessError(format!(
                "no fixtures found under {}",
                fixtures_dir.display()
            )));
        }

        let coverage_gaps = coverage_gaps(&cases, self.options.filter.is_some());
        let fixtures =
            cases.iter().enumerate().map(|(index, case)| case.test(&self, index)).collect();
        let negative = self.negative_self_test();

        let kept_workspace = self.options.keep_temp.then(|| self.workspace.keep());

        Ok(Report {
            javac_version: self.javac_version,
            releases: self.options.releases().to_vec(),
            fixtures,
            coverage_gaps,
            load_errors,
            negative,
            kept_workspace,
        })
    }

    /// Fail loudly if any requested release is not supported, rather than skipping
    /// the contract silently.
    fn ensure_releases_supported(&self) -> Result<(), HarnessError> {
        for &release in self.options.releases() {
            let dir = self.workspace.path().join(format!("probe-{release}"));
            fs::create_dir_all(&dir)
                .map_err(|error| HarnessError(format!("could not create probe dir: {error}")))?;
            let supported = self
                .javac
                .supports_release(release, &dir)
                .map_err(|error| HarnessError(format!("could not run javac: {error}")))?;
            if !supported {
                return Err(HarnessError(format!(
                    "javac does not support `--release {release}`; provision a JDK that targets \
                     every requested source level (default 8, 11, 17, 21, 25)"
                )));
            }
        }
        Ok(())
    }

    /// Load every `.java` fixture under `dir` whose name contains `--filter`,
    /// sorted by name. A fixture that cannot be loaded is reported, not fatal.
    fn load_test_cases(&self, dir: &Path) -> io::Result<(Vec<TestCase>, Vec<String>)> {
        let mut paths = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().is_some_and(|extension| extension == "java") {
                paths.push(path);
            }
        }
        paths.sort();

        let mut cases = Vec::new();
        let mut errors = Vec::new();
        for path in paths {
            let name = path.file_stem().map_or_else(
                || path.display().to_string(),
                |stem| stem.to_string_lossy().into_owned(),
            );
            if self.options.filter.as_deref().is_some_and(|needle| !name.contains(needle)) {
                continue;
            }
            match TestCase::new(name.clone(), &path) {
                Ok(case) => cases.push(case),
                Err(message) => errors.push(format!("{name}: {message}")),
            }
        }
        Ok((cases, errors))
    }

    /// Prove the matrix rejects a post-Java-8 replacement. A hypothetical bad fix
    /// that renames a local to `_` is valid Java 8 but illegal from Java 9 on. If
    /// a Java-8-only gate accepted it, the guarantee would be hollow.
    fn negative_self_test(&self) -> NegativeReport {
        const BAD: &str =
            "class RobustaNegativeProbe { void m() { int _ = 0; System.out.println(_); } }\n";

        let releases = self.options.releases();
        let mut compiled = Vec::with_capacity(releases.len());
        for &release in releases {
            let dir = self.workspace.path().join(format!("negative-{release}"));
            let success = fs::create_dir_all(&dir)
                .ok()
                .and_then(|()| self.javac.compile(BAD, release, &dir).ok())
                .is_some_and(|result| result.success);
            compiled.push((release, success));
        }

        let rejected_somewhere = compiled.iter().any(|(_, success)| !success);
        let eight_accepts =
            compiled.iter().find(|(release, _)| *release == 8).is_none_or(|(_, success)| *success);

        let (passed, message) = if !rejected_somewhere {
            (
                false,
                "release matrix accepted a `_` identifier at every level; add a Java 9+ level so \
                 post-Java-8 regressions are caught"
                    .to_owned(),
            )
        } else if !eight_accepts {
            (
                false,
                "release 8 rejected the `_` identifier probe; the self-test is not exercising the \
                 Java-8-valid-but-later-illegal transition"
                    .to_owned(),
            )
        } else {
            (
                true,
                "a `_` rename is accepted at Java 8 and rejected at a later level, as required"
                    .to_owned(),
            )
        };

        NegativeReport { compiled, passed, message }
    }
}

/// Every built-in rule that can emit a fix must own at least one fixture, so a
/// newly added fixer cannot land without a compatibility fixture. Skipped when
/// a `--filter` was supplied, since a narrowed run is not a coverage audit.
fn coverage_gaps(loaded: &[TestCase], filtered: bool) -> Vec<String> {
    if filtered {
        return Vec::new();
    }
    let mut gaps = Vec::new();
    for rule in rules::builtin_rules() {
        if !rule.meta().fix.has_fix() {
            continue;
        }
        let id = rule.id();
        if !loaded.iter().any(|fixture| fixture.rule == id) {
            gaps.push(format!(
                "rule `{id}` can emit a fix but has no conformance fixture (add one under fixtures/)"
            ));
        }
    }
    gaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_gap_reported_when_a_fixable_rule_has_no_fixture() {
        let gaps = coverage_gaps(&[], false);
        assert!(!gaps.is_empty(), "fixable built-in rules should be flagged as uncovered");
        assert!(gaps.iter().any(|gap| gap.contains("redundant-semicolons")));
    }

    #[test]
    fn coverage_is_not_audited_under_a_filter() {
        assert_eq!(coverage_gaps(&[], true).len(), 0);
    }
}
