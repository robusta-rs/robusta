use crate::support::*;

#[test]
fn config_files_lists_sorted_unique_paths_without_processing_source() {
    let temp = TempProject::new();
    let source = "not valid Java";
    let file = temp.write("src/Z.java", source);
    temp.write("src/A.java", source);
    temp.write("B.java", source);
    let absolute = working_dir_spelling(&temp.dir).join("src/Z.java");

    let output =
        robusta_in_dir(&["config", "--files", absolute.to_str().unwrap(), "./src", "."], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "B.java\nsrc/A.java\nsrc/Z.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), source);
}

#[test]
fn config_files_uses_config_gitignore_and_additional_ignore_paths() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "ignore-patterns:\n  - config/\n");
    temp.write(".gitignore", "git/\n");
    temp.write(".robustaignore", "extra/\n");
    for directory in ["config", "git", "extra", "src"] {
        temp.write(&format!("{directory}/Example.java"), "not valid Java");
    }

    let output =
        robusta_in_dir(&["config", "--files", "--ignore-path", ".robustaignore"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "src/Example.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn config_files_leaves_build_output_to_ignore_rules() {
    let temp = TempProject::new();
    temp.write("src/main/java/com/acme/build/Builder.java", "not valid Java");
    temp.write(".mvn/wrapper/Wrapper.java", "not valid Java");
    temp.write("build/generated/Generated.java", "not valid Java");

    let output = robusta_in_dir(&["config", "--files", "."], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        ".mvn/wrapper/Wrapper.java\n\
         build/generated/Generated.java\n\
         src/main/java/com/acme/build/Builder.java\n"
    );

    temp.write(".gitignore", "/build/\n");
    let output = robusta_in_dir(&["config", "--files", "."], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        ".mvn/wrapper/Wrapper.java\nsrc/main/java/com/acme/build/Builder.java\n"
    );
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn config_files_respects_nested_config_selection() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "{}\n");
    temp.write("Root.java", "not valid Java");
    temp.write("nested/robusta.yaml", "ignore-patterns:\n  - /Hidden.java\n");
    temp.write("nested/Hidden.java", "not valid Java");

    let nested = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(nested.status.code(), Some(0), "{}", stderr(&nested));
    assert_eq!(stdout(&nested), "Root.java\n");
    assert!(nested.stderr.is_empty(), "{}", stderr(&nested));

    let root = robusta_in_dir(&["config", "--files", "--no-nested-config"], &temp.dir);
    assert_eq!(root.status.code(), Some(0), "{}", stderr(&root));
    assert_eq!(stdout(&root), "Root.java\nnested/Hidden.java\n");
    assert!(root.stderr.is_empty(), "{}", stderr(&root));
}

#[test]
fn config_files_uses_an_explicit_config() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "{}\n");
    temp.write("policy/robusta.yaml", "ignore-patterns:\n  - /Hidden.java\n");
    temp.write("policy/Hidden.java", "not valid Java");
    temp.write("policy/Kept.java", "not valid Java");

    let output = robusta_in_dir(&["config", "--files", "-c", "policy/robusta.yaml"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "policy/Kept.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn config_files_lists_the_files_either_tool_keeps() {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "ignore-patterns: [shared/]
format:
  ignore-patterns: [lint-only/, neither/, \"!shared/\"]
lint:
  ignore-patterns: [format-only/, neither/, \"!shared/A.java\"]
",
    );
    for directory in ["both", "format-only", "lint-only", "neither", "shared"] {
        temp.write(&format!("{directory}/A.java"), "not valid Java");
    }

    let output = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "both/A.java\nformat-only/A.java\nlint-only/A.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));

    let output = robusta_in_dir(
        &["config", "--files", "neither/A.java", "shared/A.java", "lint-only/A.java"],
        &temp.dir,
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "lint-only/A.java\n");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[test]
fn config_files_uses_tool_ignore_patterns_from_the_selected_config() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "{}\n");
    temp.write(
        "nested/robusta.yaml",
        "format:\n  ignore-patterns: [/Hidden.java, /Linted.java]\nlint:\n  ignore-patterns: [/Hidden.java]\n",
    );
    for name in ["Root.java", "nested/Hidden.java", "nested/Linted.java"] {
        temp.write(name, "not valid Java");
    }

    for (options, expected) in [
        (&[][..], "Root.java\nnested/Linted.java\n"),
        (&["--no-nested-config"], "Root.java\nnested/Hidden.java\nnested/Linted.java\n"),
        (&["-c", "nested/robusta.yaml"], "Root.java\nnested/Linted.java\n"),
    ] {
        let args = [&["config", "--files"][..], options].concat();
        let output = robusta_in_dir(&args, &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{options:?}: {}", stderr(&output));
        assert_eq!(stdout(&output), expected, "{options:?}");
        assert!(output.stderr.is_empty(), "{options:?}: {}", stderr(&output));
    }
}

#[test]
fn config_files_fails_on_an_empty_selection() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["config", "--files"], &temp.dir);
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stderr(&output).contains("Expected at least one target file."));
    assert_eq!(output.stdout.len(), 0);
}

#[test]
fn selection_arguments_require_config_files() {
    let temp = TempProject::new();
    temp.write("Example.java", "not valid Java");

    for args in [
        &["config", "Example.java"][..],
        &["config", "--ignore-path", ".robustaignore"],
        &["config", "--ignore-pattern", "Example.java"],
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
    }
}

/// The spellings this surface replaced fail as usage errors, before any
/// configuration is read or any file is written.
#[test]
fn replaced_spellings_are_rejected_before_any_file_is_touched() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "invalid: [config");
    temp.write("Example.java", "not valid Java");

    for args in [
        &["config", "show"][..],
        &["config", "init"],
        &["config", "init", "--force"],
        &["check", "--debug=files"],
        &["format", "--debug=files"],
        &["lint", "--debug", "files"],
    ] {
        let output = robusta_in_dir(args, &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {}", stderr(&output));
        assert!(output.stdout.is_empty(), "{args:?}: {}", stdout(&output));
        assert!(!stderr(&output).contains("robusta.yaml"), "{args:?}: {}", stderr(&output));
    }
    assert_eq!(std::fs::read_to_string(temp.dir.join("robusta.yaml")).unwrap(), "invalid: [config");
}

#[test]
fn config_files_lists_only_the_java_files_among_its_paths() {
    let temp = TempProject::new();
    let source = "not valid Java";
    temp.write("src/A.java", source);
    temp.write("src/A.txt", source);
    temp.write("src/A", source);
    let root = working_dir_spelling(&temp.dir);
    let absolute_java = root.join("src/A.java");
    let absolute_text = root.join("src/A.txt");

    let selections: [&[&str]; 3] = [
        &["src/A.txt", "src/A", "src/A.java"],
        &[absolute_text.to_str().unwrap(), absolute_java.to_str().unwrap()],
        &["src", "src/A.txt"],
    ];
    for selection in selections {
        let output = robusta_in_dir(&[&["config", "--files"], selection].concat(), &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{selection:?}: {}", stderr(&output));
        assert_eq!(stdout(&output), "src/A.java\n", "{selection:?}");
    }

    let only_other = robusta_in_dir(&["config", "--files", "src/A.txt"], &temp.dir);
    assert_eq!(only_other.status.code(), Some(2), "{}", stderr(&only_other));
    assert!(stderr(&only_other).contains("Expected at least one target file."));
    assert_eq!(only_other.stdout.len(), 0);
}
