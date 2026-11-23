use robusta_formatter::{
    AfterClosingBrace, CommentLineStrategy, DescriptionWithDot, ImportGroup, ImportMatcher,
    ImportOrder, ImportSection, IndentStyle, JavadocOptions, LineEnding, LineWrappingStyle,
    Options, SeparateTagGroups,
};

use super::super::format_config::{
    AfterClosingBraceConfig, CommentLineStrategyConfig, DescriptionWithDotConfig, FormatConfig,
    ImportGroupConfig, ImportOrderConfig, ImportSectionConfig, IndentStyleConfig, JavadocConfig,
    JavadocSettingsConfig, LineEndingConfig, LineWrappingStyleConfig, SeparateTagGroupsConfig,
};

/// Convert `Options` back into the `FormatConfig` that sets every field.
pub fn to_format_config(options: &Options) -> FormatConfig {
    let Options {
        line_width,
        indent_style,
        indent_width,
        after_closing_brace,
        import_order,
        javadoc,
        line_ending,
        insert_final_newline,
    } = options;
    let ImportOrder { sections: import_sections, groups: import_groups } = import_order;

    let sections = import_sections
        .iter()
        .map(|section| match section {
            ImportSection::Static => ImportSectionConfig::Static,
            ImportSection::Module => ImportSectionConfig::Module,
            ImportSection::Type => ImportSectionConfig::Type,
        })
        .collect();
    let groups = import_groups
        .iter()
        .map(|group| {
            let ImportGroup { matchers } = group;
            ImportGroupConfig::Inline(
                matchers
                    .iter()
                    .map(|matcher| match matcher {
                        ImportMatcher::Prefix(prefix) => prefix.clone(),
                        ImportMatcher::All => "*".to_string(),
                    })
                    .collect(),
            )
        })
        .collect();

    FormatConfig {
        line_width: Some(usize::from(line_width.value())),
        indent_style: Some(match indent_style {
            IndentStyle::Space => IndentStyleConfig::Space,
            IndentStyle::Tab => IndentStyleConfig::Tab,
        }),
        indent_width: Some(usize::from(indent_width.value())),
        after_closing_brace: Some(match after_closing_brace {
            AfterClosingBrace::SameLine => AfterClosingBraceConfig::SameLine,
            AfterClosingBrace::NewLine => AfterClosingBraceConfig::NewLine,
        }),
        import_order: Some(ImportOrderConfig { sections: Some(sections), groups: Some(groups) }),
        javadoc: Some(javadoc.map_or(JavadocConfig::Enabled(false), to_javadoc_config)),
        line_ending: Some(match line_ending {
            LineEnding::Lf => LineEndingConfig::Lf,
            LineEnding::Crlf => LineEndingConfig::Crlf,
            LineEnding::Cr => LineEndingConfig::Cr,
        }),
        insert_final_newline: Some(*insert_final_newline),
    }
}

fn to_javadoc_config(options: JavadocOptions) -> JavadocConfig {
    let JavadocOptions {
        capitalize_descriptions,
        description_with_dot,
        comment_line_strategy,
        line_wrapping_style,
        separate_tag_groups,
        paragraph_html_tags,
    } = options;
    JavadocConfig::Settings(JavadocSettingsConfig {
        capitalize_descriptions: Some(capitalize_descriptions),
        description_with_dot: Some(match description_with_dot {
            DescriptionWithDot::Never => DescriptionWithDotConfig::Never,
            DescriptionWithDot::Summary => DescriptionWithDotConfig::Summary,
            DescriptionWithDot::Always => DescriptionWithDotConfig::Always,
        }),
        comment_line_strategy: Some(match comment_line_strategy {
            CommentLineStrategy::SingleLine => CommentLineStrategyConfig::SingleLine,
            CommentLineStrategy::Multiline => CommentLineStrategyConfig::Multiline,
            CommentLineStrategy::Keep => CommentLineStrategyConfig::Keep,
        }),
        line_wrapping_style: Some(match line_wrapping_style {
            LineWrappingStyle::Greedy => LineWrappingStyleConfig::Greedy,
            LineWrappingStyle::Keep => LineWrappingStyleConfig::Keep,
        }),
        separate_tag_groups: Some(match separate_tag_groups {
            SeparateTagGroups::Never => SeparateTagGroupsConfig::Never,
            SeparateTagGroups::BeforeReturn => SeparateTagGroupsConfig::BeforeReturn,
            SeparateTagGroups::Always => SeparateTagGroupsConfig::Always,
        }),
        paragraph_html_tags: Some(paragraph_html_tags),
    })
}
