// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxfmt/src/core/oxfmtrc.rs

use std::fmt;

use robusta_config::deserialize_scalar;
use schemars::JsonSchema;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};

/// Java formatter settings. Omitted values use `.editorconfig` fallbacks and
/// then Robusta's built-in defaults.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FormatConfig {
    /// Target maximum line width in Unicode display columns.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    #[schemars(range(min = 1, max = 320))]
    pub line_width: Option<usize>,
    /// Character style used for structural indentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyleConfig>,
    /// Display columns per indentation level and per tab in tab mode.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    #[schemars(range(min = 0, max = 24))]
    pub indent_width: Option<usize>,
    /// Where `else`, `catch`, and `finally` go after a closing brace. Default: `same-line`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_closing_brace: Option<AfterClosingBraceConfig>,
    /// Order imports are emitted in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_order: Option<ImportOrderConfig>,
    /// Javadoc comment formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javadoc: Option<JavadocConfig>,
    /// Whether to place the opening and closing `"""` of a text block as Google Java Style
    /// does. Default: `false`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub text_blocks: Option<bool>,
    /// Line terminator emitted by the formatter. Default: `lf`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEndingConfig>,
    /// Whether non-empty formatted output ends with one line terminator.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub insert_final_newline: Option<bool>,
}

impl FormatConfig {
    /// Replace each setting `other` names, also inside `import-order` and
    /// `javadoc` mappings.
    ///
    /// # Panics
    ///
    /// When the merged settings fail to deserialize, which two valid
    /// configurations never do.
    pub fn merge(&mut self, other: &Self) {
        let base = serde_json::to_value(&*self).unwrap();
        let overlay = serde_json::to_value(other).unwrap();
        *self = serde_json::from_value(json_deep_merge(base, overlay)).unwrap();
    }
}

/// Merge two JSON values. An overlay value replaces the base value unless both are objects.
fn json_deep_merge(base: serde_json::Value, overlay: serde_json::Value) -> serde_json::Value {
    match (base, overlay) {
        (serde_json::Value::Object(mut base_map), serde_json::Value::Object(overlay_map)) => {
            for (key, overlay_value) in overlay_map {
                let merged = match base_map.remove(&key) {
                    Some(base_value) => json_deep_merge(base_value, overlay_value),
                    None => overlay_value,
                };
                base_map.insert(key, merged);
            }
            serde_json::Value::Object(base_map)
        }
        (_base, overlay) => overlay,
    }
}

/// The character style used for structural indentation.
///
/// `space` indents with spaces. `tab` emits one tab per level and treats
/// `indent-width` as the width a tab is displayed at.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IndentStyleConfig {
    Space,
    Tab,
}

/// `same-line` keeps the keyword on the closing brace's line, and `new-line`
/// puts it on a new line.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AfterClosingBraceConfig {
    SameLine,
    NewLine,
}

/// Import-kind sections and package-prefix groups, in emitted order.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ImportOrderConfig {
    /// Import kinds in emitted section order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<ImportSectionConfig>>,
    /// Package-prefix groups in emitted order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ImportGroupConfig>>,
}

/// Javadoc formatting: `false` keeps comments as written, `true` formats them
/// with the default settings, and a mapping formats them with the named
/// settings.
#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
pub enum JavadocConfig {
    Enabled(bool),
    Settings(JavadocSettingsConfig),
}

impl<'de> Deserialize<'de> for JavadocConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct JavadocConfigVisitor;

        impl<'de> Visitor<'de> for JavadocConfigVisitor {
            type Value = JavadocConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("true, false, or a mapping of Javadoc settings")
            }

            fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
                Ok(JavadocConfig::Enabled(value))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                JavadocSettingsConfig::deserialize(MapAccessDeserializer::new(map))
                    .map(JavadocConfig::Settings)
            }
        }

        deserializer.deserialize_any(JavadocConfigVisitor)
    }
}

/// Javadoc settings. An omitted setting keeps its default.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct JavadocSettingsConfig {
    /// Capitalize the first word of each description paragraph. Default: `false`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub capitalize_descriptions: Option<bool>,
    /// Which descriptions that end in a letter or digit get a period. Default: `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_with_dot: Option<DescriptionWithDotConfig>,
    /// When a traditional comment uses the single-line form. Default: `single-line`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_line_strategy: Option<CommentLineStrategyConfig>,
    /// Whether paragraphs are rewrapped or keep line breaks that fit. Default: `greedy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_wrapping_style: Option<LineWrappingStyleConfig>,
    /// Where blank lines separate block tags. Default: `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_tag_groups: Option<SeparateTagGroupsConfig>,
    /// Start each paragraph after the first of a traditional comment with
    /// `<p>`, and remove one from the first. Default: `false`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub paragraph_html_tags: Option<bool>,
}

/// `never` adds no period, `summary` adds one to the summary javadoc shows for
/// the comment, and `always` adds one to each description paragraph.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DescriptionWithDotConfig {
    Never,
    Summary,
    Always,
}

/// `single-line` writes `/** Content. */` when a comment without block tags
/// fits, `multiline` never does, and `keep` does only for a comment already on
/// one line.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CommentLineStrategyConfig {
    SingleLine,
    Multiline,
    Keep,
}

/// `greedy` fills each line to the line width. `keep` keeps a paragraph's line
/// breaks when every line fits.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LineWrappingStyleConfig {
    Greedy,
    Keep,
}

/// `never` keeps block tags together, `before-return` adds a blank line before
/// `@return`, and `always` adds one between block tags of different kinds.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SeparateTagGroupsConfig {
    Never,
    BeforeReturn,
    Always,
}

/// `lf`, `crlf`, and `cr` end each output line with LF, CRLF, or CR.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LineEndingConfig {
    Lf,
    Crlf,
    Cr,
}

/// One import kind, as a section in the emitted order.
///
/// `static` covers static imports, `module` covers module imports, and
/// `type` covers everything else.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ImportSectionConfig {
    Static,
    Module,
    Type,
}

/// One package-prefix group in the emitted order.
///
/// A string is a single prefix; a list is several prefixes sharing one group.
/// `*` is the catch-all group every import not matched by a named prefix
/// falls into.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
pub enum ImportGroupConfig {
    Single(String),
    Inline(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::FormatConfig;

    fn format_config(json: serde_json::Value) -> FormatConfig {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn merge_replaces_each_named_setting() {
        let mut config = format_config(serde_json::json!({
            "line-width": 100,
            "indent-width": 2,
            "import-order": { "sections": ["static"], "groups": ["java"] },
            "javadoc": { "capitalize-descriptions": true },
        }));
        config.merge(&format_config(serde_json::json!({
            "line-width": 120,
            "import-order": { "groups": ["com"] },
            "javadoc": { "paragraph-html-tags": true },
        })));

        assert_eq!(
            config,
            format_config(serde_json::json!({
                "line-width": 120,
                "indent-width": 2,
                "import-order": { "sections": ["static"], "groups": ["com"] },
                "javadoc": { "capitalize-descriptions": true, "paragraph-html-tags": true },
            }))
        );

        config.merge(&format_config(serde_json::json!({ "javadoc": false })));
        assert_eq!(config.javadoc, format_config(serde_json::json!({ "javadoc": false })).javadoc);
    }
}
