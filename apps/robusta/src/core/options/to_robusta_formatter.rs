use std::path::Path;

use robusta_formatter::{
    AfterClosingBrace, CommentLineStrategy, DescriptionWithDot, ImportGroup, ImportMatcher,
    ImportOrder, ImportSection, IndentStyle, JavadocOptions, LineEnding, LineWrappingStyle,
    Options, SeparateTagGroups,
};

use super::super::format_config::{
    AfterClosingBraceConfig, CommentLineStrategyConfig, DescriptionWithDotConfig, FormatConfig,
    ImportGroupConfig, ImportOrderConfig, ImportSectionConfig, IndentStyleConfig, JavadocConfig,
    LineEndingConfig, LineWrappingStyleConfig, SeparateTagGroupsConfig,
};
use super::ValidatedOptions;
use crate::config_loader::ConfigError;

/// Convert `FormatConfig` into `Options` for `robusta_formatter`.
///
/// `validated` holds the gate's results, so this cannot fail.
pub fn to_robusta_formatter(config: &FormatConfig, validated: ValidatedOptions) -> Options {
    let ValidatedOptions { line_width, indent_width, import_order } = validated;
    let mut options = Options::default();

    if let Some(width) = line_width {
        options.line_width = width;
    }
    if let Some(style) = config.indent_style {
        options.indent_style = match style {
            IndentStyleConfig::Space => IndentStyle::Space,
            IndentStyleConfig::Tab => IndentStyle::Tab,
        };
    }
    if let Some(width) = indent_width {
        options.indent_width = width;
    }
    if let Some(placement) = config.after_closing_brace {
        options.after_closing_brace = match placement {
            AfterClosingBraceConfig::SameLine => AfterClosingBrace::SameLine,
            AfterClosingBraceConfig::NewLine => AfterClosingBrace::NewLine,
        };
    }
    if let Some(import_order) = import_order {
        options.import_order = import_order;
    }
    if let Some(javadoc) = &config.javadoc {
        options.javadoc = to_javadoc(javadoc);
    }
    if let Some(text_blocks) = config.text_blocks {
        options.text_blocks = text_blocks;
    }
    if let Some(ending) = config.line_ending {
        options.line_ending = match ending {
            LineEndingConfig::Lf => LineEnding::Lf,
            LineEndingConfig::Crlf => LineEnding::Crlf,
            LineEndingConfig::Cr => LineEnding::Cr,
        };
    }
    if let Some(insert) = config.insert_final_newline {
        options.insert_final_newline = insert;
    }

    options
}

fn to_javadoc(config: &JavadocConfig) -> Option<JavadocOptions> {
    let settings = match config {
        JavadocConfig::Enabled(enabled) => return enabled.then(JavadocOptions::default),
        JavadocConfig::Settings(settings) => settings,
    };

    let mut options = JavadocOptions::default();
    if let Some(capitalize) = settings.capitalize_descriptions {
        options.capitalize_descriptions = capitalize;
    }
    if let Some(dot) = settings.description_with_dot {
        options.description_with_dot = match dot {
            DescriptionWithDotConfig::Never => DescriptionWithDot::Never,
            DescriptionWithDotConfig::Summary => DescriptionWithDot::Summary,
            DescriptionWithDotConfig::Always => DescriptionWithDot::Always,
        };
    }
    if let Some(strategy) = settings.comment_line_strategy {
        options.comment_line_strategy = match strategy {
            CommentLineStrategyConfig::SingleLine => CommentLineStrategy::SingleLine,
            CommentLineStrategyConfig::Multiline => CommentLineStrategy::Multiline,
            CommentLineStrategyConfig::Keep => CommentLineStrategy::Keep,
        };
    }
    if let Some(style) = settings.line_wrapping_style {
        options.line_wrapping_style = match style {
            LineWrappingStyleConfig::Greedy => LineWrappingStyle::Greedy,
            LineWrappingStyleConfig::Keep => LineWrappingStyle::Keep,
        };
    }
    if let Some(policy) = settings.separate_tag_groups {
        options.separate_tag_groups = match policy {
            SeparateTagGroupsConfig::Never => SeparateTagGroups::Never,
            SeparateTagGroupsConfig::BeforeReturn => SeparateTagGroups::BeforeReturn,
            SeparateTagGroupsConfig::Always => SeparateTagGroups::Always,
        };
    }
    if let Some(tags) = settings.paragraph_html_tags {
        options.paragraph_html_tags = tags;
    }

    Some(options)
}

/// Derive the [`ImportOrder`] for the gate ([`super::validate()`]).
///
/// # Errors
///
/// Returns an error if a section or matcher is duplicated or a matcher is malformed.
pub(super) fn to_import_order(
    config: &FormatConfig,
    path: Option<&Path>,
) -> Result<Option<ImportOrder>, ConfigError> {
    let Some(ImportOrderConfig { sections, groups }) = &config.import_order else {
        return Ok(None);
    };
    let sections = to_import_sections(sections.as_deref(), path)?;
    let Some(groups) = groups else {
        return Ok(Some(ImportOrder { sections, ..ImportOrder::default() }));
    };
    let mut parsed_groups = Vec::new();
    let mut seen = Vec::new();
    for group in groups {
        let raw_matchers = import_group_matchers(group);
        if raw_matchers.is_empty() {
            return Err(import_group_error(path, "import group must not be empty"));
        }

        let mut matchers = Vec::new();
        for raw in raw_matchers {
            let matcher = parse_import_matcher(raw, path)?;
            if seen.contains(&matcher) {
                return Err(import_group_error(
                    path,
                    format!("import group {raw:?} is duplicated"),
                ));
            }

            seen.push(matcher.clone());
            matchers.push(matcher);
        }
        parsed_groups.push(ImportGroup { matchers });
    }

    append_missing_default_group(&mut parsed_groups, &seen, ImportMatcher::All);

    Ok(Some(ImportOrder { sections, groups: parsed_groups }))
}

fn to_import_sections(
    config: Option<&[ImportSectionConfig]>,
    path: Option<&Path>,
) -> Result<Vec<ImportSection>, ConfigError> {
    let Some(config) = config else {
        return Ok(ImportOrder::default().sections);
    };
    let mut sections = Vec::new();
    for section in config {
        let section = match section {
            ImportSectionConfig::Static => ImportSection::Static,
            ImportSectionConfig::Module => ImportSection::Module,
            ImportSectionConfig::Type => ImportSection::Type,
        };
        if sections.contains(&section) {
            return Err(import_section_error(
                path,
                format!("{} is duplicated", import_section_name(section)),
            ));
        }
        sections.push(section);
    }
    for section in ImportOrder::default().sections {
        if !sections.contains(&section) {
            sections.push(section);
        }
    }
    Ok(sections)
}

const fn import_section_name(section: ImportSection) -> &'static str {
    match section {
        ImportSection::Static => "static",
        ImportSection::Module => "module",
        ImportSection::Type => "type",
    }
}

fn import_group_matchers(config: &ImportGroupConfig) -> &[String] {
    match config {
        ImportGroupConfig::Single(matcher) => std::slice::from_ref(matcher),
        ImportGroupConfig::Inline(matchers) => matchers,
    }
}

fn parse_import_matcher(raw: &str, path: Option<&Path>) -> Result<ImportMatcher, ConfigError> {
    if raw.is_empty() {
        return Err(import_group_error(path, "import group matcher must not be empty"));
    }

    if raw == "*" {
        return Ok(ImportMatcher::All);
    }

    if raw.starts_with('<') || raw.ends_with('>') {
        return Err(import_group_error(
            path,
            format!("import group {raw:?} is not a dotted package prefix or `*`"),
        ));
    }
    if raw.contains('*')
        || raw.contains('/')
        || raw.contains('\\')
        || raw.contains('[')
        || raw.contains(']')
        || raw.contains('?')
    {
        return Err(import_group_error(
            path,
            format!("import group {raw:?} uses unsupported glob/path syntax"),
        ));
    }
    if raw.starts_with('.') || raw.ends_with('.') || raw.contains("..") {
        return Err(import_group_error(
            path,
            format!("import group {raw:?} is not a dotted package prefix"),
        ));
    }
    if raw.split('.').any(|segment| segment.is_empty() || segment.chars().any(char::is_whitespace))
    {
        return Err(import_group_error(
            path,
            format!("import group {raw:?} is not a dotted package prefix"),
        ));
    }

    Ok(ImportMatcher::Prefix(raw.to_string()))
}

fn append_missing_default_group(
    groups: &mut Vec<ImportGroup>,
    seen: &[ImportMatcher],
    matcher: ImportMatcher,
) {
    if seen.contains(&matcher) {
        return;
    }
    groups.push(ImportGroup { matchers: vec![matcher] });
}

fn import_group_error(path: Option<&Path>, message: impl Into<String>) -> ConfigError {
    ConfigError::ImportGroup { path: path.map(Path::to_path_buf), message: message.into() }
}

fn import_section_error(path: Option<&Path>, message: impl Into<String>) -> ConfigError {
    ConfigError::ImportSectionOrder { path: path.map(Path::to_path_buf), message: message.into() }
}
