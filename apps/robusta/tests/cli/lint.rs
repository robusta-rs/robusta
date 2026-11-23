use cow_utils::CowUtils;
use robusta_linter::{RuleCategory, rules::builtin_rules};

use crate::support::*;

#[test]
fn documented_disable_comments_suppress_imports_and_statements() {
    for ending in ["\n", "\r\n", "\r"] {
        let temp = TempProject::new();
        let source = concat!(
            "// robusta-disable-next-line unused-import -- Matches the upstream source.\n",
            "import java.util.List;\n",
            "import java.util.Map; // robusta-disable-line unused-import\n",
            "class Example {\n",
            "    void f() {\n",
            "        int stale = 1; // robusta-disable-line unused-local-variable\n",
            "    }\n",
            "}\n",
        )
        .cow_replace('\n', ending);
        temp.write("Example.java", &source);
        let output = robusta_in_dir(&["lint", "--reporter", "json", "Example.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["summary"]["diagnostics"], 0, "{report}");
        assert_eq!(std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(), source);
    }
}

#[test]
fn documented_inline_suppressions_each_silence_a_finding() {
    let temp = TempProject::new();
    let source = r#"// robusta-disable-next-line unused-import -- Matches the upstream source.
import java.util.List;
import java.util.Map; // robusta-disable-line unused-import

class Cache {
    @SuppressWarnings("robusta:unused-local-variable")
    void clear() {
        int previous = reset();
    }
}
"#;
    temp.write("Cache.java", source);
    // An unused suppression would be reported, so each one silences a finding.
    let output = robusta_in_dir(
        &["lint", "--report-unused-suppressions", "--reporter", "json", "Cache.java"],
        &temp.dir,
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["diagnostics"], 0, "{report}");
}

#[test]
fn block_disable_comments_suppress_and_prune_like_line_comments() {
    for ending in ["\n", "\r\n", "\r"] {
        let temp = TempProject::new();
        let source = concat!(
            "/* robusta-disable-next-line unused-import -- Matches the upstream source. */\n",
            "import java.util.List;\n",
            "import java.util.Map; /* robusta-disable-line unused-import */\n",
            "class Example {\n",
            "    void f() {\n",
            "        /* robusta-disable-next-line\n",
            "           unused-local-variable */\n",
            "        int stale = 1;\n",
            "    }\n",
            "}\n",
        )
        .cow_replace('\n', ending);
        temp.write("Example.java", &source);
        let output = robusta_in_dir(&["lint", "--reporter", "json", "Example.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["summary"]["diagnostics"], 0, "{report}");
    }

    let temp = TempJava::new(concat!(
        "class Example {\n",
        "    int value; /* robusta-disable-line unused-import -- retained reason */ int other;\n",
        "}\n",
    ));
    let output = robusta_with_path(&["lint", "--report-unused-suppressions", "--fix"], &temp.file);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(&temp.file).unwrap(),
        "class Example {\n    int value; /* retained reason */ int other;\n}\n"
    );
}

#[test]
fn shared_all_is_not_reported_unused_by_lint_or_check() {
    let source = "class Example {\n  @SuppressWarnings(\"all\")\n  int value;\n}\n";
    let temp = TempJava::new(source);
    for command in ["lint", "check"] {
        for severity in ["allow", "warn", "deny"] {
            for flags in [&[][..], &["--fix"][..], &["--diff"][..], &["--quiet"][..]] {
                let mut args = vec![command, "--report-unused-suppressions-severity", severity];
                args.extend_from_slice(flags);
                let output = robusta_with_path(&args, &temp.file);
                assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
                assert!(!stdout(&output).contains("unused-suppression"), "{}", stdout(&output));
                assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
            }
        }
    }
}

#[test]
fn suppression_fixes_keep_shared_and_foreign_annotation_values() {
    let source = concat!(
        "class Example {\n",
        "    @SuppressWarnings({\"all\", \"robusta:uppercase-long-suffix\", \"unchecked\"})\n",
        "    int value; // robusta-disable-line uppercase-long-suffix -- retained reason\n",
        "}\n",
    );
    for command in ["lint", "check"] {
        let temp = TempJava::new(source);
        let output =
            robusta_with_path(&[command, "--report-unused-suppressions", "--fix"], &temp.file);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        let fixed = std::fs::read_to_string(&temp.file).unwrap();
        assert!(fixed.contains("@SuppressWarnings({\"all\", \"unchecked\"})"), "{fixed}");
        assert!(fixed.contains("// retained reason"), "{fixed}");
        assert!(!fixed.contains("robusta:"), "{fixed}");
        assert!(!fixed.contains("robusta-disable"), "{fixed}");
        let again = robusta_with_path(
            &[command, "--report-unused-suppressions-severity", "deny"],
            &temp.file,
        );
        assert_eq!(again.status.code(), Some(0), "{}", stdout(&again));
    }
}

#[test]
fn lint_preview_and_write_remove_escaped_lambda_parentheses() {
    let class = |parameters: &str| {
        format!(
            "class A {{ static int x = 20; \
             java.util.function.IntUnaryOperator f = {parameters} -> x + 1; }}\n"
        )
    };
    for (parameter_list, interior) in [
        (r"\u0028x)", "x"),
        (r"(x\u0029", "x"),
        (r"\u0028x\u0029", "x"),
        (r"\uuuu0028x)", "x"),
        (r"(x\uuuu0029", "x"),
        (r"\uu0028 /* before */ \u0078 /* after */ \uuu0029", r" /* before */ \u0078 /* after */ "),
        ("\\u0028// before\r\nx // after\r\n\\u0029", "// before\r\nx // after\r\n"),
    ] {
        assert_preview_and_write_remove_lambda_parentheses(
            &class(parameter_list),
            &class(interior),
        );
    }
}

#[test]
fn lint_preview_and_write_remove_literal_lambda_parentheses() {
    for (source, expected) in [
        (
            "class A { java.util.function.IntUnaryOperator f = ( /* before */ \\u0078 /* after */ ) -> x; }\n",
            "class A { java.util.function.IntUnaryOperator f =  /* before */ \\u0078 /* after */  -> x; }\n",
        ),
        (
            "class A { java.util.function.IntUnaryOperator f() { return(value)->value + 1; } }\n",
            "class A { java.util.function.IntUnaryOperator f() { return value->value + 1; } }\n",
        ),
        (
            "class A { java.util.function.IntUnaryOperator f(int i) { return switch (i) { default -> { yield(_)->1; } }; } }\n",
            "class A { java.util.function.IntUnaryOperator f(int i) { return switch (i) { default -> { yield _->1; } }; } }\n",
        ),
    ] {
        assert_preview_and_write_remove_lambda_parentheses(source, expected);
    }
}

fn assert_preview_and_write_remove_lambda_parentheses(source: &str, expected: &str) {
    let temp = TempJava::new(source);
    let preview = robusta_with_path(
        &["lint", "-D", "redundant-lambda-parameter-parentheses", "--diff"],
        &temp.file,
    );
    assert_eq!(preview.status.code(), Some(1), "{}", stderr(&preview));
    assert_eq!(stderr_without_summary(&preview), "");
    assert_eq!(candidate_from_diff(&stdout(&preview)), expected);
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    for _ in 0..2 {
        let fixed = robusta_with_path(
            &[
                "lint",
                "-D",
                "redundant-lambda-parameter-parentheses",
                "--fix",
                "--reporter",
                "json",
            ],
            &temp.file,
        );
        assert_eq!(fixed.status.code(), Some(0), "{}", stderr(&fixed));
        assert!(fixed.stderr.is_empty(), "{}", stderr(&fixed));
        assert!(stdout(&fixed).contains("\"diagnostics\": 0"));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), expected);
    }
}

#[test]
fn lint_fix_removes_write_only_identifier_ignorable_bindings() {
    for source in [
        r"class A { void f() { int coun\u200Ct; count = 1; } }",
        r"class A { void f() { int count; coun\u200Ct = 1; } }",
        "class A { void f() { int coun\u{200c}t; count = 1; } }",
        "class A { void f() { int count; coun\u{200c}t = 1; } }",
    ] {
        let temp = TempJava::new(source);
        let output = robusta_with_path(
            &["lint", "-D", "unused-local-variable", "--fix", "--reporter", "json"],
            &temp.file,
        );
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        assert!(stdout(&output).contains("\"diagnostics\": 0"));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), "class A { void f() {   } }");
    }
}

#[test]
fn lint_fixes_preserve_supplementary_format_identifier_bindings() {
    for spelling in ["coun\u{e0001}t", r"coun\uDB40\uDC01t"] {
        for declarations in [
            format!("int {spelling}; int count; {spelling} = 1; count = 2;"),
            format!("int {spelling} = get(); int count = get();"),
            format!("int {spelling} = 1; int count = 2;"),
        ] {
            let source = format!(
                "class A {{ static int get() {{ return 1; }} \
                 int f() {{ {declarations} return {spelling} + count; }} }}"
            );
            for fix in [&["--fix"][..], &["--fix", "--unsafe-fixes"]] {
                let temp = TempJava::new(&source);
                let args =
                    [&["lint", "-D", "unused-local-variable", "--reporter", "json"][..], fix]
                        .concat();
                let output = robusta_with_path(&args, &temp.file);
                assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
                assert!(output.stderr.is_empty(), "{}", stderr(&output));
                assert!(stdout(&output).contains("\"diagnostics\": 0"));
                assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
            }
        }
    }
}

#[test]
fn lint_does_not_load_formatter_only_editorconfig() {
    let temp = TempProject::new();
    temp.write(".editorconfig", "[*.java]\ntrim_trailing_whitespace = false\n");
    temp.write("Example.java", "class Example {}\n");

    let output = robusta_in_dir(&["lint", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn lint_rules_lists_registered_metadata_without_analyzing_files() {
    let temp = TempProject::new();
    temp.write("Broken.java", "class Broken { int ; }\n");
    temp.write(".editorconfig", "not a valid EditorConfig\n");

    let output = robusta_in_dir(&["lint", "--rules", "Broken.java"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    ## Correctness (2)
    Code that is wrong or useless.
    | Rule name             | Default | Enabled? | Fixable? |
    | --------------------- | ------- | -------- | -------- |
    | unused-import         | ✅      | ✅       | 🛠️        |
    | unused-local-variable | ✅      | ✅       | 🛠️⚠️🛠️️      |

    ## Suspicious (1)
    Constructs that commonly indicate stale or misleading code.
    | Rule name         | Default | Enabled? | Fixable? |
    | ----------------- | ------- | -------- | -------- |
    | broken-null-check |         |          |          |

    ## Style (8)
    Consistency and readability concerns with no behavioral risk.
    | Rule name                              | Default | Enabled? | Fixable? |
    | -------------------------------------- | ------- | -------- | -------- |
    | duplicate-import                       |         |          | 🛠️        |
    | modifier-order                         |         |          | 🛠️        |
    | prefer-expression-lambda               |         |          | 🛠️⚠️🛠️️      |
    | redundant-lambda-parameter-parentheses |         |          | 🛠️        |
    | redundant-return                       |         |          | 🛠️        |
    | redundant-semicolons                   |         |          | 🛠️        |
    | redundant-super-call                   |         |          | 🛠️        |
    | uppercase-long-suffix                  |         |          | 🛠️        |

    Default: 2
    Total: 11
    ");
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn lint_rules_resolves_configuration_and_ordered_filters() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint: {categories: {correctness: allow, style: warn}}\n");
    temp.write("other.yaml", "lint: {rules: {broken-null-check: deny}}\n");
    for (flags, enabled) in [
        (vec![], vec!["uppercase-long-suffix"]),
        (
            vec!["--config", "other.yaml"],
            vec!["unused-import", "unused-local-variable", "broken-null-check"],
        ),
        (
            vec!["-A", "all", "-D", "correctness", "-A", "unused-import"],
            vec!["unused-local-variable"],
        ),
        (vec!["-A", "all", "-W", "uppercase-long-suffix"], vec!["uppercase-long-suffix"]),
    ] {
        let mut args = vec!["lint", "--rules"];
        args.extend(flags);
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        let text = stdout(&output);
        for name in
            ["unused-import", "unused-local-variable", "broken-null-check", "uppercase-long-suffix"]
        {
            let row = text.lines().find(|line| line.starts_with(&format!("| {name} "))).unwrap();
            let cells = row.split('|').map(str::trim).collect::<Vec<_>>();
            assert_eq!(cells[3] == "✅", enabled.contains(&name), "{args:?}: {row}");
            assert_eq!(cells[2] == "✅", matches!(name, "unused-import" | "unused-local-variable"));
        }
    }
}

#[test]
fn lint_rules_lists_sorted_metadata_as_json() {
    let temp = TempProject::new();
    let output = robusta_in_dir(&["lint", "--rules", "--reporter", "json", "-A", "all"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    let rows: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
    let mut rules: Vec<_> = builtin_rules().iter().map(|rule| rule.meta()).collect();
    rules.sort_unstable_by_key(|rule| rule.id.as_str());
    assert_eq!(rows.len(), rules.len());
    for (row, rule) in rows.iter().zip(rules) {
        assert_eq!(
            *row,
            serde_json::json!({
                "value": rule.id.as_str(),
                "category": rule.category.as_str(),
                "fix": rule.fix.as_str(),
                "default": rule.category == RuleCategory::Correctness,
            }),
        );
    }
}

#[test]
fn lint_rules_with_unsupported_reporter_produces_no_output() {
    let temp = TempProject::new();
    let output = robusta_in_dir(&["lint", "--rules", "--reporter", "checkstyle"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stdout.is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn lint_rules_reports_configuration_errors_before_rendering() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "not: [valid\n");
    for reporter in ["default", "json", "checkstyle"] {
        for flags in [vec![], vec!["--config", "missing.yaml"]] {
            let mut args = vec!["lint", "--rules", "--reporter", reporter];
            args.extend(flags);
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(2));
            assert!(output.stdout.is_empty(), "{}", stdout(&output));
            assert_ne!(output.stderr.len(), 0);
        }
    }
}

#[test]
fn lint_rules_rejects_conflicting_flags_before_loading_configuration() {
    let source = "class Example { long value = 10l; }\n";
    let temp = TempProject::new();
    temp.write("Example.java", source);
    // A configuration error would replace the expected message.
    temp.write("robusta.yaml", "not: [valid\n");
    let stdin = |flags: &[&'static str], conflict| {
        ([&["--stdin-filepath", "Virtual.java"][..], flags].concat(), conflict)
    };
    let rejected = [
        (vec!["--fix", "--diff"], "`--fix` cannot be used at the same time as `--diff`"),
        (
            vec!["--list-different", "--unsafe-fixes", "--fix"],
            "`--fix` cannot be used at the same time as `--list-different`",
        ),
        (
            vec!["--unsafe-fixes"],
            "`--unsafe-fixes` requires `--fix`, `--diff`, or `--list-different`",
        ),
        (vec!["--suppress-all", "--diff"], "`--suppress-all` cannot be combined with `--diff`"),
        (
            vec!["--list-different", "--prune-suppressions"],
            "`--prune-suppressions` cannot be combined with `--list-different`",
        ),
        (
            vec!["--prune-suppressions", "--suppress-all"],
            "`--suppress-all` cannot be combined with `--prune-suppressions`",
        ),
        stdin(&["Example.java"], "Example.java"),
        stdin(&["--ignore-path", ".robustaignore"], "--ignore-path"),
        stdin(&["--ignore-pattern", "generated/"], "--ignore-pattern"),
        stdin(&["--threads", "1"], "--threads"),
        stdin(&["--diff"], "--diff"),
        stdin(&["--unsafe-fixes", "--list-different"], "--list-different"),
        stdin(&["--suppress-all"], "--suppress-all"),
        stdin(&["--prune-suppressions"], "--prune-suppressions"),
    ];
    for (flags, message) in rejected {
        let message = if flags[0] == "--stdin-filepath" {
            format!("Error: `--stdin-filepath` cannot be combined with `{message}`\n")
        } else {
            format!("Error: {message}\n")
        };
        let args = [&["lint", "--rules"][..], &flags].concat();
        let output = robusta_with_stdin(&args, &temp.dir, source);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
        assert_eq!(stderr(&output), message, "{args:?}");
    }

    // Conflicts the argument parser owns reject the same way.
    for flags in [
        &["--diff", "--list-different"][..],
        &["--report-unused-suppressions", "--report-unused-suppressions-severity", "warn"],
    ] {
        let args = [&["lint", "--rules"][..], flags].concat();
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
        assert!(stderr(&output).contains("cannot be used at the same time"), "{args:?}");
    }
    assert_eq!(std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(), source);
    assert!(!temp.dir.join("robusta-suppressions.json").exists());
}

#[test]
fn lint_rules_lists_with_accepted_fix_and_source_flags_without_touching_files() {
    let source = "class Example { long value = 10l; }\n";
    let temp = TempProject::new();
    temp.write("Example.java", source);
    for flags in [
        &["--fix", "--unsafe-fixes", "Example.java"][..],
        &["--unsafe-fixes", "--diff", "Example.java"],
        &["--suppress-all", "Example.java"],
        &["--prune-suppressions", "--fix"],
        &["--stdin-filepath", "Virtual.java", "--fix"],
    ] {
        let args = [&["lint", "--rules", "--reporter", "json"][..], flags].concat();
        let output = robusta_with_stdin(&args, &temp.dir, source);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
        assert!(output.stderr.is_empty(), "{args:?}: {}", stderr(&output));
        let rows: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(rows.len(), builtin_rules().len(), "{args:?}");
    }
    assert_eq!(std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(), source);
    assert!(!temp.dir.join("robusta-suppressions.json").exists());
}

#[test]
fn lint_reports_configured_rules_with_rule_ids_and_severities() {
    let source = "class Example {\n    static public long value = 10l;\n}\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint"], &temp.file);
    let report = stdout_without_summary(&output);

    assert_eq!(output.status.code(), Some(1), "{report}");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(redact_dir(&report, &temp.dir), @"

     ! modifier-order: modifiers are not in the canonical order
      ,-[<temp>/Example.java:2:5]
    1 | class Example {
    2 |     static public long value = 10l;
      :     ^^^^^^|^^^^^^||
      :           |      |`-- `public` should appear earlier
      :           |      `-- `static` currently appears first
      :           `-- reorder this modifier run
    3 | }
      `----
     help: Reorder the modifiers into Java's customary sequence.

     x uppercase-long-suffix: use uppercase `L` for the long literal suffix
      ,-[<temp>/Example.java:2:34]
    1 | class Example {
    2 |     static public long value = 10l;
      :                                  ^
    3 | }
      `----
     help: Replace the lowercase suffix with `L`.
    ");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn lint_respects_source_suppressions_and_disabled_rules() {
    let source = "class Example {\n    @SuppressWarnings(\"robusta:modifier-order\")\n    static public long first = 10l;\n    static public long second = 20l;\n}\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: allow\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint"], &temp.file);
    let report = stdout_without_summary(&output);

    assert_eq!(output.status.code(), Some(0), "{report}");
    insta::assert_snapshot!(redact_dir(&report, &temp.dir), @"

     ! modifier-order: modifiers are not in the canonical order
      ,-[<temp>/Example.java:4:5]
    3 |     static public long first = 10l;
    4 |     static public long second = 20l;
      :     ^^^^^^|^^^^^^||
      :           |      |`-- `public` should appear earlier
      :           |      `-- `static` currently appears first
      :           `-- reorder this modifier run
    5 | }
      `----
     help: Reorder the modifiers into Java's customary sequence.
    ");
}

#[test]
fn lint_defaults_allow_style_findings() {
    let source = "static public class Example { long value = 10l; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["lint"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn lint_fix_applies_enabled_safe_fixes_without_formatting() {
    let source = "; static public class Example { long value = 10l; } ;\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    redundant-semicolons: warn\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    // Exact comparison: the fix output begins and ends in whitespace that an
    // inline snapshot cannot pin.
    assert_eq!(
        std::fs::read_to_string(&temp.file).unwrap(),
        " public static class Example { long value = 10L; } \n"
    );
}

#[test]
fn lint_fix_keeps_an_import_used_by_markdown_javadoc() {
    let source = concat!(
        "import java.util.List;\n",
        "/// A demo referencing [List].\n",
        "class Example {}\n",
    );
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "lint:\n  rules:\n    unused-import: warn\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn main_config_unused_suppression_severity_and_flags_follow_precedence() {
    let temp = TempProject::new();
    temp.write("Example.java", STALE_SUPPRESSION);
    for configured in ["allow", "warn", "deny"] {
        temp.write(
            "robusta.yaml",
            &format!("lint:\n  options:\n    report-unused-suppressions: {configured}\n"),
        );
        for command in ["lint", "check"] {
            for (flags, severity) in [
                (vec![], configured),
                (vec!["--report-unused-suppressions"], "warn"),
                (vec!["--report-unused-suppressions-severity", "allow"], "allow"),
                (vec!["--report-unused-suppressions-severity", "warn"], "warn"),
                (vec!["--report-unused-suppressions-severity", "deny"], "deny"),
            ] {
                let mut args = vec![command, "--reporter", "json"];
                args.extend(flags);
                args.push("Example.java");
                let output = robusta_in_dir(&args, &temp.dir);
                assert_eq!(
                    output.status.code(),
                    Some(i32::from(severity == "deny")),
                    "{configured} {args:?}: {}",
                    stderr(&output)
                );
                assert!(output.stderr.is_empty(), "{}", stderr(&output));
                let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                let findings = report["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|file| file["diagnostics"].as_array().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(
                    findings.len(),
                    usize::from(severity != "allow"),
                    "{configured} {args:?}: {report}"
                );
                if severity != "allow" {
                    assert_eq!(
                        findings[0]["severity"],
                        if severity == "warn" { "warning" } else { "error" }
                    );
                }
            }
        }
    }
}

#[test]
fn unused_suppression_severity_accepts_each_spelling() {
    let temp = TempJava::stale_suppression("");

    for (severity, code, marker) in [
        ("warn", 0, Some("! unused-suppression")),
        ("deny", 1, Some("x unused-suppression")),
        ("allow", 0, None),
    ] {
        for command in ["lint", "check"] {
            let output = robusta_with_path(
                &[command, "--report-unused-suppressions-severity", severity],
                &temp.file,
            );
            assert_eq!(
                output.status.code(),
                Some(code),
                "{command} {severity}: {}",
                stderr(&output)
            );
            let reported = stdout_without_summary(&output);
            match marker {
                Some(marker) => {
                    assert!(reported.contains(marker), "{command} {severity}: {reported}");
                }
                None => assert!(reported.is_empty(), "{command} {severity}: {reported}"),
            }
        }
    }
}

#[test]
fn unused_suppression_flags_reject_invalid_values_and_each_other() {
    let source = "class Example {\n  int clean; // robusta-disable-line\n}\n";
    let temp = TempJava::new(source);

    for args in [
        &["lint", "--report-unused-suppressions-severity", "off"][..],
        &["check", "--report-unused-suppressions-severity", "error"][..],
        &["lint", "--report-unused-suppressions-severity", "info"][..],
        &["check", "--report-unused-suppressions-severity", "2"][..],
        &["lint", "--report-unused-suppressions", "--report-unused-suppressions-severity", "warn"]
            [..],
        &[
            "check",
            "--report-unused-suppressions",
            "--report-unused-suppressions-severity",
            "allow",
        ][..],
        &["format", "--report-unused-suppressions"][..],
        &["format", "--report-unused-suppressions-severity", "warn"][..],
    ] {
        let output = robusta_with_path(args, &temp.file);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(stderr(&output).contains("Error:"), "{args:?}: {}", stderr(&output));
    }
    let invalid =
        robusta_with_path(&["lint", "--report-unused-suppressions-severity", "info"], &temp.file);
    assert!(
        stderr(&invalid).contains("expected `allow`, `warn`, or `deny`"),
        "{}",
        stderr(&invalid)
    );
}

#[test]
fn unused_suppression_options_are_main_only_and_apply_to_nested_files() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  options:\n    report-unused-suppressions: deny\n");
    temp.write("Root.java", STALE_SUPPRESSION);
    temp.write("nested/robusta.yaml", "lint:\n  rules: {}\n");
    temp.write("nested/Nested.java", STALE_SUPPRESSION);
    let output = robusta_in_dir(&["lint", "."], &temp.dir);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(run_summary(&output).contains("2 errors"), "{}", stderr(&output));
    for severity in ["allow", "warn", "deny"] {
        temp.write(
            "nested/robusta.yaml",
            &format!("lint:\n  options:\n    report-unused-suppressions: {severity}\n"),
        );
        for flags in [vec![], vec!["--report-unused-suppressions-severity", "allow"]] {
            let mut args = vec!["lint"];
            args.extend(flags);
            args.push(".");
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
            assert_eq!(output.stdout.len(), 0);
            let message = stderr(&output);
            assert!(
                message.contains("nested")
                    && message.contains("robusta.yaml")
                    && message.contains("move the option to the main configuration"),
                "{message}"
            );
        }
        for flags in [vec!["--no-nested-config"], vec!["--config", "robusta.yaml"]] {
            let mut args = vec!["lint"];
            args.extend(flags);
            args.push(".");
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(1), "{args:?}: {}", stderr(&output));
            assert!(run_summary(&output).contains("2 errors"), "{}", stderr(&output));
        }
        let nested = temp.dir.join("nested");
        let own = robusta_in_dir(&["lint", "."], &nested);
        assert_eq!(own.status.code(), Some(i32::from(severity == "deny")), "{}", stderr(&own));
    }
    let show = robusta_in_dir(&["config"], &temp.dir);
    assert_eq!(show.status.code(), Some(0));
    assert!(
        stdout(&show).contains(
            "  options:\n    deny-warnings: false\n    report-unused-suppressions: deny\n"
        ),
        "{}",
        stdout(&show)
    );
    let stdin = robusta_with_stdin(
        &["lint", "--no-nested-config", "--stdin-filepath", "nested/Virtual.java"],
        &temp.dir,
        STALE_SUPPRESSION,
    );
    assert_eq!(stdin.status.code(), Some(1), "{}", stderr(&stdin));
    assert!(stdout(&stdin).contains("x unused-suppression"));
}

#[test]
fn unused_suppression_warnings_follow_the_warning_policy() {
    let temp = TempJava::stale_suppression("");

    for command in ["lint", "check"] {
        for (extra, code, shown) in [
            (&[][..], 0, true),
            (&["--deny-warnings"][..], 1, true),
            (&["--max-warnings", "0"][..], 1, true),
            (&["--max-warnings", "1"][..], 0, true),
            (&["--quiet"][..], 0, false),
        ] {
            let mut args = vec![command, "--report-unused-suppressions-severity", "warn"];
            args.extend_from_slice(extra);
            let output = robusta_with_path(&args, &temp.file);
            assert_eq!(output.status.code(), Some(code), "{args:?}: {}", stderr(&output));
            assert_eq!(
                stdout(&output).contains("unused-suppression"),
                shown,
                "{args:?}: {}",
                stdout(&output)
            );
        }
        // `--quiet` hides warnings, never errors.
        let quiet_error = robusta_with_path(
            &[command, "--report-unused-suppressions-severity", "deny", "--quiet"],
            &temp.file,
        );
        assert_eq!(quiet_error.status.code(), Some(1), "{}", stderr(&quiet_error));
        assert!(stdout(&quiet_error).contains("x unused-suppression"), "{}", stdout(&quiet_error));
    }
}

#[test]
fn lint_reports_only_suppressions_that_match_no_finding() {
    let source = r#"class Example {
    @SuppressWarnings("robusta:modifier-order")
    static public int used;
    @SuppressWarnings("robusta:uppercase-long-suffix")
    int stale;
    long matched = 10l; // robusta-disable-line
    int unused; // robusta-disable-line -- explanation
}
"#;
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--report-unused-suppressions"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(redact_dir(&stdout_without_summary(&output), &temp.dir), @r#"

     ! unused-suppression: unused suppression for rule `uppercase-long-suffix`
      ,-[<temp>/Example.java:4:23]
    3 |     static public int used;
    4 |     @SuppressWarnings("robusta:uppercase-long-suffix")
      :                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    5 |     int stale;
      `----
     help: Remove this unused suppression.

     ! unused-suppression: unused suppression
      ,-[<temp>/Example.java:7:17]
    6 |     long matched = 10l; // robusta-disable-line
    7 |     int unused; // robusta-disable-line -- explanation
      :                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    8 | }
      `----
     help: Remove this unused suppression.
    "#);

    let denied =
        robusta_with_path(&["lint", "--report-unused-suppressions", "--deny-warnings"], &temp.file);
    assert_eq!(denied.status.code(), Some(1), "{}", stdout(&denied));
}

/// Removing an unused suppression is a safe fix that the audit offers, so
/// only an audited run admitting safe fixes applies it, and a preview shows
/// the edit a write makes.
#[test]
fn unused_suppressions_are_removed_as_safe_fixes() {
    let source = r#"class Example {
    @SuppressWarnings("robusta:uppercase-long-suffix")
    long used = 1l;
    @SuppressWarnings({"robusta:uppercase-long-suffix", "unchecked"})
    int stale;
    @SuppressWarnings("robusta:modifier-order")
    int alsoStale;
}
"#;
    let pruned = r#"class Example {
    @SuppressWarnings("robusta:uppercase-long-suffix")
    long used = 1l;
    @SuppressWarnings({"unchecked"})
    int stale;
    int alsoStale;
}
"#;
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n    modifier-order: warn\n",
    )
    .expect("failed to write config");

    for args in [
        &["lint", "--fix"][..],
        &["lint", "--report-unused-suppressions-severity", "allow", "--fix", "--unsafe-fixes"][..],
    ] {
        let output = robusta_with_path(args, &temp.file);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source, "{args:?}");
    }

    let diff = robusta_with_path(&["lint", "--report-unused-suppressions", "--diff"], &temp.file);
    assert_eq!(diff.status.code(), Some(1), "{}", stderr(&diff));
    assert_eq!(candidate_from_diff(&stdout(&diff)), pruned);
    let list = robusta_with_path(
        &["lint", "--report-unused-suppressions", "--list-different"],
        &temp.file,
    );
    assert_eq!(stdout(&list), format!("{}\n", presentation_path(&temp.file)));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);

    for args in [
        &["lint", "--report-unused-suppressions", "--fix"][..],
        &["lint", "--report-unused-suppressions", "--fix", "--unsafe-fixes"][..],
    ] {
        std::fs::write(&temp.file, source).expect("failed to reset source");
        let write = robusta_with_path(args, &temp.file);
        assert_eq!(write.status.code(), Some(0), "{args:?}: {}", stdout(&write));
        assert!(!stdout(&write).contains("unused-suppression"), "{args:?}: {}", stdout(&write));
        assert!(write.stderr.is_empty(), "{args:?}: {}", stderr(&write));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), pruned, "{args:?}");

        let again = robusta_with_path(args, &temp.file);
        assert_eq!(again.status.code(), Some(0), "{args:?}: {}", stderr(&again));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), pruned, "{args:?}");
    }
}

#[test]
fn lint_unused_suppression_audit_is_silent_when_every_suppression_matches() {
    let source = "class Example { @SuppressWarnings(\"robusta:modifier-order\") static public int value; }\n";
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "lint:\n  rules:\n    modifier-order: warn\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--report-unused-suppressions"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

/// `--unsafe-fixes` widens what `--fix` writes and what a preview shows.
#[test]
fn lint_unsafe_fixes_widen_writes_and_previews() {
    // One rule, three arms: `a` is a safe delete, and `b` and `c` are unsafe.
    const SOURCE: &str = "class Example { void f(int[] data) { int a; int b = send(); int c = send() + 1; }\n\
         int send() { return 1; } }\n";
    const SAFE: &str = "class Example { void f(int[] data) {  int b = send(); int c = send() + 1; }\n\
         int send() { return 1; } }\n";
    const UNSAFE: &str = "class Example { void f(int[] data) {  send();  }\n\
         int send() { return 1; } }\n";
    const RULES: [&str; 4] = ["-A", "all", "-D", "unused-local-variable"];

    for (flags, code, expected) in
        [(&["--fix"][..], 1, SAFE), (&["--fix", "--unsafe-fixes"][..], 0, UNSAFE)]
    {
        let temp = TempJava::new(SOURCE);
        let args = [&["lint"][..], &RULES, flags].concat();
        let output = robusta_with_path(&args, &temp.file);
        assert_eq!(output.status.code(), Some(code), "{flags:?}: {}", stderr(&output));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), expected, "{flags:?}");
    }

    let temp = TempJava::new(SOURCE);
    for (flags, expected) in [(&["--diff"][..], SAFE), (&["--diff", "--unsafe-fixes"][..], UNSAFE)]
    {
        let args = [&["lint"][..], &RULES, flags].concat();
        let diff = robusta_with_path(&args, &temp.file);
        assert_eq!(diff.status.code(), Some(1), "{flags:?}: {}", stderr(&diff));
        assert_eq!(candidate_from_diff(&stdout(&diff)), expected, "{flags:?}");
    }
    let args = [&["lint"][..], &RULES, &["--list-different", "--unsafe-fixes"]].concat();
    let list = robusta_with_path(&args, &temp.file);
    assert_eq!(stdout(&list), format!("{}\n", presentation_path(&temp.file)));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), SOURCE);
}

#[test]
fn lint_fix_does_not_rewrite_files_that_fail_to_parse() {
    let source = "class Example { long value = ; }\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["lint", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert!(stdout(&output).contains("expected"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn lint_preview_and_write_use_the_same_safe_fix_candidate() {
    let source = "class Example {\n    long value = 10l;\n}\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    uppercase-long-suffix: warn\n",
    )
    .expect("failed to write config");

    let diff = robusta_with_path(&["lint", "--diff"], &temp.file);
    assert_eq!(diff.status.code(), Some(1), "{}", stderr(&diff));
    assert_eq!(stderr_without_summary(&diff), "");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    let candidate = candidate_from_diff(&stdout(&diff));

    let list = robusta_with_path(&["lint", "--list-different"], &temp.file);
    assert_eq!(list.status.code(), Some(1), "{}", stderr(&list));
    assert_eq!(stdout(&list), format!("{}\n", presentation_path(&temp.file)));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);

    let write = robusta_with_path(&["lint", "--fix"], &temp.file);
    assert_eq!(write.status.code(), Some(0), "{}", stderr(&write));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), candidate);
}

#[test]
fn coordinated_unused_local_fixes_match_stdin_and_disk_tiers() {
    const SOURCE: &str = "class Example { void f() { int unused = 0; unused++; } }";
    const FIXED: &str = "class Example { void f() {   } }";
    for (flags, expected) in [(&["--fix"][..], SOURCE), (&["--fix", "--unsafe-fixes"], FIXED)] {
        let temp = TempJava::new(SOURCE);
        let rules = ["lint", "-A", "all", "-W", "unused-local-variable"];
        let args = [&rules[..], flags, &["--stdin-filepath", "Example.java"]].concat();
        let stdin = robusta_with_stdin(&args, &temp.dir, SOURCE);
        assert_eq!(stdin.status.code(), Some(0), "{}", stderr(&stdin));
        assert_eq!(stdout(&stdin), expected);
        let disk = robusta_with_path(&[&rules[..], flags].concat(), &temp.file);
        assert_eq!(disk.status.code(), Some(0), "{}", stderr(&disk));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), expected);
    }
}
