use crate::support::*;

#[test]
fn check_reports_enabled_lints_without_rewriting_a_formatted_file() {
    let source = "class Example {\n  long value = 10l;\n}\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["check"], &temp.file);
    let report = stdout_without_summary(&output);

    assert_eq!(output.status.code(), Some(0), "{report}");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(redact_dir(&report, &temp.dir), @"

     ! uppercase-long-suffix: use uppercase `L` for the long literal suffix
      ,-[<temp>/Example.java:2:18]
    1 | class Example {
    2 |   long value = 10l;
      :                  ^
    3 | }
      `----
     help: Replace the lowercase suffix with `L`.
    ");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn check_diff_prints_unified_diff_for_unformatted_file() {
    let temp = TempJava::new("class Example { int value ; }\n");

    let output = robusta_with_path(&["check", "--diff"], &temp.file);
    let stdout = stdout(&output);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stderr_without_summary(&output), "");
    insta::assert_snapshot!(redact_dir(&stdout, &temp.dir), @"
    --- <temp>/Example.java
    +++ <temp>/Example.java
    @@ -1,1 +1,3 @@
    -class Example { int value ; }
    +class Example {
    +  int value;
    +}
    ");
}

#[test]
fn path_preview_omits_unfixable_findings_that_make_no_source_change() {
    let source = "class Example {\n    void f() {\n        int stale = 1 + 2;\n    }\n}\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    unused-local-variable: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--list-different"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("unused-local-variable:"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn aggregate_preview_matches_fix_and_reports_remaining_findings_at_transformed_positions() {
    let source = "class Example { void f() { long value=10l; use(value); int stale=1+2; } }\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        concat!(
            "lint:\n  rules:\n",
            "    uppercase-long-suffix: warn\n",
            "    unused-local-variable: warn\n",
        ),
    )
    .expect("failed to write config");

    // The diff is the document, so the findings go to stderr.
    let preview = robusta_with_path(&["check", "--diff"], &temp.file);
    assert_eq!(preview.status.code(), Some(1), "{}", stderr(&preview));
    assert!(!stdout(&preview).contains("unused-local-variable"));
    assert!(stderr(&preview).contains(":5:9]"));
    assert!(stderr(&preview).contains("unused-local-variable:"));
    assert!(!stderr(&preview).contains("uppercase-long-suffix"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    let candidate = candidate_from_diff(&stdout(&preview));

    let write = robusta_with_path(&["check", "--fix"], &temp.file);
    assert_eq!(write.status.code(), Some(0), "{}", stdout(&write));
    assert!(stdout(&write).contains(":5:9]"));
    assert!(stdout(&write).contains("unused-local-variable:"));
    assert!(write.stderr.is_empty(), "{}", stderr(&write));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), candidate);
}

#[test]
fn aggregate_fix_honors_post_transform_warning_and_error_policy_without_rolling_back() {
    let source = "class Example { void f() { long value=10l; use(value); int stale=1+2; } }\n";
    for (severity, flags, expected_code) in [
        ("warn", &["--quiet"][..], 0),
        ("warn", &["--silent"][..], 0),
        ("warn", &["--deny-warnings"][..], 1),
        ("warn", &["--max-warnings", "0"][..], 1),
        ("deny", &[][..], 1),
    ] {
        let temp = TempJava::new(source);
        std::fs::write(
            temp.dir.join("robusta.yaml"),
            format!(
                "lint:\n  rules:\n    uppercase-long-suffix: warn\n    unused-local-variable: {severity}\n"
            ),
        )
        .expect("failed to write config");
        let mut args = vec!["check", "--fix"];
        args.extend_from_slice(flags);

        let output = robusta_with_path(&args, &temp.file);

        assert_eq!(output.status.code(), Some(expected_code), "{severity} {flags:?}");
        let written = std::fs::read_to_string(&temp.file).unwrap();
        assert!(written.contains("10L"), "{severity} {flags:?}: {written}");
        if flags.contains(&"--silent") {
            assert!(output.stderr.is_empty(), "{severity} {flags:?}");
        } else if flags.contains(&"--quiet") {
            assert!(
                stdout_without_summary(&output).is_empty(),
                "{severity} {flags:?}: {}",
                stdout(&output)
            );
        }
    }
}

#[test]
fn fix_tiers_and_output_modes_reject_ambiguous_combinations_before_writing() {
    let source = "class Example { long value = 10l; }\n";
    for args in [
        &["format", "--diff", "--fix"][..],
        &["lint", "--diff", "--fix"],
        &["check", "--list-different", "--fix"],
    ] {
        let temp = TempJava::new(source);
        let output = robusta_with_path(args, &temp.file);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    }
}

#[test]
fn preview_diff_preserves_no_final_newline_markers_without_writing() {
    let source = "class Example { int value ; }";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "--diff"], &temp.file);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("\\ No newline at end of file"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn check_diff_respects_color_control() {
    let temp = TempJava::new("class Example { int value ; }\n");

    let always = robusta_with_path(&["check", "--diff", "--color", "always"], &temp.file);
    let colored = stdout(&always);
    assert_eq!(always.status.code(), Some(1), "{}", stderr(&always));
    assert_eq!(stderr_without_summary(&always), "");
    assert!(colored.contains("\x1b[31m--- "), "{colored:?}");
    assert!(colored.contains("\x1b[32m+++ "), "{colored:?}");
    assert!(colored.contains("\x1b[36m@@ "), "{colored:?}");
    assert!(colored.contains("\x1b[31m-class Example"), "{colored:?}");
    assert!(colored.contains("\x1b[32m+class Example"), "{colored:?}");

    let never = robusta_with_path(&["check", "--diff", "--color", "never"], &temp.file);
    assert_eq!(never.status.code(), Some(1), "{}", stderr(&never));
    assert!(!stdout(&never).contains("\x1b["));
}

#[test]
fn check_list_different_lists_only_unformatted_paths() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("src/Clean.java", "class Clean {\n  static public int value;\n}\n");
    temp.write("src/Dirty.java", "class Dirty { int value; }\n");
    temp.write("src/nested/AlsoDirty.java", "class AlsoDirty { long value; }\n");

    let output = robusta_in_dir(&["check", "--list-different", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stderr_without_summary(&output), "");
    insta::assert_snapshot!(stdout(&output), @"
    src/Clean.java
    src/Dirty.java
    src/nested/AlsoDirty.java
    ");
}

#[test]
fn check_rejects_conflicting_human_output_modes() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example {}\n");

    let output =
        robusta_in_dir(&["check", "--diff", "--list-different", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    insta::assert_snapshot!(stderr(&output), @"Error: `--list-different` cannot be used at the same time as `--diff`");
}

#[test]
fn raw_fix_is_rejected_with_previews_with_or_without_unsafe_fixes() {
    let source = "class Example { long value = 10l; }\n";
    let temp = TempJava::new(source);
    let baseline = "invalid baseline must not be read\n";
    std::fs::write(temp.dir.join("robusta-suppressions.json"), baseline).unwrap();

    for command in ["check", "lint"] {
        for output_flag in ["--diff", "--list-different"] {
            for tiers in [&[][..], &["--unsafe-fixes"]] {
                for reverse in [false, true] {
                    let mut flags = vec!["--fix", output_flag];
                    flags.extend_from_slice(tiers);
                    if reverse {
                        flags.reverse();
                    }
                    let mut args = vec![command, "--reporter", "json"];
                    args.extend(flags);
                    args.push("Example.java");
                    let output = robusta_in_dir(&args, &temp.dir);

                    assert_eq!(output.status.code(), Some(2), "{args:?}");
                    assert!(output.stdout.is_empty(), "{args:?}");
                    assert_eq!(
                        stderr(&output),
                        format!(
                            "Error: `--fix` cannot be used at the same time as `{output_flag}`\n"
                        ),
                        "{args:?}",
                    );
                    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
                    assert_eq!(
                        std::fs::read_to_string(temp.dir.join("robusta-suppressions.json"))
                            .unwrap(),
                        baseline,
                    );
                }
            }
        }
    }
}

#[test]
fn unsafe_fix_previews_remain_read_only_and_include_safe_fixes() {
    let source = "class Example {\n  long value = 10l;\n}\n";
    let temp = TempJava::new(source);
    for command in ["check", "lint"] {
        for output_flag in ["--diff", "--list-different"] {
            for flags in [["--unsafe-fixes", output_flag], [output_flag, "--unsafe-fixes"]] {
                let output = robusta_with_path(
                    &[command, "-D", "uppercase-long-suffix", flags[0], flags[1]],
                    &temp.file,
                );
                assert_eq!(output.status.code(), Some(1), "{command} {flags:?}");
                assert!(!output.stdout.is_empty(), "{command} {flags:?}");
                assert!(!stderr(&output).contains("uppercase-long-suffix:"));
                assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
            }
        }
    }
}

#[test]
fn check_list_different_is_silent_for_a_clean_tree() {
    let temp = TempProject::new();
    temp.write("src/Clean.java", "class Clean {}\n");
    temp.write("src/nested/AlsoClean.java", "class AlsoClean {}\n");

    let output = robusta_in_dir(&["check", "--list-different", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(output.stdout.len(), 0);
    assert_eq!(stderr_without_summary(&output), "");
}

#[test]
fn check_diff_respects_option_terminator() {
    let temp = TempProject::new();
    temp.write("--diff/Example.java", "class Example { int value; }\n");

    let output = robusta_in_dir(&["check", "--", "--diff"], &temp.dir);

    assert_eq!(output.status.code(), Some(1));
    assert!(stdout(&output).contains("would reformat: --diff/Example.java"));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_fix_formats_file_in_place() {
    let temp = TempJava::new("class  Example { int value ; }\n");

    let output = robusta_with_path(&["check", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    class Example {
      int value;
    }
    ");
}

#[test]
fn check_fix_fixes_enabled_lints_then_formats_the_file() {
    let temp = TempJava::new("; static public class Example { long value=10l; } ;\n");
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    redundant-semicolons: warn\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["check", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    public static class Example {
      long value = 10L;
    }
    ");

    let check = robusta_with_path(&["check"], &temp.file);
    assert_eq!(check.status.code(), Some(0), "{}", stdout(&check));
    assert_eq!(stdout_without_summary(&check), "");
    assert!(check.stderr.is_empty(), "{}", stderr(&check));
}

/// `check` applies the selected fix kinds, then formats.
#[test]
fn check_applies_the_selected_fix_kinds_and_always_formats() {
    for flags in [&["--fix"][..], &["--fix", "--unsafe-fixes"][..]] {
        let temp = TempJava::new("class  Example { long value=10l; }\n");
        std::fs::write(
            temp.dir.join("robusta.yaml"),
            "lint:\n  rules:\n    uppercase-long-suffix: warn\n",
        )
        .expect("failed to write config");

        let mut args = vec!["check"];
        args.extend_from_slice(flags);
        let output = robusta_with_path(&args, &temp.file);

        assert_eq!(output.status.code(), Some(0), "{flags:?}: {}", stdout(&output));
        assert!(stdout_without_summary(&output).is_empty(), "{flags:?}: {}", stdout(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        assert_eq!(
            std::fs::read_to_string(&temp.file).unwrap(),
            "class Example {\n  long value = 10L;\n}\n",
            "{flags:?}"
        );
    }
}

#[test]
fn check_fix_keeps_source_suppressions_stable() {
    let source = r#"class Example {
@SuppressWarnings({"robusta:modifier-order", "robusta:uppercase-long-suffix"})
static public long first=10l;
long second=20l; // robusta-disable-line -- generated
}
"#;
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["check", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @r#"
    class Example {
      @SuppressWarnings({"robusta:modifier-order", "robusta:uppercase-long-suffix"})
      static public long first = 10l;
      long second = 20l; // robusta-disable-line -- generated
    }
    "#);

    let check = robusta_with_path(&["check"], &temp.file);
    assert_eq!(check.status.code(), Some(0), "{}", stdout(&check));
    assert_eq!(stdout_without_summary(&check), "");
    assert!(check.stderr.is_empty(), "{}", stderr(&check));
}

#[test]
fn check_uses_nearest_config_for_indent_width() {
    let temp = TempJava::new("class Example {\n  int value;\n}\n");
    std::fs::write(temp.dir.join("robusta.yaml"), "format:\n  indent-width: 2\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["check"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_removes_unused_suppressions_before_formatting() {
    let temp = TempJava::stale_suppression("  options:\n    report-unused-suppressions: warn\n");

    let output = robusta_in_dir(&["check", "--fix"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), "class Example {\n  int clean;\n}\n");
}

#[test]
fn check_audits_unused_suppressions_like_lint() {
    let temp = TempJava::stale_suppression("");

    let plain = robusta_in_dir(&["check", "--report-unused-suppressions"], &temp.dir);
    assert_eq!(plain.status.code(), Some(0), "{}", stdout(&plain));
    assert!(stdout(&plain).contains("! unused-suppression"), "{}", stdout(&plain));

    let json = robusta_in_dir(
        &["check", "--report-unused-suppressions-severity", "deny", "--reporter", "json"],
        &temp.dir,
    );
    assert_eq!(json.status.code(), Some(1), "{}", stdout(&json));
    let report = stdout(&json);
    assert!(report.contains(r#""ruleId": "unused-suppression""#), "{report}");
    assert!(report.contains(r#""severity": "error""#), "{report}");

    // A configured audit applies to check as it does to lint, and a fix run
    // reports a suppression that its fix keeps against the fixed candidate.
    let temp = TempJava::stale_suppression("  options:\n    report-unused-suppressions: warn\n");
    let configured = robusta_in_dir(&["check"], &temp.dir);
    assert_eq!(configured.status.code(), Some(0), "{}", stdout(&configured));
    assert!(stdout(&configured).contains("! unused-suppression"), "{}", stdout(&configured));
    let documented = "class Example {\n  @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n  /** Clean. */\n  int clean;\n}\n";
    std::fs::write(&temp.file, documented).unwrap();
    let fixed = robusta_in_dir(&["check", "--fix"], &temp.dir);
    assert_eq!(fixed.status.code(), Some(0), "{}", stdout(&fixed));
    assert!(stdout(&fixed).contains("! unused-suppression"), "{}", stdout(&fixed));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), documented);
    let off =
        robusta_in_dir(&["check", "--report-unused-suppressions-severity", "allow"], &temp.dir);
    assert!(stdout_without_summary(&off).is_empty(), "{}", stdout(&off));
    assert!(off.stderr.is_empty(), "{}", stderr(&off));
}
