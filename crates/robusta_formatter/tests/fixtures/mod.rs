#![allow(
    dead_code,
    reason = "each integration test target uses a different subset of the shared helpers"
)]

use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use robusta_formatter::{
    IndentWidth, JavadocOptions, LineWidth, Options, check_line_width, detect_code_removal,
};
use robusta_formatter_tests::{FixtureFormatter, fixture_output, stable_output};

struct JavaHarness;

impl FixtureFormatter for JavaHarness {
    type Options = Options;

    fn format(source: &str, options: &Self::Options) -> Result<String, Box<dyn Error>> {
        Ok(robusta_formatter::format(source, options)?)
    }

    /// The output side of the formatter contract: the token stream and comments
    /// survive modulo trivia, every documentation comment documents what it did
    /// before, every line the layout could have broken fits `line_width`, and
    /// non-empty output follows the configured final-newline policy. The newline
    /// check lives here because inline snapshot assertions cannot pin it: insta
    /// trims trailing newlines before comparing.
    fn check_output(source: &str, formatted: &str, options: &Self::Options) -> Result<(), String> {
        if !formatted.is_empty() {
            let has_final_newline = formatted.ends_with(['\n', '\r']);
            if has_final_newline != options.insert_final_newline {
                return Err(if options.insert_final_newline {
                    "formatted output does not end with a newline".to_string()
                } else {
                    "formatted output unexpectedly ends with a newline".to_string()
                });
            }
        }
        if formatted.ends_with("\n\n")
            || formatted.ends_with("\r\r")
            || formatted.ends_with("\r\n\r\n")
        {
            return Err("formatted output ends with a blank line".to_string());
        }

        detect_code_removal(source, formatted)?;

        let widths = check_line_width(formatted, options);
        if let Some(message) = widths.failure_message(options.line_width) {
            return Err(message);
        }

        Ok(())
    }
}

pub fn line_width(value: u16) -> LineWidth {
    LineWidth::try_from(value).expect("test line width must be valid")
}

pub fn indent_width(value: u8) -> IndentWidth {
    IndentWidth::try_from(value).expect("test indent width must be valid")
}

/// Formats one explicitly declared Java fixture and compares every output byte
/// with its paired `.expected.java` file, which also holds any second pass that
/// differs. [`format_source`] supplies the shared parse, code-removal, and
/// final-newline-policy checks.
pub fn assert_java_fixture(name: &str, source: &str, expected: &str) {
    let formatted = format_source(source);
    assert_eq!(formatted, expected, "Java formatter fixture `{name}` changed");
}

/// Proves that the statically declared fixture tests cover the complete
/// checked-in Java fixture tree. The declarations generate the named tests, so
/// comparing their paths here also rejects duplicate registrations and orphaned
/// inputs or expected outputs without relying on dynamic test discovery.
pub fn assert_java_fixtures_covered(groups: &[&[&str]]) -> Result<(), String> {
    let mut registered = BTreeSet::new();
    for &name in groups.iter().flat_map(|group| group.iter()) {
        if !registered.insert(name.to_string()) {
            return Err(format!("duplicate fixture registration `{name}`"));
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/java");
    let mut inputs = BTreeSet::new();
    let mut expected_outputs = BTreeSet::new();
    collect_java_fixtures(&root, &root, &mut inputs, &mut expected_outputs)?;

    compare_fixture_sets("input", &registered, &inputs)?;
    compare_fixture_sets("expected output", &registered, &expected_outputs)
}

fn collect_java_fixtures(
    root: &Path,
    directory: &Path,
    inputs: &mut BTreeSet<String>,
    expected_outputs: &mut BTreeSet<String>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|err| {
            format!("cannot read Java fixture directory {}: {err}", directory.display())
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("cannot read an entry in {}: {err}", directory.display()))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|err| format!("cannot inspect Java fixture {}: {err}", path.display()))?;
        if file_type.is_dir() {
            collect_java_fixtures(root, &path, inputs, expected_outputs)?;
            continue;
        }
        fs::read_to_string(&path)
            .map_err(|err| format!("cannot read Java fixture {}: {err}", path.display()))?;
        let relative = normalized_relative_path(root, &path)?;
        if let Some(name) = relative.strip_suffix(".expected.java") {
            expected_outputs.insert(name.to_string());
        } else if let Some(name) = relative.strip_suffix(".java") {
            inputs.insert(name.to_string());
        } else {
            return Err(format!(
                "unexpected file in Java fixture tree `{relative}`; expected `.java` or `.expected.java`"
            ));
        }
    }

    Ok(())
}

fn normalized_relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|err| {
        format!("fixture {} is outside {}: {err}", path.display(), root.display())
    })?;
    relative
        .components()
        .map(|component| {
            component.as_os_str().to_str().map(str::to_owned).ok_or_else(|| {
                format!(
                    "Java fixture path is not valid UTF-8: {}",
                    PathBuf::from(relative).display()
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| parts.join("/"))
}

fn compare_fixture_sets(
    kind: &str,
    registered: &BTreeSet<String>,
    checked_in: &BTreeSet<String>,
) -> Result<(), String> {
    let unvisited = checked_in.difference(registered).cloned().collect::<Vec<_>>();
    let missing = registered.difference(checked_in).cloned().collect::<Vec<_>>();
    if unvisited.is_empty() && missing.is_empty() {
        return Ok(());
    }

    Err(format!(
        "Java fixture {kind} coverage mismatch; unvisited={unvisited:?}, missing={missing:?}"
    ))
}

/// Formats `source` and enforces the formatter contract on the result: nothing
/// lost, and the configured final-newline policy. Every golden test that formats
/// through this helper gets contract coverage for free; a test that compares less
/// than the whole output uses [`format_stable`] or [`check_formatter_contract`].
#[must_use]
pub fn format_source(source: &str) -> String {
    format_source_with_options(source, &Options::default())
}

/// [`format_source`] with `options`. A second pass that differs follows the
/// output under a heading, rather than failing here, so that the expected output
/// pins an unstable input and fixing the input fails its test.
#[must_use]
pub fn format_source_with_options(source: &str, options: &Options) -> String {
    fixture_output::<JavaHarness>(source, options)
}

/// [`format_source`] for a test that compares less than the whole output, where a
/// second pass that differs has nowhere to show: it fails, until the input has a
/// test of its own that pins it through [`format_source_with_options`].
pub fn format_stable(source: &str) -> String {
    format_stable_with_options(source, &Options::default())
}

pub fn format_stable_with_options(source: &str, options: &Options) -> String {
    stable_output::<JavaHarness>(source, options)
        .unwrap_or_else(|err| panic!("formatter contract failed for {source:?}: {err}"))
}

pub fn check_formatter_contract(source: &str) -> Result<(), String> {
    check_formatter_contract_with_options(source, &Options::default())
}

/// [`format_stable`] for a source whose output no test reads.
pub fn check_formatter_contract_with_options(
    source: &str,
    options: &Options,
) -> Result<(), String> {
    stable_output::<JavaHarness>(source, options).map(drop)
}

/// Asserts that each of `examples` prints as itself under `options`, also from
/// its tokens joined onto one line, and checks the contract on it as
/// [`check_contract_at_widths_and_modes`] does.
pub fn assert_examples_print_as_shown(examples: &[&str], options: &Options) {
    for example in examples {
        assert_eq!(format_source_with_options(example, options), *example);
        let one_line = example.split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(format_source_with_options(&one_line, options), *example);
        check_contract_at_widths_and_modes(example);
    }
}

/// Checks the formatter contract at a narrow and a wide line, with and without
/// Javadoc formatting.
pub fn check_contract_at_widths_and_modes(source: &str) {
    for width in [40, 100] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { line_width: line_width(width), javadoc, ..Options::default() };
            check_formatter_contract_with_options(source, &options)
                .unwrap_or_else(|error| panic!("{source:?}: {error}"));
        }
    }
}
