use crate::support::*;

#[test]
fn import_sections_reject_unknown_kinds() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "format:\n  import-order:\n    sections: [module, normal, static]\n",
    );

    let output = robusta_in_dir(&["config"], &temp.dir);
    let message = stderr(&output);

    assert_eq!(output.status.code(), Some(2), "{message}");
    assert_eq!(output.stdout.len(), 0);
    assert!(message.contains("unknown variant `normal`"), "{message}");
    assert!(message.contains("expected one of static, module, type"), "{message}");
}

#[test]
fn removed_wrapping_settings_are_unknown_fields() {
    let temp = TempProject::new();
    for value in ["{ lists: fill }", "{ declaration-clauses: vertical }", "null", "{}"] {
        temp.write("robusta.yaml", &format!("format:\n  wrapping: {value}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        let message = stderr(&output);
        assert_eq!(output.status.code(), Some(2), "{value}: {message}");
        assert_eq!(output.stdout.len(), 0);
        assert!(message.contains("unknown field `wrapping`"), "{message}");
    }
}

#[test]
fn removed_continuation_indent_width_is_an_unknown_field() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "format:\n  continuation-indent-width: 8\n");

    let output = robusta_in_dir(&["config"], &temp.dir);
    let message = stderr(&output);

    assert_eq!(output.status.code(), Some(2), "{message}");
    assert_eq!(output.stdout.len(), 0);
    assert!(message.contains("robusta.yaml"), "{message}");
    assert!(message.contains("unknown field `continuation-indent-width`"), "{message}");
    assert!(message.contains("line 2 column 3"), "{message}");
}

#[test]
fn removed_auto_line_ending_is_an_unknown_value() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "format:\n  line-ending: auto\n");

    let output = robusta_in_dir(&["config"], &temp.dir);
    let message = stderr(&output);

    assert_eq!(output.status.code(), Some(2), "{message}");
    assert_eq!(output.stdout.len(), 0);
    assert!(message.contains("robusta.yaml"), "{message}");
    assert!(message.contains("unknown variant `auto`"), "{message}");
    assert!(message.contains("line 2 column 16"), "{message}");
}

#[test]
fn description_with_dot_rejects_booleans() {
    let temp = TempProject::new();
    for value in ["true", "false"] {
        temp.write(
            "robusta.yaml",
            &format!("format:\n  javadoc:\n    description-with-dot: {value}\n"),
        );
        let output = robusta_in_dir(&["config"], &temp.dir);
        let message = stderr(&output);
        assert_eq!(output.status.code(), Some(2), "{value}: {message}");
        assert_eq!(output.stdout.len(), 0);
        assert!(message.contains("robusta.yaml"), "{message}");
        assert!(message.contains(&format!("unknown variant `{value}`")), "{message}");
        assert!(message.contains("line 3 column 27"), "{message}");
    }
}

#[test]
fn removed_brace_settings_are_unknown_fields() {
    let temp = TempProject::new();
    for key in ["brace-placement", "continuation-placement"] {
        for value in ["same-line", "next-line", "null"] {
            temp.write("robusta.yaml", &format!("format:\n  {key}: {value}\n"));
            let output = robusta_in_dir(&["config"], &temp.dir);
            let message = stderr(&output);
            assert_eq!(output.status.code(), Some(2), "{key}: {value}: {message}");
            assert_eq!(output.stdout.len(), 0);
            assert!(message.contains(&format!("unknown field `{key}`")), "{message}");
        }
    }
}

#[test]
fn yaml_empty_documents_use_defaults() {
    let temp = TempProject::new();
    let defaults = robusta_in_dir(&["config"], &temp.dir);
    assert_eq!(defaults.status.code(), Some(0), "{}", stderr(&defaults));
    for source in ["", "# Config\n# Defaults\n", "null\n", "~\n", "---\n"] {
        temp.write("robusta.yaml", source);
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{source:?}: {}", stderr(&output));
        assert_eq!(stdout(&output), stdout(&defaults), "{source:?}");
        assert_eq!(output.stderr.len(), 0);
    }
}

#[test]
fn yaml_boolean_and_string_scalars_keep_their_types() {
    let temp = TempProject::new();
    for value in ["true", "false", "True", "False", "TRUE", "FALSE"] {
        temp.write("robusta.yaml", &format!("format:\n  insert-final-newline: {value}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{value}: {}", stderr(&output));
        assert!(
            stdout(&output).contains(&format!(
                "insert-final-newline: {}\n",
                value.eq_ignore_ascii_case("true")
            ))
        );
    }
    for value in ["yes", "no", "on", "off"] {
        temp.write("robusta.yaml", &format!("ignore-patterns: [{value}]\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{value}: {}", stderr(&output));
        temp.write("robusta.yaml", &stdout(&output));
        let reparsed = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(reparsed.status.code(), Some(0), "{}", stderr(&reparsed));
        assert_eq!(stdout(&reparsed), stdout(&output));

        temp.write("robusta.yaml", &format!("format:\n  insert-final-newline: {value}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{value}: {}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
    }
}

#[test]
fn yaml_binary_tags_preserve_string_pattern_contents() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns: [!!binary Z2VuZXJhdGVkLw==]\n");
    temp.write("generated/A.java", "not Java\n");
    temp.write("Z2VuZXJhdGVkLw==/A.java", "not Java\n");

    let output = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "generated/A.java\n");
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn yaml_null_values_follow_each_fields_omission_policy() {
    let temp = TempProject::new();
    let defaults = robusta_in_dir(&["config"], &temp.dir);
    assert_eq!(defaults.status.code(), Some(0), "{}", stderr(&defaults));
    for value in ["null", "~", ""] {
        for section in ["format", "ignore-patterns", "lint"] {
            temp.write("robusta.yaml", &format!("{section}: {value}\n"));
            let output = robusta_in_dir(&["config"], &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{section}: {value}: {}", stderr(&output));
        }
        for key in [
            "line-width",
            "indent-style",
            "indent-width",
            "after-closing-brace",
            "import-order",
            "javadoc",
            "line-ending",
            "insert-final-newline",
        ] {
            temp.write("robusta.yaml", &format!("format:\n  {key}: {value}\n"));
            let output = robusta_in_dir(&["config"], &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{key}: {value}: {}", stderr(&output));
        }
        for key in ["deny-warnings", "max-warnings", "report-unused-suppressions"] {
            temp.write("robusta.yaml", &format!("lint:\n  options:\n    {key}: {value}\n"));
            let output = robusta_in_dir(&["config"], &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{key}: {value}: {}", stderr(&output));
            assert_eq!(stdout(&output), stdout(&defaults), "{key}: {value}");
            assert_eq!(output.stderr.len(), 0);
            temp.write("robusta.yaml", &stdout(&output));
            let reparsed = robusta_in_dir(&["config"], &temp.dir);
            assert_eq!(reparsed.status.code(), Some(0), "{}", stderr(&reparsed));
            assert_eq!(stdout(&reparsed), stdout(&output));
        }
        temp.write("robusta.yaml", &format!("lint:\n  options: {value}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "options: {value}: {}", stderr(&output));
        assert_eq!(stdout(&output), stdout(&defaults), "options: {value}");
        for (section, key) in [("rules", "unused-import"), ("categories", "correctness")] {
            temp.write("robusta.yaml", &format!("lint:\n  {section}:\n    {key}: {value}\n"));
            let output = robusta_in_dir(&["config"], &temp.dir);
            assert_eq!(output.status.code(), Some(2), "{section}: {value}: {}", stderr(&output));
            assert_eq!(output.stdout.len(), 0);
        }
    }
}

#[test]
fn null_run_options_in_nested_configs_preserve_the_main_policy() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  options:\n    deny-warnings: true\n    max-warnings: 0\n    report-unused-suppressions: warn\n",
    );
    temp.write(
        "nested/Example.java",
        "class Example {\n    int clean; // robusta-disable-line\n}\n",
    );
    for value in ["null", "~", ""] {
        for key in ["deny-warnings", "max-warnings", "report-unused-suppressions"] {
            temp.write("nested/robusta.yaml", &format!("lint:\n  options:\n    {key}: {value}\n"));
            let output = robusta_in_dir(&["lint", "."], &temp.dir);
            assert_eq!(output.status.code(), Some(1), "{key}: {value}: {}", stderr(&output));
            assert!(stdout(&output).contains("unused-suppression"), "{}", stdout(&output));
            assert!(run_summary(&output).contains("1 warning"), "{}", stderr(&output));
        }
    }
}

#[test]
fn yaml_parse_errors_name_the_file_line_and_column() {
    let temp = TempProject::new();
    for source in ["format:\n  unknown: true\n", "format:\n  line-width: wrong\n", "format: [\n"] {
        temp.write("robusta.yaml", source);
        let output = robusta_in_dir(&["config"], &temp.dir);
        let message = stderr(&output);
        assert_eq!(output.status.code(), Some(2), "{message}");
        assert_eq!(output.stdout.len(), 0);
        assert!(message.contains("robusta.yaml"), "{message}");
        assert!(message.contains("failed to parse config"), "{message}");
        assert!(message.contains("line "), "{message}");
        assert!(message.contains("column "), "{message}");
    }
}

// Help/usage text embeds the argv[0] program name (`robusta.exe` on Windows)
// and its fixed-width wrapping, so this exact snapshot is asserted only on the
// primary platform.
#[cfg(not(windows))]
#[test]
fn config_help_defines_its_resolution_and_listing_contract() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["config", "--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    Print the resolved configuration for the current directory as YAML.

    Usage: robusta config [-c=PATH] [--no-nested-config] [--color=WHEN] [--files [--ignore-path=PATH]...
    [--ignore-pattern=PAT]... [PATH]...]

    Configuration options:
        -c, --config=PATH         Use this configuration file instead of discovering `robusta.yaml`.
            --no-nested-config    Use one configuration discovered from the current directory.

    Color options:
            --color=WHEN          When to use colors: auto, always, or never.

    Ignore options:
            --ignore-path=PATH    Path to an additional gitignore-syntax file. Can be specified multiple
                                  times.
            --ignore-pattern=PAT  Gitignore-syntax pattern to exclude, resolved from the working
                                  directory. Quote it so the shell does not expand it. These patterns
                                  are ordered and outrank `--ignore-path` files: the last one to match
                                  decides, and a leading `!` reincludes. Can be specified multiple
                                  times.

    Available positional items:
        PATH                      Java source file or directory to process. Defaults to the current
                                  directory. Paths must not contain a `..` component.

    Available options:
            --files               Print the paths `check` would process instead of the configuration.
        -h, --help                Prints help information
    ");
    assert_eq!(output.stderr.len(), 0);
}

// Primary-platform-only: see `config_help_defines_its_resolution_and_listing_contract`.
#[cfg(not(windows))]
#[test]
fn init_help_defines_its_refusal_contract() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["init", "--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    Create `robusta.yaml` in the current directory, failing if it exists.

    Usage: robusta init [--color=WHEN]

    Color options:
            --color=WHEN  When to use colors: auto, always, or never.

    Available options:
        -h, --help        Prints help information
    ");
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn init_writes_a_parseable_default_config_without_overwriting() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["init"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "Created robusta.yaml\n");
    assert_eq!(output.stderr.len(), 0);
    let scaffold_path = temp.dir.join("robusta.yaml");
    let scaffold = std::fs::read_to_string(&scaffold_path).expect("scaffold should be readable");
    assert_eq!(
        scaffold.lines().next(),
        Some(concat!(
            "# yaml-language-server: $schema=https://raw.githubusercontent.com/robusta-rs/robusta/v",
            env!("CARGO_PKG_VERSION"),
            "/robusta.schema.json"
        ))
    );
    assert!(scaffold.contains("line-width: 100"));
    assert!(scaffold.contains("indent-style: space"));
    assert!(scaffold.contains("indent-width: 2"));
    assert!(scaffold.contains("after-closing-brace: same-line"));
    assert!(!scaffold.contains("wrapping"));
    assert!(scaffold.contains("javadoc: false"));
    assert!(scaffold.contains("line-ending: lf"));
    assert!(scaffold.contains("insert-final-newline: true"));
    assert!(scaffold.contains("rules: {}"));

    let resolved = robusta_in_dir(&["config", "--config", "robusta.yaml"], &temp.dir);
    assert_eq!(resolved.status.code(), Some(0), "{}", stderr(&resolved));
    let defaults = TempProject::new();
    let default_resolved = robusta_in_dir(&["config"], &defaults.dir);
    assert_eq!(default_resolved.status.code(), Some(0), "{}", stderr(&default_resolved));
    assert_eq!(stdout(&resolved), stdout(&default_resolved));

    let second = robusta_in_dir(&["init"], &temp.dir);
    assert_eq!(second.status.code(), Some(2));
    assert_eq!(second.stdout.len(), 0);
    assert_eq!(stderr(&second), "robusta.yaml already exists\n");
    assert_eq!(
        std::fs::read_to_string(scaffold_path).expect("scaffold should remain readable"),
        scaffold
    );
}

#[test]
fn init_refuses_existing_files_without_changing_bytes() {
    for contents in [b"".as_slice(), b"invalid: [\r\n\xff\x00"] {
        let temp = TempProject::new();
        let path = temp.dir.join("robusta.yaml");
        std::fs::write(&path, contents).unwrap();

        let output = robusta_in_dir(&["init"], &temp.dir);

        assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
        assert_eq!(stderr(&output), "robusta.yaml already exists\n");
        assert_eq!(std::fs::read(path).unwrap(), contents);
    }
}

#[test]
fn init_rejects_force_without_writing() {
    for contents in [None, Some("invalid: [\n")] {
        let temp = TempProject::new();
        if let Some(contents) = contents {
            temp.write("robusta.yaml", contents);
        }

        let output = robusta_in_dir(&["init", "--force"], &temp.dir);

        assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr(&output).contains("--force"), "{}", stderr(&output));
        assert!(stderr(&output).contains("not expected"), "{}", stderr(&output));
        let path = temp.dir.join("robusta.yaml");
        if let Some(contents) = contents {
            assert_eq!(std::fs::read(path).unwrap(), contents.as_bytes());
        } else {
            assert!(!path.exists());
        }
    }
}

#[test]
fn config_emits_round_trip_yaml_without_analyzing_source() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "format:\n  line-width: 120\n");
    temp.write(
        "nested/robusta.yaml",
        r#"
format:
  line-width: 88
  javadoc:
    separate-tag-groups: before-return
  import-order:
    groups:
      - ["org", "com"]
  ignore-patterns: ["vendored/"]
ignore-patterns: ["generated/", "!generated/Keep.java"]
lint:
  ignore-patterns: ["legacy/", "!legacy/Keep.java"]
  options:
    max-warnings: 3
  rules:
    modifier-order: warn
    uppercase-long-suffix: deny
"#,
    );
    temp.write("nested/Broken.java", "not java\n");

    let root = robusta_in_dir(&["config"], &temp.dir);
    assert_eq!(root.status.code(), Some(0), "{}", stderr(&root));
    assert!(stdout(&root).contains("line-width: 120\n"));
    assert_eq!(root.stderr.len(), 0);

    let nested = temp.dir.join("nested");
    let show = robusta_in_dir(&["config"], &nested);
    let printed = stdout(&show);

    assert_eq!(show.status.code(), Some(0), "{}", stderr(&show));
    insta::assert_snapshot!(printed, @r#"
    format:
      line-width: 88
      indent-style: space
      indent-width: 2
      after-closing-brace: same-line
      import-order:
        sections:
        - module
        - type
        - static
        groups:
        - - org
          - com
        - - "*"
      javadoc:
        capitalize-descriptions: false
        description-with-dot: never
        comment-line-strategy: single-line
        line-wrapping-style: greedy
        separate-tag-groups: before-return
        paragraph-html-tags: false
      line-ending: lf
      insert-final-newline: true
      overrides: []
      ignore-patterns:
      - vendored/
    ignore-patterns:
    - generated/
    - "!generated/Keep.java"
    lint:
      categories: {}
      rules:
        broken-null-check: allow
        duplicate-import: allow
        modifier-order: warn
        prefer-expression-lambda: allow
        redundant-lambda-parameter-parentheses: allow
        redundant-return: allow
        redundant-semicolons: allow
        redundant-super-call: allow
        unused-import: warn
        unused-local-variable: warn
        uppercase-long-suffix: deny
      overrides: []
      options:
        deny-warnings: false
        max-warnings: 3
        report-unused-suppressions: allow
      ignore-patterns:
      - legacy/
      - "!legacy/Keep.java"
    "#);
    assert_eq!(show.stderr.len(), 0);

    let single_discovery = robusta_in_dir(&["config", "--no-nested-config"], &nested);
    assert_eq!(single_discovery.status.code(), Some(0), "{}", stderr(&single_discovery));
    assert_eq!(stdout(&single_discovery), printed);
    assert_eq!(single_discovery.stderr.len(), 0);

    temp.write("policy/resolved.yaml", &printed);
    let explicit = robusta_in_dir(&["config", "--config", "policy/resolved.yaml"], &temp.dir);

    assert_eq!(explicit.status.code(), Some(0), "{}", stderr(&explicit));
    assert_eq!(stdout(&explicit), printed);
    assert_eq!(explicit.stderr.len(), 0);
}

#[test]
fn config_materializes_editorconfig_fallbacks_for_the_current_directory() {
    let temp = TempProject::new();
    temp.write(
        ".editorconfig",
        "root = true\n\n[*]\nmax_line_length = 88\nindent_style = tab\ntab_width = 6\nend_of_line = crlf\ninsert_final_newline = false\n\n[*.java]\nmax_line_length = 60\n",
    );
    temp.write("Example.java", "not parsed by config\n");

    let output = robusta_in_dir(&["config"], &temp.dir);
    let printed = stdout(&output);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    assert!(printed.contains("line-width: 88\n"), "{printed}");
    assert!(printed.contains("indent-style: tab\n"), "{printed}");
    assert!(printed.contains("indent-width: 6\n"), "{printed}");
    assert!(printed.contains("line-ending: crlf\n"), "{printed}");
    assert!(printed.contains("insert-final-newline: false\n"), "{printed}");
}

#[test]
fn config_reports_an_invalid_ignore_pattern() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - \"src/**/*.[java\"\n");

    let output = robusta_in_dir(&["config"], &temp.dir);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"), "{stderr}");
    assert!(stderr.contains("invalid ignore pattern \"src/**/*.[java\""), "{stderr}");
}

#[test]
fn config_rejects_paths_without_files() {
    let temp = TempProject::new();
    temp.write("nested/Example.java", "not Java\n");

    for path in [".", "nested", "nested/Example.java", "missing.java"] {
        let output = robusta_in_dir(&["config", path], &temp.dir);

        assert_eq!(output.status.code(), Some(2), "{path}: {}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr(&output).contains("expected `--files`"), "{path}: {}", stderr(&output));
    }
}

#[test]
fn no_nested_config_uses_the_current_directory_config_for_the_whole_tree() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "format:\n  indent-width: 3\n");
    temp.write("project/robusta.yaml", "format:\n  indent-width: 2\n");
    temp.write("project/Root.java", "class Root { int value; }\n");
    temp.write("project/nested/robusta.yaml", "format:\n  indent-width: 6\n");
    temp.write("project/nested/Nested.java", "class Nested { int value; }\n");

    let default = robusta_in_dir(&["format", "--fix", "project"], &temp.dir);
    assert_eq!(default.status.code(), Some(0), "{}", stdout(&default));
    assert!(default.stderr.is_empty(), "{}", stderr(&default));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("project/Root.java")).unwrap(),
        "class Root {\n  int value;\n}\n"
    );
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("project/nested/Nested.java")).unwrap(),
        "class Nested {\n      int value;\n}\n"
    );
    assert_eq!(stdout_without_summary(&default), "");

    temp.write("project/Root.java", "class Root { int value; }\n");
    temp.write("project/nested/Nested.java", "class Nested { int value; }\n");
    let unnested = robusta_in_dir(&["format", "--fix", "--no-nested-config", "project"], &temp.dir);
    assert_eq!(unnested.status.code(), Some(0), "{}", stdout(&unnested));
    assert!(unnested.stderr.is_empty(), "{}", stderr(&unnested));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("project/Root.java")).unwrap(),
        "class Root {\n   int value;\n}\n"
    );
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("project/nested/Nested.java")).unwrap(),
        "class Nested {\n   int value;\n}\n"
    );
    assert_eq!(stdout_without_summary(&unnested), "");
}

/// A configuration below the selected paths never becomes the single
/// configuration: without one in the current directory the run uses the
/// built-in defaults, and an explicit `--config` still takes precedence.
#[test]
fn no_nested_config_ignores_a_config_under_the_first_path() {
    const RULE: &str = "x uppercase-long-suffix:";
    let temp = TempProject::new();
    temp.write("a/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("a/D.java", "class D { long n = 10l; }\n");
    temp.write("a/b/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: allow\n");
    temp.write("a/b/E.java", "class E { long n = 10l; }\n");

    for (args, code, errors) in [
        (&["lint", "a"][..], 1, 1),
        (&["lint", "--no-nested-config", "a"][..], 0, 0),
        (&["lint", "--no-nested-config"][..], 0, 0),
        (&["lint", "--no-nested-config", "--config", "a/robusta.yaml", "a"][..], 1, 2),
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        let report = stdout_without_summary(&output);
        assert_eq!(output.status.code(), Some(code), "{args:?}: {report}");
        assert_eq!(report.matches(RULE).count(), errors, "{args:?}: {report}");
    }

    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let output = robusta_in_dir(&["lint", "--no-nested-config", "a"], &temp.dir);
    let report = stdout_without_summary(&output);
    assert_eq!(output.status.code(), Some(1), "{report}");
    assert_eq!(report.matches(RULE).count(), 2, "{report}");
}

#[test]
fn config_rejects_removed_blank_lines() {
    let temp = TempProject::new();
    for value in
        ["{top-level: preserve}", "{type-members: compact}", "{type-members: separate}", "null"]
    {
        temp.write("robusta.yaml", &format!("format:\n  blank-lines: {value}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{value}: {}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr(&output).contains("unknown field `blank-lines`"), "{}", stderr(&output));
    }
}

const UNUSED_LOCAL: &str = "! unused-local-variable:";

/// The first statement is 110 columns wide and the second 130, so the width
/// in effect shows in which of them stay on one line.
fn override_source(name: &str) -> String {
    let a = format!("\"{}\" + \"{}\"", "a".repeat(38), "b".repeat(45));
    let b = format!("\"{}\" + \"{}\"", "c".repeat(48), "d".repeat(55));
    format!(
        "class {name} {{\n    void m() {{\n        int unused = 1;\n        String a = {a};\n        String b = {b};\n        System.out.println(a + b);\n    }}\n}}\n"
    )
}

/// How many of the two statements of `override_source` fit on one line.
fn joined_statements(path: &std::path::Path) -> usize {
    let source = std::fs::read_to_string(path).unwrap();
    source.lines().filter(|line| line.contains("= \"") && line.contains("\" + \"")).count()
}

#[test]
fn overrides_change_format_and_lint_settings_by_pattern() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        r#"format:
  overrides:
    - files: ["**/src/test/**"]
      options:
        line-width: 120
    - files: ["legacy/**"]
      exclude-files: ["legacy/api/**"]
      options:
        line-width: 140
lint:
  overrides:
    - files: ["**/src/test/**", "legacy/api/**"]
      rules:
        unused-local-variable: allow
"#,
    );
    let files = [
        ("module/src/test/T.java", 1, 0),
        ("legacy/core/L.java", 2, 1),
        ("legacy/api/A.java", 0, 0),
        ("src/main/M.java", 0, 1),
    ];
    for (path, _, _) in files {
        let name = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();
        temp.write(path, &override_source(name));
    }

    let format = robusta_in_dir(&["format", "--fix"], &temp.dir);
    assert_eq!(format.status.code(), Some(0), "{}", stderr(&format));
    let second = robusta_in_dir(&["format"], &temp.dir);
    assert_eq!(second.status.code(), Some(0), "{}", stdout(&second));
    let lint = robusta_in_dir(&["lint"], &temp.dir);
    let report = stdout_without_summary(&lint);
    for (path, joined, findings) in files {
        assert_eq!(joined_statements(&temp.dir.join(path)), joined, "{path}");
        assert_eq!(report.matches(&format!("{path}:")).count(), findings, "{path}: {report}");
    }
}

#[test]
fn overrides_win_over_editorconfig_and_rule_filters() {
    let temp = TempProject::new();
    temp.write(".editorconfig", "[*]\nmax_line_length = 80\n");
    temp.write(
        "robusta.yaml",
        r#"format:
  overrides:
    - files: ["*.java"]
      options:
        line-width: 140
    - files: ["Example.java"]
      options:
        line-width: 120
lint:
  overrides:
    - files: ["*.java"]
      rules:
        unused-local-variable: deny
    - files: ["Example.java"]
      rules:
        unused-local-variable: allow
"#,
    );
    temp.write("src/Example.java", &override_source("Example"));
    temp.write("src/Other.java", &override_source("Other"));

    let format = robusta_in_dir(&["format", "--fix"], &temp.dir);
    assert_eq!(format.status.code(), Some(0), "{}", stderr(&format));
    assert_eq!(joined_statements(&temp.dir.join("src/Example.java")), 1);
    assert_eq!(joined_statements(&temp.dir.join("src/Other.java")), 2);

    for command in ["lint", "check"] {
        for filter in ["-D", "-W", "-A"] {
            let lint = robusta_in_dir(&[command, filter, "unused-local-variable"], &temp.dir);
            let report = stdout_without_summary(&lint);
            assert_eq!(lint.status.code(), Some(1), "{command} {filter}: {report}");
            assert!(!report.contains("src/Example.java:"), "{command} {filter}: {report}");
            let denied = report.matches("x unused-local-variable:").count();
            assert_eq!(denied, 1, "{command} {filter}: {report}");
        }
    }
}

#[test]
fn overrides_come_from_the_configuration_each_file_uses() {
    let temp = TempProject::new();
    let allow = "lint:\n  overrides:\n    - files: [\"*.java\"]\n      rules:\n        unused-local-variable: allow\n";
    temp.write("robusta.yaml", allow);
    temp.write("Root.java", &override_source("Root"));
    temp.write("nested/robusta.yaml", "{}\n");
    temp.write("nested/Nested.java", &override_source("Nested"));
    temp.write("policy/robusta.yaml", "{}\n");

    for (args, findings) in [
        (&["lint"][..], 1),
        (&["lint", "--no-nested-config"][..], 0),
        (&["lint", "--config", "policy/robusta.yaml"][..], 2),
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        let report = stdout_without_summary(&output);
        assert_eq!(report.matches(UNUSED_LOCAL).count(), findings, "{args:?}: {report}");
    }

    temp.write("nested/robusta.yaml", allow);
    let output = robusta_in_dir(&["lint", "--config", "nested/robusta.yaml"], &temp.dir);
    let report = stdout_without_summary(&output);
    assert_eq!(report.matches(UNUSED_LOCAL).count(), 0, "{report}");
}

#[test]
fn override_errors_name_the_file_line_and_column() {
    let temp = TempProject::new();
    for (source, message) in [
        ("format:\n  overrides:\n    - options: {}\n", "missing field `files`"),
        ("lint:\n  overrides:\n    - rules: {}\n", "missing field `files`"),
        ("format:\n  overrides:\n    - files: []\n      rules: {}\n", "unknown field `rules`"),
        ("lint:\n  overrides:\n    - files: []\n      options: {}\n", "unknown field `options`"),
        (
            "format:\n  overrides:\n    - files: []\n      options:\n        ignore-patterns: []\n",
            "unknown field `ignore-patterns`",
        ),
        (
            "format:\n  overrides:\n    - files: []\n      options:\n        overrides: []\n",
            "unknown field `overrides`",
        ),
        (
            "lint:\n  overrides:\n    - files: []\n      categories:\n        style: warn\n",
            "unknown field `categories`",
        ),
        ("format:\n  overrides:\n    - files: [\"src/**/*.{java\"]\n", "invalid glob pattern"),
        ("lint:\n  overrides:\n    - files: [\"src/**/*.{java\"]\n", "invalid glob pattern"),
        ("overrides: []\n", "unknown field `overrides`"),
    ] {
        temp.write("robusta.yaml", source);
        let output = robusta_in_dir(&["config"], &temp.dir);
        let error = stderr(&output);
        assert_eq!(output.status.code(), Some(2), "{error}");
        assert!(error.contains("robusta.yaml: failed to parse config"), "{error}");
        assert!(error.contains(message), "{error}");
        assert!(error.contains("line ") && error.contains("column "), "{error}");
    }
}

#[test]
fn override_values_are_checked_as_their_section_checks_them() {
    let temp = TempProject::new();
    temp.write("A.java", "class A {}\n");
    for (source, message) in [
        (
            "format:\n  overrides:\n    - files: [\"*.java\"]\n      options:\n        line-width: 0\n",
            "invalid format.line-width",
        ),
        (
            "lint:\n  overrides:\n    - files: [\"*.java\"]\n      rules:\n        redundant-semicolon: warn\n",
            "unknown rule \"redundant-semicolon\"",
        ),
    ] {
        temp.write("robusta.yaml", source);
        for command in ["config", "check", "format", "lint"] {
            let output = robusta_in_dir(&[command], &temp.dir);
            let error = stderr(&output);
            assert_eq!(output.status.code(), Some(2), "{command}: {error}");
            assert!(error.contains(message), "{command}: {error}");
        }
    }
}
