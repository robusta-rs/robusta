// Directory adapted from https://github.com/oxc-project/oxc/tree/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxfmt/src/core/options

//! Converters between the typed [`super::format_config::FormatConfig`] and formatter options.
//!
//! - [`validate()`]: the only fallible step, which derives [`ValidatedOptions`]
//! - [`to_robusta_formatter()`]: `robusta_formatter::Options` built from them
//! - [`to_format_config()`]: the reverse, for printing the resolved configuration

mod to_format_config;
mod to_robusta_formatter;
mod validate;

pub use to_format_config::to_format_config;
pub use to_robusta_formatter::to_robusta_formatter;
pub use validate::{ValidatedOptions, indent_width, line_width, validate};
