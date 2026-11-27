use crate::support::*;

#[test]
fn default_indentation_and_editorconfig_tabs_use_two_columns() {
    let temp = TempProject::new();
    let source = "class A { void m() { if (x) { y(); } } }\n";
    let expected = "class A {\n  void m() {\n    if (x) {\n      y();\n    }\n  }\n}\n";
    let output =
        robusta_with_stdin(&["format", "--fix", "--stdin-filepath", "A.java"], &temp.dir, source);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), expected);

    temp.write(".editorconfig", "root = true\n\n[*]\nindent_style = tab\n");
    let source = format!("class A {{ void m() {{ call(\"{}\"); }} }}\n", "a".repeat(84));
    let output =
        robusta_with_stdin(&["format", "--fix", "--stdin-filepath", "A.java"], &temp.dir, &source);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!("class A {{\n\tvoid m() {{\n\t\tcall(\"{}\");\n\t}}\n}}\n", "a".repeat(84))
    );
    temp.write("robusta.yaml", "format:\n  indent-width: 4\n");
    let wider =
        robusta_with_stdin(&["format", "--fix", "--stdin-filepath", "A.java"], &temp.dir, &source);
    assert_eq!(wider.status.code(), Some(0), "{}", stderr(&wider));
    assert_ne!(stdout(&wider), stdout(&output));
}

#[test]
fn format_preview_and_write_use_the_same_candidate_without_previewing_source_to_disk() {
    let source = "class Example { int value ; }";
    let temp = TempJava::new(source);

    let diff = robusta_with_path(&["format", "--diff"], &temp.file);
    assert_eq!(diff.status.code(), Some(1), "{}", stderr(&diff));
    assert_eq!(stderr_without_summary(&diff), "");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    let candidate = candidate_from_diff(&stdout(&diff));

    let list = robusta_with_path(&["format", "--list-different"], &temp.file);
    assert_eq!(list.status.code(), Some(1), "{}", stderr(&list));
    assert_eq!(stdout(&list), format!("{}\n", presentation_path(&temp.file)));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);

    let write = robusta_with_path(&["format", "--fix"], &temp.file);
    assert_eq!(write.status.code(), Some(0), "{}", stderr(&write));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), candidate);
}

#[test]
fn format_rewrites_a_crlf_file_to_lf_by_default() {
    let temp = TempJava::new("class Example {\r\n    int value;\r\n}\r\n");

    let check = robusta_with_path(&["format"], &temp.file);
    assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
    assert_eq!(
        stdout_without_summary(&check),
        format!("would reformat: {}\n", presentation_path(&temp.file))
    );

    let fix = robusta_with_path(&["format", "--fix"], &temp.file);
    assert_eq!(fix.status.code(), Some(0), "{}", stderr(&fix));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), "class Example {\n  int value;\n}\n");

    let verify = robusta_with_path(&["format"], &temp.file);
    assert_eq!(verify.status.code(), Some(0), "{}", stdout(&verify));
    assert!(stdout_without_summary(&verify).is_empty(), "{}", stdout(&verify));
}

#[test]
fn configured_line_endings_keep_matching_files_and_convert_lf_files() {
    for (value, ending) in [("crlf", "\r\n"), ("cr", "\r")] {
        for (config_file, config) in [
            ("robusta.yaml", format!("format:\n  line-ending: {value}\n")),
            (".editorconfig", format!("[*.java]\nend_of_line = {value}\n")),
        ] {
            let temp = TempProject::new();
            temp.write(config_file, &config);
            let kept = format!("class Kept {{{ending}  int value;{ending}}}{ending}");
            temp.write("Kept.java", &kept);
            temp.write("Converted.java", "class Converted {\n  int value;\n}\n");

            let output =
                robusta_in_dir(&["format", "--fix", "Kept.java", "Converted.java"], &temp.dir);

            assert_eq!(output.status.code(), Some(0), "{config_file} {value}: {}", stderr(&output));
            assert_eq!(std::fs::read_to_string(temp.dir.join("Kept.java")).unwrap(), kept);
            assert_eq!(
                std::fs::read_to_string(temp.dir.join("Converted.java")).unwrap(),
                format!("class Converted {{{ending}  int value;{ending}}}{ending}"),
                "{config_file} {value}",
            );
        }
    }
}

#[test]
fn format_diff_splits_crlf_and_cr_lines_without_false_eof_markers() {
    for (value, line_ending) in [("crlf", "\r\n"), ("cr", "\r")] {
        let source = format!(
            "class Example {{{line_ending}int first ;{line_ending}int second ;{line_ending}}}{line_ending}"
        );
        let temp = TempJava::new(&source);
        std::fs::write(temp.dir.join("robusta.yaml"), format!("format:\n  line-ending: {value}\n"))
            .unwrap();

        let output = robusta_with_path(&["format", "--diff"], &temp.file);

        assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
        assert_eq!(stderr_without_summary(&output), "");
        assert_eq!(
            stdout(&output),
            format!(
                "--- {path}\n+++ {path}\n@@ -1,4 +1,4 @@\n \
                 class Example {{{line_ending}-int first ;{line_ending}\
                 -int second ;{line_ending}+  int first;{line_ending}\
                 +  int second;{line_ending} }}{line_ending}",
                path = presentation_path(&temp.file),
            ),
            "line ending {line_ending:?}",
        );
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
    }
}

#[test]
fn format_diff_elides_unchanged_lines_between_distant_changes() {
    // A formatter's changes are typically scattered through a file. Each change
    // region gets its own hunk with three lines of context, so the unchanged
    // run between them is elided instead of being reprinted as a deletion and
    // an identical insertion.
    use std::fmt::Write;

    let mut source = String::from("class Example {\n  int a=1;\n");
    for index in 0..40 {
        writeln!(source, "  int filler{index} = {index};").expect("writing to a String");
    }
    source.push_str("  int z=2;\n}\n");
    let temp = TempJava::new(&source);

    let output = robusta_with_path(&["format", "--diff"], &temp.file);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!(
            concat!(
                "--- {path}\n+++ {path}\n",
                "@@ -1,5 +1,5 @@\n",
                " class Example {{\n",
                "-  int a=1;\n",
                "+  int a = 1;\n",
                "   int filler0 = 0;\n",
                "   int filler1 = 1;\n",
                "   int filler2 = 2;\n",
                "@@ -40,5 +40,5 @@\n",
                "   int filler37 = 37;\n",
                "   int filler38 = 38;\n",
                "   int filler39 = 39;\n",
                "-  int z=2;\n",
                "+  int z = 2;\n",
                " }}\n",
            ),
            path = presentation_path(&temp.file),
        )
    );
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_diff_describes_configured_crlf_normalization_without_writing() {
    let source = "class Example { int value ; }\r\n";
    let project = TempProject::new();
    project.write("robusta.yaml", "format:\n  line-ending: lf\n");
    let file = project.write("Example.java", source);

    let output = robusta_with_path(&["format", "--diff"], &file);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stdout(&output).contains("-class Example { int value ; }\r\n"));
    assert!(stdout(&output).contains("+class Example {\n"));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), source);
}

#[test]
fn bare_format_verifies_without_rewriting_or_printing_source() {
    let source = "class  Example { int value ; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format"], &temp.file);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(
        stdout_without_summary(&output),
        format!("would reformat: {}\n", presentation_path(&temp.file))
    );
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_reports_parser_diagnostics_with_source_positions() {
    let temp = TempJava::new("class Example {\n    int ;\n}\n");

    let output = robusta_with_path(&["format"], &temp.file);
    let report = stdout(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert!(report.contains(&format!("{}:2:9]", presentation_path(&temp.file))), "{report}");
}

#[test]
fn format_fix_reports_all_recovered_errors_without_rewriting() {
    let source = "class Example {\n    int ;\n    long ;\n}\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "--fix"], &temp.file);
    let report = stdout(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    let path = presentation_path(&temp.file);
    assert!(report.contains(&format!("{path}:2:9]")), "{report}");
    assert!(report.contains(&format!("{path}:3:10]")), "{report}");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_fix_writes_in_place() {
    let temp = TempJava::new("class  Example { int value ; }\n");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    class Example {
      int value;
    }
    ");
}

#[test]
fn format_fix_preserves_only_an_input_bom() {
    const UTF8_BOM: &str = "\u{feff}";
    const FORMATTED_BODY: &str = "class Example {\n  int value;\n}\n";

    for prefix in ["", UTF8_BOM] {
        let temp = TempJava::new(&format!("{prefix}class  Example {{ int value ; }}\n"));

        let output = robusta_with_path(&["format", "--fix"], &temp.file);

        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert_eq!(stdout_without_summary(&output), "");
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        assert_eq!(
            std::fs::read_to_string(&temp.file).unwrap(),
            format!("{prefix}{FORMATTED_BODY}"),
            "input prefix {prefix:?}",
        );
    }
}

#[test]
fn final_sub_is_accepted_and_removed_by_formatting() {
    for sub in ["\x1a", r"\u001a"] {
        let source = format!("class Example {{}}\n{sub}");
        let temp = TempJava::new(&source);
        let lint = robusta_with_path(&["lint"], &temp.file);
        assert_eq!(lint.status.code(), Some(0), "{}", stderr(&lint));
        assert!(lint.stderr.is_empty(), "{}", stderr(&lint));
        assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);

        let check = robusta_with_path(&["check"], &temp.file);
        assert_eq!(check.status.code(), Some(1), "{}", stderr(&check));
        assert!(check.stderr.is_empty(), "{}", stderr(&check));
        assert_eq!(
            stdout_without_summary(&check),
            format!("would reformat: {}\n", presentation_path(&temp.file))
        );

        for command in ["format", "check"] {
            std::fs::write(&temp.file, &source).unwrap();
            let formatted = robusta_with_path(&[command, "--fix"], &temp.file);
            assert_eq!(formatted.status.code(), Some(0), "{}", stderr(&formatted));
            assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), "class Example {}\n");
        }
        let check = robusta_with_path(&["check"], &temp.file);
        assert_eq!(check.status.code(), Some(0), "{}", stderr(&check));
    }
}

#[test]
fn format_uses_nearest_config_for_indent_width() {
    let temp = TempJava::new("class  Example { int value ; }\n");
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "format:\n  line-width: 100\n  indent-width: 2\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    class Example {
      int value;
    }
    ");
}

#[test]
fn format_uses_editorconfig_fallbacks_for_tabs_line_endings_and_final_newline() {
    let temp = TempProject::new();
    temp.write(
        ".editorconfig",
        "root = true\n\n[*.java]\nindent_style = tab\ntab_width = 4\nend_of_line = lf\ninsert_final_newline = false\ntrim_trailing_whitespace = true\ncharset = utf-8-bom\n",
    );
    temp.write("Example.java", "class Example { int value; }\r\n");

    let output = robusta_in_dir(&["format", "--fix", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(),
        "class Example {\n\tint value;\n}"
    );
}

#[test]
fn editorconfig_zero_indent_size_formats_flush_left() {
    let temp = TempProject::new();
    temp.write(".editorconfig", "[*]\nindent_size = 0\n");
    temp.write("Example.java", "class Example { void m() { run(); } }\n");

    let output = robusta_in_dir(&["format", "--fix", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(),
        "class Example {\nvoid m() {\nrun();\n}\n}\n"
    );
}

#[test]
fn robusta_yaml_wins_over_editorconfig_where_it_speaks() {
    let temp = TempProject::new();
    temp.write(
        ".editorconfig",
        "[*]\nmax_line_length = 40\nindent_style = tab\ntab_width = 2\nend_of_line = crlf\ninsert_final_newline = false\n",
    );
    temp.write(
        "robusta.yaml",
        "format:\n  line-width: 100\n  indent-style: space\n  indent-width: 6\n  line-ending: lf\n  insert-final-newline: true\n",
    );
    temp.write("Example.java", "class Example { int value; }\r\n");

    let output = robusta_in_dir(&["format", "--fix", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(),
        "class Example {\n      int value;\n}\n"
    );
}

#[test]
fn editorconfig_sections_resolve_per_java_file() {
    let temp = TempProject::new();
    temp.write(
        ".editorconfig",
        "[*]\nindent_style = space\nindent_size = 2\n\n[Special.java]\nindent_style = tab\ntab_width = 8\n",
    );
    temp.write("Regular.java", "class Regular { int value; }\n");
    temp.write("Special.java", "class Special { int value; }\n");

    let output = robusta_in_dir(&["format", "--fix", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Regular.java")).unwrap(),
        "class Regular {\n  int value;\n}\n"
    );
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Special.java")).unwrap(),
        "class Special {\n\tint value;\n}\n"
    );
}

#[test]
fn editorconfig_discovery_is_independent_of_input_order_and_yaml_scope() {
    for yaml in [false, true] {
        let temp = TempProject::new();
        if yaml {
            temp.write("robusta.yaml", "format:\n  line-width: 100\n");
        }
        temp.write("a/.editorconfig", "[*]\nindent_size = 2\n");
        temp.write("b/.editorconfig", "[*.java]\nindent_size = 8\n");
        for paths in [
            vec!["a", "b", "c"],
            vec!["b", "a", "c"],
            vec!["c", "a", "b"],
            vec!["b/B.java", "c/C.java", "a/A.java"],
            vec!["."],
            vec![],
        ] {
            for no_nested in [false, true] {
                for (path, name) in [("a/A.java", "A"), ("b/B.java", "B"), ("c/C.java", "C")] {
                    temp.write(path, &format!("class {name} {{ int value; }}\n"));
                }
                let mut args = vec!["format", "--fix"];
                if no_nested {
                    args.push("--no-nested-config");
                }
                args.extend(&paths);
                let output = robusta_in_dir(&args, &temp.dir);
                assert_eq!(output.status.code(), Some(0), "{args:?}: {}", stderr(&output));
                assert!(output.stderr.is_empty(), "{}", stderr(&output));
                for (path, name, width) in
                    [("a/A.java", "A", 2), ("b/B.java", "B", 8), ("c/C.java", "C", 2)]
                {
                    assert_eq!(
                        std::fs::read_to_string(temp.dir.join(path)).unwrap(),
                        format!("class {name} {{\n{}int value;\n}}\n", " ".repeat(width)),
                        "{args:?}: {path}, yaml={yaml}",
                    );
                }
            }
        }
    }
}

#[test]
fn editorconfig_files_apply_up_to_the_first_root() {
    let temp = TempProject::new();
    temp.write(".editorconfig", "[*]\ninsert_final_newline = false\n");
    temp.write("outer/.editorconfig", "root = true\n[*]\nindent_size = 8\n");
    temp.write("outer/proj/.editorconfig", "[*.java]\nmax_line_length = 20\n");
    temp.write("outer/proj/sub/.editorconfig", "[*.java]\nindent_size = unset\n");
    let source = "class A { void m() { call(first, second); } }\n";
    let expected = |indent: &str| {
        format!(
            "class A {{\n{indent}void m() {{\n{indent}{indent}call(\n{indent}{indent}{indent}first,\n{indent}{indent}{indent}second\n{indent}{indent});\n{indent}}}\n}}\n"
        )
    };
    for (path, indent) in
        [("outer/proj/A.java", " ".repeat(8)), ("outer/proj/sub/B.java", " ".repeat(2))]
    {
        let output =
            robusta_with_stdin(&["format", "--fix", "--stdin-filepath", path], &temp.dir, source);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert_eq!(stdout(&output), expected(&indent), "stdin {path}");

        temp.write(path, source);
    }

    let output = robusta_in_dir(&["format", "--fix", "outer"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    for (path, indent) in
        [("outer/proj/A.java", " ".repeat(8)), ("outer/proj/sub/B.java", " ".repeat(2))]
    {
        assert_eq!(
            std::fs::read_to_string(temp.dir.join(path)).unwrap(),
            expected(&indent),
            "{path}"
        );
    }
}

#[test]
fn editorconfig_scopes_preserve_yaml_precedence_and_stdin_resolution() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "format:\n  indent-width: 6\n");
    temp.write("a/robusta.yaml", "format:\n  indent-width: 3\n");
    temp.write("a/.editorconfig", "[*]\nindent_size = 2\ninsert_final_newline = false\n");
    temp.write("b/.editorconfig", "[*]\nindent_size = 8\n");
    for (flags, a_width, b_width) in [
        (vec![], 3, 6),
        (vec!["--no-nested-config"], 6, 6),
        (vec!["--config", "a/robusta.yaml"], 3, 3),
    ] {
        for (path, name, width, newline) in
            [("a/A.java", "A", a_width, ""), ("b/B.java", "B", b_width, "\n")]
        {
            let source = format!("class {name} {{ int value; }}\n");
            let expected = format!("class {name} {{\n{}int value;\n}}{newline}", " ".repeat(width));
            temp.write(path, &source);
            let mut args = vec!["format", "--fix", "--stdin-filepath", path];
            args.extend(&flags);
            let output = robusta_with_stdin(&args, &temp.dir, &source);
            assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
            assert_eq!(stdout(&output), expected, "{args:?}");
            assert_eq!(std::fs::read_to_string(temp.dir.join(path)).unwrap(), source);

            let mut args = vec!["format", "--fix", path];
            args.extend(&flags);
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
            assert_eq!(std::fs::read_to_string(temp.dir.join(path)).unwrap(), expected, "{args:?}");
        }
    }
}

#[test]
fn editorconfig_invalid_values_preserve_earlier_sections() {
    let temp = TempProject::new();
    temp.write(
        ".editorconfig",
        "[*]\nindent_size = 3\n\n[*.java]\nindent_size = wide\n\n[Unset.java]\nindent_size = unset\n",
    );
    for name in ["One", "Two", "Unset"] {
        temp.write(&format!("{name}.java"), &format!("class {name} {{ int value; }}\n"));
    }

    let output = robusta_in_dir(&["format", "--fix", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    for (name, indent) in [("One", "   "), ("Two", "   "), ("Unset", "  ")] {
        assert_eq!(
            std::fs::read_to_string(temp.dir.join(format!("{name}.java"))).unwrap(),
            format!("class {name} {{\n{indent}int value;\n}}\n"),
        );
    }
    assert_eq!(stdout_without_summary(&output), "");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn editorconfig_unused_values_are_silently_ignored() {
    let temp = TempProject::new();
    for (width, charset) in [("off", "latin1"), ("wide", "unknown")] {
        temp.write(
            ".editorconfig",
            &format!("[*.java]\nindent_size = wide\nmax_line_length = {width}\nend_of_line = unknown\ninsert_final_newline = sometimes\ntrim_trailing_whitespace = false\ncharset = {charset}\nij_java_blank_lines_after_package = 2\n"),
        );
        for name in ["One", "Two"] {
            temp.write(&format!("{name}.java"), &format!("class {name} {{ int value; }}\n"));
        }

        let output = robusta_in_dir(&["format", "--fix", "."], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(output.stderr.is_empty(), "{}", stderr(&output));
        assert_eq!(stdout_without_summary(&output), "");
        for name in ["One", "Two"] {
            assert_eq!(
                std::fs::read_to_string(temp.dir.join(format!("{name}.java"))).unwrap(),
                format!("class {name} {{\n  int value;\n}}\n"),
            );
        }
    }
}

#[test]
fn editorconfig_width_outside_its_option_range_fails_once() {
    let temp = TempProject::new();
    temp.write(".editorconfig", "[*.java]\nmax_line_length = 1000\n");
    let source = "class Example { int value; }\n";
    for name in ["One", "Two"] {
        temp.write(&format!("{name}.java"), source);
    }
    let error = format!(
        "{}: invalid max_line_length: must be between 1 and 320; got 1000\n",
        working_dir_spelling(&temp.dir).join(".editorconfig").display()
    );

    for command in ["format", "check"] {
        let output = robusta_in_dir(&[command, "--fix", "."], &temp.dir);

        assert_eq!(output.status.code(), Some(2), "{command}: {}", stderr(&output));
        assert_eq!(stderr(&output), error, "{command}");
        assert!(output.stdout.is_empty(), "{command}: {}", stdout(&output));
        for name in ["One", "Two"] {
            assert_eq!(
                std::fs::read_to_string(temp.dir.join(format!("{name}.java"))).unwrap(),
                source
            );
        }
    }

    temp.write("robusta.yaml", "format:\n  line-width: 100\n");
    let output = robusta_in_dir(&["format", "--fix", "."], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
}

#[test]
fn format_config_override_uses_named_file_and_its_ignore_root() {
    let temp = TempProject::new();
    temp.write(
        "policy/custom.yaml",
        "format:\n  indent-width: 2\nignore-patterns:\n  - /src/Skipped.java\n",
    );
    temp.write("policy/src/robusta.yaml", "format:\n  indent-width: 6\n");
    temp.write("policy/src/Example.java", "class Example { int value; }\n");
    temp.write("policy/src/Skipped.java", "class Skipped { int value; }\n");

    let output = robusta_in_dir(
        &["format", "--fix", "--config", "policy/custom.yaml", "policy/src"],
        &temp.dir,
    );

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(temp.dir.join("policy/src/Example.java")).unwrap(), @"
    class Example {
      int value;
    }
    ");
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("policy/src/Skipped.java")).unwrap(),
        "class Skipped { int value; }\n"
    );
}

#[test]
fn format_uses_configured_after_closing_brace() {
    let temp =
        TempJava::new("class Example { void f() { if (ready) { run(); } else { skip(); } } }\n");
    std::fs::write(temp.dir.join("robusta.yaml"), "format:\n  after-closing-brace: new-line\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    class Example {
      void f() {
        if (ready) {
          run();
        }
        else {
          skip();
        }
      }
    }
    ");
}

#[test]
fn format_uses_configured_import_order_groups() {
    let temp = TempJava::new(
        r"import static z.Tools.z;
import static org.Tools.o;
import static java.util.Collections.emptyList;
import module java.base;
import com.other.Z;
import org.beta.B;
import javax.inject.Inject;
import java.util.List;
import org.alpha.A;

class Uses {}
",
    );
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        r#"
format:
  import-order:
    groups:
      - ["java", "javax"]
      - "org"
"#,
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    import module java.base;

    import java.util.List;
    import javax.inject.Inject;

    import org.alpha.A;
    import org.beta.B;

    import com.other.Z;

    import static java.util.Collections.emptyList;

    import static org.Tools.o;

    import static z.Tools.z;

    class Uses {}
    ");
}

#[test]
fn format_appends_unlisted_import_sections() {
    let temp = TempJava::new(
        "import java.util.List; import module java.base; import static java.util.List.of; \
         class Uses {}\n",
    );
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "format:\n  import-order:\n    sections: [static]\n",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    import static java.util.List.of;

    import module java.base;

    import java.util.List;

    class Uses {}
    ");

    let verify = robusta_with_path(&["format"], &temp.file);
    assert_eq!(verify.status.code(), Some(0), "{}", stdout(&verify));
    assert!(stdout_without_summary(&verify).is_empty(), "{}", stdout(&verify));
    assert!(verify.stderr.is_empty(), "{}", stderr(&verify));
}

#[test]
fn format_composes_layout_options_through_yaml() {
    let temp = TempJava::new(
        "import z.Normal; import static z.Tools.value; class Uses { void f() { \
         extremelyLongTargetName = call(alphaAlphaAlpha, betaBetaBeta, gammaGammaGamma); } }\n",
    );
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        r#"
format:
  line-width: 60
  indent-width: 2
  import-order:
    sections: [static, module, type]
    groups: ["*"]
"#,
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    import static z.Tools.value;

    import z.Normal;

    class Uses {
      void f() {
        extremelyLongTargetName = call(
          alphaAlphaAlpha,
          betaBetaBeta,
          gammaGammaGamma
        );
      }
    }
    ");
}

#[test]
fn format_resolves_declaration_clause_indentation_through_yaml() {
    let temp = TempJava::new(
        "class Service implements AlphaContract, BetaContract, GammaContract { \
         void run() throws AlphaFailureException, BetaFailureException {} }\n",
    );
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        r"
format:
  line-width: 48
  indent-width: 2
",
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);

    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout_without_summary(&output).is_empty(), "{}", stdout(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    insta::assert_snapshot!(std::fs::read_to_string(&temp.file).unwrap(), @"
    class Service
        implements AlphaContract,
            BetaContract,
            GammaContract {
      void run()
          throws AlphaFailureException,
              BetaFailureException {}
    }
    ");
}

#[test]
fn format_reports_malformed_config() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "format:\n  line_widht: 100\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["format"], &temp.file);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"));
    assert!(stderr.contains("line_widht"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_rejects_unknown_config_root_without_processing_files() {
    let source = "class Example { int value ; }\n";
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "foramt:\n  line-width: 100\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["format", "--fix"], &temp.file);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"));
    assert!(stderr.contains("unknown field `foramt`"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_reports_invalid_formatter_option_values() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "format:\n  line-width: 0\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["format"], &temp.file);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"));
    assert!(stderr.contains("format.line-width"));
    assert!(stderr.contains("between 1 and 320; got 0"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_reports_invalid_import_order_groups() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        r#"
format:
  import-order:
    groups:
      - "java"
      - "java"
"#,
    )
    .expect("failed to write config");

    let output = robusta_with_path(&["format"], &temp.file);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"));
    assert!(stderr.contains("import group \"java\" is duplicated"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_rejects_the_removed_files_section() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);
    std::fs::write(temp.dir.join("robusta.yaml"), "files:\n  exclude:\n    - \"build/**\"\n")
        .expect("failed to write config");

    let output = robusta_with_path(&["format"], &temp.file);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr.contains("robusta.yaml"));
    assert!(stderr.contains("unknown field `files`"), "{stderr}");
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn long_binary_chains_format_and_check_on_worker_threads() {
    let project = TempProject::new();
    let source = format!("class Example {{ String value = \"\"{}; }}\n", " + \"x\"".repeat(20_000));
    project.write("First.java", &source);
    project.write("Second.java", &source);

    for command in ["format", "check"] {
        let output = robusta_in_dir(&[command, "--no-nested-config", "."], &project.dir);
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stderr(&output));
    }
    let fixed = robusta_in_dir(&["format", "--fix", "--no-nested-config", "."], &project.dir);
    assert_eq!(fixed.status.code(), Some(0), "{}", stderr(&fixed));
    let first = std::fs::read_to_string(project.dir.join("First.java")).unwrap();
    assert_eq!(first.matches("+ \"x\"").count(), 20_000);
    assert_eq!(first, std::fs::read_to_string(project.dir.join("Second.java")).unwrap());
    for command in ["format", "check"] {
        let output = robusta_in_dir(&[command, "--no-nested-config", "."], &project.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
    }
}

#[test]
fn long_array_member_chains_format_and_check_on_worker_threads() {
    let project = TempProject::new();
    for name in ["A", "B"] {
        let source = format!(
            "class {name} {{ {name}[] f() {{ return new {name}[] {{ this }}; }} {name} n = this{}; }}\n",
            ".f()[0]".repeat(10_000)
        );
        project.write(&format!("{name}.java"), &source);
    }
    for command in ["format", "check"] {
        let output = robusta_in_dir(&[command, "--threads", "2", "."], &project.dir);
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stderr(&output));
    }
    let fixed = robusta_in_dir(&["format", "--fix", "--threads", "2", "."], &project.dir);
    assert_eq!(fixed.status.code(), Some(0), "{}", stderr(&fixed));
    for name in ["A", "B"] {
        let formatted = std::fs::read_to_string(project.dir.join(format!("{name}.java"))).unwrap();
        assert_eq!(formatted.matches(".f()[0]").count(), 10_000);
    }
    for command in ["format", "check"] {
        let output = robusta_in_dir(&[command, "--threads", "2", "."], &project.dir);
        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
    }
}

#[test]
fn long_receiver_chains_lint_and_check_on_worker_threads() {
    for tail in [".f()", ".f().field"] {
        let project = TempProject::new();
        let source = format!("class Example {{ Object value = f(){}; }}\n", tail.repeat(16_000));
        project.write("First.java", &source);
        project.write("Second.java", &source);
        for (command, code) in [("lint", 0), ("check", 1)] {
            let output = robusta_in_dir(&[command, "--threads", "2", "."], &project.dir);
            assert_eq!(output.status.code(), Some(code), "{tail}/{command}: {}", stderr(&output));
        }
    }
}

#[test]
fn long_dotted_names_format_and_lint_on_worker_threads() {
    let project = TempProject::new();
    let dots = ".a".repeat(20_000);
    let source =
        format!("import a{dots};\n@a{dots} class E {{ a{dots} t; Object v = a{dots}; }}\n");
    project.write("First.java", &source);
    project.write("Second.java", &source);
    let fixed = robusta_in_dir(&["format", "--fix", "--threads", "2", "."], &project.dir);
    assert_eq!(fixed.status.code(), Some(0), "{}", stderr(&fixed));
    let formatted = std::fs::read_to_string(project.dir.join("First.java")).unwrap();
    assert_eq!(formatted.matches(".a").count(), 80_000);
    for (command, code) in [("format", 0), ("lint", 0)] {
        let output = robusta_in_dir(&[command, "--threads", "2", "."], &project.dir);
        assert_eq!(output.status.code(), Some(code), "{command}: {}", stderr(&output));
    }
}

#[test]
fn format_rejects_removed_declaration_clause_option() {
    let source = "class Example { int value ; }\n";
    let temp = TempJava::new(source);
    std::fs::write(
        temp.dir.join("robusta.yaml"),
        "format:\n  wrapping:\n    declaration-clauses: fill\n",
    )
    .unwrap();
    let output = robusta_with_path(&["format", "--fix"], &temp.file);
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("wrapping"), "{}", stderr(&output));
    assert_eq!(output.stdout.len(), 0);
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}
