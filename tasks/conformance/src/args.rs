use std::{error::Error, path::PathBuf};

use pico_args::Arguments;

/// Default Java source levels the gate compiles against.
///
/// Java 8 is the compatibility floor. The later LTS boundaries are necessary
/// because a fix that is valid Java 8 can still be rejected later: `_` is an
/// ordinary identifier in Java 8 but a keyword from Java 9 onward, so a
/// Java-8-only compile would miss that incompatible transition.
pub const DEFAULT_RELEASES: [u16; 5] = [8, 11, 17, 21, 25];

/// Compile Robusta's emitted fixes across supported Java source levels.
#[derive(Debug, Clone)]
pub struct Args {
    /// Java `--release` targets to compile against. Defaults to 8, 11, 17,
    /// 21, and 25 when empty.
    pub release: Vec<u16>,
    /// Select fixtures whose file name contains this text.
    pub filter: Option<String>,
    /// Fixture directory, scanned for `.java` files. Defaults to the task's
    /// own `fixtures/` corpus.
    pub fixtures: Option<PathBuf>,
    /// Preserve the temporary compile workspace instead of deleting it.
    pub keep_temp: bool,
    /// Corpus manifests to fix and compile instead of the fixtures.
    pub corpus: Vec<PathBuf>,
}

impl Args {
    /// The release targets to compile against, applying the default set when
    /// none were requested.
    pub fn releases(&self) -> &[u16] {
        if self.release.is_empty() { &DEFAULT_RELEASES } else { &self.release }
    }

    /// The fixture directory, defaulting to the corpus shipped with the task.
    pub fn fixtures_dir(&self) -> PathBuf {
        self.fixtures
            .clone()
            .unwrap_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures")))
    }
}

/// Read repeatable `--release N`, `--filter TEXT`, `--fixtures DIR`,
/// `--keep-temp`, and repeatable `--corpus PATH`.
///
/// # Errors
///
/// Returns an error for an invalid option value, an unexpected argument, or a
/// fixture option combined with `--corpus`.
pub fn parse(mut args: Arguments) -> Result<Args, Box<dyn Error>> {
    let keep_temp = args.contains("--keep-temp");
    let parsed = Args {
        release: args.values_from_str("--release")?,
        filter: args.opt_value_from_str("--filter")?,
        fixtures: args.opt_value_from_str("--fixtures")?,
        keep_temp,
        corpus: args.values_from_str("--corpus")?,
    };
    if let Some(argument) = args.finish().first() {
        return Err(format!("unexpected argument '{}'", argument.to_string_lossy()).into());
    }
    let fixture_options =
        !parsed.release.is_empty() || parsed.filter.is_some() || parsed.fixtures.is_some();
    if !parsed.corpus.is_empty() && fixture_options {
        return Err("`--corpus` takes its releases from the manifest and runs no fixtures".into());
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_args(input: &[&str]) -> Result<Args, Box<dyn Error>> {
        parse(Arguments::from_vec(input.iter().map(Into::into).collect()))
    }

    #[test]
    fn defaults_to_the_full_release_matrix_and_bundled_fixtures() {
        let args = parse_args(&[]).unwrap();

        assert_eq!(args.releases(), DEFAULT_RELEASES);
        assert!(args.fixtures_dir().ends_with("fixtures"));
        assert!(!args.keep_temp);
    }

    #[test]
    fn accepts_explicit_releases_and_paths() {
        let args =
            parse_args(&["--release", "8", "--release", "21", "--filter", "unused", "--keep-temp"])
                .unwrap();

        assert_eq!(args.releases(), [8, 21]);
        assert_eq!(args.filter.as_deref(), Some("unused"));
        assert!(args.keep_temp);
    }

    #[test]
    fn rejects_an_invalid_release_and_an_unexpected_argument() {
        assert!(parse_args(&["--release", "latest"]).is_err());
        let error = parse_args(&["--bogus"]).unwrap_err();

        assert_eq!(error.to_string(), "unexpected argument '--bogus'");
    }

    #[test]
    fn accepts_corpus_manifests_but_not_with_fixture_options() {
        let args = parse_args(&["--corpus", "a.json", "--corpus", "b.json"]).unwrap();
        assert_eq!(args.corpus, [PathBuf::from("a.json"), PathBuf::from("b.json")]);

        for option in [["--release", "8"], ["--filter", "unused"], ["--fixtures", "dir"]] {
            assert!(parse_args(&["--corpus", "a.json", option[0], option[1]]).is_err());
        }
    }

    #[test]
    fn rejects_a_switch_as_an_option_value() {
        assert!(parse_args(&["--filter", "--keep-temp"]).is_err());
    }
}
