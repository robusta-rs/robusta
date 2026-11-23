use cow_utils::CowUtils;

use crate::support::*;

#[test]
fn file_and_stdin_accept_the_same_utf8_source() {
    for source in [
        "",
        "class Example {}\n",
        "class Café { int 𝒂; void méthode(int 变量٢) {} }\n",
        "\u{feff}class Café { String text = \"こんにちは 🌍\"; }\n",
    ] {
        let temp = TempJava::new(source);
        for command in ["format", "lint", "check"] {
            std::fs::write(&temp.file, source).unwrap();
            let file = robusta_in_dir(&[command, "--fix", "Example.java"], &temp.dir);
            let stdin = robusta_with_stdin(
                &[command, "--fix", "--stdin-filepath", "Example.java"],
                &temp.dir,
                source,
            );
            assert_eq!(file.status.code(), Some(0), "{command}: {}", stderr(&file));
            assert_eq!(stdin.status.code(), Some(0), "{command}: {}", stderr(&stdin));
            assert_eq!(std::fs::read(&temp.file).unwrap(), stdin.stdout, "{command}: {source:?}");
        }
    }
}

#[test]
fn invalid_utf8_file_is_a_tool_failure_and_is_not_written() {
    for invalid in [
        &b"\xc2"[..],
        &b"\x80"[..],
        &b"\xc0\xaf"[..],
        &b"\xed\xa0\x80"[..],
        &b"\xf4\x90\x80\x80"[..],
        &b"\xef\xbb"[..],
    ] {
        for padding in [0, 43, 4096] {
            let temp = TempJava::new("");
            let source =
                [b"class Example {} // ", vec![b' '; padding].as_slice(), invalid].concat();
            std::fs::write(&temp.file, &source).unwrap();
            for command in ["format", "lint", "check"] {
                let output = robusta_in_dir(&[command, "--fix", "Example.java"], &temp.dir);
                assert_eq!(output.status.code(), Some(2), "{command}: {}", stderr(&output));
                assert!(
                    stderr(&output).contains("Example.java: stream did not contain valid UTF-8")
                );
                assert_eq!(stdout_without_summary(&output), "");
                assert_eq!(std::fs::read(&temp.file).unwrap(), source);
            }
        }
    }
}

#[test]
fn format_fix_reads_stdin_and_writes_formatted_source_to_stdout() {
    let temp = TempProject::new();

    let output = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "Example.java"],
        &temp.dir,
        "class  Example { int value ; }\n",
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    class Example {
      int value;
    }
    ");
    assert_eq!(stderr_without_summary(&output), "");
    assert!(!temp.dir.join("Example.java").exists());
}

#[cfg(feature = "detect_code_removal")]
#[test]
fn runtime_integrity_accepts_own_line_separator_deferral() {
    let temp = TempProject::new();
    let output = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "Example.java"],
        &temp.dir,
        "class A { void f(int x) { switch (x) { case 1\n/* body */\n: case 2: break; } } }\n",
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stderr_without_summary(&output), "");
    insta::assert_snapshot!(stdout(&output), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1:
          /* body */
          case 2:
            break;
        }
      }
    }
    ");
}

#[test]
fn format_stdin_preserves_only_an_input_bom() {
    const UTF8_BOM: &str = "\u{feff}";
    const FORMATTED_BODY: &str = "class Example {\n  int value;\n}\n";
    let temp = TempProject::new();

    for prefix in ["", UTF8_BOM] {
        let output = robusta_with_stdin(
            &["format", "--fix", "--stdin-filepath", "Example.java"],
            &temp.dir,
            &format!("{prefix}class  Example {{ int value ; }}\n"),
        );

        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert_eq!(stdout(&output), format!("{prefix}{FORMATTED_BODY}"));
        assert_eq!(stderr_without_summary(&output), "");
    }
}

#[test]
fn format_stdin_accepts_unicode_identifiers_and_rejects_hard_keyword_names() {
    let temp = TempProject::new();

    // spellchecker:off
    let valid = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "Café.java"],
        &temp.dir,
        r"class  Caf\u00e9 { int 𝒂 ; \u0076oid méthode ( int 变量٢ ) { } }",
    );
    assert_eq!(valid.status.code(), Some(0), "{}", stderr(&valid));
    insta::assert_snapshot!(stdout(&valid), @r"
    class Caf\u00e9 {
      int 𝒂;

      void méthode(int 变量٢) {}
    }
    ");
    // spellchecker:on
    assert_eq!(stderr_without_summary(&valid), "");

    for (path, source) in
        [("Raw.java", "class int {}"), ("Escaped.java", r"class A { Object \u0069nt; }")]
    {
        let invalid =
            robusta_with_stdin(&["format", "--fix", "--stdin-filepath", path], &temp.dir, source);
        assert_eq!(invalid.status.code(), Some(1), "{source}");
        assert!(invalid.stdout.is_empty(), "{source}");
        assert!(stderr(&invalid).contains(path), "{source}");
    }
}

#[test]
fn format_stdin_uses_nearest_config_for_virtual_path() {
    let temp = TempProject::new();
    temp.write("nested/robusta.yaml", "format:\n  indent-width: 2\n");

    let output = robusta_with_stdin(
        &["format", "--fix", "--stdin-filepath", "nested/src/main/java/Example.java"],
        &temp.dir,
        "class Example { int value; }\n",
    );

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    class Example {
      int value;
    }
    ");
    assert_eq!(stderr_without_summary(&output), "");
}

#[test]
fn stdin_passes_an_excluded_virtual_path_through_unchanged() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - generated/\n");
    let source = "class  Example { int value ; }\n";

    for command in ["check", "format", "lint"] {
        let fixed = robusta_with_stdin(
            &[command, "--fix", "--stdin-filepath", "generated/Example.java"],
            &temp.dir,
            source,
        );
        assert_eq!(fixed.status.code(), Some(0), "{command}: {}", stderr(&fixed));
        assert_eq!(stdout(&fixed), source, "{command}");
        let report = stderr(&fixed);
        assert!(report.starts_with("Finished in ") && report.contains(" on 0 files "), "{report}");

        let verified = robusta_with_stdin(
            &[command, "--stdin-filepath", "generated/Example.java"],
            &temp.dir,
            source,
        );
        assert_eq!(verified.status.code(), Some(0), "{command}: {}", stderr(&verified));
        let report = stdout(&verified);
        assert!(report.starts_with("Finished in ") && report.contains(" on 0 files "), "{report}");
        assert!(verified.stderr.is_empty(), "{command}: {}", stderr(&verified));
    }

    let fixed = robusta_with_stdin(
        &["check", "--fix", "--reporter", "json", "--stdin-filepath", "generated/Example.java"],
        &temp.dir,
        source,
    );
    assert_eq!(fixed.status.code(), Some(0), "{}", stderr(&fixed));
    assert_eq!(stdout(&fixed), source);
    insta::assert_snapshot!(stderr(&fixed), @r#"
    {
      "files": [],
      "summary": {
        "diagnostics": 0,
        "unformatted": 0
      }
    }
    "#);
    let verified = robusta_with_stdin(
        &["lint", "--reporter", "json", "--stdin-filepath", "generated/Example.java"],
        &temp.dir,
        source,
    );
    assert_eq!(verified.status.code(), Some(0), "{}", stderr(&verified));
    insta::assert_snapshot!(stdout(&verified), @r#"
    {
      "files": [],
      "summary": {
        "diagnostics": 0
      }
    }
    "#);
    assert!(verified.stderr.is_empty(), "{}", stderr(&verified));
}

#[test]
fn stdin_check_runs_only_the_tools_that_keep_its_virtual_path() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "format:\n  ignore-patterns: [lint-only/, neither/]\nlint:\n  ignore-patterns: [format-only/, neither/]\n  rules:\n    uppercase-long-suffix: deny\n",
    );
    let source = "class A{long n=1l;}\n";

    for (path, expected) in [
        ("format-only/A.java", "class A {\n  long n = 1l;\n}\n"),
        ("lint-only/A.java", "class A{long n=1L;}\n"),
        ("neither/A.java", source),
    ] {
        let fixed =
            robusta_with_stdin(&["check", "--fix", "--stdin-filepath", path], &temp.dir, source);
        assert_eq!(fixed.status.code(), Some(0), "{path}: {}", stderr(&fixed));
        assert_eq!(stdout(&fixed), expected, "{path}");
    }
}

#[test]
fn format_stdin_reports_parse_errors_against_virtual_path() {
    let temp = TempProject::new();

    let output = robusta_with_stdin(
        &["format", "--stdin-filepath", "Virtual.java"],
        &temp.dir,
        "class Example {\n    int ;\n}\n",
    );

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(stdout_without_summary(&output), @"

     x expected a member name
      ,-[Virtual.java:2:9]
    1 | class Example {
    2 |     int ;
      :         |
      :         `-- a member name expected here
    3 | }
      `----
    ");
}

#[test]
fn format_stdin_rejects_the_old_flag_spelling() {
    let temp = TempProject::new();
    let output = robusta_with_stdin(
        &["format", "--stdin-file-path", "Example.java"],
        &temp.dir,
        "class Example {}\n",
    );

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(output.stdout.len(), 0);
    assert!(!temp.dir.join("Example.java").exists());
}

/// Without a fix flag, every command reports on the document and writes none
/// of it, so the report keeps stdout.
#[test]
fn stdin_without_a_fix_flag_reports_and_writes_no_document() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let source = "class Example { long value = 1l; }\n";

    for (command, reformat) in [("check", true), ("format", true), ("lint", false)] {
        let output =
            robusta_with_stdin(&[command, "--stdin-filepath", "Example.java"], &temp.dir, source);
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stderr(&output));
        assert!(output.stderr.is_empty(), "{command}: {}", stderr(&output));
        let report = stdout_without_summary(&output);
        assert_eq!(report.contains("would reformat: Example.java\n"), reformat, "{command}");
        assert_eq!(report.contains("uppercase-long-suffix"), command != "format", "{command}");
        assert!(!report.contains("{\n    long value"), "{command}: {report}");
    }
}

/// A fix flag sends the command's own transformation to stdout: formatting
/// for `format`, lint fixes for `lint`, and lint fixes then formatting for
/// `check`.
#[test]
fn stdin_fix_prints_each_commands_transformation() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let source = "class Example { long value = 1l; }\n";

    for (command, expected) in [
        ("check", "class Example {\n  long value = 1L;\n}\n"),
        ("format", "class Example {\n  long value = 1l;\n}\n"),
        ("lint", "class Example { long value = 1L; }\n"),
    ] {
        for silent in [&[][..], &["--silent"][..]] {
            let args =
                [&[command, "--fix", "--stdin-filepath", "Example.java"][..], silent].concat();
            let output = robusta_with_stdin(&args, &temp.dir, source);
            assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
            assert_eq!(stdout(&output), expected, "{args:?}");
            assert!(stderr_without_summary(&output).is_empty(), "{args:?}: {}", stderr(&output));
            if !silent.is_empty() {
                assert!(output.stderr.is_empty(), "{args:?}: {}", stderr(&output));
            }
        }
    }
}

/// Findings a fix cannot remove still fail the run, and the fixed document is
/// still printed so an editor can apply what was fixed.
#[test]
fn stdin_fix_prints_the_document_and_fails_on_remaining_findings() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    broken-null-check: deny\n",
    );
    let source = "class Example {\n    long value = 1l;\n\n    boolean f(String s) {\n        return s == null && s.isEmpty();\n    }\n}\n";

    let output = robusta_with_stdin(
        &["lint", "--fix", "--stdin-filepath", "Example.java"],
        &temp.dir,
        source,
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stdout(&output), source.cow_replace("1l", "1L"));
    assert!(stderr(&output).contains("broken-null-check"), "{}", stderr(&output));
    assert!(!stderr(&output).contains("uppercase-long-suffix"), "{}", stderr(&output));
}

/// The virtual path names a real file here, and no run, fixing or not, may
/// write it or anything else on disk.
#[test]
fn stdin_never_writes_the_virtual_path() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
    let on_disk = "class  Example { long value = 2l; }\n";
    let file = temp.write("Example.java", on_disk);
    let entries = || {
        let mut names = std::fs::read_dir(&temp.dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>();
        names.sort();
        names
    };
    let before = entries();

    for command in ["check", "format", "lint"] {
        for fix in [&[][..], &["--fix"], &["--fix", "--unsafe-fixes"]] {
            if command == "format" && fix.contains(&"--unsafe-fixes") {
                continue;
            }
            let args = [&[command, "--stdin-filepath", "Example.java"][..], fix].concat();
            let output =
                robusta_with_stdin(&args, &temp.dir, "class Example { long value = 1l; }\n");
            assert_ne!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
            assert_eq!(std::fs::read(&file).unwrap(), on_disk.as_bytes(), "{args:?}");
            assert_eq!(entries(), before, "{args:?}");
        }
    }
}

/// When the document owns stdout, a machine report moves to stderr; without a
/// fix flag it stays on stdout.
#[test]
fn stdin_moves_a_machine_report_to_stderr_under_a_fix_flag() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    broken-null-check: deny\n",
    );
    let source = "class Example {\n    boolean f(String s) {\n        return s == null && s.isEmpty();\n    }\n}\n";

    let fixed = robusta_with_stdin(
        &["lint", "--fix", "--reporter", "json", "--stdin-filepath", "Example.java"],
        &temp.dir,
        source,
    );
    assert_eq!(fixed.status.code(), Some(1), "{}", stderr(&fixed));
    assert_eq!(stdout(&fixed), source);
    assert!(stderr(&fixed).contains("\"broken-null-check\""), "{}", stderr(&fixed));

    let verified = robusta_with_stdin(
        &["lint", "--reporter", "json", "--stdin-filepath", "Example.java"],
        &temp.dir,
        source,
    );
    assert_eq!(verified.status.code(), Some(1), "{}", stderr(&verified));
    assert!(stdout(&verified).contains("\"broken-null-check\""), "{}", stdout(&verified));
    assert!(verified.stderr.is_empty(), "{}", stderr(&verified));
}

#[test]
fn stdin_rejects_file_selection_previews_and_baseline_updates() {
    let temp = TempProject::new();
    let rejected: [(&[&str], &str); 8] = [
        (&["check", "Other.java"], "Other.java"),
        (&["format", "--ignore-path", ".robustaignore"], "--ignore-path"),
        (&["lint", "--ignore-pattern", "generated/"], "--ignore-pattern"),
        (&["check", "--threads", "1"], "--threads"),
        (&["format", "--diff"], "--diff"),
        (&["lint", "--list-different"], "--list-different"),
        (&["lint", "--suppress-all"], "--suppress-all"),
        (&["check", "--prune-suppressions"], "--prune-suppressions"),
    ];
    for (args, conflict) in rejected {
        let args = [args, &["--stdin-filepath", "Virtual.java"]].concat();
        let output = robusta_with_stdin(&args, &temp.dir, "class Example {}\n");
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        assert_eq!(
            stderr(&output),
            format!("Error: `--stdin-filepath` cannot be combined with `{conflict}`\n"),
            "{args:?}"
        );
    }
    assert!(!temp.dir.join("robusta-suppressions.json").exists());

    // Selection policy with nothing to select from is harmless, so it is not
    // rejected.
    let unmatched = robusta_with_stdin(
        &["format", "--stdin-filepath", "Virtual.java", "--no-error-on-unmatched-pattern"],
        &temp.dir,
        "class Example {}\n",
    );
    assert_eq!(unmatched.status.code(), Some(0), "{}", stderr(&unmatched));
}

#[test]
fn format_tolerates_stdout_closing_early() {
    let temp = TempJava::new(&oversized_java_source());

    let output = robusta_with_closed_stdout(&["format", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn check_diff_tolerates_stdout_closing_early() {
    #[expect(clippy::disallowed_methods, reason = "the generated source is a temporary")]
    let source = oversized_java_source().replace("  private", "private");
    let temp = TempJava::new(&source);

    let output = robusta_with_closed_stdout(&["check", "--diff", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stderr_without_summary(&output), "");
}

#[test]
fn check_machine_report_tolerates_stdout_closing_early() {
    // A redundant semicolon on every field makes the buffered machine
    // document itself outgrow the pipe buffer.
    #[expect(clippy::disallowed_methods, reason = "the generated source is a temporary")]
    let source = oversized_java_source().replace(';', ";;");
    let temp = TempJava::new(&source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "lint:\n  rules:\n    redundant-semicolons: warn\n",
    )
    .expect("failed to write config");

    let output =
        robusta_with_closed_stdout(&["check", "--reporter", "json", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(stderr(&output), "");
}

/// The load-bearing guarantee of the parallel file loop: for the same input
/// set, `--threads 1` (the serial fast path) and a multi-threaded run must
/// produce byte-identical stdout, stderr, and exit codes across the human,
/// JSON, and checkstyle surfaces. Sixteen files with a mix of reformat needs
/// and lint findings exercise ordered emission on both streams.

#[test]
fn parallel_output_matches_serial_output() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  rules:\n    uppercase-long-suffix: deny\n    modifier-order: warn\n",
    );
    for index in 0..16 {
        let body = match index % 3 {
            0 => format!("class File{index:02} {{ int x ; }}\n"),
            1 => format!("class File{index:02} {{ static public long v = 1l; }}\n"),
            _ => format!("class File{index:02} {{\n    int y;\n}}\n"),
        };
        temp.write(&format!("src/File{index:02}.java"), &body);
    }

    let bases: [&[&str]; 4] = [
        &["format"],
        &["check"],
        &["check", "--reporter", "json"],
        &["lint", "--reporter", "checkstyle"],
    ];
    for base in bases {
        let mut serial_args = base.to_vec();
        serial_args.extend(["--threads", "1", "src"]);
        let serial = robusta_in_dir(&serial_args, &temp.dir);

        let mut parallel_args = base.to_vec();
        parallel_args.extend(["--threads", "8", "src"]);
        let parallel = robusta_in_dir(&parallel_args, &temp.dir);

        assert_eq!(serial.status.code(), parallel.status.code(), "exit code for {base:?}");
        // The summary is deliberately excluded: it reports the thread count
        // these two runs differ by, and an elapsed time neither can repeat.
        assert_eq!(
            stdout_without_summary(&serial),
            stdout_without_summary(&parallel),
            "stdout for {base:?}"
        );
        assert_eq!(stderr(&serial), stderr(&parallel), "stderr for {base:?}");
    }
}

#[test]
fn stdin_formats_the_document_whatever_its_virtual_path_is_named() {
    let temp = TempProject::new();
    temp.write("nested/robusta.yaml", "format:\n  indent-width: 4\n");
    let source = "class Example { int value; }\n";

    for (path, formatted) in [
        ("untitled", "class Example {\n  int value;\n}\n"),
        ("A.txt", "class Example {\n  int value;\n}\n"),
        ("nested/untitled", "class Example {\n    int value;\n}\n"),
        ("nested/A.txt", "class Example {\n    int value;\n}\n"),
    ] {
        let output =
            robusta_with_stdin(&["format", "--fix", "--stdin-filepath", path], &temp.dir, source);

        assert_eq!(output.status.code(), Some(0), "{path}: {}", stderr(&output));
        assert_eq!(stdout(&output), formatted, "{path}");
        assert!(!temp.dir.join(path).exists(), "{path}");
    }
}
