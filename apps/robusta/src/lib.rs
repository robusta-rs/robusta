use std::process::{ExitCode, Termination};

use robusta_diagnostics::Output;

mod baseline;
mod command;
mod config_file;
mod config_loader;
mod core;
mod lint;
mod mode;
mod output_formatter;
mod parallel;
mod result;
mod run;
mod walk;

pub use config_file::ConfigFile;
pub use output_formatter::get_all_rules_json;

/// The command surface, for documentation generation.
///
/// The website generators render these parsers rather than restating the
/// flags, which is what keeps a generated reference and `--help` one contract.
pub mod cli {
    pub use crate::command::{
        Command, check_command, config_command, format_command, init_command, lint_command, robusta,
    };
}

/// Parse arguments, dispatch the command, and flush process output.
///
/// Flushing owns the last word: a failed write turns any outcome into an
/// operational failure.
pub fn run_cli() -> ExitCode {
    let mut out = Output::stdio();
    let command = match command::parse(&mut out) {
        Ok(command) => command,
        Err(result) => return out.finish(result.report()),
    };
    let color = output_formatter::ColorPolicy::resolve(command.color());
    let result = run::run(command, &mut out, color);
    out.finish(result.report())
}
