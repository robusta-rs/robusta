use std::path::Path;

use robusta_formatter::{ImportOrder, IndentWidth, LineWidth};

use super::super::format_config::FormatConfig;
use super::to_robusta_formatter::to_import_order;
use crate::config_loader::ConfigError;

/// Every value whose derivation can fail, derived once. `None` keeps the default.
#[derive(Debug)]
pub struct ValidatedOptions {
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub import_order: Option<ImportOrder>,
}

/// The validation gate between a filled [`FormatConfig`] and its options.
///
/// # Errors
///
/// Returns an error if a width is out of range or the import order is invalid.
pub fn validate(
    config: &FormatConfig,
    path: Option<&Path>,
) -> Result<ValidatedOptions, ConfigError> {
    let invalid =
        |key, message| ConfigError::OptionValue { path: path.map(Path::to_path_buf), key, message };
    Ok(ValidatedOptions {
        line_width: config
            .line_width
            .map(line_width)
            .transpose()
            .map_err(|message| invalid("line-width", message))?,
        indent_width: config
            .indent_width
            .map(indent_width)
            .transpose()
            .map_err(|message| invalid("indent-width", message))?,
        import_order: to_import_order(config, path)?,
    })
}

/// Check `value` against the `line-width` range.
///
/// # Errors
///
/// Returns the range and `value` if it is outside the range.
pub fn line_width(value: usize) -> Result<LineWidth, String> {
    let width = u16::try_from(value).ok().and_then(|value| LineWidth::try_from(value).ok());
    width.ok_or_else(|| {
        format!("must be between {} and {}; got {value}", LineWidth::MIN, LineWidth::MAX)
    })
}

/// Check `value` against the `indent-width` range.
///
/// # Errors
///
/// Returns the range and `value` if it is outside the range.
pub fn indent_width(value: usize) -> Result<IndentWidth, String> {
    let width = u8::try_from(value).ok().and_then(|value| IndentWidth::try_from(value).ok());
    width.ok_or_else(|| {
        format!("must be between {} and {}; got {value}", IndentWidth::MIN, IndentWidth::MAX)
    })
}
