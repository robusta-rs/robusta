use robusta_allocator::AllocatorPool;
use robusta_diagnostics::Output;
use robusta_formatter::Options;

use crate::lint::{
    CliRunner, Execution, FileFailure, PreviewOutput, SourceCommand, redundant_safe_preview,
    rule_execution, with_parsed_source,
};
use crate::result::CliRunResult;
use crate::{command, mode, output_formatter};

/// Execute a parsed command against the process-owned output and color state.
pub fn run(
    command: command::Command,
    out: &mut Output,
    color: output_formatter::ColorPolicy,
) -> CliRunResult {
    match command {
        command::Command::Check(command::CheckCommand {
            config_options,
            ignore_options,
            filter_options,
            fix_options,
            output,
            prune_suppressions,
            report_unused_suppressions,
            reporter,
            debug,
            warning_options,
            runtime_options,
            color_options: _,
            input,
        }) => match rule_execution(out, fix_options, output, false, prune_suppressions) {
            Ok((fix, baseline)) => CliRunner::new(SourceCommand {
                input,
                config_options,
                ignore_options,
                filters: filter_options.filters,
                execution: Execution::Check(fix, baseline),
                list_rules: false,
                report_unused_suppressions,
                reporter,
                debug,
                warning_options,
                runtime_options,
                color,
            })
            .run(out),
            Err(result) => result,
        },
        command::Command::Format(command::FormatCommand {
            config_options,
            ignore_options,
            fix,
            output,
            reporter,
            debug,
            warning_options,
            runtime_options,
            color_options: _,
            input,
        }) => match format_execution(out, fix, output) {
            Ok(execution) => CliRunner::new(SourceCommand {
                input,
                config_options,
                ignore_options,
                // `format` runs no rules, so it has no filter or audit surface.
                filters: Vec::new(),
                execution,
                list_rules: false,
                report_unused_suppressions: command::ReportUnusedSuppressions::Configured,
                reporter,
                debug,
                warning_options,
                runtime_options,
                color,
            })
            .run(out),
            Err(result) => result,
        },
        command::Command::Lint(command::LintCommand {
            config_options,
            ignore_options,
            filter_options,
            fix_options,
            output,
            reporter,
            debug,
            warning_options,
            runtime_options,
            color_options: _,
            suppression_options: command::SuppressionOptions { suppress_all, prune_suppressions },
            report_unused_suppressions,
            list_rules,
            input,
        }) => match rule_execution(out, fix_options, output, suppress_all, prune_suppressions) {
            Ok((fix, baseline)) => CliRunner::new(SourceCommand {
                input,
                config_options,
                ignore_options,
                filters: filter_options.filters,
                execution: Execution::Lint(fix, baseline),
                list_rules,
                report_unused_suppressions,
                reporter,
                debug,
                warning_options,
                runtime_options,
                color,
            })
            .run(out),
            Err(result) => result,
        },
        command::Command::Init(command::InitCommand { color_options: _ }) => mode::run_init(out),
        command::Command::Config(command::ConfigCommand {
            files: None,
            config_options,
            color_options: _,
        }) => mode::run_print_config(out, &config_options),
        command::Command::Config(command::ConfigCommand {
            files: Some(listing),
            config_options,
            color_options: _,
        }) => mode::run_debug_files(out, &listing, &config_options),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatExecution {
    Verify,
    Preview(PreviewOutput),
    Write,
}

fn format_execution(
    out: &mut Output,
    fix: bool,
    output: command::ChangeOutput,
) -> Result<Execution, CliRunResult> {
    let execution = match (fix, PreviewOutput::from_change_output(output)) {
        (true, Some(output)) => return Err(redundant_safe_preview(out, output)),
        (false, Some(output)) => FormatExecution::Preview(output),
        (true, None) => FormatExecution::Write,
        (false, None) => FormatExecution::Verify,
    };
    Ok(Execution::Format(execution))
}

pub fn format_source(
    allocator_pool: &AllocatorPool,
    source: &str,
    options: &Options,
) -> Result<String, FileFailure> {
    with_parsed_source(allocator_pool, source, |allocator, result| {
        let formatted =
            robusta_formatter::format_compilation_unit(allocator, &result.unit, options);
        verify_formatted_source(source, &formatted, options);
        formatted
    })
}

/// Run the opt-in formatter safety nets at the application boundary, on each
/// file's final output. Both features are off by default: the source-integrity
/// pass lexes each revision once and parses one that has a documentation
/// comment, while idempotency deliberately pays for a second parse and format
/// only when requested.
#[cfg_attr(
    not(all(feature = "detect_code_removal", feature = "check_idempotency")),
    expect(unused_variables)
)]
pub fn verify_formatted_source(source: &str, formatted: &str, options: &Options) {
    #[cfg(feature = "detect_code_removal")]
    if let Err(error) = robusta_formatter::detect_code_removal(source, formatted) {
        unreachable!("formatter source-integrity failure:\n{error}");
    }

    #[cfg(feature = "check_idempotency")]
    {
        let failure = match robusta_formatter::check_idempotency(formatted, options) {
            Err(error) => Some(format!("formatted output parse diagnostics: {error:?}")),
            Ok(report) => report.failure_message(),
        };
        if let Some(error) = failure {
            unreachable!("formatter idempotency failure:\n{error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_resolves_only_formatting_operations() {
        let mut out = Output::stdio();
        for (fix, flag, expected) in [
            (false, command::ChangeOutput::Default, FormatExecution::Verify),
            (true, command::ChangeOutput::Default, FormatExecution::Write),
            (false, command::ChangeOutput::Diff, FormatExecution::Preview(PreviewOutput::Diff)),
            (
                false,
                command::ChangeOutput::ListDifferent,
                FormatExecution::Preview(PreviewOutput::ListDifferent),
            ),
        ] {
            assert_eq!(format_execution(&mut out, fix, flag).unwrap(), Execution::Format(expected),);
        }
    }
}
