use std::{fs, path::Path};

use robusta_linter::{FixKind, RuleId, rules};

use crate::{CEILINGS, CompileFailure, FixtureReport, TestRunner, pipeline};

/// The header marker that declares a fixture's covered rule.
const HEADER_PREFIX: &str = "// robusta-fixture: rule=";

/// One loaded fixture: its display name, declared rule, and normalized source.
#[derive(Debug)]
pub struct TestCase {
    /// File name without extension, used for deterministic reporting.
    name: String,
    /// The rule ID the fixture is written to exercise.
    pub rule: RuleId,
    /// Source normalized to LF so output is identical on every platform.
    source: String,
}

impl TestCase {
    /// Read the fixture at `path`, or say why it is unreadable or lacks a known
    /// rule header.
    pub fn new(name: String, path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;
        let source = normalize_newlines(&raw);
        let declared = parse_rule_header(&source)
            .ok_or_else(|| format!("missing `{HEADER_PREFIX}<rule-id>` header line"))?;
        let rule = resolve_rule(declared)
            .ok_or_else(|| format!("header names unknown rule `{declared}`"))?;
        Ok(Self { name, rule, source })
    }

    /// Fix the fixture at each ceiling and compile the original and every
    /// distinct output at each release. `index` keeps its compile directories
    /// apart from other fixtures'.
    pub fn test(&self, runner: &TestRunner, index: usize) -> FixtureReport {
        let mut report = FixtureReport {
            name: self.name.clone(),
            rule: self.rule.as_str().to_owned(),
            applied_at_unsafe: 0,
            error: None,
            failures: Vec::new(),
        };
        // The original must be genuinely Java-8-valid across the whole matrix.
        let mut variants: Vec<(String, String)> =
            vec![("original".to_owned(), self.source.clone())];

        let diagnostics = match pipeline::lint(&self.source) {
            Ok(diagnostics) => diagnostics,
            Err(message) => {
                report.error = Some(message);
                return report;
            }
        };
        for (label, ceiling) in CEILINGS {
            match pipeline::fix(&self.source, &diagnostics, ceiling) {
                Ok(output) => {
                    if ceiling == FixKind::ALL {
                        report.applied_at_unsafe = output.applied;
                    }
                    if !variants.iter().any(|(_, code)| *code == output.code) {
                        variants.push((label.to_owned(), output.code));
                    }
                }
                Err(message) => {
                    report.error = Some(format!("{label} ceiling: {message}"));
                    return report;
                }
            }
        }

        if report.applied_at_unsafe == 0 {
            report.error = Some("fixture triggers no fix at the unsafe ceiling".to_owned());
            return report;
        }

        for (variant_index, (label, code)) in variants.iter().enumerate() {
            for &release in runner.options.releases() {
                let dir = runner
                    .workspace
                    .path()
                    .join(format!("fixture-{index}-{variant_index}-{release}"));
                if let Err(error) = fs::create_dir_all(&dir) {
                    report.failures.push(CompileFailure {
                        variant: label.clone(),
                        release,
                        diagnostics: format!("could not create compile dir: {error}"),
                    });
                    continue;
                }
                match runner.javac.compile(code, release, &dir) {
                    Ok(result) if result.success => {}
                    Ok(result) => report.failures.push(CompileFailure {
                        variant: label.clone(),
                        release,
                        diagnostics: result.diagnostics,
                    }),
                    Err(error) => report.failures.push(CompileFailure {
                        variant: label.clone(),
                        release,
                        diagnostics: format!("could not run javac: {error}"),
                    }),
                }
            }
        }

        report
    }
}

/// Match a declared rule name against a built-in rule, returning its static ID.
fn resolve_rule(declared: &str) -> Option<RuleId> {
    rules::builtin_rules().iter().map(|rule| rule.id()).find(|id| id.as_str() == declared)
}

/// Normalize CRLF and lone CR line endings to LF.
///
/// Windows checkouts may materialize the corpus with CRLF; normalizing here
/// keeps both the linted input and the emitted fixed output byte-identical
/// across platforms, which the compile matrix depends on.
#[expect(clippy::disallowed_methods, reason = "the normalized source is returned owned")]
fn normalize_newlines(source: &str) -> String {
    source.replace("\r\n", "\n").replace('\r', "\n")
}

fn parse_rule_header(source: &str) -> Option<&str> {
    source.lines().find_map(|line| line.trim().strip_prefix(HEADER_PREFIX).map(str::trim))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_rule_header_ignoring_surrounding_lines() {
        let source = "// robusta-fixture: rule=unused-local-variable\nclass A {}\n";
        assert_eq!(parse_rule_header(source), Some("unused-local-variable"));
    }

    #[test]
    fn missing_header_is_none() {
        assert_eq!(parse_rule_header("class A {}\n"), None);
    }

    #[test]
    fn resolves_a_real_rule_and_rejects_unknown_names() {
        assert!(resolve_rule("redundant-semicolons").is_some());
        assert_eq!(resolve_rule("no-such-rule"), None);
    }

    #[test]
    fn normalizes_crlf_and_cr_to_lf() {
        assert_eq!(normalize_newlines("a\r\nb\rc\n"), "a\nb\nc\n");
    }
}
