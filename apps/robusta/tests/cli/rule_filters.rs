use crate::support::*;

/// The `<severity> <rule-id>` pairs a run reported, in order. The human
/// reporter opens each rendered finding with its severity marker and rule ID,
/// so this reads the surface a user actually sees rather than a count.
fn findings(output: &std::process::Output) -> Vec<(char, String)> {
    stdout_without_summary(output)
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let (marker, rest) = line.split_at(line.char_indices().nth(1)?.0);
            let marker = marker.chars().next()?;
            if marker != '!' && marker != 'x' {
                return None;
            }
            let rule = rest.trim_start().split_once(':')?.0;
            rule.chars()
                .all(|character| character.is_ascii_lowercase() || character == '-')
                .then(|| (marker, rule.to_owned()))
        })
        .collect()
}

fn rules_reported(output: &std::process::Output) -> Vec<String> {
    let mut rules = findings(output).into_iter().map(|(_, rule)| rule).collect::<Vec<_>>();
    rules.sort_unstable();
    rules.dedup();
    rules
}

const SOURCE: &str = "class Example {\n  static public long value = 10l;\n}\n";

/// Both offending rules are `style`, which is what lets one category filter and
/// one rule filter overlap in these orderings.
const MODIFIER_ORDER: &str = "modifier-order";
const UPPERCASE_LONG_SUFFIX: &str = "uppercase-long-suffix";

const UNUSED_SOURCE: &str =
    "import java.util.List;\n\nclass Example {\n  void f() {\n    int unused = 1;\n  }\n}\n";

#[test]
fn correctness_defaults_cover_disk_stdin_and_empty_configurations() {
    for config in [
        None,
        Some(""),
        Some("format: {line-width: 100}\n"),
        Some("lint: {}\n"),
        Some("lint: {categories: {}}\n"),
        Some("lint: {rules: {}}\n"),
    ] {
        let temp = TempProject::new();
        temp.write("Example.java", UNUSED_SOURCE);
        if let Some(config) = config {
            temp.write("robusta.yaml", config);
        }
        for command in ["lint", "check"] {
            let disk = robusta_in_dir(&[command, "Example.java"], &temp.dir);
            let stdin = robusta_with_stdin(
                &[command, "--stdin-filepath", "Example.java"],
                &temp.dir,
                UNUSED_SOURCE,
            );
            for output in [disk, stdin] {
                assert_eq!(output.status.code(), Some(0), "{config:?}: {}", stderr(&output));
                assert_eq!(
                    findings(&output),
                    [('!', "unused-import".to_owned()), ('!', "unused-local-variable".to_owned())],
                    "{config:?}: {}",
                    stdout(&output)
                );
            }
        }
        let resolved = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(resolved.status.code(), Some(0), "{}", stderr(&resolved));
        temp.write("robusta.yaml", &stdout(&resolved));
        let round_trip = robusta_in_dir(&["lint", "Example.java"], &temp.dir);
        assert_eq!(round_trip.status.code(), Some(0), "{}", stderr(&round_trip));
        assert_eq!(rules_reported(&round_trip), ["unused-import", "unused-local-variable"]);
    }
}

#[test]
fn categories_then_rules_then_ordered_cli_filters_resolve_each_input() {
    for (category, rule, filters, expected) in [
        ("allow", "allow", &[][..], &[][..]),
        ("warn", "allow", &[][..], &[('!', "unused-local-variable")][..]),
        ("allow", "deny", &[][..], &[('x', "unused-import")][..]),
        ("deny", "warn", &[][..], &[('!', "unused-import"), ('x', "unused-local-variable")][..]),
        ("allow", "deny", &["-A", "correctness"][..], &[][..]),
        (
            "allow",
            "allow",
            &["-W", "correctness"][..],
            &[('!', "unused-import"), ('!', "unused-local-variable")][..],
        ),
        (
            "deny",
            "deny",
            &["-A", "correctness", "-W", "unused-import"][..],
            &[('!', "unused-import")][..],
        ),
        (
            "warn",
            "warn",
            &["-D", "correctness", "-A", "unused-import"][..],
            &[('x', "unused-local-variable")][..],
        ),
    ] {
        let temp = TempProject::new();
        temp.write("robusta.yaml", "lint: {categories: {correctness: deny}}\n");
        temp.write("nested/Example.java", UNUSED_SOURCE);
        temp.write("nested/robusta.yaml", &format!(
            "lint:\n  categories:\n    correctness: {category}\n  rules:\n    unused-import: {rule}\n"
        ));
        for command in ["lint", "check"] {
            let mut args = vec![command];
            args.extend_from_slice(filters);
            args.push("nested/Example.java");
            let disk = robusta_in_dir(&args, &temp.dir);
            args.pop();
            args.extend_from_slice(&["--stdin-filepath", "nested/Example.java"]);
            let stdin = robusta_with_stdin(&args, &temp.dir, UNUSED_SOURCE);
            for output in [disk, stdin] {
                let code = i32::from(expected.iter().any(|(marker, _)| *marker == 'x'));
                assert_eq!(output.status.code(), Some(code), "{}", stdout(&output));
                let expected = expected
                    .iter()
                    .map(|(marker, rule)| (*marker, (*rule).to_owned()))
                    .collect::<Vec<_>>();
                assert_eq!(
                    findings(&output),
                    expected,
                    "{category}/{rule}/{filters:?}: {}",
                    stdout(&output)
                );
            }
        }
    }
}

#[test]
fn category_warnings_obey_the_main_warning_policy() {
    let temp = TempProject::new();
    temp.write("nested/Example.java", UNUSED_SOURCE);
    temp.write("nested/robusta.yaml", "lint: {categories: {correctness: warn}}\n");
    for (options, flags, code) in [
        ("deny-warnings: true", &[][..], 1),
        ("max-warnings: 1", &[][..], 1),
        ("max-warnings: 1", &["--max-warnings", "2"][..], 0),
        ("max-warnings: 2", &["--max-warnings", "1"][..], 1),
    ] {
        temp.write("robusta.yaml", &format!("lint: {{options: {{{options}}}}}\n"));
        for command in ["lint", "check"] {
            let mut args = vec![command];
            args.extend_from_slice(flags);
            args.push("nested/Example.java");
            let output = robusta_in_dir(&args, &temp.dir);
            assert_eq!(output.status.code(), Some(code), "{}", stderr(&output));
            assert_eq!(findings(&output).len(), 2);
        }
    }
}

#[test]
fn category_configuration_rejects_unknown_names_and_noncanonical_severities() {
    let temp = TempProject::new();
    for (category, severity, message) in [
        ("unknown-category", "warn", "unknown category"),
        ("all", "warn", "unknown category"),
        ("correctness", "warning", "warning"),
        ("correctness", "off", "off"),
        ("correctness", "error", "error"),
        ("correctness", "0", "0"),
    ] {
        temp.write("robusta.yaml", &format!("lint: {{categories: {{{category}: {severity}}}}}\n"));
        let output = robusta_in_dir(&["config"], &temp.dir);
        assert_eq!(output.status.code(), Some(2), "{}", stdout(&output));
        assert!(stderr(&output).contains(message), "{}", stderr(&output));
        assert!(stderr(&output).contains("robusta.yaml"), "{}", stderr(&output));
    }
}

#[test]
fn other_categories_keep_their_remaining_members() {
    let temp = TempProject::new();
    temp.write(
        "Example.java",
        "class Example { long n = 1l; boolean f(String x) { return x == null && x.isEmpty(); } }\n",
    );
    for (category, expected) in
        [("style", "uppercase-long-suffix"), ("suspicious", "broken-null-check")]
    {
        temp.write("robusta.yaml", &format!("lint: {{categories: {{{category}: warn}}}}\n"));
        let output = robusta_in_dir(&["lint", "Example.java"], &temp.dir);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert_eq!(rules_reported(&output), [expected]);
    }
}

#[test]
fn style_rules_are_opt_in_until_a_filter_enables_them() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    let output = robusta_in_dir(&["lint", "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(findings(&output).len(), 0);
}

/// Two overlapping filters mean different runs depending on their order, which
/// is only true if each one sees the previous one's result.
#[test]
fn overlapping_filters_are_applied_in_the_order_they_were_given() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    let allowed_last = robusta_in_dir(
        &["lint", "-D", UPPERCASE_LONG_SUFFIX, "-A", UPPERCASE_LONG_SUFFIX, "Example.java"],
        &temp.dir,
    );
    assert_eq!(allowed_last.status.code(), Some(0), "{}", stderr(&allowed_last));
    assert_eq!(findings(&allowed_last).len(), 0);

    let denied_last = robusta_in_dir(
        &["lint", "-A", UPPERCASE_LONG_SUFFIX, "-D", UPPERCASE_LONG_SUFFIX, "Example.java"],
        &temp.dir,
    );
    assert_eq!(denied_last.status.code(), Some(1), "{}", stderr(&denied_last));
    assert_eq!(findings(&denied_last), [('x', UPPERCASE_LONG_SUFFIX.to_owned())]);

    // `all` is a selector like any other, so clearing then denying one rule
    // leaves exactly that rule enabled.
    let only_one = robusta_in_dir(
        &["lint", "-A", "all", "-D", UPPERCASE_LONG_SUFFIX, "Example.java"],
        &temp.dir,
    );
    assert_eq!(only_one.status.code(), Some(1), "{}", stderr(&only_one));
    assert_eq!(rules_reported(&only_one), [UPPERCASE_LONG_SUFFIX]);
}

/// `-D style -A uppercase-long-suffix` excludes that rule from the category and
/// reversing the two flags enables it as an error.
#[test]
fn a_category_filter_and_a_rule_filter_overlap_in_the_order_they_were_given() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    let excluded = robusta_in_dir(
        &["lint", "-D", "style", "-A", UPPERCASE_LONG_SUFFIX, "Example.java"],
        &temp.dir,
    );
    assert_eq!(excluded.status.code(), Some(1), "{}", stderr(&excluded));
    assert_eq!(rules_reported(&excluded), [MODIFIER_ORDER]);

    let reversed = robusta_in_dir(
        &["lint", "-A", UPPERCASE_LONG_SUFFIX, "-D", "style", "Example.java"],
        &temp.dir,
    );
    assert_eq!(reversed.status.code(), Some(1), "{}", stderr(&reversed));
    assert_eq!(rules_reported(&reversed), [MODIFIER_ORDER, UPPERCASE_LONG_SUFFIX]);
    assert!(findings(&reversed).iter().all(|(marker, _)| *marker == 'x'));
}

/// Filters are applied after each file's configuration resolves, so they reach
/// the root config, a nested config, and the no-config defaults alike rather
/// than only whichever config the run discovered first.
#[test]
fn filters_reach_every_resolved_configuration() {
    // `bare/` sits outside every config in the project, so the run resolves
    // three different policies: a root config, a nested config, and the
    // no-config defaults.
    let temp = TempProject::new();
    temp.write("configured/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write("configured/Root.java", SOURCE);
    temp.write("configured/nested/robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("configured/nested/Nested.java", SOURCE);
    temp.write("bare/Bare.java", SOURCE);

    // Without a filter each scope reports only what its own config enables,
    // and the scope with no config reports nothing.
    let configured = robusta_in_dir(&["lint", "."], &temp.dir);
    assert_eq!(configured.status.code(), Some(0), "{}", stderr(&configured));
    assert_eq!(rules_reported(&configured), [MODIFIER_ORDER, UPPERCASE_LONG_SUFFIX]);
    assert_eq!(findings(&configured).len(), 2);

    // Denying both rules reaches all three scopes, re-grading the one each
    // config enabled and adding the one it did not: two errors per file.
    let denied = robusta_in_dir(
        &["lint", "-D", MODIFIER_ORDER, "-D", UPPERCASE_LONG_SUFFIX, "."],
        &temp.dir,
    );
    assert_eq!(denied.status.code(), Some(1), "{}", stderr(&denied));
    assert_eq!(findings(&denied).len(), 6);
    assert!(findings(&denied).iter().all(|(marker, _)| *marker == 'x'));

    // And `-A` removes a rule a config enabled, in whichever scope enabled it.
    let allowed = robusta_in_dir(&["lint", "-A", UPPERCASE_LONG_SUFFIX, "."], &temp.dir);
    assert_eq!(allowed.status.code(), Some(0), "{}", stderr(&allowed));
    assert_eq!(rules_reported(&allowed), [MODIFIER_ORDER]);
}

#[test]
fn filters_apply_under_an_explicit_config_and_a_single_config_run() {
    let temp = TempProject::new();
    temp.write("shared/robusta.yaml", "lint:\n  rules:\n    uppercase-long-suffix: warn\n");
    temp.write("nested/robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    temp.write("nested/Nested.java", SOURCE);

    let explicit = robusta_in_dir(
        &["lint", "--config", "shared/robusta.yaml", "-D", MODIFIER_ORDER, "nested"],
        &temp.dir,
    );
    assert_eq!(explicit.status.code(), Some(1), "{}", stderr(&explicit));
    assert_eq!(rules_reported(&explicit), [MODIFIER_ORDER, UPPERCASE_LONG_SUFFIX]);

    let single = robusta_in_dir(
        &[
            "lint",
            "--no-nested-config",
            "-A",
            MODIFIER_ORDER,
            "-W",
            UPPERCASE_LONG_SUFFIX,
            "nested",
        ],
        &temp.dir,
    );
    assert_eq!(single.status.code(), Some(0), "{}", stderr(&single));
    assert_eq!(findings(&single), [('!', UPPERCASE_LONG_SUFFIX.to_owned())]);
}

/// The severity a filter selects is the real configured severity, so it drives
/// the warning policies and the exit status rather than only the rendered mark.
#[test]
fn a_filters_severity_drives_the_warning_policies_and_exit_status() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    // Naming both offending rules pins the run at exactly two warnings, so the
    // `--max-warnings` boundary stays meaningful as the registry grows.
    let warn_both = ["-W", MODIFIER_ORDER, "-W", UPPERCASE_LONG_SUFFIX];
    let lint_warning = |extra: &[&str]| {
        let mut args = vec!["lint"];
        args.extend_from_slice(&warn_both);
        args.extend_from_slice(extra);
        args.push("Example.java");
        robusta_in_dir(&args, &temp.dir)
    };

    let warned = lint_warning(&[]);
    assert_eq!(warned.status.code(), Some(0), "{}", stderr(&warned));
    assert_eq!(findings(&warned).len(), 2);
    assert!(findings(&warned).iter().all(|(marker, _)| *marker == '!'));

    let denied_warnings = lint_warning(&["--deny-warnings"]);
    assert_eq!(denied_warnings.status.code(), Some(1), "{}", stderr(&denied_warnings));

    let under_threshold = lint_warning(&["--max-warnings", "2"]);
    assert_eq!(under_threshold.status.code(), Some(0), "{}", stderr(&under_threshold));

    let over_threshold = lint_warning(&["--max-warnings", "1"]);
    assert_eq!(over_threshold.status.code(), Some(1), "{}", stderr(&over_threshold));

    // `--quiet` reports only errors, so a warning-graded filter reports none.
    let quiet = lint_warning(&["--quiet"]);
    assert_eq!(quiet.status.code(), Some(0), "{}", stderr(&quiet));
    assert_eq!(findings(&quiet).len(), 0);
}

/// A rule the configuration never enabled contributes its fix once a filter
/// enables it, and a rule the configuration did enable stops contributing one
/// once a filter removes it.
#[test]
fn filters_decide_which_rules_may_fix_the_source() {
    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    let file = temp.write("Example.java", SOURCE);

    let enabled =
        robusta_in_dir(&["lint", "-D", UPPERCASE_LONG_SUFFIX, "--fix", "Example.java"], &temp.dir);
    assert_eq!(enabled.status.code(), Some(0), "{}", stderr(&enabled));
    assert_eq!(
        std::fs::read_to_string(&file).expect("read fixed source"),
        "class Example {\n  public static long value = 10L;\n}\n",
        "both the configured and the CLI-enabled rule should have fixed"
    );

    let temp = TempProject::new();
    temp.write("robusta.yaml", "lint:\n  rules:\n    modifier-order: warn\n");
    let file = temp.write("Example.java", SOURCE);

    let removed =
        robusta_in_dir(&["lint", "-A", MODIFIER_ORDER, "--fix", "Example.java"], &temp.dir);
    assert_eq!(removed.status.code(), Some(0), "{}", stderr(&removed));
    assert_eq!(
        std::fs::read_to_string(&file).expect("read unfixed source"),
        SOURCE,
        "the removed rule should not have fixed anything"
    );
}

#[test]
fn check_accepts_the_same_filters_as_lint() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    let output = robusta_in_dir(&["check", "-D", UPPERCASE_LONG_SUFFIX, "Example.java"], &temp.dir);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert_eq!(findings(&output), [('x', UPPERCASE_LONG_SUFFIX.to_owned())]);
}

/// An unknown selector is rejected while arguments are parsed, before any file
/// is read, rather than resolving to a filter that matches no rule.
#[test]
fn an_unknown_selector_fails_the_run_with_the_accepted_vocabulary() {
    let temp = TempProject::new();
    temp.write("Example.java", SOURCE);

    for command in ["lint", "check"] {
        let output =
            robusta_in_dir(&[command, "-D", "uppercase-long-suffixes", "Example.java"], &temp.dir);
        let stderr = stderr(&output);

        assert_eq!(output.status.code(), Some(2), "{stderr}");
        assert_eq!(output.stdout.len(), 0);
        assert!(stderr.contains("uppercase-long-suffixes"), "{stderr}");
        assert!(stderr.contains("not a rule ID"), "{stderr}");
        assert!(stderr.contains("style"), "{stderr}");
        assert!(stderr.contains("lint --rules"), "{stderr}");
    }
}
