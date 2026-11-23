// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxfmt/src/core/oxfmtrc.rs

use std::fmt;

use robusta_config::{GlobSet, IgnorePatterns};
use robusta_linter::LintConfigFile;
use schemars::JsonSchema;
use serde::de::{
    DeserializeSeed, IntoDeserializer, MapAccess, Visitor, value::MapAccessDeserializer,
};
use serde::{Deserialize, Deserializer, Serialize};

use crate::core::format_config::FormatConfig;

/// Robusta Configuration File
///
/// Robusta reads `robusta.yaml` from the directory it processes, or from the
/// nearest ancestor directory that has one. Every section is optional: an
/// omitted option falls back to `.editorconfig` where one applies, and then to
/// Robusta's built-in default.
///
/// Usage: `robusta check -c robusta.yaml`
///
/// Example
///
/// `robusta.yaml`
///
/// ```yaml
/// format: { line-width: 100, indent-style: space, indent-width: 4 }
/// ignore-patterns: ["generated/"]
/// lint: { rules: { unused-import: warn } }
/// ```
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ConfigFile {
    /// Java formatter policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSection>,
    /// Java files to skip when checking, formatting, and linting.
    ///
    /// `.gitignore` files and the command-line ignore options still apply: a
    /// `!` pattern here re-includes only a path an earlier entry in this list
    /// skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_patterns: Option<IgnorePatterns>,
    /// Which lint rules run, how loudly, and the run-wide lint options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lint: Option<LintConfigFile>,
}

/// Java formatter settings, their overrides, and the Java files to skip when
/// formatting. Omitted settings use `.editorconfig` fallbacks and then
/// Robusta's built-in defaults.
#[derive(Debug, Default, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FormatSection {
    #[serde(flatten)]
    pub format_config: FormatConfig,
    /// Format settings for the Java files that match globs. When a file
    /// matches several overrides, the later one wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<FormatOverrideConfig>>,
    /// Java files to skip when formatting, besides those the top-level
    /// `ignore-patterns` skips.
    ///
    /// A `!` pattern re-includes only a path an earlier entry in this list
    /// skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_patterns: Option<IgnorePatterns>,
}

// Read through the mapping instead of `#[serde(flatten)]`, whose buffering
// would report a setting's error at its key rather than its value.
impl<'de> Deserialize<'de> for FormatSection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FormatSectionVisitor;

        impl<'de> Visitor<'de> for FormatSectionVisitor {
            type Value = FormatSection;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a mapping of format settings")
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                let mut overrides = None;
                let mut ignore_patterns = None;
                let settings = FormatSettings {
                    map,
                    overrides: &mut overrides,
                    ignore_patterns: &mut ignore_patterns,
                };
                let format_config =
                    FormatConfig::deserialize(MapAccessDeserializer::new(settings))?;
                Ok(FormatSection { format_config, overrides, ignore_patterns })
            }
        }

        deserializer.deserialize_map(FormatSectionVisitor)
    }
}

/// The `format` mapping without `overrides` and `ignore-patterns`, which it
/// takes aside.
struct FormatSettings<'a, A> {
    map: A,
    overrides: &'a mut Option<Vec<FormatOverrideConfig>>,
    ignore_patterns: &'a mut Option<IgnorePatterns>,
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for FormatSettings<'_, A> {
    type Error = A::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, A::Error> {
        while let Some(key) = self.map.next_key::<String>()? {
            match key.as_str() {
                "overrides" => *self.overrides = self.map.next_value()?,
                "ignore-patterns" => *self.ignore_patterns = self.map.next_value()?,
                _ => return seed.deserialize(key.into_deserializer()).map(Some),
            }
        }
        Ok(None)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, A::Error> {
        self.map.next_value_seed(seed)
    }
}

/// Format settings for the Java files one override matches.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FormatOverrideConfig {
    /// Glob patterns for the files this override applies to.
    pub files: GlobSet,
    /// Glob patterns for the files this override leaves out.
    #[serde(default, skip_serializing_if = "GlobSet::is_empty")]
    pub exclude_files: GlobSet,
    /// Format settings that replace the ones in effect, each named setting on its own.
    #[serde(default)]
    pub options: FormatConfig,
}
