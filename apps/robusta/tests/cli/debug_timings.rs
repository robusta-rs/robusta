use std::process::Output;

use crate::support::*;

/// A project whose one file trips `uppercase-long-suffix` twice and leaves
/// `redundant-semicolons` enabled but unreported, so a timing table has both a
/// rule that ran and a rule that did not.
fn timed_project() -> TempProject {
    let temp = TempProject::new();
    temp.write(
        "robusta.yaml",
        "lint:\n  categories:\n    correctness: allow\n  rules:\n    uppercase-long-suffix: deny\n    redundant-semicolons: warn\n",
    );
    temp.write("Example.java", "class Example {\n  long first = 1l;\n  long second = 2l;\n}\n");
    temp
}

/// The timing table a run closed its report with, as `(rule, calls)` rows.
///
/// The measured columns are dropped rather than normalized: a process test
/// cannot pin a duration, and the table's exact column layout is pinned by the
/// renderer's own unit tests.
fn timing_rows(output: &Output) -> Vec<(String, String)> {
    let text = stdout(output);
    let Some(table) = text.split_once("Rule timings:\n") else {
        return Vec::new();
    };
    table
        .1
        .lines()
        .skip(2) // the header and its underline
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            let [rule, _time, _relative, calls] = fields[..] else {
                panic!("a timing row has four columns: {line}");
            };
            (rule.to_owned(), calls.to_owned())
        })
        .collect()
}

/// The table names every enabled rule, counts the nodes each was dispatched to,
/// and puts the measured ones first — including the rule that was enabled and
/// never reached, which is the answer "this rule cost nothing".
#[test]
fn debug_timings_reports_every_enabled_rule_with_its_call_count() {
    let temp = timed_project();

    for command in ["check", "lint"] {
        let output = robusta_in_dir(&[command, "--debug=timings"], &temp.dir);
        assert_eq!(output.status.code(), Some(1), "{command}: {}", stdout(&output));

        let text = stdout(&output);
        assert!(text.contains("Rule timings:"), "{command}: {text}");
        assert!(text.contains("Time (ms)  Relative  Calls"), "{command}: {text}");

        let rows = timing_rows(&output);
        assert_eq!(rows.len(), 2, "{command}: {rows:?}");
        assert_eq!(rows[0], ("uppercase-long-suffix".into(), "2".into()));
        assert_eq!(rows[1], ("redundant-semicolons".into(), "0".into()));
    }
}

/// The table follows the closing summary rather than replacing it, so a run
/// that asks for timings still reports its totals and its findings.
#[test]
fn debug_timings_follows_the_execution_summary_and_leaves_the_run_intact() {
    let temp = timed_project();
    let output = robusta_in_dir(&["lint", "--debug=timings"], &temp.dir);

    let text = stdout(&output);
    let findings = text.find("uppercase-long-suffix: use uppercase").expect("the run reports");
    let summary = text.find("Finished in ").expect("the run closes with its summary");
    let table = text.find("Rule timings:").expect("the run closes with its table");
    assert!(findings < summary && summary < table, "{text}");
    assert_eq!(text.matches("Rule timings:").count(), 1, "{text}");
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(temp.dir.join("Example.java")).unwrap(),
        "class Example {\n  long first = 1l;\n  long second = 2l;\n}\n",
        "a timed run reports and changes nothing",
    );
}

/// A rule dispatched twice by a fix run — once to find, once to confirm — is
/// counted twice. The table describes the work the run actually did.
#[test]
fn debug_timings_counts_every_dispatch_of_a_fix_run() {
    let temp = timed_project();
    let output = robusta_in_dir(&["lint", "--fix", "--debug=timings"], &temp.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));

    let rows = timing_rows(&output);
    assert_eq!(rows[0].0, "uppercase-long-suffix");
    assert_eq!(rows[0].1, "4", "the rule is dispatched again over the fixed candidate");
}

/// Options repeat and tolerate the empty fields a comma list can carry.
#[test]
fn debug_timings_accepts_repeated_options_and_empty_comma_fields() {
    let temp = timed_project();

    for value in ["timings", ",timings,,timings,", "timings,timings"] {
        let output = robusta_in_dir(&["lint", "--debug", value], &temp.dir);
        assert_eq!(output.status.code(), Some(1), "{value}: {}", stdout(&output));
        assert_eq!(stdout(&output).matches("Rule timings:").count(), 1, "{value}");
        assert_eq!(timing_rows(&output).len(), 2, "{value}");
    }
}

/// An unknown option is named, whichever position it holds in the list.
#[test]
fn debug_timings_reports_unknown_options_by_name() {
    let temp = timed_project();

    for command in ["check", "format", "lint"] {
        for value in ["timing", "timings,timing", "timing,timings"] {
            let output = robusta_in_dir(&[command, "--debug", value], &temp.dir);
            assert_eq!(output.status.code(), Some(2), "{command} {value}: {}", stderr(&output));
            assert!(
                stderr(&output).contains("'timing' is not a known debug option"),
                "{command} {value}: {}",
                stderr(&output),
            );
            assert!(output.stdout.is_empty(), "{command} {value}");
        }
    }
}

/// The table is part of the closing summary, so it reaches the streaming human
/// reporter and nothing else: a buffered format writes one document,
/// and `--silent` withholds the summary the table belongs to.
#[test]
fn the_timing_table_reaches_only_the_streaming_human_reporter() {
    let temp = timed_project();

    let buffered = robusta_in_dir(&["lint", "--debug=timings", "--reporter=json"], &temp.dir);
    assert_eq!(buffered.status.code(), Some(1), "{}", stderr(&buffered));
    assert!(!stderr(&buffered).contains("Rule timings:"), "{}", stderr(&buffered));
    assert!(!stdout(&buffered).contains("Rule timings:"), "{}", stdout(&buffered));
    assert!(stdout(&buffered).contains("uppercase-long-suffix"), "{}", stdout(&buffered));

    let silent = robusta_in_dir(&["lint", "--debug=timings", "--silent"], &temp.dir);
    assert_eq!(silent.status.code(), Some(1), "{}", stderr(&silent));
    assert!(silent.stderr.is_empty(), "{}", stderr(&silent));
    assert!(silent.stdout.is_empty(), "{}", stdout(&silent));
}

/// A run with no rule to measure prints no table. `format` runs none by
/// definition, and neither does a lint run whose rule set is empty.
#[test]
fn a_run_that_measured_nothing_prints_no_table() {
    let temp = timed_project();
    let formatting = robusta_in_dir(&["format", "--debug=timings"], &temp.dir);
    assert_eq!(formatting.status.code(), Some(0), "{}", stdout(&formatting));
    assert!(stdout(&formatting).contains("Finished in "), "{}", stdout(&formatting));
    assert!(!stdout(&formatting).contains("Rule timings:"), "{}", stdout(&formatting));

    let ruleless = TempProject::new();
    ruleless.write("robusta.yaml", "lint: {categories: {correctness: allow}}\n");
    ruleless.write("Example.java", "class Example {}\n");
    let output = robusta_in_dir(&["lint", "--debug=timings"], &ruleless.dir);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout(&output).contains("with 0 rules"), "{}", stdout(&output));
    assert!(!stdout(&output).contains("Rule timings:"), "{}", stdout(&output));
}

/// Timing measures a run, so it combines with transformations and previews.
#[test]
fn debug_timings_is_accepted_alongside_transformations_and_previews() {
    let temp = timed_project();

    let preview = robusta_in_dir(&["lint", "--debug=timings", "--diff"], &temp.dir);
    assert_eq!(preview.status.code(), Some(1), "{}", stderr(&preview));
    assert!(stdout(&preview).contains("+  long first = 1L;"), "{}", stdout(&preview));
    // The diff is the document, so the summary and its table go to stderr.
    assert!(stderr(&preview).contains("Rule timings:"), "{}", stderr(&preview));

    let suppress = robusta_in_dir(&["lint", "--debug=timings", "--suppress-all"], &temp.dir);
    assert_eq!(suppress.status.code(), Some(0), "{}", stderr(&suppress));
    assert!(stdout(&suppress).contains("Rule timings:"), "{}", stdout(&suppress));
}
