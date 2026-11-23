use crate::support::*;

#[test]
fn version_prints_package_version() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["--version"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), concat!("Version: ", env!("CARGO_PKG_VERSION"), "\n\n"));
    assert_eq!(stderr(&output), "");
}

// Help/usage text embeds the argv[0] program name (`robusta.exe` on Windows)
// and its fixed-width wrapping, so this exact snapshot is asserted only on the
// primary platform.
#[cfg(not(windows))]
#[test]
fn generated_help_defines_the_public_command_surface() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    Format and lint Java source files.

    Usage: robusta COMMAND ...

    Available options:
        -h, --help     Prints help information
        -V, --version  Prints version information

    Available commands:
        check          Verify Java formatting and lints; optionally preview or apply fixes.
        format         Verify Java formatting; optionally preview or apply changes.
        lint           Verify enabled lint rules; optionally preview or apply transformations.
        init           Create `robusta.yaml` in the current directory, failing if it exists.
        config         Print the resolved configuration for the current directory as YAML.
    ");
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn missing_and_unknown_commands_are_argument_errors() {
    let temp = TempProject::new();

    // Without a command, the root help is the usage error: the text `--help`
    // prints, moved to standard error with a failing status.
    let missing = robusta_in_dir(&[], &temp.dir);
    let help = robusta_in_dir(&["--help"], &temp.dir);
    assert_eq!(missing.status.code(), Some(2));
    assert_eq!(missing.stdout.len(), 0);
    assert_eq!(help.status.code(), Some(0));
    assert_eq!(help.stderr.len(), 0);
    assert_eq!(missing.stderr, help.stdout);

    let unknown = robusta_in_dir(&["formt", "."], &temp.dir); // spellchecker:disable-line
    assert_eq!(unknown.status.code(), Some(2));
    assert_eq!(unknown.stdout.len(), 0);
    assert!(stderr(&unknown).contains("no such command or positional: `formt`")); // spellchecker:disable-line
}

// Primary-platform-only: see `generated_help_defines_the_public_command_surface`.
#[cfg(not(windows))]
#[test]
fn format_uses_generated_command_help() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["format", "--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @"
    Verify Java formatting; optionally preview or apply changes.

    Usage: robusta format [-c=PATH] [--no-nested-config] [--ignore-path=PATH]... [--ignore-pattern=PAT
    ]... [--fix] [--diff | --list-different] [--reporter=FORMAT] [--debug=OPTIONS] [--quiet] [
    --deny-warnings] [--max-warnings=INT] [--silent] [--no-error-on-unmatched-pattern] [--threads=INT] [
    --color=WHEN] [--stdin-filepath=PATH] [PATH]...

    Configuration options:
        -c, --config=PATH         Use this configuration file instead of discovering `robusta.yaml`.
            --no-nested-config    Use one configuration discovered from the current directory.

    Ignore options:
            --ignore-path=PATH    Path to an additional gitignore-syntax file. Can be specified multiple
                                  times.
            --ignore-pattern=PAT  Gitignore-syntax pattern to exclude, resolved from the working
                                  directory. Quote it so the shell does not expand it. These patterns
                                  are ordered and outrank `--ignore-path` files: the last one to match
                                  decides, and a leading `!` reincludes. Can be specified multiple
                                  times.

    Warning options:
            --quiet               Disable reporting on warnings; only errors are reported.
            --deny-warnings       Ensure warnings produce a non-zero exit code.
            --max-warnings=INT    Fail when warning-level rule violations exceed this threshold.
            --silent              Do not display any diagnostics.

    Runtime options:
            --no-error-on-unmatched-pattern  Do not exit with an error when no files are selected.
            --threads=INT         Number of threads to use. Set to 1 for using only 1 CPU core.

    Color options:
            --color=WHEN          When to use colors: auto, always, or never.

    Available positional items:
        PATH                      Java source file or directory to process. Defaults to the current
                                  directory. Paths must not contain a `..` component.

    Available options:
            --fix                 Rewrite files in place; with --stdin-filepath, print the formatted
                                  document instead.
            --diff                Print a unified diff for files that would be changed, and the report
                                  to stderr.
            --list-different      Print only the paths of files that would be changed, and the report to
                                  stderr.
            --reporter=FORMAT     Report format: default, json, checkstyle, github, gitlab,
                                  azure-devops, unix, agent, sarif, stylish, or junit.
            --debug=OPTIONS       Enable comma-separated debug options: timings reports per-rule lint
                                  timing.
            --stdin-filepath=PATH  Read one document from stdin instead of selecting files, resolving
                                  configuration against PATH. --fix prints the result to stdout and the
                                  report to stderr; no file is written.
        -h, --help                Prints help information
    ");
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn every_command_exposes_color_control() {
    let temp = TempProject::new();

    for command in [
        &["check", "--help"][..],
        &["format", "--help"],
        &["lint", "--help"],
        &["init", "--help"],
        &["config", "--help"],
    ] {
        let output = robusta_in_dir(command, &temp.dir);
        let help = stdout(&output);

        assert_eq!(output.status.code(), Some(0), "{command:?}: {}", stderr(&output));
        assert!(help.contains("--color=WHEN"), "{command:?}: {help}");
        assert!(
            help.contains("When to use colors: auto, always, or never."),
            "{command:?}: {help}"
        );
        assert!(output.stderr.is_empty(), "{command:?}: {}", stderr(&output));
    }
}

#[test]
fn every_file_command_uses_oxc_threads_help() {
    let temp = TempProject::new();

    for command in ["check", "format", "lint"] {
        let output = robusta_in_dir(&[command, "--help"], &temp.dir);
        let help = stdout(&output);

        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        assert!(help.contains("--threads=INT"), "{command}: {help}");
        assert!(
            help.contains("Number of threads to use. Set to 1 for using only 1 CPU core."),
            "{command}: {help}"
        );
        assert!(output.stderr.is_empty(), "{command}: {}", stderr(&output));
    }
}

#[test]
fn lint_bearing_commands_expose_warning_controls() {
    let temp = TempProject::new();

    for command in ["check", "lint"] {
        let output = robusta_in_dir(&[command, "--help"], &temp.dir);
        let help = stdout(&output);

        assert_eq!(output.status.code(), Some(0), "{command}: {}", stderr(&output));
        for option in ["--quiet", "--deny-warnings", "--max-warnings=INT", "--silent"] {
            assert!(help.contains(option), "{command}: {help}");
        }
        assert!(output.stderr.is_empty(), "{command}: {}", stderr(&output));
    }
}

#[test]
fn removed_config_spellings_are_rejected() {
    let temp = TempProject::new();

    for command in ["check", "lint"] {
        let output = robusta_in_dir(&[command, "--print-config"], &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{command}: {}", stderr(&output));
        assert!(output.stdout.is_empty(), "{command}");
        assert!(
            stderr(&output).contains("`--print-config` is not expected in this context"),
            "{command}: {}",
            stderr(&output)
        );
    }
}

#[test]
fn fmt_alias_is_rejected() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["fmt"], &temp.dir);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("expected `COMMAND ...`, got `fmt`"));
}

#[test]
fn fix_command_is_rejected() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["fix", "."], &temp.dir);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("expected `COMMAND ...`, got `fix`"));
}

// Primary-platform-only: see `generated_help_defines_the_public_command_surface`.
#[cfg(not(windows))]
#[test]
fn lint_help_documents_fix_mode() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["lint", "--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @r#"
    Verify enabled lint rules; optionally preview or apply transformations.

    Usage: robusta lint [-c=PATH] [--no-nested-config] [--ignore-path=PATH]... [--ignore-pattern=PAT]...
    [-A=NAME | -W=NAME | -D=NAME]... [--fix] [--unsafe-fixes] [--diff | --list-different] [
    --suppress-all] [--prune-suppressions] [--report-unused-suppressions |
    --report-unused-suppressions-severity=SEVERITY] [--reporter=FORMAT] [--debug=OPTIONS] [--quiet] [
    --deny-warnings] [--max-warnings=INT] [--silent] [--no-error-on-unmatched-pattern] [--threads=INT] [
    --color=WHEN] [--stdin-filepath=PATH] [PATH]...

    Configuration options:
        -c, --config=PATH         Use this configuration file instead of discovering `robusta.yaml`.
            --no-nested-config    Use one configuration discovered from the current directory.

    Ignore options:
            --ignore-path=PATH    Path to an additional gitignore-syntax file. Can be specified multiple
                                  times.
            --ignore-pattern=PAT  Gitignore-syntax pattern to exclude, resolved from the working
                                  directory. Quote it so the shell does not expand it. These patterns
                                  are ordered and outrank `--ignore-path` files: the last one to match
                                  decides, and a leading `!` reincludes. Can be specified multiple
                                  times.

    Rule filter options, applied over the configuration, left to right:
       For example `-D style -A uppercase-long-suffix`.
       The categories are:
       * `correctness` - Code that is wrong or useless.
       * `style` - Consistency and readability concerns with no behavioral risk.
       * `suspicious` - Constructs that commonly indicate stale or misleading code.
       * `all` - All registered rules.
        -A, --allow=NAME          Disable a rule, a category, or `all` for this run.
        -W, --warn=NAME           Set a rule, a category, or `all` to warn for this run.
        -D, --deny=NAME           Set a rule, a category, or `all` to deny for this run.

    Fix options:
            --fix                 Fix as many issues as possible. Only unfixed issues are reported.
            --unsafe-fixes        Include unsafe fixes, which may change what the code does or stop it
                                  compiling, in what --fix writes and what --diff and --list-different
                                  preview.

    Warning options:
            --quiet               Disable reporting on warnings; only errors are reported.
            --deny-warnings       Ensure warnings produce a non-zero exit code.
            --max-warnings=INT    Fail when warning-level rule violations exceed this threshold.
            --silent              Do not display any diagnostics.

    Runtime options:
            --no-error-on-unmatched-pattern  Do not exit with an error when no files are selected.
            --threads=INT         Number of threads to use. Set to 1 for using only 1 CPU core.

    Color options:
            --color=WHEN          When to use colors: auto, always, or never.

    Available positional items:
        PATH                      Java source file or directory to process. Defaults to the current
                                  directory. Paths must not contain a `..` component.

    Available options:
            --rules               List registered rules using the default or JSON reporter.
            --diff                Print a unified diff for files that would be changed, and the report
                                  to stderr.
            --list-different      Print only the paths of files that would be changed, and the report to
                                  stderr.
            --suppress-all        Record each file's error counts per rule in
                                  `robusta-suppressions.json`. With `--fix`, fix first and record the
                                  errors that remain.
            --prune-suppressions  Lower or remove obsolete counts in `robusta-suppressions.json`; never
                                  raise them.
            --report-unused-suppressions  Report suppressions like `@SuppressWarnings("robusta:<rule>")`
                                  and `// robusta-disable-line` that match no finding. --fix removes
                                  Robusta's own entries and keeps shared Java warning names.
            --report-unused-suppressions-severity=SEVERITY  Same as --report-unused-suppressions, at
                                  severity allow, warn, or deny. Overrides the configuration.
            --reporter=FORMAT     Report format: default, json, checkstyle, github, gitlab,
                                  azure-devops, unix, agent, sarif, stylish, or junit.
            --debug=OPTIONS       Enable comma-separated debug options: timings reports per-rule lint
                                  timing.
            --stdin-filepath=PATH  Read one document from stdin instead of selecting files, resolving
                                  configuration against PATH. --fix prints the result to stdout and the
                                  report to stderr; no file is written.
        -h, --help                Prints help information
    "#);
    assert_eq!(output.stderr.len(), 0);
}

// Primary-platform-only: see `generated_help_defines_the_public_command_surface`.
#[cfg(not(windows))]
#[test]
fn check_help_documents_fix_mode() {
    let temp = TempProject::new();

    let output = robusta_in_dir(&["check", "--help"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output), @r#"
    Verify Java formatting and lints; optionally preview or apply fixes.

    Usage: robusta check [-c=PATH] [--no-nested-config] [--ignore-path=PATH]... [--ignore-pattern=PAT
    ]... [-A=NAME | -W=NAME | -D=NAME]... [--fix] [--unsafe-fixes] [--diff | --list-different] [
    --prune-suppressions] [--report-unused-suppressions | --report-unused-suppressions-severity=SEVERITY
    ] [--reporter=FORMAT] [--debug=OPTIONS] [--quiet] [--deny-warnings] [--max-warnings=INT] [--silent]
    [--no-error-on-unmatched-pattern] [--threads=INT] [--color=WHEN] [--stdin-filepath=PATH] [PATH]...

    Configuration options:
        -c, --config=PATH         Use this configuration file instead of discovering `robusta.yaml`.
            --no-nested-config    Use one configuration discovered from the current directory.

    Ignore options:
            --ignore-path=PATH    Path to an additional gitignore-syntax file. Can be specified multiple
                                  times.
            --ignore-pattern=PAT  Gitignore-syntax pattern to exclude, resolved from the working
                                  directory. Quote it so the shell does not expand it. These patterns
                                  are ordered and outrank `--ignore-path` files: the last one to match
                                  decides, and a leading `!` reincludes. Can be specified multiple
                                  times.

    Rule filter options, applied over the configuration, left to right:
       For example `-D style -A uppercase-long-suffix`.
       The categories are:
       * `correctness` - Code that is wrong or useless.
       * `style` - Consistency and readability concerns with no behavioral risk.
       * `suspicious` - Constructs that commonly indicate stale or misleading code.
       * `all` - All registered rules.
        -A, --allow=NAME          Disable a rule, a category, or `all` for this run.
        -W, --warn=NAME           Set a rule, a category, or `all` to warn for this run.
        -D, --deny=NAME           Set a rule, a category, or `all` to deny for this run.

    Fix options:
            --fix                 Fix as many issues as possible. Only unfixed issues are reported.
            --unsafe-fixes        Include unsafe fixes, which may change what the code does or stop it
                                  compiling, in what --fix writes and what --diff and --list-different
                                  preview.

    Warning options:
            --quiet               Disable reporting on warnings; only errors are reported.
            --deny-warnings       Ensure warnings produce a non-zero exit code.
            --max-warnings=INT    Fail when warning-level rule violations exceed this threshold.
            --silent              Do not display any diagnostics.

    Runtime options:
            --no-error-on-unmatched-pattern  Do not exit with an error when no files are selected.
            --threads=INT         Number of threads to use. Set to 1 for using only 1 CPU core.

    Color options:
            --color=WHEN          When to use colors: auto, always, or never.

    Available positional items:
        PATH                      Java source file or directory to process. Defaults to the current
                                  directory. Paths must not contain a `..` component.

    Available options:
            --diff                Print a unified diff for files that would be changed, and the report
                                  to stderr.
            --list-different      Print only the paths of files that would be changed, and the report to
                                  stderr.
            --prune-suppressions  Lower or remove obsolete counts in `robusta-suppressions.json`; never
                                  raise them.
            --report-unused-suppressions  Report suppressions like `@SuppressWarnings("robusta:<rule>")`
                                  and `// robusta-disable-line` that match no finding. --fix removes
                                  Robusta's own entries and keeps shared Java warning names.
            --report-unused-suppressions-severity=SEVERITY  Same as --report-unused-suppressions, at
                                  severity allow, warn, or deny. Overrides the configuration.
            --reporter=FORMAT     Report format: default, json, checkstyle, github, gitlab,
                                  azure-devops, unix, agent, sarif, stylish, or junit.
            --debug=OPTIONS       Enable comma-separated debug options: timings reports per-rule lint
                                  timing.
            --stdin-filepath=PATH  Read one document from stdin instead of selecting files, resolving
                                  configuration against PATH. --fix prints the result to stdout and the
                                  report to stderr; no file is written.
        -h, --help                Prints help information
    "#);
    assert_eq!(output.stderr.len(), 0);
}

#[test]
fn format_rejects_unknown_fix_flag_typo() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "--fx"], &temp.file);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("no such flag: `--fx`"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_write_flag_is_rejected() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "--write"], &temp.file);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("no such flag: `--write`"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn format_short_write_flag_is_rejected() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["format", "-w"], &temp.file);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("`-w` is not expected in this context"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}

#[test]
fn check_rejects_unknown_flag() {
    let temp = TempJava::new("class Example { int value; }\n");

    let output = robusta_with_path(&["check", "-x"], &temp.file);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("`-x` is not expected in this context"));
}

#[test]
fn apply_command_is_rejected() {
    let source = "class Example { int value; }\n";
    let temp = TempJava::new(source);

    let output = robusta_with_path(&["apply"], &temp.file);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    assert!(stderr(&output).contains("expected `COMMAND ...`, got `apply`"));
    assert_eq!(std::fs::read_to_string(&temp.file).unwrap(), source);
}
