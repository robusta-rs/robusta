use std::path::{Path, PathBuf};

use cow_utils::CowUtils;

use crate::support::*;

const BASELINE: &str = "robusta-suppressions.json";

/// `uppercase-long-suffix` is an error and `modifier-order` a warning.
const CONFIG: &str = "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n";

/// Two `uppercase-long-suffix` errors and one `modifier-order` warning.
const TWO_ERRORS: &str = "class A {\n  static public long a = 1l;\n  long b = 2l;\n}\n";

const TWO_ALLOWED: &str =
    "{\n  \"A.java\": {\n    \"uppercase-long-suffix\": {\n      \"count\": 2\n    }\n  }\n}\n";

fn project(source: &str) -> TempProject {
    let temp = TempProject::new();
    temp.write("robusta.yaml", CONFIG);
    temp.write("A.java", source);
    temp
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn baseline_path(temp: &TempProject) -> PathBuf {
    temp.dir.join(BASELINE)
}

#[test]
fn suppress_all_records_error_counts_that_later_runs_allow() {
    let temp = project(TWO_ERRORS);

    let suppress = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    let report = stdout_without_summary(&suppress);
    assert_eq!(suppress.status.code(), Some(0), "{report}");
    // Warnings are never recorded, so the one warning is still reported.
    assert!(report.contains("modifier-order"), "{report}");
    assert!(!report.contains("uppercase-long-suffix"), "{report}");
    assert_eq!(stderr(&suppress), "robusta-suppressions.json: created\n");
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);

    for command in ["lint", "check"] {
        let output =
            robusta_in_dir(&[command, "--deny-warnings", "-A", "modifier-order"], &temp.dir);
        let report = stdout_without_summary(&output);
        assert_eq!(output.status.code(), Some(0), "{command}: {report}");
        assert!(!report.contains("uppercase-long-suffix"), "{command}: {report}");
    }
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
}

#[test]
fn the_documented_baseline_hides_its_counted_error() {
    let documented = r#"{
  "src/main/java/com/example/Names.java": {
    "unused-import": { "count": 1 }
  }
}
"#;
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    unused-import: deny\n");
    let names = "import java.util.List;\nimport java.util.Map;\n\nclass Names {\n    List<String> all;\n}\n";
    temp.write("src/main/java/com/example/Names.java", names);
    temp.write(BASELINE, documented);

    let output = robusta_in_dir(&["lint"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(!stdout(&output).contains("unused-import"), "{}", stdout(&output));
    assert_eq!(read(&baseline_path(&temp)), documented);

    std::fs::remove_file(baseline_path(&temp)).unwrap();
    let output = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    let written: serde_json::Value = serde_json::from_str(&read(&baseline_path(&temp))).unwrap();
    assert_eq!(written, serde_json::from_str::<serde_json::Value>(documented).unwrap());
}

#[test]
fn a_file_over_its_allowance_reports_every_finding_until_suppress_all_raises_it() {
    let temp = project(TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);
    temp.write("A.java", "class A {\n    long a = 1l;\n    long b = 2l;\n    long c = 3l;\n}\n");

    let output = robusta_in_dir(&["lint"], &temp.dir);
    let report = stdout_without_summary(&output);
    assert_eq!(output.status.code(), Some(1), "{report}");
    assert_eq!(report.matches("x uppercase-long-suffix").count(), 3, "{report}");
    assert_eq!(
        stderr(&output),
        "robusta-suppressions.json: 1 allowance exceeded, so all of its findings are reported\n"
    );

    let suppress = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    assert_eq!(suppress.status.code(), Some(0), "{}", stdout(&suppress));
    assert_eq!(stderr(&suppress), "robusta-suppressions.json: updated\n");
    assert!(read(&baseline_path(&temp)).contains("\"count\": 3"));
    assert_eq!(robusta_in_dir(&["lint"], &temp.dir).status.code(), Some(0));
}

/// The baseline counts findings rather than locating them, so replacing one
/// finding with another of the same rule in the same file goes unnoticed. The
/// public documentation states this limit.
#[test]
fn an_equal_count_replacement_escapes_detection() {
    let temp = project(TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);
    temp.write("A.java", "class A {\n    long fresh = 3l;\n    long b = 2l;\n}\n");

    let output = robusta_in_dir(&["lint"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
}

#[test]
fn an_obsolete_allowance_fails_until_it_is_pruned() {
    let temp = project("class A {\n  long b = 2l;\n}\n");
    temp.write(BASELINE, TWO_ALLOWED);

    let output = robusta_in_dir(&["lint"], &temp.dir);
    let report = stdout_without_summary(&output);
    assert_eq!(output.status.code(), Some(1), "{report}");
    assert!(
        report.contains(
            "x uppercase-long-suffix: A.java: robusta-suppressions.json allows 2 errors, but only 1 \
             remains; lower the allowance with `--prune-suppressions`"
        ),
        "{report}"
    );
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);

    for command in ["check", "lint"] {
        let prune = robusta_in_dir(&[command, "--prune-suppressions", "A.java"], &temp.dir);
        let report = stdout_without_summary(&prune);
        assert_eq!(prune.status.code(), Some(0), "{command}: {report}");
        assert!(!report.contains("allows 2 errors"), "{command}: {report}");
        assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED.cow_replace("2\n", "1\n"), "{command}");
    }
    let unchanged = robusta_in_dir(&["lint", "--prune-suppressions"], &temp.dir);
    assert_eq!(unchanged.status.code(), Some(0), "{}", stdout(&unchanged));
    assert!(!stdout(&unchanged).contains(BASELINE), "{}", stdout(&unchanged));
    assert!(unchanged.stderr.is_empty(), "{}", stderr(&unchanged));
}

#[test]
fn fixes_apply_to_baselined_findings_and_prune_the_baseline() {
    let temp = project(TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);
    let fixed = "class A {\n  public static long a = 1L;\n  long b = 2L;\n}\n";

    let diff = robusta_in_dir(&["lint", "--diff"], &temp.dir);
    assert_eq!(diff.status.code(), Some(1), "{}", stderr(&diff));
    assert_eq!(candidate_from_diff(&stdout(&diff)), fixed);
    assert_eq!(read(&temp.dir.join("A.java")), TWO_ERRORS);
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);

    for command in ["lint", "check"] {
        for explicit in [false, true] {
            temp.write("A.java", TWO_ERRORS);
            temp.write(BASELINE, TWO_ALLOWED);
            let mut args = vec![command, "--fix"];
            if explicit {
                args.push("--prune-suppressions");
            }
            let fix = robusta_in_dir(&args, &temp.dir);
            assert_eq!(fix.status.code(), Some(0), "{args:?}: {}", stdout(&fix));
            assert_eq!(read(&temp.dir.join("A.java")), fixed);
            assert_eq!(read(&baseline_path(&temp)), "{}\n");
        }
    }
}

/// With a fix flag, `--suppress-all` records only what the fixes leave.
#[test]
fn suppress_all_with_fixes_records_the_errors_that_remain() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    unused-local-variable: deny\n",
    );
    temp.write("A.java", "class A {\n    long b = 2l;\n    void f() { int unused = send(); }\n    int send() { return 1; }\n}\n");

    let output = robusta_in_dir(&["lint", "--fix", "--suppress-all"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(read(&temp.dir.join("A.java")).contains("long b = 2L;"));
    assert_eq!(
        read(&baseline_path(&temp)),
        "{\n  \"A.java\": {\n    \"unused-local-variable\": {\n      \"count\": 1\n    }\n  }\n}\n"
    );
}

#[test]
fn suppress_all_refreshes_lower_and_zero_counts() {
    for (source, expected) in [
        ("class A {\n    long a = 1l;\n}\n", TWO_ALLOWED.cow_replace('2', "1").into_owned()),
        ("class A {}\n", "{}\n".to_owned()),
    ] {
        let temp = project(source);
        temp.write(BASELINE, TWO_ALLOWED);
        let output = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
        assert_eq!(read(&baseline_path(&temp)), expected);
        assert_eq!(read(&temp.dir.join("A.java")), source);
        assert_eq!(robusta_in_dir(&["lint"], &temp.dir).status.code(), Some(0));
    }
}

#[test]
fn unsafe_fixes_prune_the_errors_they_fix() {
    let source = "class A {\n    long a = 1l;\n    void f() {\n        int unused = send();\n    }\n    int send() {\n        return 1;\n    }\n}\n";
    for command in ["lint", "check"] {
        let temp = project(source);
        temp.write(
            "robusta.yaml",
            "lint:\n  rules:\n    uppercase-long-suffix: deny\n    unused-local-variable: deny\n",
        );
        temp.write(BASELINE, "{\"A.java\":{\"uppercase-long-suffix\":{\"count\":1},\"unused-local-variable\":{\"count\":1}}}");
        let args = [command, "--fix", "--unsafe-fixes"];
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stdout(&output));
        let fixed = read(&temp.dir.join("A.java"));
        assert!(fixed.contains("send();"), "{fixed}");
        assert!(!fixed.contains("int unused"), "{fixed}");
        assert!(!fixed.contains("1l"), "{args:?}: {fixed}");
        assert_eq!(read(&baseline_path(&temp)), "{}\n", "{args:?}");
    }
}

#[test]
fn unsafe_fixes_prune_hidden_unsafe_findings() {
    for command in ["lint", "check"] {
        let temp = project(
            "class A {\n    void f() {\n        int unused = send() + 1;\n    }\n    int send() {\n        return 1;\n    }\n}\n",
        );
        temp.write("robusta.yaml", "lint:\n  rules:\n    unused-local-variable: deny\n");
        temp.write(BASELINE, "{\"A.java\":{\"unused-local-variable\":{\"count\":1}}}");
        let output = robusta_in_dir(&[command, "--fix", "--unsafe-fixes"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stdout(&output));
        assert!(!read(&temp.dir.join("A.java")).contains("send() + 1"));
        assert_eq!(read(&baseline_path(&temp)), "{}\n");
    }
}

#[test]
fn a_file_check_does_not_lint_keeps_its_allowances() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", &format!("{CONFIG}  ignore-patterns:\n    - A.java\n"));
    temp.write("A.java", TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);

    for args in [&["check"][..], &["check", "--fix"], &["check", "--prune-suppressions"]] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stdout(&output));
        assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED, "{args:?}");
    }
}

#[test]
fn a_file_check_does_not_format_still_settles_its_allowances() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", &format!("{CONFIG}format:\n  ignore-patterns:\n    - A.java\n"));
    temp.write("A.java", "class A{long b=2l;}\n");
    temp.write(BASELINE, TWO_ALLOWED);

    let output = robusta_in_dir(&["check"], &temp.dir);
    let report = stdout_without_summary(&output);
    assert_eq!(output.status.code(), Some(1), "{report}");
    assert!(report.contains("allows 2 errors, but only 1 remains"), "{report}");
    assert!(!report.contains("not formatted"), "{report}");

    let prune = robusta_in_dir(&["check", "--prune-suppressions"], &temp.dir);
    assert_eq!(prune.status.code(), Some(0), "{}", stdout(&prune));
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED.cow_replace("2\n", "1\n"));
    assert_eq!(read(&temp.dir.join("A.java")), "class A{long b=2l;}\n");
}

#[test]
fn baseline_refresh_preserves_unselected_and_ignored_files() {
    for flags in [&["A.java"][..], &["--ignore-pattern", "B.java"][..]] {
        let temp = project("class A {\n    long a = 1l;\n}\n");
        temp.write("B.java", "class B {}\n");
        let original = serde_json::json!({
            "A.java": { "uppercase-long-suffix": { "count": 2 } },
            "B.java": { "uppercase-long-suffix": { "count": 2 } },
        });
        temp.write(BASELINE, &original.to_string());
        let mut args = vec!["lint", "--suppress-all"];
        args.extend(flags);
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stdout(&output));
        let result: serde_json::Value = serde_json::from_str(&read(&baseline_path(&temp))).unwrap();
        assert_eq!(result["A.java"]["uppercase-long-suffix"]["count"], 1);
        assert_eq!(result["B.java"]["uppercase-long-suffix"]["count"], 2);
    }
}

#[test]
fn inline_suppressions_are_left_alone_and_their_findings_are_not_counted() {
    let source = "class A {\n    @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n    long a = 1l;\n    long b = 2l;\n}\n";
    let temp = project(source);

    let output = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(read(&temp.dir.join("A.java")), source);
    assert!(read(&baseline_path(&temp)).contains("\"count\": 1"));
}

/// Hygiene findings are never baselined, even at error severity.
#[test]
fn unused_suppression_findings_are_never_hidden() {
    let source =
        "class A {\n    @SuppressWarnings(\"robusta:modifier-order\")\n    long a = 1L;\n}\n";
    let temp = project(source);
    let args = ["lint", "--suppress-all", "--report-unused-suppressions-severity", "deny"];

    let output = robusta_in_dir(&args, &temp.dir);
    let report = stdout_without_summary(&output);

    assert_eq!(output.status.code(), Some(1), "{report}");
    assert!(report.contains("unused-suppression"), "{report}");
    assert!(!baseline_path(&temp).exists());
}

/// An inline suppression counts as used before the baseline hides anything,
/// so a baselined finding beside it never makes it look unused.
#[test]
fn inline_suppressions_are_audited_before_the_baseline() {
    let source = "class A {\n    @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n    long a = 1l;\n    long b = 2l;\n}\n";
    let temp = project(source);
    temp.write(BASELINE, &TWO_ALLOWED.cow_replace('2', "1"));
    let args = ["lint", "--report-unused-suppressions-severity", "deny"];

    let output = robusta_in_dir(&args, &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn allowances_for_disabled_or_warning_rules_are_obsolete() {
    let source = "class A {\n  long a = 1l;\n}\n";
    let temp = project(source);
    for command in ["lint", "check"] {
        for severity in ["-A", "-W"] {
            temp.write(BASELINE, TWO_ALLOWED);
            let args = [command, severity, "uppercase-long-suffix"];
            let output = robusta_in_dir(&args, &temp.dir);
            let report = stdout_without_summary(&output);
            assert_eq!(output.status.code(), Some(1), "{args:?}: {report}");
            assert!(report.contains("allows 2 errors, but none remain"), "{report}");
            assert_eq!(report.contains("! uppercase-long-suffix"), severity == "-W", "{report}");
            assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);

            let prune = robusta_in_dir(
                &[command, severity, "uppercase-long-suffix", "--prune-suppressions"],
                &temp.dir,
            );
            assert_eq!(prune.status.code(), Some(0), "{}", stdout(&prune));
            assert_eq!(read(&baseline_path(&temp)), "{}\n");
            assert_eq!(read(&temp.dir.join("A.java")), source);
        }
    }
}

#[test]
fn baseline_settlement_uses_nested_configs_and_ordered_filters() {
    for flags in [&[][..], &["-A", "all", "-D", "uppercase-long-suffix"][..]] {
        let source = "class A {\n    long a = 1l;\n}\n";
        let temp = project(source);
        temp.write("off/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: allow\n");
        temp.write("warn/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
        temp.write("off/A.java", source);
        temp.write("warn/A.java", source);
        let original = serde_json::json!({
            "A.java": { "uppercase-long-suffix": { "count": 2 } },
            "off/A.java": { "uppercase-long-suffix": { "count": 2 } },
            "warn/A.java": { "uppercase-long-suffix": { "count": 2 } },
        });
        temp.write(BASELINE, &original.to_string());
        let mut args = vec!["lint"];
        args.extend(flags);
        let verify = robusta_in_dir(&args, &temp.dir);
        let report = stdout_without_summary(&verify);
        assert_eq!(verify.status.code(), Some(1), "{args:?}: {report}");
        assert_eq!(report.matches("allows 2 errors").count(), 3, "{report}");
        assert_eq!(read(&baseline_path(&temp)), original.to_string());

        args.push("--prune-suppressions");
        let prune = robusta_in_dir(&args, &temp.dir);
        assert_eq!(prune.status.code(), Some(0), "{args:?}: {}", stdout(&prune));
        let result: serde_json::Value = serde_json::from_str(&read(&baseline_path(&temp))).unwrap();
        assert_eq!(result["A.java"]["uppercase-long-suffix"]["count"], 1);
        if flags.is_empty() {
            assert!(result.get("off/A.java").is_none());
            assert!(result.get("warn/A.java").is_none());
            assert!(stdout(&prune).contains("! uppercase-long-suffix"));
        } else {
            assert_eq!(result["off/A.java"]["uppercase-long-suffix"]["count"], 1);
            assert_eq!(result["warn/A.java"]["uppercase-long-suffix"]["count"], 1);
        }
    }
}

#[test]
fn suppress_all_removes_warning_allowances_without_hiding_warnings() {
    let temp = project(TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);
    let output =
        robusta_in_dir(&["lint", "--suppress-all", "-W", "uppercase-long-suffix"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(stdout(&output).matches("! uppercase-long-suffix").count(), 2);
    assert_eq!(read(&baseline_path(&temp)), "{}\n");
}

#[test]
fn prune_never_increases_allowances() {
    let temp = project(TWO_ERRORS);
    let original = TWO_ALLOWED.cow_replace('2', "1");
    temp.write(BASELINE, &original);
    let output = robusta_in_dir(&["lint", "--prune-suppressions"], &temp.dir);
    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    assert_eq!(stdout(&output).matches("x uppercase-long-suffix").count(), 2);
    assert_eq!(read(&baseline_path(&temp)), original);
}

#[test]
fn stdin_fixes_and_previews_leave_the_baseline_alone() {
    let temp = project(TWO_ERRORS);
    temp.write(BASELINE, TWO_ALLOWED);
    for command in ["lint", "check"] {
        for (flags, expected) in [
            (&["--diff"][..], 1),
            (&["--list-different"][..], 1),
            (&["--unsafe-fixes", "--diff"][..], 1),
        ] {
            let mut args = vec![command];
            args.extend(flags);
            let preview = robusta_in_dir(&args, &temp.dir);
            assert_eq!(preview.status.code(), Some(expected), "{args:?}: {}", stderr(&preview));
            assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
            assert_eq!(read(&temp.dir.join("A.java")), TWO_ERRORS);
        }
        let output = robusta_with_stdin(
            &[command, "--fix", "--stdin-filepath", "A.java"],
            &temp.dir,
            TWO_ERRORS,
        );
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stderr(&output));
        assert!(stdout(&output).contains("1L"));
        assert!(stderr(&output).contains("allows 2 errors, but none remain"));
        assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
        assert_eq!(read(&temp.dir.join("A.java")), TWO_ERRORS);
    }
}

#[test]
fn ordinary_fixes_and_prune_do_not_create_a_baseline() {
    for command in ["lint", "check"] {
        for flags in [&["--fix"][..], &["--prune-suppressions"][..]] {
            let temp = project("class A {\n  long a = 1l;\n}\n");
            let mut args = vec![command];
            args.extend(flags);
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(
                output.status.code(),
                Some(i32::from(flags[0] != "--fix")),
                "{args:?}: {}",
                stdout(&output)
            );
            assert!(!baseline_path(&temp).exists());
        }
    }
}

#[test]
fn empty_pruning_drops_deleted_files_without_rewriting_an_unchanged_baseline() {
    for command in ["lint", "check"] {
        for paths in [&[][..], &["."][..], &["src"][..]] {
            let temp = TempProject::new();
            temp.write("robusta.yaml", CONFIG);
            std::fs::create_dir(temp.dir.join("src")).unwrap();
            temp.write(BASELINE, &TWO_ALLOWED.cow_replace("A.java", "src/Deleted.java"));
            let mut args = vec![command, "--prune-suppressions"];
            args.extend(paths);
            let unmatched = robusta_in_dir(&args, &temp.dir);
            assert_eq!(unmatched.status.code(), Some(2), "{args:?}: {}", stderr(&unmatched));
            assert!(read(&baseline_path(&temp)).contains("src/Deleted.java"));

            args.push("--no-error-on-unmatched-pattern");
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
            assert_eq!(stdout_without_summary(&output), "", "{args:?}");
            assert!(run_summary(&output).contains(" on 0 files "), "{args:?}: {}", stdout(&output));
            assert_eq!(stderr(&output), "robusta-suppressions.json: updated\n");
            assert_eq!(read(&baseline_path(&temp)), "{}\n");

            let modified = std::fs::metadata(baseline_path(&temp)).unwrap().modified().unwrap();
            let again = robusta_in_dir(&args, &temp.dir);
            assert_eq!(again.status.code(), Some(0), "{args:?}: {}", stderr(&again));
            assert!(again.stderr.is_empty(), "{}", stderr(&again));
            assert_eq!(read(&baseline_path(&temp)), "{}\n");
            assert_eq!(
                std::fs::metadata(baseline_path(&temp)).unwrap().modified().unwrap(),
                modified
            );

            std::fs::remove_file(baseline_path(&temp)).unwrap();
            let missing = robusta_in_dir(&args, &temp.dir);
            assert_eq!(missing.status.code(), Some(0), "{args:?}: {}", stderr(&missing));
            assert!(!baseline_path(&temp).exists());
        }
    }
}

#[test]
fn empty_pruning_preserves_entries_outside_the_selected_directory() {
    for command in ["lint", "check"] {
        let temp = TempProject::new();
        temp.write("robusta.yaml", CONFIG);
        std::fs::create_dir(temp.dir.join("src")).unwrap();
        temp.write("Kept.java", TWO_ERRORS);
        let all = serde_json::json!({
            "src/Deleted.java": { "uppercase-long-suffix": { "count": 1 } },
            "Deleted.java": { "uppercase-long-suffix": { "count": 1 } },
            "Kept.java": { "uppercase-long-suffix": { "count": 2 } },
        });
        temp.write(BASELINE, &all.to_string());
        let output = robusta_in_dir(
            &[command, "src", "--prune-suppressions", "--no-error-on-unmatched-pattern"],
            &temp.dir,
        );
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        let mut expected = all;
        expected.as_object_mut().unwrap().remove("src/Deleted.java");
        let actual: serde_json::Value = serde_json::from_str(&read(&baseline_path(&temp))).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(read(&temp.dir.join("Kept.java")), TWO_ERRORS);
    }
}

#[test]
fn empty_pruning_obeys_cli_config_git_and_traversal_exclusions() {
    for command in ["lint", "check"] {
        for exclusion in [
            "pattern",
            "ignore-file",
            "config",
            "lint-config",
            "nested-config",
            "git",
            "git-root",
            "version-control",
        ] {
            let temp = TempProject::new();
            temp.write("robusta.yaml", CONFIG);
            let directory = if exclusion == "version-control" { ".svn" } else { "ignored" };
            std::fs::create_dir(temp.dir.join(directory)).unwrap();
            let key = format!("{directory}/Deleted.java");
            let all = serde_json::json!({
                "Deleted.java": { "uppercase-long-suffix": { "count": 1 } },
                &key: { "uppercase-long-suffix": { "count": 1 } },
            });
            temp.write(BASELINE, &all.to_string());
            let mut args = vec![command, "--prune-suppressions", "--no-error-on-unmatched-pattern"];
            match exclusion {
                "pattern" => args.extend(["--ignore-pattern", "ignored/"]),
                "ignore-file" => {
                    temp.write("exclude", "ignored/\n");
                    args.extend(["--ignore-path", "exclude"]);
                }
                "config" => {
                    temp.write(
                        "robusta.yaml",
                        &format!("{CONFIG}ignore-patterns:\n  - ignored/\n"),
                    );
                }
                "lint-config" => {
                    temp.write(
                        "robusta.yaml",
                        &format!("{CONFIG}  ignore-patterns:\n    - ignored/\n"),
                    );
                }
                "nested-config" => {
                    temp.write("ignored/robusta.yaml", "ignore-patterns:\n  - '*.java'\n");
                }
                "git" | "git-root" => {
                    temp.write(".gitignore", "ignored/\n");
                    if exclusion == "git-root" {
                        args.push("ignored");
                    }
                }
                "version-control" => {}
                _ => unreachable!(),
            }
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{args:?}/{exclusion}: {}", stderr(&output));
            let actual: serde_json::Value =
                serde_json::from_str(&read(&baseline_path(&temp))).unwrap();
            let expected = if exclusion == "git-root" {
                all
            } else {
                serde_json::json!({ &key: { "uppercase-long-suffix": { "count": 1 } } })
            };
            assert_eq!(actual, expected, "{command}/{exclusion}");
        }
    }
}

#[cfg(unix)]
#[test]
fn empty_pruning_preserves_untraversed_symlink_entries() {
    for command in ["lint", "check"] {
        let temp = TempProject::new();
        let outside = TempProject::new();
        temp.write("robusta.yaml", CONFIG);
        std::os::unix::fs::symlink(&outside.dir, temp.dir.join("linked")).unwrap();
        let original = TWO_ALLOWED.cow_replace("A.java", "linked/Deleted.java");
        temp.write(BASELINE, &original);
        let mut args = vec![command, "--prune-suppressions", "--no-error-on-unmatched-pattern"];
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        assert_eq!(read(&baseline_path(&temp)), original);

        args.push("linked");
        let explicit = robusta_in_dir(&args, &temp.dir);
        assert_eq!(explicit.status.code(), Some(0), "{command}: {}", stderr(&explicit));
        assert_eq!(read(&baseline_path(&temp)), "{}\n");
    }
}

#[test]
fn empty_pruning_does_not_write_after_a_scope_configuration_failure() {
    for command in ["lint", "check"] {
        let temp = TempProject::new();
        temp.write("robusta.yaml", CONFIG);
        temp.write("nested/robusta.yaml", "not-an-option: true\n");
        let original = serde_json::json!({
            "Deleted.java": { "uppercase-long-suffix": { "count": 1 } },
            "nested/Deleted.java": { "uppercase-long-suffix": { "count": 1 } },
        })
        .to_string();
        temp.write(BASELINE, &original);
        let output = robusta_in_dir(
            &[command, "--prune-suppressions", "--no-error-on-unmatched-pattern"],
            &temp.dir,
        );
        assert_eq!(output.status.code(), Some(2), "{command}: {}", stderr(&output));
        assert!(stderr(&output).contains("not-an-option"), "{}", stderr(&output));
        assert_eq!(read(&baseline_path(&temp)), original);
    }
}

#[test]
fn empty_runs_without_pruning_and_excluded_stdin_preserve_the_baseline() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", CONFIG);
    temp.write(BASELINE, TWO_ALLOWED);
    for command in ["lint", "check"] {
        let output = robusta_in_dir(&[command, "--no-error-on-unmatched-pattern"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        assert_eq!(stdout_without_summary(&output), "", "{command}");
        assert!(output.stderr.is_empty(), "{command}: {}", stderr(&output));
        assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);

        temp.write("robusta.yaml", &format!("{CONFIG}ignore-patterns:\n  - A.java\n"));
        let stdin = robusta_with_stdin(
            &[command, "--fix", "--stdin-filepath", "A.java"],
            &temp.dir,
            TWO_ERRORS,
        );
        assert_eq!(stdin.status.code(), Some(0), "{command}: {}", stderr(&stdin));
        assert_eq!(stdout(&stdin), TWO_ERRORS);
        assert_eq!(stderr_without_summary(&stdin), "", "{command}");
        assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
    }
    let suppress =
        robusta_in_dir(&["lint", "--suppress-all", "--no-error-on-unmatched-pattern"], &temp.dir);
    assert_eq!(suppress.status.code(), Some(0), "{}", stderr(&suppress));
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
}

#[test]
fn pruning_drops_deleted_files_only_within_the_selected_paths() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", CONFIG);
    temp.write("src/a/Kept.java", "class Kept { long a = 1l; }\n");
    temp.write("src/b/Other.java", "class Other {}\n");
    let entry = |key: &str| {
        format!(
            "  \"{key}\": {{\n    \"uppercase-long-suffix\": {{\n      \"count\": 1\n    }}\n  }}"
        )
    };
    let all = format!(
        "{{\n{},\n{},\n{},\n{}\n}}\n",
        entry("src/a/Deleted.java"),
        entry("src/a/Deleted.txt"),
        entry("src/a/Kept.java"),
        entry("src/b/Deleted.java")
    );
    temp.write(BASELINE, &all);

    let enforce = robusta_in_dir(&["lint", "src/a"], &temp.dir);
    let report = stdout_without_summary(&enforce);
    assert_eq!(enforce.status.code(), Some(1), "{report}");
    assert!(
        report.contains("src/a/Deleted.java: robusta-suppressions.json allows 1 error, but the file no longer exists"),
        "{report}"
    );
    assert!(!report.contains("src/b/Deleted.java"), "{report}");

    let prune = robusta_in_dir(&["lint", "--prune-suppressions", "src/a"], &temp.dir);
    assert_eq!(prune.status.code(), Some(0), "{}", stdout(&prune));
    assert_eq!(
        read(&baseline_path(&temp)),
        format!("{{\n{},\n{}\n}}\n", entry("src/a/Kept.java"), entry("src/b/Deleted.java"))
    );

    // An ignored subtree is not proof that its entries are obsolete.
    let empty = robusta_in_dir(
        &[
            "lint",
            "--prune-suppressions",
            "--no-error-on-unmatched-pattern",
            "--ignore-pattern",
            "**",
            "src/b",
        ],
        &temp.dir,
    );
    assert_eq!(empty.status.code(), Some(0), "{}", stdout(&empty));
    assert!(read(&baseline_path(&temp)).contains("src/b/Deleted.java"));
}

/// Only the current directory's baseline counts: one beside a nested config is
/// not discovered, and keys are relative to the current directory.
#[test]
fn the_baseline_is_read_only_from_the_current_directory() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write("sub/robusta.yaml", CONFIG);
    temp.write("sub/A.java", TWO_ERRORS);
    temp.write("sub/robusta-suppressions.json", TWO_ALLOWED);

    let unbaselined = robusta_in_dir(&["lint"], &temp.dir);
    assert_eq!(unbaselined.status.code(), Some(1), "{}", stdout(&unbaselined));

    let suppress = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    assert_eq!(suppress.status.code(), Some(0), "{}", stdout(&suppress));
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED.cow_replace("A.java", "sub/A.java"));
    assert_eq!(read(&temp.dir.join("sub/robusta-suppressions.json")), TWO_ALLOWED);
    assert_eq!(robusta_in_dir(&["lint"], &temp.dir).status.code(), Some(0));
}

#[test]
fn source_outside_the_current_directory_is_not_baselined() {
    let outside = TempProject::new();
    outside.write("robusta.yaml", CONFIG);
    let file = outside.write("A.java", TWO_ERRORS);
    let temp = TempProject::new();

    let output = robusta_in_dir(&["lint", "--suppress-all", file.to_str().unwrap()], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    assert!(stdout(&output).contains("uppercase-long-suffix"), "{}", stdout(&output));
    assert!(!baseline_path(&temp).exists());
}

#[test]
fn a_malformed_baseline_fails_before_any_source_is_written() {
    let temp = project(TWO_ERRORS);
    for (contents, expected) in [
        ("{", "EOF while parsing"),
        ("{\"../A.java\": {\"uppercase-long-suffix\": {\"count\": 1}}}", "not a relative path"),
        ("{\"A.java\": {\"no-such-rule\": {\"count\": 1}}}", "`no-such-rule` is not a rule ID"),
        ("{\"A.java\": {\"uppercase-long-suffix\": {\"count\": 0}}}", "at least 1"),
    ] {
        temp.write(BASELINE, contents);
        for command in ["check", "lint"] {
            let output = robusta_in_dir(&[command, "--fix"], &temp.dir);
            let stderr = stderr(&output);
            assert_eq!(output.status.code(), Some(2), "{contents}: {stderr}");
            assert!(stderr.starts_with("robusta-suppressions.json: "), "{contents}: {stderr}");
            assert!(stderr.contains(expected), "{contents}: {stderr}");
            assert_eq!(read(&temp.dir.join("A.java")), TWO_ERRORS);
            assert_eq!(read(&baseline_path(&temp)), contents);
        }
    }

    // `format` never reads the baseline.
    let format = robusta_in_dir(&["format"], &temp.dir);
    assert!(!stdout(&format).contains(BASELINE), "{}", stdout(&format));
    assert!(!stderr(&format).contains(BASELINE), "{}", stderr(&format));
}

/// A run that could not analyze every file leaves the baseline as it was.
#[test]
fn an_unparsable_file_prevents_every_baseline_update() {
    let temp = project(TWO_ERRORS);
    temp.write("Broken.java", "class Broken { int ; }\n");

    let not_updated = "robusta-suppressions.json: not updated, because the run did not complete\n";
    let suppress = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    assert_eq!(suppress.status.code(), Some(1), "{}", stdout(&suppress));
    assert_eq!(stderr(&suppress), not_updated);
    assert!(!baseline_path(&temp).exists());

    temp.write(BASELINE, &TWO_ALLOWED.cow_replace('2', "5"));
    let prune = robusta_in_dir(&["lint", "--prune-suppressions"], &temp.dir);
    let report = stdout_without_summary(&prune);
    assert_eq!(prune.status.code(), Some(1), "{report}");
    assert!(report.contains("allows 5 errors, but only 2 remain"), "{report}");
    assert_eq!(stderr(&prune), not_updated);
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED.cow_replace('2', "5"));

    for command in ["lint", "check"] {
        for flags in [&["--fix"][..], &["--fix", "--suppress-all"][..]] {
            if command == "check" && flags.contains(&"--suppress-all") {
                continue;
            }
            temp.write("A.java", TWO_ERRORS);
            temp.write(BASELINE, TWO_ALLOWED);
            let mut args = vec![command];
            args.extend(flags);
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(1), "{args:?}: {}", stdout(&output));
            assert_eq!(stderr(&output), not_updated);
            assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
        }
    }
}

/// Pruning that changes nothing leaves the file's bytes alone, even when they
/// are not in the canonical layout.
#[test]
fn a_no_op_update_does_not_rewrite_the_file() {
    let temp = project(TWO_ERRORS);
    let compact = "{\"A.java\":{\"uppercase-long-suffix\":{\"count\":2}}}";
    temp.write(BASELINE, compact);

    for args in [&["lint", "--prune-suppressions"][..], &["lint", "--suppress-all"][..]] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stdout(&output));
        assert_eq!(read(&baseline_path(&temp)), compact, "{args:?}");
    }
}

#[cfg(unix)]
#[test]
fn a_failed_write_is_a_tool_failure() {
    use std::os::unix::fs::PermissionsExt;

    let temp = project(TWO_ERRORS);
    let readonly = std::fs::Permissions::from_mode(0o555);
    std::fs::set_permissions(&temp.dir, readonly).unwrap();
    let probe = std::fs::write(temp.dir.join("probe"), "");
    let output = robusta_in_dir(&["lint", "--suppress-all"], &temp.dir);
    std::fs::set_permissions(&temp.dir, std::fs::Permissions::from_mode(0o755)).unwrap();
    if probe.is_ok() {
        // Running as a user that ignores directory permissions.
        return;
    }

    let stderr = stderr(&output);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("failed to write robusta-suppressions.json"), "{stderr}");
    assert!(!baseline_path(&temp).exists());
}

#[test]
fn baseline_flags_are_rejected_with_previews_and_each_other() {
    let temp = project(TWO_ERRORS);
    for args in [
        &["lint", "--suppress-all", "--prune-suppressions"][..],
        &["lint", "--suppress-all", "--diff"][..],
        &["lint", "--suppress-all", "--list-different"][..],
        &["lint", "--prune-suppressions", "--diff"][..],
        &["check", "--prune-suppressions", "--list-different"][..],
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(stderr(&output).starts_with("Error: `--"), "{args:?}: {}", stderr(&output));
        assert!(!baseline_path(&temp).exists(), "{args:?}");
    }
}

/// The source-editing suppression flags are gone; using one is an argument
/// error that changes nothing.
#[test]
fn removed_suppression_flags_are_argument_errors() {
    let temp = project(TWO_ERRORS);
    for args in [
        &["lint", "--suppress"][..],
        &["lint", "--suppress-all", "--reason", "legacy"][..],
        &["check", "--suppress-all"][..],
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(
            stderr(&output).contains("is not expected in this context"),
            "{args:?}: {}",
            stderr(&output)
        );
        assert_eq!(read(&temp.dir.join("A.java")), TWO_ERRORS);
        assert!(!baseline_path(&temp).exists(), "{args:?}");
    }
}

#[test]
fn suppress_all_records_only_the_java_files_of_a_mixed_selection() {
    let temp = project(TWO_ERRORS);
    temp.write("A.txt", TWO_ERRORS);

    let output = robusta_in_dir(&["lint", "--suppress-all", "A.txt", "A.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(read(&baseline_path(&temp)), TWO_ALLOWED);
}
