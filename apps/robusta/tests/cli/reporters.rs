use cow_utils::CowUtils;

use crate::support::*;

#[test]
fn windows_paths_are_normalized_and_redacted_without_changing_source_escapes() {
    let path = std::path::Path::new(r"C:\temp\Example.java");
    assert_eq!(presentation_path(path), "C:/temp/Example.java");
    // spellchecker:off
    assert_eq!(
        redact_dir(r"C:/temp/Example.java: class Caf\u00e9 {}", std::path::Path::new(r"C:\temp")),
        r"<temp>/Example.java: class Caf\u00e9 {}"
    );
    // spellchecker:on
}

#[test]
fn warning_controls_set_output_and_exit_policy_for_lint_bearing_commands() {
    let warnings = TempProject::new();
    warnings.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    warnings.write("Example.java", "class Example {\n  static public int value;\n}\n");

    for command in ["lint", "check"] {
        let default = robusta_in_dir(&[command, "Example.java"], &warnings.dir);
        assert_eq!(default.status.code(), Some(0), "{command}: {}", stdout(&default));
        assert!(stdout(&default).contains("modifier-order:"), "{command}");
        assert!(default.stderr.is_empty(), "{command}: {}", stderr(&default));

        let denied = robusta_in_dir(&[command, "--deny-warnings", "Example.java"], &warnings.dir);
        assert_eq!(denied.status.code(), Some(1), "{command}: {}", stdout(&denied));

        let threshold =
            robusta_in_dir(&[command, "--max-warnings", "0", "Example.java"], &warnings.dir);
        assert_eq!(threshold.status.code(), Some(1), "{command}: {}", stdout(&threshold));
        // The verdict explains the exit status, so it stays off the report's stream.
        assert_eq!(
            stderr(&threshold),
            "Exceeded maximum number of warnings. Found 1.\n",
            "{command}"
        );

        let within_budget =
            robusta_in_dir(&[command, "--max-warnings", "1", "Example.java"], &warnings.dir);
        assert_eq!(within_budget.status.code(), Some(0), "{command}");

        let quiet_denied =
            robusta_in_dir(&[command, "--quiet", "--deny-warnings", "Example.java"], &warnings.dir);
        assert_eq!(quiet_denied.status.code(), Some(1), "{command}");
        assert!(
            stdout_without_summary(&quiet_denied).is_empty(),
            "{command}: {}",
            stdout(&quiet_denied)
        );
        assert!(quiet_denied.stderr.is_empty(), "{command}: {}", stderr(&quiet_denied));

        let silent_threshold = robusta_in_dir(
            &[command, "--silent", "--max-warnings", "0", "Example.java"],
            &warnings.dir,
        );
        assert_eq!(silent_threshold.status.code(), Some(1), "{command}");
        assert!(silent_threshold.stdout.is_empty(), "{command}");
        assert!(silent_threshold.stderr.is_empty(), "{command}");
    }

    let mixed = TempProject::new();
    mixed.write(
        "robusta.yaml",
        "lint:\n  rules:\n    modifier-order: warn\n    uppercase-long-suffix: deny\n",
    );
    mixed.write("Example.java", "class Example {\n  static public long value = 10l;\n}\n");

    for command in ["lint", "check"] {
        let quiet = robusta_in_dir(&[command, "--quiet", "Example.java"], &mixed.dir);
        assert_eq!(quiet.status.code(), Some(1), "{command}: {}", stdout(&quiet));
        assert!(!stdout(&quiet).contains("modifier-order:"), "{command}");
        assert!(stdout(&quiet).contains("uppercase-long-suffix:"), "{command}");
        assert!(quiet.stderr.is_empty(), "{command}: {}", stderr(&quiet));

        let silent = robusta_in_dir(&[command, "--silent", "Example.java"], &mixed.dir);
        assert_eq!(silent.status.code(), Some(1), "{command}");
        assert!(silent.stdout.is_empty(), "{command}");
        assert!(silent.stderr.is_empty(), "{command}");
    }
}

#[test]
fn configured_warning_limits_combine_with_the_flags() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example {\n  static public int value;\n}\n");
    let rules = "  rules:\n    modifier-order: warn\n";

    for (option, flags, code, exceeded) in [
        ("deny-warnings: true", &[][..], 1, false),
        ("deny-warnings: false", &[][..], 0, false),
        ("deny-warnings: false", &["--deny-warnings"][..], 1, false),
        ("deny-warnings: true", &["--max-warnings", "1"][..], 1, false),
        ("max-warnings: 0", &[][..], 1, true),
        ("max-warnings: 1", &[][..], 0, false),
        ("max-warnings: 0", &["--max-warnings", "1"][..], 0, false),
        ("max-warnings: 1", &["--max-warnings", "0"][..], 1, true),
        ("max-warnings: 1", &["--deny-warnings"][..], 1, false),
    ] {
        temp.write("robusta.yaml", &format!("lint:\n  options:\n    {option}\n{rules}"));
        for command in ["lint", "check"] {
            let mut args = vec![command];
            args.extend_from_slice(flags);
            args.push("Example.java");
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(code), "{option} {args:?}: {}", stderr(&output));
            let verdict =
                if exceeded { "Exceeded maximum number of warnings. Found 1.\n" } else { "" };
            assert_eq!(stderr(&output), verdict, "{option} {args:?}");
        }
        if flags.is_empty() {
            let stdin = robusta_with_stdin(
                &["lint", "--stdin-filepath", "Virtual.java"],
                &temp.dir,
                "class Virtual {\n  static public int value;\n}\n",
            );
            assert_eq!(stdin.status.code(), Some(code), "{option}: {}", stderr(&stdin));
        }
    }

    // `format` reports no warnings, so the limits leave a formatted file passing.
    temp.write("robusta.yaml", "lint:\n  options:\n    deny-warnings: true\n    max-warnings: 0\n");
    temp.write("Example.java", "class Example {}\n");
    let format = robusta_in_dir(&["format", "Example.java"], &temp.dir);
    assert_eq!(format.status.code(), Some(0), "{}", stderr(&format));
}

#[test]
fn warning_limits_are_main_only_for_every_command() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  options:\n    max-warnings: 0\n  rules:\n    modifier-order: warn\n",
    );
    temp.write("nested/Nested.java", "class Nested {\n  static public int value;\n}\n");

    for (option, name) in [
        ("deny-warnings: true", "lint.options.deny-warnings"),
        ("max-warnings: 0", "lint.options.max-warnings"),
    ] {
        temp.write(
            "nested/robusta.yaml",
            &format!("lint:\n  options:\n    {option}\n  rules:\n    modifier-order: warn\n"),
        );
        for command in ["lint", "check", "format"] {
            let output = robusta_in_dir(&[command, "."], &temp.dir);
            assert_eq!(output.status.code(), Some(2), "{command} {option}: {}", stderr(&output));
            assert!(output.stdout.is_empty(), "{command} {option}");
            let message = stderr(&output);
            assert!(
                message.contains("nested")
                    && message.contains(&format!(
                        "robusta.yaml: {name} is only supported in the main configuration; move the option to the main configuration"
                    )),
                "{command} {option}: {message}"
            );
        }
        // Only the main configuration is read, so its limit applies to the nested file.
        let single = robusta_in_dir(&["lint", "--no-nested-config", "."], &temp.dir);
        assert_eq!(single.status.code(), Some(1), "{option}: {}", stderr(&single));
        assert!(stderr(&single).contains("Exceeded maximum number of warnings. Found 1."));
        // An explicit `--config` is the main configuration, so its option applies.
        let explicit = robusta_in_dir(&["lint", "--config", "nested/robusta.yaml", "."], &temp.dir);
        assert_eq!(explicit.status.code(), Some(1), "{option}: {}", stderr(&explicit));
        // Run from inside, the nested configuration is the main one.
        let inside = robusta_in_dir(&["lint", "."], &temp.dir.join("nested"));
        assert_eq!(inside.status.code(), Some(1), "{option}: {}", stderr(&inside));
    }
}

#[test]
fn silent_hides_parse_diagnostics_without_hiding_failure() {
    let temp = TempProject::new();
    temp.write("Broken.java", "class Broken { int ; }\n");

    for command in ["lint", "check"] {
        let output = robusta_in_dir(&[command, "--silent", "Broken.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(1), "{command}");
        assert!(output.stdout.is_empty(), "{command}");
        assert!(output.stderr.is_empty(), "{command}");
    }
}

#[test]
fn machine_reporters_filter_output_without_changing_warning_counts() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    modifier-order: warn\n    uppercase-long-suffix: deny\n",
    );
    temp.write("Example.java", "class Example {\n  static public long value = 10l;\n}\n");

    for command in ["lint", "check"] {
        let quiet =
            robusta_in_dir(&[command, "--quiet", "--reporter", "json", "Example.java"], &temp.dir);
        assert_eq!(quiet.status.code(), Some(1), "{command}: {}", stderr(&quiet));
        assert!(quiet.stderr.is_empty(), "{command}");
        assert!(!stdout(&quiet).contains("modifier-order"), "{command}");
        assert!(stdout(&quiet).contains("uppercase-long-suffix"), "{command}");
        assert!(stdout(&quiet).contains("\"diagnostics\": 1"), "{command}");

        let counted = robusta_in_dir(
            &[command, "--quiet", "--max-warnings", "0", "--reporter", "json", "Example.java"],
            &temp.dir,
        );
        assert_eq!(counted.status.code(), Some(1), "{command}");
        assert!(!stdout(&counted).contains("modifier-order"), "{command}");

        let silent =
            robusta_in_dir(&[command, "--silent", "--reporter", "json", "Example.java"], &temp.dir);
        assert_eq!(silent.status.code(), Some(1), "{command}");
        assert!(silent.stdout.is_empty(), "{command}");
        assert!(silent.stderr.is_empty(), "{command}");
    }
}

#[test]
fn default_reporter_is_explicit_and_human_alias_is_rejected() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Example.java", "class Example { long value = 10l; }\n");

    let output = robusta_in_dir(&["lint", "--reporter", "default", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(stdout_without_summary(&output), @"

     x uppercase-long-suffix: use uppercase `L` for the long literal suffix
      ,-[Example.java:1:32]
    1 | class Example { long value = 10l; }
      :                                ^
      `----
     help: Replace the lowercase suffix with `L`.
    ");

    let alias = robusta_in_dir(&["lint", "--reporter", "human", "Example.java"], &temp.dir);
    assert_eq!(alias.status.code(), Some(2));
    assert_eq!(alias.stdout.len(), 0);
    assert!(stderr(&alias).cow_replace('\n', " ").contains(
        "unknown reporter `human` (expected default, json, checkstyle, github, gitlab, azure-devops, unix, agent, sarif, stylish, or junit)"
    ));
}

#[test]
fn default_reporter_displays_source_controls_without_terminal_effects() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    unused-import: deny\n");
    temp.write(
        "Example.java",
        "// \u{1b}[2J\u{1b}]0;title\u{7}\u{8}\0\u{7f}\u{85}\nimport java.util.Li\u{1b}cst;\nclass Example {}\n",
    );

    for color in ["never", "always"] {
        let output = robusta_in_dir(&["lint", "--color", color, "Example.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        let report = stdout_without_summary(&output);
        assert!(report.contains("␛[2J␛]0;title␇␈␀␡<U+0085>"), "{report:?}");
        assert!(report.contains("imported name `Licst`"), "{report:?}");
        assert!(report.contains("import java.util.Li␛cst;"), "{report:?}");
        assert!(!report.chars().any(|c| c.is_control() && !matches!(c, '\n' | '\u{1b}')));
        assert!(!report.contains("\u{1b}[2J") && !report.contains("\u{1b}]"));
        if color == "never" {
            assert!(!report.contains('\u{1b}'));
            assert!(report.contains("^^^^^^"), "{report:?}");
        } else {
            assert!(report.contains("\u{1b}["));
        }
    }
}

#[test]
fn default_reporter_color_control_preserves_plain_output() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    modifier-order: warn\n    uppercase-long-suffix: deny\n",
    );
    temp.write("Example.java", "class Example { static public long value = 10l; }\n");

    let always = robusta_in_dir(
        &["lint", "--color", "always", "--reporter", "default", "Example.java"],
        &temp.dir,
    );
    let colored = stdout(&always);
    assert_eq!(always.status.code(), Some(1), "{colored}");
    assert!(always.stderr.is_empty(), "{}", stderr(&always));
    assert!(colored.contains("\x1b["), "{colored:?}");
    assert!(colored.contains("modifier-order:"), "{colored:?}");
    assert!(colored.contains("uppercase-long-suffix:"), "{colored:?}");
    assert!(colored.contains("Example.java"), "{colored:?}");

    let never = robusta_in_dir(
        &["lint", "--color", "never", "--reporter", "default", "Example.java"],
        &temp.dir,
    );
    assert_eq!(never.status.code(), Some(1), "{}", stdout(&never));
    assert!(!stdout(&never).contains("\x1b["));
    insta::assert_snapshot!(stdout_without_summary(&never), @"

     ! modifier-order: modifiers are not in the canonical order
      ,-[Example.java:1:17]
    1 | class Example { static public long value = 10l; }
      :                 ^^^^^^|^^^^^^||
      :                       |      |`-- `public` should appear earlier
      :                       |      `-- `static` currently appears first
      :                       `-- reorder this modifier run
      `----
     help: Reorder the modifiers into Java's customary sequence.

     x uppercase-long-suffix: use uppercase `L` for the long literal suffix
      ,-[Example.java:1:46]
    1 | class Example { static public long value = 10l; }
      :                                              ^
      `----
     help: Replace the lowercase suffix with `L`.
    ");

    let no_color = robusta_in_dir_with_env(
        &["lint", "--color", "auto", "--reporter", "default", "Example.java"],
        &temp.dir,
        "NO_COLOR",
        "1",
    );
    assert_eq!(no_color.status.code(), Some(1), "{}", stdout(&no_color));
    assert_eq!(stdout_without_summary(&no_color), stdout_without_summary(&never));
}

#[test]
fn machine_reporters_are_never_colored_and_the_human_document_obeys_the_policy() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Example.java", "class Example { long value = 10l; }\n");

    for reporter in [
        "json",
        "checkstyle",
        "github",
        "gitlab",
        "azure-devops",
        "unix",
        "agent",
        "sarif",
        "junit",
    ] {
        let output = robusta_in_dir(
            &["lint", "--color", "always", "--reporter", reporter, "Example.java"],
            &temp.dir,
        );

        assert_eq!(output.status.code(), Some(1), "{reporter}: {}", stderr(&output));
        assert!(output.stderr.is_empty(), "{reporter}");
        assert!(!stdout(&output).contains("\x1b["), "{reporter}: {}", stdout(&output));
    }

    // `stylish` is written for a person, so it colors on request and stays
    // plain otherwise, exactly like the streaming default reporter.
    let colored = robusta_in_dir(
        &["lint", "--color", "always", "--reporter", "stylish", "Example.java"],
        &temp.dir,
    );
    assert_eq!(colored.status.code(), Some(1), "{}", stderr(&colored));
    assert_eq!(colored.stderr.len(), 0);
    assert!(stdout(&colored).contains("\x1b["), "{}", stdout(&colored));

    let plain = robusta_in_dir(
        &["lint", "--color", "never", "--reporter", "stylish", "Example.java"],
        &temp.dir,
    );
    assert_eq!(plain.status.code(), Some(1), "{}", stderr(&plain));
    assert!(!stdout(&plain).contains("\x1b["), "{}", stdout(&plain));
}

#[test]
fn stylish_reporter_covers_check_format_lint_filtering_and_clean_runs() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example { static public long value = 10l; }\n");

    let check = robusta_in_dir(&["check", "--reporter", "stylish", "Example.java"], &temp.dir);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    assert_eq!(check.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&check), @r###"

    Example.java
      1:1   warning  file is not formatted; run `robusta format --fix`  format
      1:17  warning  modifiers are not in the canonical order  modifier-order
      1:46  error  use uppercase `L` for the long literal suffix  uppercase-long-suffix

    ✖ 3 problems (1 error, 2 warnings)
    "###);

    let format = robusta_in_dir(&["format", "--reporter", "stylish", "Example.java"], &temp.dir);
    assert_eq!(format.status.code(), Some(1), "{}", stderr(&format));
    insta::assert_snapshot!(stdout(&format), @r###"

    Example.java
      1:1  warning  file is not formatted; run `robusta format --fix`  format

    ✖ 1 problem (0 errors, 1 warning)
    "###);

    // `--quiet` shrinks the block and the count it closes with together.
    let quiet =
        robusta_in_dir(&["lint", "--quiet", "--reporter", "stylish", "Example.java"], &temp.dir);
    assert_eq!(quiet.status.code(), Some(1), "{}", stderr(&quiet));
    insta::assert_snapshot!(stdout(&quiet), @r###"

    Example.java
      1:46  error  use uppercase `L` for the long literal suffix  uppercase-long-suffix

    ✖ 1 problem (1 error, 0 warnings)
    "###);

    let silent =
        robusta_in_dir(&["lint", "--silent", "--reporter", "stylish", "Example.java"], &temp.dir);
    assert_eq!(silent.status.code(), Some(1), "{}", stderr(&silent));
    assert_eq!(silent.stdout.len(), 0);
    assert_eq!(silent.stderr.len(), 0);

    temp.write("Example.java", "class Example {}\n");
    let clean = robusta_in_dir(&["check", "--reporter", "stylish", "Example.java"], &temp.dir);
    assert_eq!(clean.status.code(), Some(0), "{}", stderr(&clean));
    assert_eq!(clean.stdout.len(), 0);
    assert_eq!(clean.stderr.len(), 0);
}

#[test]
fn junit_reporter_reports_files_as_suites_across_check_format_and_lint() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("First.java", "class First { static public long value = 10l; }\n");
    temp.write("Second.java", "class Second {\n  long other = 20L;\n}\n");

    let lint = robusta_in_dir(&["lint", "--reporter", "junit", "."], &temp.dir);
    assert_eq!(lint.status.code(), Some(1), "{}", stderr(&lint));
    assert_eq!(lint.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&lint), @r###"
    <?xml version="1.0" encoding="UTF-8"?>
    <testsuites name="Robusta" tests="2" failures="1" errors="1">
        <testsuite name="First.java" tests="2" disabled="0" errors="1" failures="1">
            <testcase name="modifier-order">
                <failure message="modifiers are not in the canonical order">line 1, column 15, modifiers are not in the canonical order</failure>
            </testcase>
            <testcase name="uppercase-long-suffix">
                <error message="use uppercase `L` for the long literal suffix">line 1, column 44, use uppercase `L` for the long literal suffix</error>
            </testcase>
        </testsuite>
    </testsuites>
    "###);

    // A formatting-only scope reports the synthetic `format` finding, and a
    // file with nothing to report is not a suite.
    let format = robusta_in_dir(&["format", "--reporter", "junit", "."], &temp.dir);
    assert_eq!(format.status.code(), Some(1), "{}", stderr(&format));
    insta::assert_snapshot!(stdout(&format), @r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <testsuites name="Robusta" tests="1" failures="1" errors="0">
        <testsuite name="First.java" tests="1" disabled="0" errors="0" failures="1">
            <testcase name="format">
                <failure message="file is not formatted; run `robusta format --fix`">line 1, column 1, file is not formatted; run `robusta format --fix`</failure>
            </testcase>
        </testsuite>
    </testsuites>
    "#);

    let check = robusta_in_dir(&["check", "--reporter", "junit", "First.java"], &temp.dir);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    let report = stdout(&check);
    assert!(report.contains(r#"tests="3" failures="2" errors="1""#), "{report}");
    assert!(report.contains(r#"<testcase name="format">"#), "{report}");

    let silent = robusta_in_dir(&["lint", "--silent", "--reporter", "junit", "."], &temp.dir);
    assert_eq!(silent.status.code(), Some(1), "{}", stderr(&silent));
    assert_eq!(silent.stdout.len(), 0);
    assert_eq!(silent.stderr.len(), 0);

    temp.write("First.java", "class First {\n  long value = 10L;\n}\n");
    let clean = robusta_in_dir(&["check", "--reporter", "junit", "."], &temp.dir);
    assert_eq!(clean.status.code(), Some(0), "{}", stderr(&clean));
    insta::assert_snapshot!(stdout(&clean), @r###"
    <?xml version="1.0" encoding="UTF-8"?>
    <testsuites name="Robusta" tests="0" failures="0" errors="0">

    </testsuites>
    "###);
}

#[test]
fn check_json_reporter_combines_formatting_and_lint_findings() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example { static public long value = 10l; }\n");

    let output = robusta_in_dir(&["check", "--reporter", "json", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @r#"
    {
      "files": [
        {
          "path": "Example.java",
          "formatted": false,
          "diagnostics": [
            {
              "ruleId": "modifier-order",
              "severity": "warning",
              "message": "modifiers are not in the canonical order",
              "line": 1,
              "column": 17
            },
            {
              "ruleId": "uppercase-long-suffix",
              "severity": "error",
              "message": "use uppercase `L` for the long literal suffix",
              "line": 1,
              "column": 46
            }
          ]
        }
      ],
      "summary": {
        "diagnostics": 2,
        "unformatted": 1
      }
    }
    "#);
}

#[test]
fn lint_checkstyle_reporter_emits_rule_ids_severities_and_positions() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example {\n    static public long value = 10l;\n}\n");

    let output = robusta_in_dir(&["lint", "--reporter", "checkstyle", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @r#"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"><file name="Example.java"><error line="2" column="5" severity="warning" message="modifiers are not in the canonical order" source="modifier-order" /><error line="2" column="34" severity="error" message="use uppercase `L` for the long literal suffix" source="uppercase-long-suffix" /></file></checkstyle>"#);
}

#[test]
fn xml_reporters_stay_well_formed_when_a_path_holds_a_control_character() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    unused-import: deny\n");

    for reporter in ["checkstyle", "junit"] {
        let output = robusta_with_stdin(
            &["lint", "--reporter", reporter, "--stdin-filepath", "sr\u{1b}c/Example.java"],
            &temp.dir,
            "import java.util.List;\nclass Example {}\n",
        );
        let document = stdout(&output);

        assert_eq!(output.status.code(), Some(1), "{reporter}: {}", stderr(&output));
        assert!(document.contains(r#"name="sr\u001Bc/Example.java""#), "{reporter}: {document}");
        assert!(!document.chars().any(|c| c < ' ' && c != '\n'), "{reporter}: {document:?}");
    }
}

#[test]
fn lint_github_reporter_emits_workflow_annotations() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example {\n    static public long value = 10l;\n}\n");

    let output = robusta_in_dir(&["lint", "--reporter", "github", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @r#"
    ::warning file=Example.java,line=2,endLine=2,col=5,endColumn=18,title=modifier-order::Example.java:2:5: modifiers are not in the canonical order
    ::error file=Example.java,line=2,endLine=2,col=34,endColumn=35,title=uppercase-long-suffix::Example.java:2:34: use uppercase `L` for the long literal suffix
    "#);
}

#[test]
fn lint_azure_devops_reporter_emits_native_annotations() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example {\n    static public long value = 10l;\n}\n");

    let output = robusta_in_dir(&["lint", "--reporter", "azure-devops", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @r#"
    ##vso[task.logissue type=warning;sourcepath=Example.java;linenumber=2;columnnumber=5;code=modifier-order;]modifiers are not in the canonical order
    ##vso[task.logissue type=error;sourcepath=Example.java;linenumber=2;columnnumber=34;code=uppercase-long-suffix;]use uppercase `L` for the long literal suffix
    "#);
    assert!(!stdout(&output).contains("task.complete"));
}

#[test]
fn gitlab_and_unix_reporters_cover_check_format_and_lint() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example {\n static public long value=10l;\n}\n");

    for reporter in ["gitlab", "unix"] {
        for command in ["check", "format", "lint"] {
            let output =
                robusta_in_dir(&[command, "--reporter", reporter, "Example.java"], &temp.dir);
            let report = stdout(&output);

            assert_eq!(output.status.code(), Some(1), "{command}/{reporter}: {}", stderr(&output));
            assert!(output.stderr.is_empty(), "{command}/{reporter}: {}", stderr(&output));
            assert!(report.contains("Example.java"), "{command}/{reporter}: {report}");

            let checks_formatting = command != "lint";
            let checks_lints = command != "format";
            assert_eq!(
                report.contains(if reporter == "gitlab" {
                    "\"check_name\": \"format\""
                } else {
                    "[Warning/format]"
                }),
                checks_formatting,
                "{command}/{reporter}: {report}"
            );
            assert_eq!(
                report.contains("uppercase-long-suffix"),
                checks_lints,
                "{command}/{reporter}: {report}"
            );
            assert_eq!(
                report.contains("modifier-order"),
                checks_lints,
                "{command}/{reporter}: {report}"
            );

            if reporter == "gitlab" {
                assert!(report.contains("\"fingerprint\":"), "{command}: {report}");
                if checks_lints {
                    assert!(report.contains("\"severity\": \"critical\""), "{command}: {report}");
                    assert!(report.contains("\"severity\": \"major\""), "{command}: {report}");
                }
            } else {
                let total = usize::from(checks_formatting) + 2 * usize::from(checks_lints);
                let suffix = if total == 1 { "problem" } else { "problems" };
                assert!(report.ends_with(&format!("\n{total} {suffix}\n")), "{command}: {report}");
                if checks_lints {
                    assert!(
                        report.contains("[Error/uppercase-long-suffix]"),
                        "{command}: {report}"
                    );
                    assert!(report.contains("[Warning/modifier-order]"), "{command}: {report}");
                }
            }
        }
    }
}

#[test]
fn agent_reporter_covers_check_format_lint_filtering_and_clean_runs() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example { static public long value = 10l; }\n");

    let check = robusta_in_dir(&["check", "--reporter", "agent", "Example.java"], &temp.dir);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    assert_eq!(check.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&check), @"
    Example.java:1:1: warning format: file is not formatted; run `robusta format --fix`
    Example.java:1:17: warning modifier-order: modifiers are not in the canonical order help: Reorder the modifiers into Java's customary sequence.
    Example.java:1:46: error uppercase-long-suffix: use uppercase `L` for the long literal suffix help: Replace the lowercase suffix with `L`.
    ");

    let format = robusta_in_dir(&["format", "--reporter", "agent", "Example.java"], &temp.dir);
    assert_eq!(format.status.code(), Some(1), "{}", stderr(&format));
    assert_eq!(format.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&format), @"Example.java:1:1: warning format: file is not formatted; run `robusta format --fix`");

    let lint = robusta_in_dir(&["lint", "--reporter", "agent", "Example.java"], &temp.dir);
    assert_eq!(lint.status.code(), Some(1), "{}", stderr(&lint));
    assert_eq!(lint.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&lint), @"
    Example.java:1:17: warning modifier-order: modifiers are not in the canonical order help: Reorder the modifiers into Java's customary sequence.
    Example.java:1:46: error uppercase-long-suffix: use uppercase `L` for the long literal suffix help: Replace the lowercase suffix with `L`.
    ");

    let quiet =
        robusta_in_dir(&["lint", "--quiet", "--reporter", "agent", "Example.java"], &temp.dir);
    assert_eq!(quiet.status.code(), Some(1), "{}", stderr(&quiet));
    assert!(!stdout(&quiet).contains("modifier-order"));
    assert!(stdout(&quiet).contains("uppercase-long-suffix"));

    temp.write("Example.java", "class Example {}\n");
    let clean = robusta_in_dir(&["check", "--reporter", "agent", "Example.java"], &temp.dir);
    assert_eq!(clean.status.code(), Some(0), "{}", stderr(&clean));
    assert_eq!(clean.stdout.len(), 0);
    assert_eq!(clean.stderr.len(), 0);
}

#[test]
fn azure_devops_reporter_respects_warning_filters_and_policy_counts() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("Example.java", "class Example { static public int value; }\n");

    let quiet = robusta_in_dir(
        &["lint", "--quiet", "--reporter", "azure-devops", "Example.java"],
        &temp.dir,
    );
    assert_eq!(quiet.status.code(), Some(0), "{}", stderr(&quiet));
    assert_eq!(quiet.stdout.len(), 0);
    assert_eq!(quiet.stderr.len(), 0);

    let counted = robusta_in_dir(
        &["lint", "--quiet", "--max-warnings", "0", "--reporter", "azure-devops", "Example.java"],
        &temp.dir,
    );
    assert_eq!(counted.status.code(), Some(1), "{}", stderr(&counted));
    assert_eq!(counted.stdout.len(), 0);

    let silent = robusta_in_dir(
        &["lint", "--silent", "--reporter", "azure-devops", "Example.java"],
        &temp.dir,
    );
    assert_eq!(silent.status.code(), Some(0), "{}", stderr(&silent));
    assert_eq!(silent.stdout.len(), 0);
    assert_eq!(silent.stderr.len(), 0);
}

#[test]
fn check_github_reporter_exposes_formatting_as_an_annotation() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example { int value; }\n");

    let output = robusta_in_dir(&["check", "--reporter", "github", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @"::warning file=Example.java,line=1,endLine=1,col=1,endColumn=1,title=format::Example.java:1:1: file is not formatted; run `robusta format --fix`");
}

#[test]
fn check_azure_devops_reporter_exposes_formatting_as_an_annotation() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example { int value; }\n");

    let output =
        robusta_in_dir(&["check", "--reporter", "azure-devops", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @"##vso[task.logissue type=warning;sourcepath=Example.java;linenumber=1;columnnumber=1;code=format;]file is not formatted%3B run `robusta format --fix`");
    assert!(!stdout(&output).contains("task.complete"));
}

#[test]
fn invalid_reporter_spellings_are_rejected() {
    let temp = TempProject::new();

    for spelling in ["azure", "git-lab"] {
        let output = robusta_in_dir(&["lint", "--reporter", spelling, "."], &temp.dir);

        assert_eq!(output.status.code(), Some(2), "{spelling}");
        assert!(output.stdout.is_empty(), "{spelling}");
        assert!(
            stderr(&output).cow_replace('\n', " ").contains(&format!(
                "unknown reporter `{spelling}` (expected default, json, checkstyle, github, gitlab, azure-devops, unix, agent, sarif, stylish, or junit)"
            )),
            "{spelling}"
        );
    }
}

#[test]
fn check_sarif_reporter_exposes_formatting_as_a_rule_result() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example { int value; }\n");

    let output = robusta_in_dir(&["check", "--reporter", "sarif", "Example.java"], &temp.dir);
    let stdout = stdout(&output);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    assert!(stdout.contains(r#""version": "2.1.0""#));
    assert!(stdout.contains(r#""ruleId": "format""#));
    assert!(stdout.contains(r#""uri": "Example.java""#));
    assert!(stdout.contains(r#""startLine": 1"#));
    assert!(stdout.contains(r#""startColumn": 1"#));
}

#[test]
fn sarif_and_gitlab_reporters_carry_the_whole_finding_range() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("Multi.java", "class Multi {\n    static\n    public int value;\n}\n");
    temp.write("Single.java", "class Single { static public int value; }\n");

    let multi = robusta_in_dir(&["lint", "--reporter", "sarif", "Multi.java"], &temp.dir);
    let region = stdout(&multi);
    assert_eq!(multi.status.code(), Some(0), "{}", stderr(&multi));
    assert!(region.contains(r#""startLine": 2"#), "{region}");
    assert!(region.contains(r#""startColumn": 5"#), "{region}");
    assert!(region.contains(r#""endLine": 3"#), "{region}");
    assert!(region.contains(r#""endColumn": 11"#), "{region}");

    let single = robusta_in_dir(&["lint", "--reporter", "sarif", "Single.java"], &temp.dir);
    let region = stdout(&single);
    assert_eq!(single.status.code(), Some(0), "{}", stderr(&single));
    assert!(region.contains(r#""startLine": 1"#), "{region}");
    assert!(region.contains(r#""startColumn": 16"#), "{region}");
    assert!(region.contains(r#""endLine": 1"#), "{region}");
    assert!(region.contains(r#""endColumn": 29"#), "{region}");

    let multi = robusta_in_dir(&["lint", "--reporter", "gitlab", "Multi.java"], &temp.dir);
    let lines = stdout(&multi);
    assert_eq!(multi.status.code(), Some(0), "{}", stderr(&multi));
    assert!(lines.contains("\"begin\": 2"), "{lines}");
    assert!(lines.contains("\"end\": 3"), "{lines}");

    let single = robusta_in_dir(&["lint", "--reporter", "gitlab", "Single.java"], &temp.dir);
    let lines = stdout(&single);
    assert_eq!(single.status.code(), Some(0), "{}", stderr(&single));
    assert!(lines.contains("\"begin\": 1"), "{lines}");
    assert!(lines.contains("\"end\": 1"), "{lines}");
}

#[test]
fn the_gitlab_reporter_reports_paths_relative_to_the_repository_root() {
    let rooted = TempProject::new();
    std::fs::create_dir(rooted.dir.join(".git")).expect("failed to create .git dir");
    rooted.write("modules/core/Example.java", "class Example { int value; }\n");

    let nested = rooted.dir.join("modules").join("core");
    let below_root = robusta_in_dir(&["check", "--reporter", "gitlab", "Example.java"], &nested);
    let report = stdout(&below_root);
    assert_eq!(below_root.status.code(), Some(1), "{}", stderr(&below_root));
    assert!(report.contains(r#""path": "modules/core/Example.java""#), "{report}");

    // Outside a repository the selected path is reported unchanged, and the run
    // still succeeds.
    let unrooted = TempProject::new();
    unrooted.write("Example.java", "class Example { int value; }\n");

    let outside = robusta_in_dir(&["check", "--reporter", "gitlab", "Example.java"], &unrooted.dir);
    let report = stdout(&outside);
    assert_eq!(outside.status.code(), Some(1), "{}", stderr(&outside));
    assert!(report.contains(r#""path": "Example.java""#), "{report}");
}

/// A file name is not a URI, so the reporter has to encode one into the other.
/// The run reports three names that URI syntax would otherwise misread: a
/// space and a fragment marker, a name that only looks percent-encoded
/// already, and a non-ASCII name.
#[test]
fn the_sarif_reporter_encodes_artifact_paths_as_uri_references() {
    let temp = TempProject::new();
    let names = ["Example #1.java", "Example%20.java", "Caf\u{e9}.java"]; // spellchecker:disable-line
    for name in names {
        temp.write(name, "class Example { int value; }\n");
    }

    let mut args = vec!["check", "--reporter", "sarif"];
    args.extend(names);
    let output = robusta_in_dir(&args, &temp.dir);
    let stdout = stdout(&output);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);

    let document: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("{error}: {stdout}"));
    let mut uris: Vec<&str> = document["runs"][0]["results"]
        .as_array()
        .unwrap_or_else(|| panic!("no results: {stdout}"))
        .iter()
        .map(|result| {
            result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
                .as_str()
                .unwrap_or_else(|| panic!("no uri: {stdout}"))
        })
        .collect();
    uris.sort_unstable();

    // A raw `%` is encoded as data, so the name holding `%20` stays distinct
    // from the name holding a real space.
    // spellchecker:disable-next-line
    assert_eq!(uris, ["Caf%C3%A9.java", "Example%20%231.java", "Example%2520.java"], "{stdout}");

    // Each reference still names the file the run actually read.
    let mut decoded: Vec<String> = uris
        .iter()
        .map(|uri| {
            percent_encoding::percent_decode_str(uri)
                .decode_utf8()
                .expect("an encoded UTF-8 path decodes as UTF-8")
                .into_owned()
        })
        .collect();
    decoded.sort();
    let mut expected: Vec<String> = names.iter().map(|&name| name.to_owned()).collect();
    expected.sort();
    assert_eq!(decoded, expected);
    for name in names {
        assert!(temp.dir.join(name).is_file(), "{name}");
    }
}

#[test]
fn lint_sarif_reporter_describes_declared_rules() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("Example.java", "class Example { static public int value; }\n");

    let output = robusta_in_dir(&["lint", "--reporter", "sarif", "Example.java"], &temp.dir);
    let stdout = stdout(&output);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    assert!(stdout.contains(r#""id": "modifier-order""#));
    assert!(stdout.contains(
        r#""text": "Reports keyword modifiers that are not in Java's customary order.""#
    ));
}

#[test]
fn machine_reporters_emit_clean_documents() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example {}\n");

    let clean = robusta_in_dir(&["check", "--reporter", "json", "Example.java"], &temp.dir);
    assert_eq!(clean.status.code(), Some(0), "{}", stderr(&clean));
    assert_eq!(clean.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&clean), @r#"
    {
      "files": [],
      "summary": {
        "diagnostics": 0,
        "unformatted": 0
      }
    }
    "#);

    let clean_azure =
        robusta_in_dir(&["check", "--reporter", "azure-devops", "Example.java"], &temp.dir);
    assert_eq!(clean_azure.status.code(), Some(0), "{}", stderr(&clean_azure));
    assert_eq!(clean_azure.stdout.len(), 0);
    assert_eq!(clean_azure.stderr.len(), 0);

    let clean_gitlab =
        robusta_in_dir(&["check", "--reporter", "gitlab", "Example.java"], &temp.dir);
    assert_eq!(clean_gitlab.status.code(), Some(0), "{}", stderr(&clean_gitlab));
    assert_eq!(stdout(&clean_gitlab), "[]\n");
    assert_eq!(clean_gitlab.stderr.len(), 0);

    let clean_unix = robusta_in_dir(&["check", "--reporter", "unix", "Example.java"], &temp.dir);
    assert_eq!(clean_unix.status.code(), Some(0), "{}", stderr(&clean_unix));
    assert_eq!(clean_unix.stdout.len(), 0);
    assert_eq!(clean_unix.stderr.len(), 0);
}

/// A preview's document owns stdout, so the selected report moves to stderr
/// whole, in the same run, for the buffered formats as for the default one.
#[test]
fn a_preview_moves_every_report_to_stderr() {
    let temp = TempProject::new();
    temp.write("Example.java", "class  Example {}\n");

    let diff =
        robusta_in_dir(&["check", "--diff", "--reporter", "json", "Example.java"], &temp.dir);
    assert_eq!(diff.status.code(), Some(1), "{}", stderr(&diff));
    assert!(stdout(&diff).starts_with("--- Example.java\n+++ Example.java\n"), "{}", stdout(&diff));
    insta::assert_snapshot!(stderr(&diff), @r#"
    {
      "files": [
        {
          "path": "Example.java",
          "formatted": false,
          "diagnostics": []
        }
      ],
      "summary": {
        "diagnostics": 0,
        "unformatted": 1
      }
    }
    "#);

    for reporter in ["default", "gitlab", "unix", "agent", "stylish", "junit"] {
        for flag in ["--diff", "--list-different"] {
            let output =
                robusta_in_dir(&["check", flag, "--reporter", reporter, "Example.java"], &temp.dir);
            let (document, report) = (stdout(&output), stderr(&output));
            assert_eq!(output.status.code(), Some(1), "{reporter}/{flag}: {report}");
            if flag == "--diff" {
                assert!(document.contains("+class Example {}"), "{reporter}/{flag}: {document}");
            } else {
                assert_eq!(document, "Example.java\n", "{reporter}/{flag}");
            }
            assert!(!document.contains("Found "), "{reporter}/{flag}: {document}");
            let expected = if reporter == "default" { "Found 0 warnings" } else { "Example.java" };
            assert!(report.contains(expected), "{reporter}/{flag}: {report}");
        }
    }
}

#[test]
fn silent_preserves_previews_for_every_reporter() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    broken-null-check: deny\n",
    );
    let baseline = "{}\n";
    let baseline_path = temp.write("robusta-suppressions.json", baseline);
    let source = "class  Example { long value = 1l; boolean f(String s) { return s == null && s.isEmpty(); } }\n";
    let file = temp.write("Example.java", source);
    let clean_source = "class Clean {}\n";
    let clean_file = temp.write("Clean.java", clean_source);

    for reporter in [
        "default",
        "json",
        "checkstyle",
        "github",
        "gitlab",
        "azure-devops",
        "unix",
        "agent",
        "sarif",
        "stylish",
        "junit",
    ] {
        for command in ["check", "format", "lint"] {
            for flag in ["--diff", "--list-different"] {
                let args = [command, flag, "--reporter", reporter, "Example.java"];
                let preview = robusta_in_dir(&args, &temp.dir);
                let silent = robusta_in_dir(&[&args[..], &["--silent"]].concat(), &temp.dir);
                assert_eq!(preview.status.code(), Some(1), "{args:?}: {}", stderr(&preview));
                assert_eq!(silent.status.code(), preview.status.code(), "{args:?}");
                assert!(!preview.stdout.is_empty(), "{args:?}");
                assert_eq!(silent.stdout, preview.stdout, "{args:?}");
                assert!(silent.stderr.is_empty(), "{args:?}: {}", stderr(&silent));
                if flag == "--list-different" {
                    assert_eq!(stdout(&silent), "Example.java\n", "{args:?}");
                } else {
                    assert!(stdout(&silent).starts_with("--- Example.java\n+++ Example.java\n"));
                }

                let clean = robusta_in_dir(
                    &[command, flag, "--silent", "--reporter", reporter, "Clean.java"],
                    &temp.dir,
                );
                assert_eq!(clean.status.code(), Some(0), "{args:?}: {}", stderr(&clean));
                assert!(clean.stdout.is_empty(), "{args:?}: {}", stdout(&clean));
                assert!(clean.stderr.is_empty(), "{args:?}: {}", stderr(&clean));
                assert_eq!(std::fs::read(&file).unwrap(), source.as_bytes(), "{args:?}");
                assert_eq!(
                    std::fs::read(&clean_file).unwrap(),
                    clean_source.as_bytes(),
                    "{args:?}"
                );
                assert_eq!(std::fs::read(&baseline_path).unwrap(), baseline.as_bytes(), "{args:?}");
            }
        }
    }
}

/// A file the run cannot parse is reported like any other finding, so the
/// document still carries every other file's result instead of being discarded.
#[test]
fn machine_reporters_report_unparsable_files_beside_the_rest_of_the_run() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Good.java", "class Good { long value = 10l; }\n");
    temp.write("Broken.java", "class Broken { int ; }\n");

    for format in [
        "json",
        "checkstyle",
        "github",
        "gitlab",
        "azure-devops",
        "unix",
        "agent",
        "sarif",
        "stylish",
        "junit",
    ] {
        let output =
            robusta_in_dir(&["lint", "--reporter", format, "Broken.java", "Good.java"], &temp.dir);
        let document = stdout(&output);

        assert_eq!(output.status.code(), Some(1), "{format}: {}", stderr(&output));
        assert!(document.contains("expected a member name"), "{format}: {document}");
        assert!(
            document.contains("use uppercase `L` for the long literal suffix"),
            "{format}: {document}"
        );
        // The selected document owns the run's output; nothing is re-rendered
        // for a person on the side.
        assert!(output.stderr.is_empty(), "{format}: {}", stderr(&output));
    }
}

#[cfg(unix)]
#[test]
fn reports_identify_a_file_whose_name_holds_a_literal_backslash() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write(r"src\A.java", "class A { long n = 1l; }\n");

    for (reporter, expected) in [
        ("default", r"[src\A.java:1:"),
        ("json", r#""path": "src\\A.java""#),
        ("checkstyle", r#"<file name="src\A.java">"#),
        ("github", r"file=src\A.java,"),
        ("gitlab", r#""path": "src\\A.java""#),
        ("azure-devops", r"sourcepath=src\A.java;"),
        ("unix", r"src\A.java:1:"),
        ("agent", r"src\A.java:1:"),
        ("sarif", r#""uri": "src%5CA.java""#),
        ("stylish", r"src\A.java"),
        ("junit", r#"<testsuite name="src\A.java""#),
    ] {
        let output = robusta_in_dir(&["lint", "--reporter", reporter, r"src\A.java"], &temp.dir);
        let report = format!("{}{}", stdout(&output), stderr(&output));
        assert!(report.contains(expected), "{reporter}: {report}");
    }
}

/// A file the tool could not open is reported the same way, but as an error
/// against the file rather than a place inside it, and it outranks a parse
/// failure when the run picks its status.
#[cfg(unix)]
#[test]
fn an_unreadable_file_is_reported_as_a_positionless_finding() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Good.java", "class Good { long value = 10l; }\n");
    temp.write("Broken.java", "class Broken { int ; }\n");
    let locked = temp.write("Locked.java", "class Locked { int value; }\n");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000))
        .expect("failed to lock file");

    let machine = robusta_in_dir(&["lint", "--reporter", "unix", "."], &temp.dir);
    let human = robusta_in_dir(&["lint", "."], &temp.dir);

    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o644))
        .expect("failed to unlock file");

    // A tool-side failure outranks the source-side one, so the run exits 2.
    assert_eq!(machine.status.code(), Some(2), "{}", stderr(&machine));
    let document = stdout(&machine);
    assert!(document.contains("Locked.java:0:0: Locked.java: "), "{document}");
    assert!(document.contains("expected a member name"), "{document}");
    assert!(document.contains("use uppercase `L` for the long literal suffix"), "{document}");
    assert!(document.contains("3 problems"), "{document}");

    assert_eq!(human.status.code(), Some(2), "{}", stdout(&human));
    assert!(stderr(&machine).contains("Error: Locked.java: "));
    let rendered = stderr(&human);
    // No excerpt and no position: the failure is about the file, not a place
    // in it.
    assert!(rendered.contains("x Locked.java: "), "{rendered}");
    assert!(!rendered.contains("[Locked.java:"), "{rendered}");
    assert!(!stdout(&human).contains("Locked.java: "));
    assert!(stdout(&human).contains("Found 0 warnings and 3 errors."));
}

#[test]
fn file_read_failures_reach_stderr_without_corrupting_reports_or_previews() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let source = "class Good { long value = 10l; }\n";
    let good = temp.write("Good.java", source);
    temp.write("Broken.java", "class Broken { int ; }\n");
    let locked = temp.write("Locked.java", "");
    std::fs::write(&locked, [0xff]).unwrap();
    let message = format!("Locked.java: {}", std::fs::read_to_string(&locked).unwrap_err());

    for command in ["check", "format", "lint"] {
        for reporter in [
            "default",
            "json",
            "checkstyle",
            "github",
            "gitlab",
            "azure-devops",
            "unix",
            "agent",
            "sarif",
            "stylish",
            "junit",
        ] {
            for mode in ["", "--diff", "--list-different"] {
                let mut args = vec![command, "--reporter", reporter];
                if !mode.is_empty() {
                    args.push(mode);
                }
                args.extend(["Locked.java", "Good.java", "Broken.java"]);
                let output = robusta_in_dir(&args, &temp.dir);
                let errors = stderr(&output);
                let document = stdout(&output);
                assert_eq!(output.status.code(), Some(2), "{args:?}: {errors}");
                let occurrences = if reporter == "junit" && !mode.is_empty() { 2 } else { 1 };
                assert_eq!(errors.matches(&message).count(), occurrences, "{args:?}: {errors}");
                let report = if mode.is_empty() { &document } else { &errors };
                assert!(report.contains("expected a member name"), "{args:?}: {report}");
                if reporter == "default" || !mode.is_empty() {
                    assert!(!document.contains("Locked.java"), "{args:?}: {document}");
                } else {
                    assert!(document.contains(&message), "{args:?}: {document}");
                    assert_eq!(errors, format!("Error: {message}\n"), "{args:?}");
                }
                if ["json", "gitlab", "sarif"].contains(&reporter) {
                    serde_json::from_str::<serde_json::Value>(report).expect("valid report JSON");
                }
                if mode == "--diff" {
                    assert!(document.starts_with("--- Good.java\n+++ Good.java\n"));
                } else if mode == "--list-different" {
                    assert_eq!(document, "Good.java\n");
                }

                args.push("--silent");
                let silent = robusta_in_dir(&args, &temp.dir);
                assert_eq!(silent.status.code(), Some(2), "{args:?}");
                assert!(silent.stderr.is_empty(), "{args:?}: {}", stderr(&silent));
                if mode.is_empty() {
                    assert!(silent.stdout.is_empty(), "{args:?}");
                } else {
                    assert_eq!(silent.stdout, output.stdout, "{args:?}");
                }
                assert_eq!(std::fs::read(&good).unwrap(), source.as_bytes());
                assert_eq!(std::fs::read(&locked).unwrap(), [0xff]);
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn write_failures_reach_stderr_and_preserve_source() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let source = "class Example { long value = 10l; }\n";
    let file = temp.write("locked/Example.java", source);
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o444)).unwrap();
    let mut results = Vec::new();
    for command in ["check", "format", "lint"] {
        for reporter in [
            "default",
            "json",
            "checkstyle",
            "github",
            "gitlab",
            "azure-devops",
            "unix",
            "agent",
            "sarif",
            "stylish",
            "junit",
        ] {
            for flag in ["--quiet", "--silent"] {
                let output = robusta_in_dir(
                    &[command, "--fix", "--reporter", reporter, flag, "locked/Example.java"],
                    &temp.dir,
                );
                results.push((command, reporter, flag, output));
            }
        }
    }

    for (command, reporter, flag, output) in results {
        let errors = stderr(&output);
        let document = stdout(&output);
        assert_eq!(output.status.code(), Some(2), "{command} {reporter} {flag}: {errors}");
        if flag == "--silent" {
            assert_eq!(output.stdout.len(), 0);
            assert_eq!(output.stderr.len(), 0);
        } else {
            assert_eq!(errors.matches("locked/Example.java: ").count(), 1, "{errors}");
            if reporter == "default" {
                assert!(!document.contains("locked/Example.java"), "{document}");
            } else {
                assert!(document.contains("locked/Example.java"), "{document}");
                assert!(errors.starts_with("Error: locked/Example.java: "), "{errors}");
            }
        }
        assert_eq!(std::fs::read(&file).unwrap(), source.as_bytes());
    }
}

/// Parse errors count as errors, not warnings, so they neither consume the
/// `--max-warnings` budget nor satisfy `--deny-warnings`.
#[test]
fn an_unparsable_file_counts_as_an_error_and_leaves_the_warning_budget_alone() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write("Warned.java", "class Warned { long value = 10l; }\n");
    temp.write("Broken.java", "class Broken { int ; }\n");

    let budgeted = robusta_in_dir(&["lint", "--max-warnings", "1", "."], &temp.dir);
    let within = stdout(&budgeted);

    assert_eq!(budgeted.status.code(), Some(1), "{within}");
    assert!(within.contains("Found 1 warning and 1 error."), "{within}");
    assert!(budgeted.stderr.is_empty(), "{}", stderr(&budgeted));

    let over_budget = robusta_in_dir(&["lint", "--max-warnings", "0", "."], &temp.dir);

    assert_eq!(over_budget.status.code(), Some(1), "{}", stdout(&over_budget));
    assert_eq!(stderr(&over_budget), "Exceeded maximum number of warnings. Found 1.\n");
}

#[test]
fn lint_fix_honors_machine_reporters() {
    let source = "class Example { long value = 10l; }\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--fix", "--reporter", "json"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    assert!(stdout(&output).contains("\"diagnostics\": 0"));
    assert_eq!(
        std::fs::read_to_string(&temp.file).unwrap(),
        "class Example { long value = 10L; }\n"
    );
}

#[test]
fn formatting_only_machine_reports_use_the_shared_file_report_model() {
    let source = "class Example { int value ; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "--reporter", "json"], &temp.file);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    assert!(stdout(&output).contains(r#""formatted": false"#));
    assert!(stdout(&output).contains(r#""diagnostics": 0"#));
    assert!(stdout(&output).contains(r#""unformatted": 1"#));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn formatting_verification_honors_quiet_silent_and_complete_machine_documents() {
    let source = "class Example { int value ; }\n";
    let temp = TempJava::new(source);

    let quiet = robusta_with_path(&["format", "--quiet"], &temp.file);
    assert_eq!(quiet.status.code(), Some(1), "{}", stderr(&quiet));
    assert_eq!(
        stdout_without_summary(&quiet),
        format!("would reformat: {}\n", presentation_path(&temp.file))
    );
    assert!(quiet.stderr.is_empty(), "{}", stderr(&quiet));

    let silent = robusta_with_path(&["format", "--silent"], &temp.file);
    assert_eq!(silent.status.code(), Some(1), "{}", stderr(&silent));
    assert_eq!(silent.stdout.len(), 0);
    assert_eq!(silent.stderr.len(), 0);
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);

    let project = TempProject::new();
    project.write("Good.java", source);
    project.write("Broken.java", "class Broken { int ; }\n");
    let machine =
        robusta_in_dir(&["format", "--reporter", "json", "Good.java", "Broken.java"], &project.dir);
    let document = stdout(&machine);
    assert_eq!(machine.status.code(), Some(1));
    assert!(document.contains(r#""path": "Good.java""#), "{document}");
    assert!(document.contains(r#""formatted": false"#), "{document}");
    // The file that never parsed has no formatting state to report, so the
    // field is omitted rather than claiming the file was in order.
    assert!(document.contains(r#""path": "Broken.java""#), "{document}");
    assert!(document.contains(r#""line": 1"#), "{document}");
    assert!(document.contains(r#""column": 20"#), "{document}");
    assert_eq!(document.matches(r#""formatted""#).count(), 1, "{document}");
}

#[test]
fn aggregate_fix_writes_valid_partial_improvements_through_every_reporter() {
    let source = "class Example { void f() { long value=10l; use(value); int stale=1+2; } }\n";
    for reporter in [
        "default",
        "json",
        "checkstyle",
        "github",
        "gitlab",
        "azure-devops",
        "unix",
        "agent",
        "sarif",
        "stylish",
        "junit",
    ] {
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

        let output = robusta_with_path(&["check", "--fix", "--reporter", reporter], &temp.file);

        let report = stdout(&output);
        assert_eq!(output.status.code(), Some(0), "{reporter}: {report}");
        assert!(report.contains("unused-local-variable"), "{reporter}: {report}");
        assert!(output.stderr.is_empty(), "{reporter}: {}", stderr(&output));
        let written = std::fs::read_to_string(&temp.file).unwrap();
        assert!(written.contains("10L"), "{reporter}: {written}");
        assert!(!written.contains("10l"), "{reporter}: {written}");
    }
}

#[test]
fn run_summary_reports_totals_files_rules_and_threads_per_command() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  categories:\n    correctness: allow\n  rules:\n    modifier-order: warn\n    uppercase-long-suffix: deny\n",
    );
    temp.write("Example.java", "class Example {\n  static public long value = 10l;\n}\n");

    // One file, one warning, one error: every singular form at once.
    let lint = robusta_in_dir(&["lint", "--threads", "1", "Example.java"], &temp.dir);
    assert_eq!(lint.status.code(), Some(1), "{}", stderr(&lint));
    insta::assert_snapshot!(run_summary(&lint), @"

    Found 1 warning and 1 error.
    Finished in <time> on 1 file with 2 rules using 1 threads.
    ");

    // `check` runs the same rule set and adds the formatting verification.
    let check = robusta_in_dir(&["check", "--threads", "1", "Example.java"], &temp.dir);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    insta::assert_snapshot!(run_summary(&check), @"

    Found 1 warning and 1 error.
    Finished in <time> on 1 file with 2 rules using 1 threads.
    ");

    // `format` runs no rules, so the clause is dropped, and its totals stay at
    // zero however many findings the lint scopes reported.
    let format = robusta_in_dir(&["format", "--threads", "1", "Example.java"], &temp.dir);
    assert_eq!(format.status.code(), Some(0), "{}", stderr(&format));
    insta::assert_snapshot!(run_summary(&format), @"
    Found 0 warnings and 0 errors.
    Finished in <time> on 1 file using 1 threads.
    ");
}

#[test]
fn run_summary_pluralizes_files_and_counts_a_clean_multi_file_run() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  categories:\n    correctness: allow\n  rules:\n    uppercase-long-suffix: deny\n",
    );
    temp.write("src/First.java", "class First {\n    long value = 10L;\n}\n");
    temp.write("src/Second.java", "class Second {\n    long value = 20L;\n}\n");

    let output = robusta_in_dir(&["lint", "--threads", "1", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(run_summary(&output), @"
    Found 0 warnings and 0 errors.
    Finished in <time> on 2 files with 1 rules using 1 threads.
    ");
}

/// The summary is present exactly when a run went worst, and its file total
/// counts a file that failed as a file the run processed.
#[test]
fn run_summary_closes_a_run_that_could_not_process_every_file() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  categories:\n    correctness: allow\n  rules:\n    uppercase-long-suffix: deny\n",
    );
    temp.write("src/Broken.java", "class Broken { int ; }\n");
    temp.write("src/Good.java", "class Good {\n    long value = 10L;\n}\n");

    let output = robusta_in_dir(&["lint", "--threads", "1", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    insta::assert_snapshot!(run_summary(&output), @"
    Found 0 warnings and 1 error.
    Finished in <time> on 2 files with 1 rules using 1 threads.
    ");
}

#[test]
fn run_summary_drops_the_rule_clause_when_nested_configs_disagree() {
    let temp = TempProject::new();
    temp.write("first/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write("first/First.java", "class First {\n    long value = 10L;\n}\n");
    temp.write(
        "second/robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    );
    temp.write("second/Second.java", "class Second {\n    long value = 20L;\n}\n");

    let output = robusta_in_dir(&["lint", "--threads", "1", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(run_summary(&output), @"
    Found 0 warnings and 0 errors.
    Finished in <time> on 2 files using 1 threads.
    ");
}

#[test]
fn warning_controls_hide_the_run_summary_without_changing_its_counts() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  categories:\n    correctness: allow\n  rules:\n    modifier-order: warn\n",
    );
    temp.write("Example.java", "class Example {\n    static public int value;\n}\n");

    // `--quiet` filters the finding out of the stream, never out of the counts.
    let quiet = robusta_in_dir(&["lint", "--quiet", "--threads", "1", "Example.java"], &temp.dir);
    assert_eq!(quiet.status.code(), Some(0), "{}", stdout(&quiet));
    assert!(!stdout(&quiet).contains("modifier-order:"));
    assert!(quiet.stderr.is_empty(), "{}", stderr(&quiet));
    insta::assert_snapshot!(run_summary(&quiet), @"

    Found 1 warning and 0 errors.
    Finished in <time> on 1 file with 1 rules using 1 threads.
    ");

    // The exceeded warning budget is a failure, so it goes to stderr and the
    // summary stays whole.
    let exceeded = robusta_in_dir(
        &["lint", "--max-warnings", "0", "--threads", "1", "Example.java"],
        &temp.dir,
    );
    assert_eq!(exceeded.status.code(), Some(1));
    insta::assert_snapshot!(run_summary(&exceeded), @"

    Found 1 warning and 0 errors.
    Finished in <time> on 1 file with 1 rules using 1 threads.
    ");
    assert_eq!(stderr(&exceeded), "Exceeded maximum number of warnings. Found 1.\n");

    let silent = robusta_in_dir(&["lint", "--silent", "Example.java"], &temp.dir);
    assert_eq!(silent.status.code(), Some(0));
    assert!(silent.stdout.is_empty(), "{}", stdout(&silent));
    assert_eq!(silent.stderr.len(), 0);
}

#[test]
fn machine_reporters_emit_no_run_summary_on_either_stream() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Example.java", "class Example { long value = 10l; }\n");

    for reporter in [
        "json",
        "checkstyle",
        "github",
        "gitlab",
        "azure-devops",
        "unix",
        "agent",
        "sarif",
        "stylish",
        "junit",
    ] {
        let output = robusta_in_dir(&["lint", "--reporter", reporter, "Example.java"], &temp.dir);

        assert_eq!(output.status.code(), Some(1), "{reporter}");
        assert!(output.stderr.is_empty(), "{reporter}: {}", stderr(&output));
        assert!(!stdout(&output).contains("Finished in "), "{reporter}: {}", stdout(&output));
        assert!(!stdout(&output).contains("Found 1 warning"), "{reporter}: {}", stdout(&output));
    }
}
