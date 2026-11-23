use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::Serialize;

use robusta_diagnostics::{FileReport, Severity};

use super::{Info, to_pretty_document};
use crate::walk;

#[derive(Serialize)]
struct GitlabFinding<'a> {
    description: &'a str,
    check_name: &'a str,
    fingerprint: String,
    severity: &'static str,
    location: GitlabLocation,
}

#[derive(Serialize)]
struct GitlabLocation {
    path: String,
    lines: GitlabLines,
}

#[derive(Serialize)]
struct GitlabLines {
    begin: usize,
    end: usize,
}

pub(super) fn render(reports: &[&FileReport]) -> String {
    render_from_repository_root(reports, repository_prefix().as_deref())
}

fn render_from_repository_root(
    reports: &[&FileReport],
    repository_prefix: Option<&Path>,
) -> String {
    let mut findings = Vec::new();
    for report in reports {
        // Resolve the path once per file, not per finding.
        let path = &repository_path(report.path(), repository_prefix);
        findings.extend(Info::of(report).map(|info| gitlab_finding(path, &info)));
    }
    to_pretty_document(&findings)
}

// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/apps/oxlint/src/output_formatter/gitlab.rs
fn gitlab_finding<'report>(path: &str, info: &Info<'report>) -> GitlabFinding<'report> {
    let Info { start, end, message, severity, rule, .. } = *info;
    let severity = gitlab_severity(severity);

    let fingerprint = {
        let mut hasher = DefaultHasher::new();
        start.line.hash(&mut hasher);
        start.column.hash(&mut hasher);
        end.line.hash(&mut hasher);
        end.column.hash(&mut hasher);
        path.hash(&mut hasher);
        rule.id.hash(&mut hasher);
        message.hash(&mut hasher);
        severity.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    };

    GitlabFinding {
        description: message,
        check_name: rule.id,
        fingerprint,
        severity,
        location: GitlabLocation {
            path: path.to_string(),
            lines: GitlabLines { begin: start.line, end: end.line },
        },
    }
}

/// Find the repository root by walking up from `start`.
fn repository_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        // Worktrees and submodules keep `.git` as a file.
        if current.join(".git").exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// The repository-relative path of `directory`, or `None` at the root or outside a repository.
fn repository_prefix_from(directory: &Path) -> Option<PathBuf> {
    let root = repository_root(directory)?;
    let prefix = directory.strip_prefix(root).ok()?;
    (!prefix.as_os_str().is_empty()).then(|| prefix.to_path_buf())
}

fn repository_prefix() -> Option<PathBuf> {
    repository_prefix_from(&std::env::current_dir().ok()?)
}

/// Make `path` repository-relative unless it is absolute or no repository was found.
fn repository_path(path: &str, repository_prefix: Option<&Path>) -> String {
    match repository_prefix {
        // GitLab expects `/`, but the join adds native separators.
        Some(prefix) => walk::display_path(&prefix.join(path)).to_string_lossy().into_owned(),
        None => path.to_owned(),
    }
}

fn gitlab_severity(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "critical",
        Severity::Warning => "major",
        Severity::Advice => "minor",
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{
        DEMO_RULE, RULELESS, UNDESCRIBED_RULE, finding, position, ranged_finding, report_with,
    };
    use super::*;

    #[test]
    #[cfg_attr(
        not(all(target_endian = "little", target_pointer_width = "64")),
        ignore = "fingerprints hash native-width, native-endian positions"
    )]
    fn emits_code_quality_findings_with_stable_fingerprints() {
        let report = report_with(
            "src/C.java",
            Some(false),
            vec![
                ranged_finding(
                    DEMO_RULE,
                    Severity::Warning,
                    "use \"quotes\"\nnext",
                    position(2, 5),
                    position(3, 7),
                ),
                finding(RULELESS, Severity::Error, "Expected ':' but found ]", 4, 9),
            ],
        );
        insta::assert_snapshot!(render_from_repository_root(&[&report], None), @r#"
        [
          {
            "description": "file is not formatted; run `robusta format --fix`",
            "check_name": "format",
            "fingerprint": "e67369a7a9c0da85",
            "severity": "major",
            "location": {
              "path": "src/C.java",
              "lines": {
                "begin": 1,
                "end": 1
              }
            }
          },
          {
            "description": "use \"quotes\"\nnext",
            "check_name": "demo",
            "fingerprint": "d2c710db021307a5",
            "severity": "major",
            "location": {
              "path": "src/C.java",
              "lines": {
                "begin": 2,
                "end": 3
              }
            }
          },
          {
            "description": "Expected ':' but found ]",
            "check_name": "",
            "fingerprint": "5b66d13f16f049a7",
            "severity": "critical",
            "location": {
              "path": "src/C.java",
              "lines": {
                "begin": 4,
                "end": 4
              }
            }
          }
        ]
        "#);
    }

    #[test]
    fn each_finding_on_a_line_has_its_own_fingerprint() {
        let report = report_with(
            "src/C.java",
            None,
            vec![
                finding(DEMO_RULE, Severity::Warning, "same message", 2, 5),
                finding(DEMO_RULE, Severity::Warning, "same message", 2, 11),
                finding(UNDESCRIBED_RULE, Severity::Warning, "same message", 2, 11),
            ],
        );
        let fingerprints = || {
            let rendered = render_from_repository_root(&[&report], None);
            let json: serde_json::Value = serde_json::from_str(&rendered).unwrap();
            json.as_array()
                .unwrap()
                .iter()
                .map(|value| value["fingerprint"].as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };

        let first = fingerprints();
        assert_eq!(first.len(), 3);
        assert_ne!(first[0], first[1]);
        assert_ne!(first[1], first[2]);
        assert_eq!(first, fingerprints());
    }

    #[test]
    fn clean_report_is_an_empty_array() {
        let report = report_with("src/Clean.java", Some(true), Vec::new());
        assert_eq!(render_from_repository_root(&[&report], None), "[]\n");
    }

    #[test]
    fn paths_and_fingerprints_are_resolved_against_the_repository_root() {
        let report = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Warning, "msg", 2, 5)],
        );
        let rendered =
            render_from_repository_root(&[&report], Some(&PathBuf::from("modules").join("core")));

        assert!(rendered.contains(r#""path": "modules/core/src/C.java""#), "{rendered}");
        let resolved = report_with(
            "modules/core/src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Warning, "msg", 2, 5)],
        );
        assert_eq!(rendered, render_from_repository_root(&[&resolved], None));
    }

    #[test]
    fn an_absolute_path_is_reported_as_it_stands() {
        let absolute = if cfg!(windows) { r"C:/repo/src/C.java" } else { "/repo/src/C.java" };
        let report = report_with(absolute, Some(false), Vec::new());
        let rendered = render_from_repository_root(&[&report], Some(Path::new("modules/core")));

        assert!(rendered.contains(&format!(r#""path": "{absolute}""#)), "{rendered}");
    }

    #[cfg(unix)]
    #[test]
    fn a_literal_backslash_stays_part_of_the_file_name() {
        assert_eq!(
            repository_path(r"src\A.java", Some(Path::new("modules/core"))),
            r"modules/core/src\A.java"
        );
    }

    #[test]
    fn the_prefix_spans_the_working_directory_below_the_repository_root() {
        let root = tempfile::tempdir().expect("failed to create temp dir");
        let nested = root.path().join("modules").join("core");
        std::fs::create_dir_all(&nested).expect("failed to create nested dir");
        std::fs::create_dir(root.path().join(".git")).expect("failed to create .git dir");

        assert_eq!(
            repository_prefix_from(&nested),
            Some(PathBuf::from("modules").join("core")),
            "a run below the root carries the path down to it"
        );
        assert_eq!(repository_prefix_from(root.path()), None, "a run at the root needs no prefix");
    }

    #[test]
    fn a_directory_outside_a_repository_has_no_prefix() {
        let outside = tempfile::tempdir().expect("failed to create temp dir");

        assert_eq!(repository_prefix_from(outside.path()), None);
    }

    #[test]
    fn advice_maps_to_the_gitlab_minor_severity() {
        let report = report_with(
            "src/C.java",
            None,
            vec![finding(DEMO_RULE, Severity::Advice, "consider this", 2, 5)],
        );
        let rendered = render_from_repository_root(&[&report], None);
        assert!(rendered.contains("\"severity\": \"minor\""), "got: {rendered}");
    }
}
