use std::{error::Error, path::PathBuf};

use pico_args::Arguments;

use crate::AppArgs;

/// The tool a run checks. Without one, every tool runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Parser,
    Formatter,
}

/// Read `[parser|formatter] [--filter TEXT] [--detail] [--debug] [CORPUS...]`.
///
/// # Errors
///
/// Returns an error for an invalid option value or an unknown option.
pub fn parse(mut args: Arguments) -> Result<(Option<Tool>, AppArgs), Box<dyn Error>> {
    let mut corpora = Vec::new();
    let tool = match args.subcommand()?.as_deref() {
        Some("parser") => Some(Tool::Parser),
        Some("formatter") => Some(Tool::Formatter),
        Some(root) => {
            corpora.push(PathBuf::from(root));
            None
        }
        None => None,
    };
    let debug = args.contains("--debug");
    let detail = args.contains("--detail");
    let filter = args.opt_value_from_str("--filter")?;
    corpora.extend(args.finish().into_iter().map(PathBuf::from));
    if let Some(option) = corpora.iter().find(|path| path.to_string_lossy().starts_with('-')) {
        return Err(format!("unexpected argument '{}'", option.display()).into());
    }
    Ok((tool, AppArgs { debug, filter, detail, corpora }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_args(input: &[&str]) -> Result<(Option<Tool>, AppArgs), Box<dyn Error>> {
        parse(Arguments::from_vec(input.iter().map(Into::into).collect()))
    }

    #[test]
    fn parses_tool_corpora_filter_detail_and_debug_mode() {
        let (tool, args) =
            parse_args(&["parser", "--filter", "misc", "--detail", "--debug", "trino"]).unwrap();

        assert_eq!(tool, Some(Tool::Parser));
        assert_eq!(args.filter.as_deref(), Some("misc"));
        assert!(args.detail);
        assert!(args.debug);
        assert_eq!(args.corpora, [PathBuf::from("trino")]);
    }

    #[test]
    fn a_first_argument_that_names_no_tool_is_a_corpus() {
        let (tool, args) = parse_args(&["trino", "commons-lang"]).unwrap();

        assert_eq!(tool, None);
        assert_eq!(args.corpora, [PathBuf::from("trino"), PathBuf::from("commons-lang")]);
    }

    #[test]
    fn rejects_an_unknown_option() {
        let error = parse_args(&["--bogus", "trino"]).unwrap_err();

        assert_eq!(error.to_string(), "unexpected argument '--bogus'");
    }
}
