// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_formatter_tests/src/harness.rs

//! Language-independent fixture-test runtime.
//!
//! Formats a fixture, checks the output against the formatter's contract, and
//! formats the output again. A second pass that differs is pinned after the
//! output rather than asserted, so an expected output records an unstable input
//! and fixing the input fails its test.
//!
//! Each formatter crate provides its language by implementing
//! [`FixtureFormatter`].

use std::error::Error;

/// Per-language hook for the fixture harness.
pub trait FixtureFormatter {
    /// The typed format-option struct for this language.
    type Options;

    /// Formats `source` using `options`.
    ///
    /// # Errors
    /// The formatter's error, such as a parse error, which fails the test.
    fn format(source: &str, options: &Self::Options) -> Result<String, Box<dyn Error>>;

    /// Checks that formatting `source` into `formatted` kept what the formatter
    /// must keep and met the output rules `options` set.
    ///
    /// # Errors
    /// A description of the first rule the output breaks.
    fn check_output(source: &str, formatted: &str, options: &Self::Options) -> Result<(), String>;
}

/// The output for `source`, followed under a heading by a second pass that
/// differs.
///
/// # Panics
/// Panics when `source` or its output cannot be formatted, or the output fails
/// [`FixtureFormatter::check_output`].
pub fn fixture_output<F: FixtureFormatter>(source: &str, options: &F::Options) -> String {
    let formatted = checked_output::<F>(source, options)
        .unwrap_or_else(|err| panic!("formatter contract failed for {source:?}: {err}"));
    match differing_second_pass::<F>(&formatted, options).unwrap_or_else(|err| panic!("{err}")) {
        Some(reformatted) => {
            format!("{formatted}---------- Not idempotent (second pass) ----------\n{reformatted}")
        }
        None => formatted,
    }
}

/// The output for `source`, for a test that compares less than the whole
/// output, where a second pass that differs has nowhere to show.
///
/// # Errors
/// Returns an error when `source` or its output cannot be formatted, the output
/// fails [`FixtureFormatter::check_output`], or the second pass differs.
pub fn stable_output<F: FixtureFormatter>(
    source: &str,
    options: &F::Options,
) -> Result<String, String> {
    let formatted = checked_output::<F>(source, options)?;
    match differing_second_pass::<F>(&formatted, options)? {
        Some(reformatted) => Err(format!(
            "second pass differs, so this input needs a test that shows it:\n{reformatted}"
        )),
        None => Ok(formatted),
    }
}

fn checked_output<F: FixtureFormatter>(
    source: &str,
    options: &F::Options,
) -> Result<String, String> {
    let formatted =
        F::format(source, options).map_err(|err| format!("input parse errors: {err}"))?;
    F::check_output(source, &formatted, options)?;
    Ok(formatted)
}

/// The second pass over `formatted`, when it differs.
fn differing_second_pass<F: FixtureFormatter>(
    formatted: &str,
    options: &F::Options,
) -> Result<Option<String>, String> {
    let reformatted = F::format(formatted, options)
        .map_err(|err| format!("formatted output parse errors: {err}"))?;
    Ok((reformatted != formatted).then_some(reformatted))
}
