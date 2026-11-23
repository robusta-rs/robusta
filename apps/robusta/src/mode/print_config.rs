use std::path::Path;

use robusta_diagnostics::Output;
use robusta_linter::LintConfigFile;

use crate::command::ConfigOptions;
use crate::config_file::{ConfigFile, FormatSection};
use crate::config_loader::Config;
use crate::core::options::to_format_config;
use crate::result::CliRunResult;
use crate::walk::{SelectionFeatures, config_for_input};

pub fn run_print_config(out: &mut Output, options: &ConfigOptions) -> CliRunResult {
    let Ok(resolved) = config_for_input(out, options, Path::new("."), SelectionFeatures::CHECK)
    else {
        return CliRunResult::ConfigResolutionFailed;
    };
    match to_yaml(&resolved) {
        Ok(yaml) => {
            out.stdout(&yaml);
            CliRunResult::ConfigPrinted
        }
        Err(error) => {
            out.stderr_line(&format!("failed to serialize resolved config: {error}"));
            CliRunResult::ConfigSerializationFailed
        }
    }
}

/// Serialize the resolved configuration as `robusta.yaml`, with formatter
/// defaults and disabled lint rules written out.
pub fn to_yaml(config: &Config) -> Result<String, serde_saphyr::SerializeError> {
    serde_saphyr::to_string(&resolved_config_file(config))
}

/// Materialize the resolved state as the schema `robusta.yaml` is parsed
/// from, so every printed name is a name deserialization accepts.
fn resolved_config_file(config: &Config) -> ConfigFile {
    let Config {
        options,
        ignore_patterns,
        format_ignore_patterns,
        lint_ignore_patterns,
        format_overrides,
        lint,
    } = config;

    ConfigFile {
        format: Some(FormatSection {
            format_config: to_format_config(options),
            overrides: Some(format_overrides.clone()),
            ignore_patterns: Some(format_ignore_patterns.clone()),
        }),
        ignore_patterns: Some(ignore_patterns.clone()),
        lint: Some(LintConfigFile {
            ignore_patterns: Some(lint_ignore_patterns.clone()),
            ..lint.resolve_final_config_file()
        }),
    }
}
