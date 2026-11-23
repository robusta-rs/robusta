use std::path::Path;

use cow_utils::CowUtils;

use crate::support::*;

#[test]
fn every_file_command_exposes_selection_options() {
    let temp = TempProject::new();

    for command in ["check", "format", "lint"] {
        let output = robusta_in_dir(&[command, "--help"], &temp.dir);
        let help = stdout(&output);

        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        assert!(help.contains("-c, --config=PATH"), "{command}: {help}");
        assert!(help.contains("--no-nested-config"), "{command}: {help}");
        assert!(help.contains("--ignore-path=PATH"), "{command}: {help}");
        assert!(help.contains("--ignore-pattern=PAT"), "{command}: {help}");
        assert!(help.contains("--no-error-on-unmatched-pattern"), "{command}: {help}");
        assert!(output.stderr.is_empty(), "{command}: {}", stderr(&output));
    }
}

#[test]
fn parent_paths_fail_before_any_file_is_processed() {
    let temp = TempProject::new();
    let source = "class  Example { long value = 10l; }\n";
    let good = temp.write("work/Example.java", source);
    let outside = temp.write("Outside.java", source);
    temp.write("work/child/keep.txt", "");
    let work = temp.dir.join("work");
    let absolute = work.join("child").join("..").join("Example.java");

    for command in ["check", "format", "lint"] {
        for mode in [None, Some("--fix"), Some("--diff"), Some("--list-different")] {
            for invalid in
                ["..", "../Outside.java", "child/../Example.java", absolute.to_str().unwrap()]
            {
                for paths in [["Example.java", invalid], [invalid, "Example.java"]] {
                    let mut args = vec![command];
                    args.extend(mode);
                    args.extend(paths);
                    let output = robusta_in_dir(&args, &work);
                    assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
                    assert!(
                        stderr(&output)
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .contains("PATH must not contain a `..` component"),
                        "{args:?}: {}",
                        stderr(&output)
                    );
                    assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
                    assert!(
                        !stderr(&output).contains("Finished in"),
                        "{args:?}: {}",
                        stderr(&output)
                    );
                    assert_eq!(std::fs::read_to_string(&good).unwrap(), source, "{args:?}");
                    assert_eq!(std::fs::read_to_string(&outside).unwrap(), source, "{args:?}");
                }
            }
        }
    }
}

#[test]
fn source_paths_accept_current_directory_two_dots_in_names_and_absolute_paths() {
    let temp = TempProject::new();
    temp.write("work/Example.java", "class Example {}\n");
    temp.write("work/Example..java", "class Other {}\n");
    let outside = temp.write("Outside.java", "class Outside {}\n");
    let work = temp.dir.join("work");

    for command in ["check", "format", "lint"] {
        for paths in [
            vec![],
            vec!["."],
            vec!["./Example.java"],
            vec!["Example..java"],
            vec![outside.to_str().unwrap()],
        ] {
            let args = [&[command][..], &paths].concat();
            let output = robusta_in_dir(&args, &work);
            assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
            assert!(stdout_without_summary(&output).is_empty(), "{args:?}: {}", stdout(&output));
            assert!(output.stderr.is_empty(), "{}", stderr(&output));
        }
    }
}

#[test]
fn every_spelling_of_a_selected_file_displays_its_working_directory_relative_path() {
    let temp = TempProject::new();
    temp.write("src/A.java", "class A{long n=1l;}\n");
    let absolute = working_dir_spelling(&temp.dir).join("src/A.java");

    for paths in [vec![], vec!["."], vec!["./src/A.java"], vec![absolute.to_str().unwrap()]] {
        let listed =
            robusta_in_dir(&[&["format", "--list-different"][..], &paths].concat(), &temp.dir);
        assert_eq!(stdout_without_summary(&listed), "src/A.java\n", "{paths:?}");

        let diff = robusta_in_dir(&[&["format", "--diff"][..], &paths].concat(), &temp.dir);
        assert!(
            stdout(&diff).starts_with("--- src/A.java\n+++ src/A.java\n"),
            "{paths:?}: {}",
            stdout(&diff)
        );
    }
}

#[test]
fn parent_components_remain_valid_in_config_ignore_and_stdin_paths() {
    let temp = TempProject::new();
    temp.write("settings.yaml", "ignore-patterns: []\n");
    temp.write("extra.ignore", "Broken.java\n");
    temp.write("work/Example.java", "class Example {}\n");
    temp.write("work/Broken.java", "not Java");
    let work = temp.dir.join("work");

    for command in ["check", "format", "lint"] {
        let output = robusta_in_dir(
            &[command, "--config", "../settings.yaml", "--ignore-path", "../extra.ignore", "."],
            &work,
        );
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
    }
    let output = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "../Virtual.java"],
        &work,
        "class Virtual {}\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "class Virtual {}\n");
    assert!(!temp.dir.join("Virtual.java").exists());
}

#[test]
fn format_without_paths_defaults_to_current_directory() {
    let temp = TempProject::new();
    let file = temp.write("Example.java", "class  Example { int value ; }\n");

    let output = robusta_in_dir(&["format", "--fix"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(file).unwrap(), "class Example {\n  int value;\n}\n");
}

#[test]
fn check_without_paths_matches_explicit_current_directory() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - generated/\n");
    temp.write("Example.java", "class  Example { int value ; }\n");
    temp.write("generated/Ignored.java", "class  Ignored { int value ; }\n");

    let implicit = robusta_in_dir(&["check"], &temp.dir);
    let explicit = robusta_in_dir(&["check", "."], &temp.dir);

    assert_eq!(implicit.status.code(), explicit.status.code());
    assert_eq!(stdout_without_summary(&implicit), stdout_without_summary(&explicit));
    assert_eq!(implicit.stderr, explicit.stderr);
    assert_eq!(implicit.status.code(), Some(1), "{}", stderr(&implicit));
}

#[test]
fn check_fix_without_paths_defaults_to_current_directory() {
    let temp = TempProject::new();
    let file = temp.write("Example.java", "class  Example { int value ; }\n");

    let output = robusta_in_dir(&["check", "--fix"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(file).unwrap(), "class Example {\n  int value;\n}\n");
}

#[test]
fn every_file_command_errors_when_no_files_are_selected_unless_opted_out() {
    const MESSAGE: &str = concat!(
        "Expected at least one target file. ",
        "All matched files may have been excluded by ignore rules.\n"
    );
    let temp = TempProject::new();

    for command in ["check", "format", "lint"] {
        let default = robusta_in_dir(&[command], &temp.dir);
        assert_eq!(default.status.code(), Some(2), "{command}: {}", stderr(&default));
        assert!(default.stdout.is_empty(), "{command}: {}", stdout(&default));
        assert_eq!(stderr(&default), MESSAGE, "{command}");

        let allowed = robusta_in_dir(
            &[command, "--no-error-on-unmatched-pattern", "--threads", "1"],
            &temp.dir,
        );
        assert_eq!(allowed.status.code(), Some(0), "{command}: {}", stderr(&allowed));
        assert_eq!(stdout_without_summary(&allowed), "", "{command}");
        assert_eq!(
            run_summary(&allowed),
            "Finished in <time> on 0 files using 1 threads.\n",
            "{command}"
        );
        assert!(allowed.stderr.is_empty(), "{command}: {}", stderr(&allowed));
    }
}

#[test]
fn unmatched_policy_uses_the_final_filtered_union() {
    let temp = TempProject::new();
    temp.write("empty/keep.txt", "not Java\n");
    temp.write("Example.java", "class Example {}\n");

    for command in ["check", "format", "lint"] {
        let output = robusta_in_dir(&[command, "empty", "Example.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        assert!(stdout_without_summary(&output).is_empty(), "{command}: {}", stdout(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
    }

    temp.write("robusta.yaml", "ignore-patterns:\n  - \"*.java\"\n");
    let excluded = robusta_in_dir(&["check", "."], &temp.dir);
    assert_eq!(excluded.status.code(), Some(2));
    assert_eq!(excluded.stdout.len(), 0);
    assert!(stderr(&excluded).contains("Expected at least one target file."));

    let allowed = robusta_in_dir(&["check", "--no-error-on-unmatched-pattern", "."], &temp.dir);
    assert_eq!(allowed.status.code(), Some(0), "{}", stderr(&allowed));
    assert_eq!(stdout_without_summary(&allowed), "");
    assert!(allowed.stderr.is_empty(), "{}", stderr(&allowed));
}

#[test]
fn unmatched_opt_out_prints_an_empty_document_or_nothing_per_reporter() {
    let temp = TempProject::new();
    let run = |args: &[&str]| {
        let args = [args, &["--no-error-on-unmatched-pattern"]].concat();
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
        assert!(output.stderr.is_empty(), "{args:?}: {}", stderr(&output));
        stdout(&output)
    };

    let check = run(&["check", "--reporter", "json"]);
    assert_eq!(run(&["format", "--reporter", "json"]), check);
    insta::assert_snapshot!(check, @r#"
    {
      "files": [],
      "summary": {
        "diagnostics": 0,
        "unformatted": 0
      }
    }
    "#);
    insta::assert_snapshot!(run(&["lint", "--reporter", "json"]), @r#"
    {
      "files": [],
      "summary": {
        "diagnostics": 0
      }
    }
    "#);
    for command in ["check", "format", "lint"] {
        for reporter in ["checkstyle", "github", "stylish"] {
            assert_eq!(run(&[command, "--reporter", reporter]), "", "{command} {reporter}");
        }
        assert_eq!(run(&[command, "--silent"]), "", "{command}");
    }
}

#[test]
fn lint_rejects_unknown_flags_and_defaults_paths_to_current_directory() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("Example.java", "class Example { long value = 10l; }\n");

    let unknown = robusta_in_dir(&["lint", "--quiets", "."], &temp.dir);
    assert_eq!(unknown.status.code(), Some(2));
    assert_eq!(unknown.stdout.len(), 0);
    assert!(stderr(&unknown).contains("no such flag: `--quiets`"));

    let implicit = robusta_in_dir(&["lint"], &temp.dir);
    let explicit = robusta_in_dir(&["lint", "."], &temp.dir);
    assert_eq!(implicit.status.code(), explicit.status.code());
    assert_eq!(stdout_without_summary(&implicit), stdout_without_summary(&explicit));
    assert_eq!(implicit.stderr, explicit.stderr);
    assert_eq!(implicit.status.code(), Some(1), "{}", stderr(&implicit));
}

#[test]
fn check_accepts_directory_roots() {
    let temp = TempProject::new();
    temp.write("src/main/java/Example.java", "class Example { int value ; }\n");
    temp.write("src/main/java/readme.txt", "not java");

    let output = robusta_in_dir(&["check", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("Example.java"));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_prunes_version_control_dirs() {
    let temp = TempProject::new();
    temp.write("src/main/java/Example.java", "class Example {\n  int value;\n}\n");
    temp.write(".git/hooks/Broken.java", "not java");
    temp.write(".hg/store/Broken.java", "not java");
    temp.write(".jj/repo/Broken.java", "not java");
    temp.write(".sl/store/Broken.java", "not java");
    temp.write(".svn/pristine/Broken.java", "not java");

    let output = robusta_in_dir(&["check", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_walks_explicitly_requested_pruned_directory_root() {
    let temp = TempProject::new();
    temp.write(".svn/Example.java", "class Example { int value ; }\n");

    let output = robusta_in_dir(&["check", ".svn"], &temp.dir);
    let expected = Path::new(".svn").join("Example.java");

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains(&format!("would reformat: {}", presentation_path(&expected))));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_traverses_hidden_directories() {
    let temp = TempProject::new();
    temp.write(".hidden/Example.java", "class Example { int value ; }\n");

    let output = robusta_in_dir(&["check", "."], &temp.dir);
    let expected = Path::new(".hidden").join("Example.java");

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains(&presentation_path(&expected)));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_respects_gitignore_by_default() {
    let temp = TempProject::new();
    temp.write("src/Included.java", "class Included {}\n");
    temp.write("generated/Broken.java", "not java");
    temp.write(".gitignore", "generated/\n");

    let output = robusta_in_dir(&["check", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_skips_version_control_metadata() {
    let temp = TempProject::new();
    temp.write("src/Included.java", "class Included {}\n");
    for directory in [".git", ".hg", ".jj", ".sl", ".svn"] {
        temp.write(&format!("{directory}/Broken.java"), "not java");
    }

    let output = robusta_in_dir(&["check", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn directory_target_inside_a_gitignored_directory_selects_nothing() {
    let temp = TempProject::new();
    std::fs::create_dir_all(temp.dir.join(".git")).expect("create repository marker");
    temp.write("src/.gitignore", "deep/\n");
    temp.write("src/deep/Broken.java", "not java");
    let deep = temp.dir.join("src").join("deep");

    for (args, dir) in
        [(["check", "src"], &temp.dir), (["check", "src/deep"], &temp.dir), (["check", "."], &deep)]
    {
        let output = robusta_in_dir(&args, dir);

        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(
            stderr(&output).contains("Expected at least one target file."),
            "{args:?}: {}",
            stderr(&output)
        );
    }
}

#[test]
fn explicit_file_target_ignores_gitignore() {
    let temp = TempProject::new();
    std::fs::create_dir_all(temp.dir.join(".git")).expect("create repository marker");
    temp.write(".gitignore", "gen/\n");
    temp.write("gen/Example.java", "class Example { int value ; }\n");

    let output = robusta_in_dir(&["check", "gen/Example.java"], &temp.dir);
    let expected = Path::new("gen").join("Example.java");

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains(&presentation_path(&expected)));
}

#[test]
fn check_directory_respects_repeatable_ignore_paths() {
    let temp = TempProject::new();
    temp.write("src/Included.java", "class Included {}\n");
    temp.write("generated/Broken.java", "not java");
    temp.write("vendor/Broken.java", "not java");
    temp.write(".robustaignore", "generated/\n");
    temp.write(".robustaignore-extra", "vendor/\n");

    let output = robusta_in_dir(
        &["check", "--ignore-path", ".robustaignore", "--ignore-path", ".robustaignore-extra", "."],
        &temp.dir,
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_combines_ignore_paths_with_config_ignore_patterns() {
    let temp = TempProject::new();
    temp.write("src/Included.java", "class Included {}\n");
    temp.write("generated/Broken.java", "not java");
    temp.write("vendor/Broken.java", "not java");
    temp.write(".robustaignore", "vendor/\n");
    temp.write("robusta.yaml", "ignore-patterns:\n  - generated/\n");

    let output = robusta_in_dir(&["check", "--ignore-path", ".robustaignore", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_directory_respects_repeatable_ignore_patterns() {
    let temp = TempProject::new();
    temp.write("src/Included.java", "class Included {}\n");
    temp.write("generated/Broken.java", "not java");
    temp.write("src/vendor/nested/Broken.java", "not java");

    let output = robusta_in_dir(
        &["check", "--ignore-pattern", "generated/", "--ignore-pattern", "src/vendor/**", "."],
        &temp.dir,
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn ignore_patterns_apply_in_order_and_reinclude() {
    let temp = TempProject::new();
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("src/Broken.java", "not java");

    let output = robusta_in_dir(
        &[
            "config",
            "--files",
            "--ignore-pattern",
            "*.java",
            "--ignore-pattern",
            "!src/Kept.java",
            ".",
        ],
        &temp.dir,
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Kept.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn ignore_pattern_anchors_a_leading_slash_at_the_working_directory() {
    let temp = TempProject::new();
    temp.write("Root.java", "not java");
    temp.write("src/Root.java", "class Root {}\n");

    let output =
        robusta_in_dir(&["config", "--files", "--ignore-pattern", "/Root.java", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Root.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn ignore_patterns_bind_explicit_arguments_and_every_traversal_root_alike() {
    let temp = TempProject::new();
    temp.write("first/Included.java", "class Included {}\n");
    temp.write("first/Broken.java", "not java");
    temp.write("second/Broken.java", "not java");
    temp.write("second/Explicit.java", "not java");
    // Spelled the way the CLI resolves its working directory, so the pattern
    // root and the argument agree.
    let explicit = working_dir_spelling(&temp.dir).join("second/Explicit.java");

    let selection = [
        "--ignore-pattern",
        "Broken.java",
        "--ignore-pattern",
        "second/Explicit.java",
        "first",
        "second",
        explicit.to_str().expect("explicit path is UTF-8"),
    ];
    let listed = robusta_in_dir(&[&["config", "--files"][..], &selection].concat(), &temp.dir);
    let processed = robusta_in_dir(&[&["check"][..], &selection].concat(), &temp.dir);

    assert_eq!(listed.status.code(), Some(0), "{}", stderr(&listed));
    assert_eq!(stdout(&listed), "first/Included.java\n");
    assert_eq!(processed.status.code(), Some(0), "{}", stderr(&processed));
    assert_eq!(stdout_without_summary(&processed), "");
    assert!(processed.stderr.is_empty(), "{}", stderr(&processed));
}

#[test]
fn ignore_patterns_decide_over_ignore_paths_and_add_to_git_ignores() {
    let temp = TempProject::new();
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("src/Excluded.java", "not java");
    temp.write("git/Broken.java", "not java");
    temp.write("plain/Broken.java", "not java");
    temp.write(".gitignore", "git/\n");
    temp.write(".robustaignore", "src/*.java\n");

    let output = robusta_in_dir(
        &[
            "config",
            "--files",
            "--ignore-path",
            ".robustaignore",
            "--ignore-pattern",
            "!src/Kept.java",
            "--ignore-pattern",
            "plain/",
            ".",
        ],
        &temp.dir,
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Kept.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn a_reincluding_pattern_does_not_widen_the_walk_past_a_git_ignore() {
    let temp = TempProject::new();
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("git/Broken.java", "not java");
    temp.write(".gitignore", "git/\n");

    let output = robusta_in_dir(
        &["config", "--files", "--ignore-pattern", "!git/Broken.java", "."],
        &temp.dir,
    );

    // Git ignores are the walker's own layer, so a command-line pattern can
    // only narrow what the pattern and `--ignore-path` layers exclude.
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Kept.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn ignore_patterns_may_exclude_every_file_when_unmatched_paths_are_allowed() {
    let temp = TempProject::new();
    temp.write("Example.java", "not java");

    let rejected = robusta_in_dir(&["check", "--ignore-pattern", "*.java", "."], &temp.dir);
    let allowed = robusta_in_dir(
        &["check", "--ignore-pattern", "*.java", "--no-error-on-unmatched-pattern", "."],
        &temp.dir,
    );

    assert_eq!(rejected.status.code(), Some(2));
    assert!(stderr(&rejected).contains("Expected at least one target file."));
    assert_eq!(allowed.status.code(), Some(0), "{}", stderr(&allowed));
    assert_eq!(stdout_without_summary(&allowed), "");
}

#[test]
fn malformed_ignore_pattern_is_an_operational_error() {
    let temp = TempProject::new();
    let file = temp.write("Example.java", "class  Example { int value ; }\n");

    let output = robusta_in_dir(&["check", "--fix", "--ignore-pattern", "[", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("failed to parse ignore pattern ["));
    assert_eq!(
        std::fs::read_to_string(file).unwrap(),
        "class  Example { int value ; }\n",
        "a rejected pattern must not reach the source"
    );
}

/// `{First,Second}.java` names one file, so every source selects the other three.
#[test]
fn every_ignore_source_reads_braces_literally_in_every_command() {
    let source = "class A {}\n";
    let pattern = "{First,Second}.java";
    let inputs: [(&str, &str, &[&str]); 5] = [
        (".gitignore", pattern, &[]),
        (".git/info/exclude", pattern, &[]),
        ("robusta.yaml", "ignore-patterns:\n  - \"{First,Second}.java\"\n", &[]),
        ("extra.ignore", pattern, &["--ignore-path", "extra.ignore"]),
        ("unused", "", &["--ignore-pattern", pattern]),
    ];
    let run = |temp: &TempProject, command: &[&str], options: &[&str]| {
        let args = [command, options, &["."]].concat();
        robusta_in_dir(&args, &temp.dir)
    };

    for (file, contents, options) in inputs {
        let temp = TempProject::new();
        std::fs::create_dir_all(temp.dir.join(".git/info")).unwrap();
        temp.write(file, contents);
        for name in ["First.java", "Second.java", "Keep.java"] {
            temp.write(name, source);
        }
        temp.write("{First,Second}.java", "not valid Java");

        let listed = run(&temp, &["config", "--files"], options);
        assert_eq!(listed.status.code(), Some(0), "{file}: {}", stderr(&listed));
        assert_eq!(stdout(&listed), "First.java\nKeep.java\nSecond.java\n", "{file}");

        for command in ["check", "format", "lint"] {
            let output = run(&temp, &[command], options);
            let report = format!("{}{}", stdout(&output), stderr(&output));
            assert_eq!(output.status.code(), Some(0), "{file}: {command}: {report}");
            assert!(report.contains("on 3 files"), "{file}: {command}: {report}");
        }
    }
}

#[test]
fn a_literal_open_brace_loads_and_an_open_class_matches_nothing_in_a_repository_file() {
    let temp = TempProject::new();
    let source = "not valid Java";
    temp.write("robusta.yaml", "ignore-patterns:\n  - \"Open{.java\"\n");
    temp.write(".gitignore", "Open[.java\nGone.java\n");
    for name in ["Open{.java", "Open[.java", "Gone.java", "Keep.java"] {
        temp.write(name, source);
    }

    let output = robusta_in_dir(&["config", "--files", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "Keep.java\nOpen[.java\n");
}

#[test]
fn character_classes_escaped_braces_and_negation_keep_their_meaning() {
    let temp = TempProject::new();
    let source = "not valid Java";
    temp.write(".gitignore", "[AB].java\n\\{C\\}.java\n*.gen.java\n!Kept.gen.java\n");
    for name in ["A.java", "B.java", "D.java", "{C}.java", "C.java", "X.gen.java", "Kept.gen.java"]
    {
        temp.write(name, source);
    }

    let output = robusta_in_dir(&["config", "--files", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "C.java\nD.java\nKept.gen.java\n");
}

#[test]
fn missing_ignore_path_is_an_operational_error() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example {}\n");

    let output = robusta_in_dir(&["check", "--ignore-path", "missing.ignore", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("ignore file missing.ignore"));
}

#[cfg(unix)]
#[test]
fn a_literal_backslash_stays_part_of_a_file_name() {
    let source = "class A{long n=1l;}\n";
    let temp = TempProject::new();
    let slash = temp.write("src/A.java", source);
    let backslash = temp.write(r"src\A.java", source);

    let files = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(stdout(&files), "src/A.java\nsrc\\A.java\n", "{}", stderr(&files));

    let listed = robusta_in_dir(&["format", "--list-different", r"src\A.java"], &temp.dir);
    assert_eq!(stdout(&listed), "src\\A.java\n", "{}", stderr(&listed));

    let verified = robusta_in_dir(&["format", r"src\A.java"], &temp.dir);
    assert!(
        stdout(&verified).starts_with("would reformat: src\\A.java\n"),
        "{}",
        stdout(&verified)
    );

    let diff = robusta_in_dir(&["format", "--diff", r"src\A.java"], &temp.dir);
    assert!(stdout(&diff).starts_with("--- src\\A.java\n+++ src\\A.java\n"), "{}", stdout(&diff));

    assert_eq!(std::fs::read_to_string(&slash).unwrap(), source);
    assert_eq!(std::fs::read_to_string(&backslash).unwrap(), source);
}

#[cfg(unix)]
#[test]
fn check_directory_skips_discovered_symlinked_directories() {
    let temp = TempProject::new();
    temp.write("real/Linked.java", "class Linked { int value ; }\n");
    temp.write("src/Example.java", "class Example { int value ; }\n");
    std::os::unix::fs::symlink(temp.dir.join("real"), temp.dir.join("src/linked"))
        .expect("failed to create directory symlink");

    let output = robusta_in_dir(&["check", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("would reformat: src/Example.java"));
    assert!(!stdout(&output).contains("Linked.java"));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[cfg(unix)]
#[test]
fn check_directory_includes_discovered_symlinked_java_files() {
    let temp = TempProject::new();
    temp.write("real/Target.java", "class Target { int value ; }\n");
    temp.write("src/Example.java", "class Example { int value ; }\n");
    std::os::unix::fs::symlink(temp.dir.join("real/Target.java"), temp.dir.join("src/Alias.java"))
        .expect("failed to create file symlink");

    let output = robusta_in_dir(&["check", "src"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("would reformat: src/Alias.java"));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[cfg(unix)]
#[test]
fn writes_preserve_selected_file_symlinks_and_target_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempProject::new();
    temp.write("real/Target.java", "class Target { int value ; }\n");
    let target = temp.dir.join("real/Target.java");
    let alias = temp.dir.join("Alias.java");
    std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o640))
        .expect("failed to set target permissions");
    std::os::unix::fs::symlink(&target, &alias).expect("failed to create file symlink");

    let output = robusta_in_dir(&["format", "--fix", "Alias.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(std::fs::symlink_metadata(&alias).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "class Target {\n  int value;\n}\n");
    assert_eq!(std::fs::metadata(&target).unwrap().permissions().mode() & 0o777, 0o640);
}

#[cfg(unix)]
#[test]
fn check_follows_explicit_symlinked_directory_root() {
    let temp = TempProject::new();
    temp.write("real/Example.java", "class Example { int value ; }\n");
    std::os::unix::fs::symlink(temp.dir.join("real"), temp.dir.join("link"))
        .expect("failed to create directory symlink");

    let output = robusta_in_dir(&["check", "link"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("would reformat: link/Example.java"));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[cfg(unix)]
#[test]
fn check_reports_unreadable_directory_as_operational_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempProject::new();
    temp.write("src/ok/Example.java", "class Example {\n  int value;\n}\n");
    let locked = temp.dir.join("src/locked");
    std::fs::create_dir_all(&locked).expect("failed to create locked dir");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000))
        .expect("failed to lock dir");

    let output = robusta_in_dir(&["check", "src"], &temp.dir);

    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755))
        .expect("failed to unlock dir");
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("src: "));
    assert!(stderr(&output).cow_to_lowercase().contains("permission denied"));
}

#[test]
fn check_fix_directory_respects_config_ignore_patterns() {
    let temp = TempProject::new();
    let selected = temp.write("src/main/java/Selected.java", "class Selected { int value ; }\n");
    let reincluded = temp.write("examples/Kept.java", "class Kept { int value ; }\n");
    let skipped = temp.write("examples/Skipped.java", "class Skipped { int value ; }\n");
    let generated = temp.write("src/main/generated/Broken.java", "not java");
    temp.write(
        "robusta.yaml",
        r#"
ignore-patterns:
  - "generated/"
  - "examples/*.java"
  - "!examples/Kept.java"
"#,
    );

    let output = robusta_in_dir(&["check", "--fix", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(selected).unwrap(), @"
    class Selected {
      int value;
    }
    ");
    assert_eq!(std::fs::read_to_string(reincluded).unwrap(), "class Kept {\n  int value;\n}\n");
    assert_eq!(std::fs::read_to_string(skipped).unwrap(), "class Skipped { int value ; }\n");
    assert_eq!(std::fs::read_to_string(generated).unwrap(), "not java");
}

#[test]
fn config_negation_cannot_reinclude_a_file_below_an_excluded_directory() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - generated/\n  - \"!generated/Keep.java\"\n");
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("generated/Keep.java", "not java");
    temp.write("generated/nested/Broken.java", "not java");

    let output = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Kept.java\n");
}

#[test]
fn explicit_targets_cannot_reinclude_below_an_excluded_directory() {
    let temp = TempProject::new();
    temp.write("vendor/Keep.java", "not java");
    temp.write("vendor/sub/X.java", "not java");
    temp.write("excluded.ignore", "vendor/\n!vendor/Keep.java\n!vendor/sub/\n");
    temp.write("parent.ignore", "vendor/\n");

    for ignore in [
        vec![
            "--ignore-pattern",
            "vendor/",
            "--ignore-pattern",
            "!vendor/Keep.java",
            "--ignore-pattern",
            "!vendor/sub/",
        ],
        vec!["--ignore-path", "excluded.ignore"],
        vec![
            "--ignore-path",
            "parent.ignore",
            "--ignore-pattern",
            "!vendor/Keep.java",
            "--ignore-pattern",
            "!vendor/sub/",
        ],
    ] {
        for target in [".", "vendor/Keep.java", "vendor/sub", "vendor/sub/X.java"] {
            let mut args = vec!["config", "--files"];
            args.extend_from_slice(&ignore);
            args.push(target);
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
            assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
            assert!(stderr(&output).contains("Expected at least one target file."));
        }
    }
}

#[test]
fn config_negation_cannot_reinclude_an_excluded_ancestor_on_disk_or_stdin() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns: [\"vendor/\", \"!vendor/sub/\"]\n");
    let source = "class X{int n;}\n";
    let path = temp.write("vendor/sub/X.java", source);

    for target in [".", "vendor/sub", "vendor/sub/X.java"] {
        let output = robusta_in_dir(&["config", "--files", target], &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{target}: {}", stderr(&output));
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr(&output).contains("Expected at least one target file."));
    }
    let output = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "vendor/sub/X.java"],
        &temp.dir,
        source,
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), source);
    assert_eq!(std::fs::read_to_string(path).unwrap(), source);
}

#[test]
fn config_ignore_patterns_are_rooted_at_the_config_directory() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - /src/generated/\n");
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("src/generated/Broken.java", "not java");
    temp.write("other/src/generated/Kept.java", "class Kept {}\n");

    let from_root = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(from_root.status.code(), Some(0), "{}", stderr(&from_root));
    assert_eq!(stdout(&from_root), "other/src/generated/Kept.java\nsrc/Kept.java\n");

    let from_subdirectory = robusta_in_dir(&["config", "--files"], &temp.dir.join("src"));
    assert_eq!(from_subdirectory.status.code(), Some(0), "{}", stderr(&from_subdirectory));
    assert_eq!(stdout(&from_subdirectory), "Kept.java\n");
}

#[test]
fn config_negation_cannot_reinclude_git_or_command_line_ignores() {
    let temp = TempProject::new();
    temp.write(".gitignore", "vendor/\n");
    temp.write(
        "robusta.yaml",
        "ignore-patterns:\n  - \"!vendor/Broken.java\"\n  - \"!cli/Broken.java\"\n",
    );
    temp.write("src/Kept.java", "class Kept {}\n");
    temp.write("vendor/Broken.java", "not java");
    temp.write("cli/Broken.java", "not java");

    let output = robusta_in_dir(&["config", "--files", "--ignore-pattern", "cli/"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Kept.java\n");
}

#[test]
fn config_ignore_patterns_skip_explicitly_named_files() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - generated/\n");
    temp.write("generated/Broken.java", "not java");

    let output = robusta_in_dir(&["check", "generated/Broken.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("Expected at least one target file."));
}

#[test]
fn check_directory_reports_invalid_config_ignore_patterns() {
    let temp = TempProject::new();
    temp.write("src/main/java/Example.java", "class Example { int value; }\n");

    for (pattern, reason) in [
        ("../x", "`..` is not supported"),
        ("*.[java", "a character class must end with `]`"),
        ("[[:word:]]", "`[:word:]` is not a character class"),
    ] {
        temp.write("robusta.yaml", &format!("ignore-patterns:\n  - {pattern:?}\n"));

        let output = robusta_in_dir(&["check", "."], &temp.dir);
        let stderr = stderr(&output);

        assert_eq!(output.status.code(), Some(2), "{stderr}");
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr.contains("robusta.yaml"), "{stderr}");
        assert!(stderr.contains(&format!("{pattern:?}")), "{stderr}");
        assert!(stderr.contains(reason), "{stderr}");
    }
}

#[test]
fn a_shared_malformed_config_is_reported_once_per_run() {
    let temp = TempProject::new();
    temp.write("src/main/java/First.java", "class First {}\n");
    temp.write("src/main/java/Second.java", "class Second {}\n");
    temp.write("robusta.yaml", "format: [\n");

    let output = robusta_in_dir(&["check", "."], &temp.dir);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert_eq!(stderr.matches("failed to parse config").count(), 1, "{stderr}");
}

#[test]
fn option_terminator_allows_dash_prefixed_file_names() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);
    let dash_file = temp.dir.join("-Example.java");
    std::fs::rename(&temp.file, &dash_file).expect("failed to rename temp Java source");

    let output = robusta_in_dir(&["check", "--", "-Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert!(stdout(&output).contains("would reformat: -Example.java"), "{}", stdout(&output));
}

#[test]
fn format_deduplicates_overlapping_inputs() {
    let temp = TempProject::new();
    temp.write("Example.java", "class  Example { int value ; }\n");
    let output = robusta_in_dir(&["format", "Example.java", "./Example.java", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "would reformat: Example.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_reports_each_file_once_for_overlapping_inputs() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example { int value ; }\n");

    let human = robusta_in_dir(&["check", "Example.java", ".", "."], &temp.dir);
    assert_eq!(human.status.code(), Some(1), "{}", stderr(&human));
    assert!(human.stderr.is_empty(), "{}", stderr(&human));
    insta::assert_snapshot!(stdout_without_summary(&human), @"would reformat: Example.java");

    let machine = robusta_in_dir(&["check", "--reporter", "json", "Example.java", "."], &temp.dir);
    assert_eq!(machine.status.code(), Some(1), "{}", stderr(&machine));
    assert_eq!(machine.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&machine), @r#"
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
}

#[test]
fn lint_reports_each_duplicated_input_once() {
    let temp = TempProject::new();
    temp.write("Example.java", "class Example {\n    long value = 10l;\n}\n");
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");

    let human = robusta_in_dir(&["lint", "Example.java", "./Example.java"], &temp.dir);
    assert_eq!(human.status.code(), Some(1), "{}", stderr(&human));
    assert!(human.stderr.is_empty(), "{}", stderr(&human));
    insta::assert_snapshot!(stdout_without_summary(&human), @"

     x uppercase-long-suffix: use uppercase `L` for the long literal suffix
      ,-[Example.java:2:20]
    1 | class Example {
    2 |     long value = 10l;
      :                    ^
    3 | }
      `----
     help: Replace the lowercase suffix with `L`.
    ");

    let machine =
        robusta_in_dir(&["lint", "--reporter", "checkstyle", "Example.java", "."], &temp.dir);
    assert_eq!(machine.status.code(), Some(1), "{}", stderr(&machine));
    assert_eq!(machine.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&machine), @r#"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"><file name="Example.java"><error line="2" column="20" severity="error" message="use uppercase `L` for the long literal suffix" source="uppercase-long-suffix" /></file></checkstyle>"#);
}

#[test]
fn an_excluded_first_occurrence_stays_excluded_across_overlapping_roots() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "{}\n");
    temp.write("nested/robusta.yaml", "ignore-patterns:\n  - /Hidden.java\n");
    temp.write("nested/Hidden.java", "not valid Java");
    temp.write("nested/deep/Kept.java", "not valid Java");

    let output = robusta_in_dir(
        &[
            "config",
            "--files",
            "nested/Hidden.java",
            "nested/deep",
            "nested",
            ".",
            "nested/Hidden.java",
        ],
        &temp.dir,
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "nested/deep/Kept.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_fix_accepts_overlapping_inputs() {
    let temp = TempProject::new();
    let file = temp.write("Example.java", "class  Example { long value = 10l; }\n");
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");

    let output =
        robusta_in_dir(&["check", "--fix", "Example.java", "./Example.java", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&file).unwrap(), @"
    class Example {
      long value = 10L;
    }
    ");
}

#[test]
fn lint_fix_accepts_overlapping_inputs() {
    let temp = TempProject::new();
    let file = temp.write("Example.java", "class Example { long value = 10l; }\n");
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");

    let output =
        robusta_in_dir(&["lint", "--fix", ".", "Example.java", "./Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "class Example { long value = 10L; }\n");
}

#[test]
fn lint_applies_each_nested_config_rule_set_within_one_run() {
    // Nested discovery gives two files in one run different rule sets. Each file
    // must be linted with its own resolved config's rules, so a run may not
    // collapse to a single shared lint runner. Both sources carry the same two
    // findings; each config enables only one rule, so a leak would report the
    // other directory's rule against a file it never governs.
    let temp = TempProject::new();
    temp.write("a/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    temp.write("a/Example.java", "class Example { long value = 10l;; }\n");
    temp.write("b/robusta.yaml", "lint:\n  rules:\n    redundant-semicolons: deny\n");
    temp.write("b/Example.java", "class Example { long value = 10l;; }\n");

    let output = robusta_in_dir(&["lint", "--reporter", "checkstyle", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(output.stderr.len(), 0);
    insta::assert_snapshot!(stdout(&output), @r#"<?xml version="1.0" encoding="utf-8"?><checkstyle version="4.3"><file name="a/Example.java"><error line="1" column="32" severity="error" message="use uppercase `L` for the long literal suffix" source="uppercase-long-suffix" /></file> <file name="b/Example.java"><error line="1" column="34" severity="error" message="redundant semicolon" source="redundant-semicolons" /></file></checkstyle>"#);
}

/// `A.java` in one directory per selection: the shared list skips `shared/`,
/// each tool's list skips the other tool's directory, and both skip `neither/`.
fn per_tool_ignore_project() -> TempProject {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "ignore-patterns: [shared/]
format:
  ignore-patterns: [lint-only/, neither/]
lint:
  ignore-patterns: [format-only/, neither/]
  rules:
    uppercase-long-suffix: deny
",
    );
    for directory in ["both", "format-only", "lint-only", "neither", "shared"] {
        temp.write(&format!("{directory}/A.java"), "class A{long n=1l;}\n");
    }
    temp
}

#[test]
fn each_tool_processes_only_the_files_its_ignore_patterns_keep() {
    let temp = per_tool_ignore_project();
    let unformatted = "1:1: file is not formatted; run `robusta format --fix` [Warning/format]";
    let lowercase =
        "1:17: use uppercase `L` for the long literal suffix [Error/uppercase-long-suffix]";

    for (command, expected) in [
        (
            "check",
            format!(
                "both/A.java:{unformatted}\nboth/A.java:{lowercase}\nformat-only/A.java:{unformatted}\nlint-only/A.java:{lowercase}\n\n4 problems\n"
            ),
        ),
        (
            "format",
            format!("both/A.java:{unformatted}\nformat-only/A.java:{unformatted}\n\n2 problems\n"),
        ),
        ("lint", format!("both/A.java:{lowercase}\nlint-only/A.java:{lowercase}\n\n2 problems\n")),
    ] {
        let output = robusta_in_dir(&[command, "--reporter", "unix"], &temp.dir);
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stderr(&output));
        assert_eq!(stdout(&output), expected, "{command}");
    }
}

#[test]
fn check_fix_applies_each_tool_only_to_the_files_its_ignore_patterns_keep() {
    let temp = per_tool_ignore_project();

    let output = robusta_in_dir(&["check", "--fix"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(run_summary(&output).contains("on 3 files"), "{}", stdout(&output));
    for (directory, expected) in [
        ("both", "class A {\n  long n = 1L;\n}\n"),
        ("format-only", "class A {\n  long n = 1l;\n}\n"),
        ("lint-only", "class A{long n=1L;}\n"),
        ("neither", "class A{long n=1l;}\n"),
        ("shared", "class A{long n=1l;}\n"),
    ] {
        let source = std::fs::read_to_string(temp.dir.join(directory).join("A.java")).unwrap();
        assert_eq!(source, expected, "{directory}");
    }
}

/// A file with both a formatting and a lint change, whichever name it carries.
const FIXABLE: &str = "class A{long n=1l;}\n";
const DENY_LONG_SUFFIX: &str = "lint:\n  rules:\n    uppercase-long-suffix: deny\n";

/// `src/A.java`, `src/A.txt`, and the extensionless `src/A`, all holding [`FIXABLE`].
fn java_and_other_files() -> TempProject {
    let temp = TempProject::new();
    temp.write("robusta.yaml", DENY_LONG_SUFFIX);
    for name in ["src/A.java", "src/A.txt", "src/A"] {
        temp.write(name, FIXABLE);
    }
    temp
}

fn assert_other_files_untouched(temp: &TempProject) {
    for name in ["src/A.txt", "src/A"] {
        assert_eq!(std::fs::read_to_string(temp.dir.join(name)).unwrap(), FIXABLE, "{name}");
    }
}

#[test]
fn fixes_write_only_the_java_files_of_a_mixed_selection() {
    for (command, fixed) in [
        ("check", "class A {\n  long n = 1L;\n}\n"),
        ("format", "class A {\n  long n = 1l;\n}\n"),
        ("lint", "class A{long n=1L;}\n"),
    ] {
        let temp = java_and_other_files();
        let root = working_dir_spelling(&temp.dir);
        let (absolute_java, absolute_text) = (root.join("src/A.java"), root.join("src/A.txt"));
        let selections: [&[&str]; 3] = [
            &["src/A.txt", "src/A", "src/A.java"],
            &[absolute_text.to_str().unwrap(), absolute_java.to_str().unwrap()],
            &["src/A.txt", "src/A", "src"],
        ];

        for selection in selections {
            let java = temp.write("src/A.java", FIXABLE);

            let output = robusta_in_dir(&[&[command, "--fix"], selection].concat(), &temp.dir);

            assert_eq!(output.status.code(), Some(0), "{selection:?}: {}", stderr(&output));
            assert!(run_summary(&output).contains("on 1 file"), "{}", stdout(&output));
            assert_eq!(std::fs::read_to_string(java).unwrap(), fixed, "{selection:?}");
            assert_other_files_untouched(&temp);
        }
    }
}

#[test]
fn read_only_runs_report_only_the_java_files_of_a_mixed_selection() {
    let temp = java_and_other_files();
    let args: [&[&str]; 7] = [
        &["check"],
        &["format"],
        &["lint"],
        &["check", "--list-different"],
        &["format", "--list-different"],
        &["check", "--diff"],
        &["lint", "--diff"],
    ];

    for args in args {
        let paths = ["src/A.txt", "src/A", "src/A.java"];
        let output = robusta_in_dir(&[args, &paths].concat(), &temp.dir);
        let report = stdout(&output);

        assert_eq!(output.status.code(), Some(1), "{args:?}: {}", stderr(&output));
        assert!(report.contains("src/A.java"), "{args:?}: {report}");
        // The paths of both other files start with `src/A`.
        assert!(
            !report.split("src/A.java").any(|rest| rest.contains("src/A")),
            "{args:?}: {report}"
        );
        assert_eq!(std::fs::read_to_string(temp.dir.join("src/A.java")).unwrap(), FIXABLE);
        assert_other_files_untouched(&temp);
    }
}

#[test]
fn only_other_files_follow_the_empty_selection_policy() {
    let temp = java_and_other_files();
    let absolute = working_dir_spelling(&temp.dir).join("src/A.txt");

    for path in ["src/A.txt", "src/A", absolute.to_str().unwrap()] {
        for command in ["check", "format", "lint"] {
            for fix in [&[][..], &["--fix"][..]] {
                let args = [&[command][..], fix, &[path]].concat();
                let failed = robusta_in_dir(&args, &temp.dir);
                assert_eq!(failed.status.code(), Some(2), "{args:?}: {}", stderr(&failed));
                assert!(stderr(&failed).contains("Expected at least one target file."));
                assert_eq!(failed.stdout.len(), 0, "{args:?}");

                let args = [&args[..], &["--no-error-on-unmatched-pattern"]].concat();
                let allowed = robusta_in_dir(&args, &temp.dir);
                assert_eq!(allowed.status.code(), Some(0), "{args:?}: {}", stderr(&allowed));
                assert_other_files_untouched(&temp);
            }
        }
    }
}

#[test]
fn a_missing_other_file_is_still_an_input_error() {
    let temp = java_and_other_files();

    let output = robusta_in_dir(&["check", "src/Missing.txt", "src/A.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("Missing.txt"), "{}", stderr(&output));
}

#[test]
fn configuration_and_suppressions_govern_the_java_files_of_a_mixed_selection() {
    let temp = java_and_other_files();
    temp.write("robusta.yaml", &format!("{DENY_LONG_SUFFIX}ignore-patterns:\n  - Ignored.java\n"));
    temp.write("Ignored.java", FIXABLE);
    temp.write(
        "Suppressed.java",
        "class Suppressed {\n  @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n  long n = 1l;\n}\n",
    );

    let output =
        robusta_in_dir(&["lint", "src/A.txt", "Ignored.java", "Suppressed.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(run_summary(&output).contains("on 1 file"), "{}", stdout(&output));
}
