// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxlint/src/mode/rules.rs
use std::path::Path;

use robusta_diagnostics::Output;
use robusta_linter::LintFilter;

use crate::command::ConfigOptions;
use crate::output_formatter::ReportFormat;
use crate::result::CliRunResult;
use crate::walk::{SelectionFeatures, config_for_input};

pub fn run_rules(
    out: &mut Output,
    options: &ConfigOptions,
    filters: &[LintFilter],
    reporter: ReportFormat,
) -> CliRunResult {
    let Ok(config) = config_for_input(out, options, Path::new("."), SelectionFeatures::LINT) else {
        return CliRunResult::ConfigResolutionFailed;
    };
    let config = config.lint.with_filters(filters).build(Path::new(""));
    let enabled = config.rules().iter().map(|rule| rule.rule().id()).collect();
    if let Some(output) = reporter.all_rules(&enabled) {
        out.stdout(&output);
    }
    CliRunResult::RulesListed
}
