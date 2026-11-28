// Directory adapted from https://github.com/oxc-project/oxc/tree/566b11b2db6b664e787e67e2199d4ffcaa31c30a/apps/oxlint/src/command

mod ignore;
mod lint;

use std::path::{Component, PathBuf};

use bpaf::{Bpaf, OptionParser, Parser};
use robusta_diagnostics::Output;

use crate::output_formatter::{ColorChoice, ReportFormat};
use crate::result::CliRunResult;

pub use self::{
    ignore::IgnoreOptions,
    lint::{
        DebugOption, DebugOptions, FilterOptions, FixOptions, ReportUnusedSuppressions,
        SuppressionOptions, WarningOptions,
    },
};
use self::{
    ignore::ignore_options,
    lint::{
        debug_options, filter_options, fix_options, report_unused_suppressions,
        suppression_options, warning_options,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeOutput {
    Default,
    Diff,
    ListDifferent,
}

fn change_output() -> impl Parser<ChangeOutput> {
    let diff = bpaf::long("diff")
        .help("Print a unified diff for files that would be changed, and the report to stderr.")
        .req_flag(ChangeOutput::Diff);
    let list_different = bpaf::long("list-different")
        .help("Print only the paths of files that would be changed, and the report to stderr.")
        .req_flag(ChangeOutput::ListDifferent);
    bpaf::construct!([diff, list_different]).fallback(ChangeOutput::Default)
}

/// Configuration options:
#[derive(Debug, Clone, Bpaf)]
pub struct ConfigOptions {
    /// Use this configuration file instead of discovering `robusta.yaml`.
    #[bpaf(short, long, argument("PATH"))]
    pub config: Option<PathBuf>,
    /// Use one configuration discovered from the current directory.
    #[bpaf(long)]
    pub no_nested_config: bool,
}

/// Runtime options:
#[derive(Debug, Clone, Bpaf)]
pub struct RuntimeOptions {
    /// Do not exit with an error when no files are selected.
    #[bpaf(long)]
    pub no_error_on_unmatched_pattern: bool,
    /// Number of threads to use. Set to 1 for using only 1 CPU core.
    #[bpaf(long, argument("INT"))]
    pub threads: Option<usize>,
}

/// Color options:
#[derive(Debug, Clone, Copy, Bpaf)]
pub struct ColorOptions {
    /// When to use colors: auto, always, or never.
    #[bpaf(long, argument("WHEN"), fallback(ColorChoice::Auto))]
    pub color: ColorChoice,
}

/// The selection `config --files` lists instead of printing the configuration.
#[derive(Debug, Clone)]
pub struct FileListing {
    pub ignore_options: IgnoreOptions,
    pub paths: Vec<PathBuf>,
}

fn file_listing() -> impl Parser<FileListing> {
    let files = bpaf::long("files")
        .help("Print the paths `check` would process instead of the configuration.")
        .req_flag(());
    let ignore_options = ignore_options();
    let paths = source_paths();
    bpaf::construct!(files, ignore_options, paths)
        .map(|((), ignore_options, paths)| FileListing { ignore_options, paths })
}

/// A parsed `robusta` command.
#[derive(Debug, Clone)]
pub enum Command {
    Check(CheckCommand),
    Format(FormatCommand),
    Lint(LintCommand),
    Init(InitCommand),
    Config(ConfigCommand),
}

/// The `robusta` parser over `commands`. The CLI takes every command; a
/// generated reference takes the ones it documents.
pub fn robusta(commands: impl Parser<Command> + 'static) -> OptionParser<Command> {
    bpaf::cargo_helper("robusta", commands)
        .to_options()
        .descr("Format and lint Java source files.")
        .version(VERSION)
}

pub fn command() -> OptionParser<Command> {
    let check = check_command().map(Command::Check);
    let format = format_command().map(Command::Format);
    let lint = lint_command().map(Command::Lint);
    let init = init_command().map(Command::Init);
    let config = config_command().map(Command::Config);
    robusta(bpaf::construct!([check, format, lint, init, config]))
}

/// Verify Java formatting and lints; optionally preview or apply fixes.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("check"))]
pub struct CheckCommand {
    #[bpaf(external)]
    pub config_options: ConfigOptions,
    #[bpaf(external)]
    pub ignore_options: IgnoreOptions,
    #[bpaf(external)]
    pub filter_options: FilterOptions,
    #[bpaf(external)]
    pub fix_options: FixOptions,
    #[bpaf(external(change_output))]
    pub output: ChangeOutput,
    /// Lower or remove obsolete counts in `robusta-suppressions.json`; never raise them.
    #[bpaf(long)]
    pub prune_suppressions: bool,
    #[bpaf(external(report_unused_suppressions))]
    pub report_unused_suppressions: ReportUnusedSuppressions,
    /// Report format: default, json, checkstyle, github, gitlab, azure-devops, unix, agent, sarif, stylish, or junit.
    #[bpaf(long, argument("FORMAT"), fallback(ReportFormat::Default))]
    pub reporter: ReportFormat,
    #[bpaf(external(debug_options))]
    pub debug: DebugOptions,
    #[bpaf(external)]
    pub warning_options: WarningOptions,
    #[bpaf(external)]
    pub runtime_options: RuntimeOptions,
    #[bpaf(external)]
    pub color_options: ColorOptions,
    #[bpaf(external)]
    pub input: Input,
}

/// Verify Java formatting; optionally preview or apply changes.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("format"))]
pub struct FormatCommand {
    #[bpaf(external)]
    pub config_options: ConfigOptions,
    #[bpaf(external)]
    pub ignore_options: IgnoreOptions,
    /// Rewrite files in place; with --stdin-filepath, print the formatted document instead.
    #[bpaf(long)]
    pub fix: bool,
    #[bpaf(external(change_output))]
    pub output: ChangeOutput,
    /// Report format: default, json, checkstyle, github, gitlab, azure-devops, unix, agent, sarif, stylish, or junit.
    #[bpaf(long, argument("FORMAT"), fallback(ReportFormat::Default))]
    pub reporter: ReportFormat,
    #[bpaf(external(debug_options))]
    pub debug: DebugOptions,
    #[bpaf(external)]
    pub warning_options: WarningOptions,
    #[bpaf(external)]
    pub runtime_options: RuntimeOptions,
    #[bpaf(external)]
    pub color_options: ColorOptions,
    #[bpaf(external)]
    pub input: Input,
}

/// Verify enabled lint rules; optionally preview or apply transformations.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("lint"))]
pub struct LintCommand {
    /// List registered rules using the default, JSON, or agent reporter.
    #[bpaf(long("rules"), switch, hide_usage)]
    pub list_rules: bool,
    #[bpaf(external)]
    pub config_options: ConfigOptions,
    #[bpaf(external)]
    pub ignore_options: IgnoreOptions,
    #[bpaf(external)]
    pub filter_options: FilterOptions,
    #[bpaf(external)]
    pub fix_options: FixOptions,
    #[bpaf(external(change_output))]
    pub output: ChangeOutput,
    #[bpaf(external)]
    pub suppression_options: SuppressionOptions,
    #[bpaf(external(report_unused_suppressions))]
    pub report_unused_suppressions: ReportUnusedSuppressions,
    /// Report format: default, json, checkstyle, github, gitlab, azure-devops, unix, agent, sarif, stylish, or junit.
    #[bpaf(long, argument("FORMAT"), fallback(ReportFormat::Default))]
    pub reporter: ReportFormat,
    #[bpaf(external(debug_options))]
    pub debug: DebugOptions,
    #[bpaf(external)]
    pub warning_options: WarningOptions,
    #[bpaf(external)]
    pub runtime_options: RuntimeOptions,
    #[bpaf(external)]
    pub color_options: ColorOptions,
    #[bpaf(external)]
    pub input: Input,
}

/// Create `robusta.yaml` in the current directory, failing if it exists.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("init"))]
pub struct InitCommand {
    #[bpaf(external)]
    pub color_options: ColorOptions,
}

/// Print the resolved configuration for the current directory as YAML.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("config"))]
pub struct ConfigCommand {
    #[bpaf(external)]
    pub config_options: ConfigOptions,
    #[bpaf(external)]
    pub color_options: ColorOptions,
    #[bpaf(external(file_listing), optional)]
    pub files: Option<FileListing>,
}

impl Command {
    pub const fn color(&self) -> ColorChoice {
        match self {
            Self::Check(CheckCommand { color_options, .. })
            | Self::Format(FormatCommand { color_options, .. })
            | Self::Lint(LintCommand { color_options, .. })
            | Self::Init(InitCommand { color_options })
            | Self::Config(ConfigCommand { color_options, .. }) => color_options.color,
        }
    }
}

fn source_paths() -> impl Parser<Vec<PathBuf>> {
    bpaf::positional::<PathBuf>("PATH")
        .help("Java source file or directory to process. Defaults to the current directory. Paths must not contain a `..` component.")
        .many()
        .guard(
            |paths| {
                paths.iter().all(|path| {
                    !path.components().any(|component| component == Component::ParentDir)
                })
            },
            "PATH must not contain a `..` component",
        )
}

/// Where a run reads source: the selected files, or one document from
/// standard input.
#[derive(Debug, Clone)]
pub struct Input {
    pub stdin_filepath: Option<PathBuf>,
    pub paths: Vec<PathBuf>,
}

fn input() -> impl Parser<Input> {
    let stdin_filepath = bpaf::long("stdin-filepath")
        .help(
            "Read one document from stdin instead of selecting files, resolving configuration \
             against PATH. --fix prints the result to stdout and the report to stderr; no file is \
             written.",
        )
        .argument::<PathBuf>("PATH")
        .optional();
    let paths = source_paths();
    bpaf::construct!(Input { stdin_filepath, paths })
}

/// Parse the process arguments, printing help, version, or the usage error when
/// they do not name a command to run.
pub fn parse(out: &mut Output) -> Result<Command, CliRunResult> {
    // Without arguments, the root help is the usage error: the same text as
    // `--help`, but on standard error and with a failing status.
    let no_arguments = std::env::args_os().len() <= 1;
    let parser = if no_arguments { command().fallback_to_usage() } else { command() };
    parser.run_inner(bpaf::Args::current_args()).map_err(|failure| match failure {
        bpaf::ParseFailure::Stdout(help, full) if no_arguments => {
            out.stderr_line(&help.monochrome(full));
            CliRunResult::InvalidArguments
        }
        failure => {
            let is_help_or_version = failure.clone().exit_code() == 0;
            failure.print_message(100);
            if is_help_or_version {
                CliRunResult::HelpPrinted
            } else {
                CliRunResult::InvalidArguments
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn positional_paths_reject_parent_components() {
        let absolute =
            std::env::current_dir().unwrap().join("child").join("..").join("Example.java");
        for verb in ["check", "format", "lint"] {
            for path in
                ["..", "../Example.java", "child/../Example.java", absolute.to_str().unwrap()]
            {
                let failure = super::command()
                    .run_inner(&[verb, path][..])
                    .expect_err("parent components must be rejected");
                let message =
                    failure.unwrap_stderr().split_whitespace().collect::<Vec<_>>().join(" ");
                assert!(
                    message.contains("PATH must not contain a `..` component"),
                    "{verb} {path}: {message}"
                );
            }
            #[cfg(windows)]
            for path in [r"..\Example.java", r"child\..\Example.java", r"C:\child\..\Example.java"]
            {
                assert!(super::command().run_inner(&[verb, path][..]).is_err());
            }
        }
    }

    #[test]
    fn positional_paths_keep_valid_spellings() {
        let absolute = std::env::current_dir().unwrap().join("Example.java");
        for verb in ["check", "format", "lint"] {
            for paths in [
                vec![],
                vec!["."],
                vec!["./Example.java", "Example..java", absolute.to_str().unwrap()],
            ] {
                let args = [&[verb][..], &paths].concat();
                let parsed = match super::command().run_inner(args.as_slice()).unwrap() {
                    Command::Check(super::CheckCommand { input, .. })
                    | Command::Lint(super::LintCommand { input, .. })
                    | Command::Format(super::FormatCommand { input, .. }) => input.paths,
                    command => panic!("expected file input, got {command:?}"),
                };
                assert_eq!(parsed, paths.iter().map(std::path::PathBuf::from).collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn path_options_allow_parent_components() {
        for verb in ["check", "format", "lint"] {
            for option in ["--config", "--ignore-path"] {
                let args = [verb, option, "../settings", "Example.java"];
                assert!(super::command().run_inner(args.as_slice()).is_ok(), "{args:?}");
            }
        }
        for verb in ["check", "format", "lint"] {
            let args = [verb, "--stdin-filepath", "../Example.java"];
            assert!(super::command().run_inner(args.as_slice()).is_ok(), "{args:?}");
        }
    }

    #[test]
    fn parser_satisfies_bpaf_invariants() {
        super::command().check_invariants(false);
    }

    #[test]
    fn version_is_available_without_a_subcommand() {
        let failure = super::command()
            .run_inner(&["--version"])
            .expect_err("version should produce successful informational output");
        assert_eq!(failure.clone().exit_code(), 0);
        assert_eq!(failure.unwrap_stdout(), concat!("Version: ", env!("CARGO_PKG_VERSION"), "\n"));
    }
}
