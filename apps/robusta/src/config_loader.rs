use std::fmt;
use std::path::{Path, PathBuf};

use robusta_config::IgnorePatterns;
use robusta_formatter::Options;
use robusta_linter::{ConfigStoreBuilder, OptionsConfig};

use crate::config_file::{ConfigFile, FormatOverrideConfig, FormatSection};
use crate::core::format_config::FormatConfig;
use crate::core::options::{to_robusta_formatter, validate};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Config {
    pub options: Options,
    pub ignore_patterns: IgnorePatterns,
    pub format_ignore_patterns: IgnorePatterns,
    pub lint_ignore_patterns: IgnorePatterns,
    pub format_overrides: Vec<FormatOverrideConfig>,
    pub lint: ConfigStoreBuilder,
}

#[derive(Debug)]
pub struct LoadedConfig {
    pub path: PathBuf,
    pub config: Config,
    /// The format section as written, for `.editorconfig` to fill.
    pub(crate) format: FormatConfig,
    /// The first run-wide option the file sets, even to its default.
    pub(crate) run_option: Option<&'static str>,
}

#[derive(Debug)]
pub enum ConfigError {
    Read { path: PathBuf, source: std::io::Error },
    Parse { path: Option<PathBuf>, source: serde_saphyr::Error },
    ImportGroup { path: Option<PathBuf>, message: String },
    ImportSectionOrder { path: Option<PathBuf>, message: String },
    OptionValue { path: Option<PathBuf>, key: &'static str, message: String },
    Lint { path: Option<PathBuf>, message: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(f, "{}: failed to read config: {source}", path.display())
            }
            Self::Parse { path: Some(path), source } => {
                write!(f, "{}: failed to parse config: {source}", path.display())
            }
            Self::Parse { path: None, source } => write!(f, "failed to parse config: {source}"),
            Self::ImportGroup { path: Some(path), message } => {
                write!(f, "{}: {message}", path.display())
            }
            Self::ImportGroup { path: None, message } => write!(f, "{message}"),
            Self::ImportSectionOrder { path: Some(path), message } => {
                write!(f, "{}: invalid format.import-order.sections: {message}", path.display())
            }
            Self::ImportSectionOrder { path: None, message } => {
                write!(f, "invalid format.import-order.sections: {message}")
            }
            Self::OptionValue { path: Some(path), key, message } => {
                write!(f, "{}: invalid format.{key}: {message}", path.display())
            }
            Self::OptionValue { path: None, key, message } => {
                write!(f, "invalid format.{key}: {message}")
            }
            Self::Lint { path: Some(path), message } => {
                write!(f, "{}: invalid lint config: {message}", path.display())
            }
            Self::Lint { path: None, message } => write!(f, "invalid lint config: {message}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::ImportGroup { .. }
            | Self::ImportSectionOrder { .. }
            | Self::OptionValue { .. }
            | Self::Lint { .. } => None,
        }
    }
}

fn into_config(config: ConfigFile, path: Option<&Path>) -> Result<Config, ConfigError> {
    // Destructure exhaustively so a new schema field cannot be added without
    // forcing this projection to consume it.
    let ConfigFile { format, ignore_patterns, lint } = config;

    let FormatSection {
        format_config: format,
        overrides: format_overrides,
        ignore_patterns: format_ignore_patterns,
    } = format.unwrap_or_default();
    let options = to_robusta_formatter(&format, validate(&format, path)?);
    let format_overrides = format_overrides.unwrap_or_default();
    for r#override in &format_overrides {
        validate(&r#override.options, path)?;
    }
    let mut lint = lint.unwrap_or_default();
    let lint_ignore_patterns = lint.ignore_patterns.take();

    Ok(Config {
        options,
        ignore_patterns: ignore_patterns.unwrap_or_default(),
        format_ignore_patterns: format_ignore_patterns.unwrap_or_default(),
        lint_ignore_patterns: lint_ignore_patterns.unwrap_or_default(),
        format_overrides,
        lint: ConfigStoreBuilder::from_config(lint).map_err(|error| ConfigError::Lint {
            path: path.map(Path::to_path_buf),
            message: error.to_string(),
        })?,
    })
}

/// # Errors
///
/// Returns an error when the YAML is malformed or an option value is invalid.
#[cfg(test)]
pub fn parse_config(source: &str) -> Result<Config, ConfigError> {
    into_config(parse_config_file(source, None)?, None)
}

fn parse_config_file(source: &str, path: Option<&Path>) -> Result<ConfigFile, ConfigError> {
    Ok(serde_saphyr::from_str_with_options::<Option<ConfigFile>>(
        source,
        serde_saphyr::options! {
            strict_booleans: true,
            merge_keys: serde_saphyr::MergeKeyPolicy::AsOrdinary,
            ignore_binary_tag_for_string: true,
        },
    )
    .map_err(|source| ConfigError::Parse { path: path.map(Path::to_path_buf), source })?
    .unwrap_or_default())
}

fn load_config_with_path(source: &str, path: &Path) -> Result<LoadedConfig, ConfigError> {
    let config = parse_config_file(source, Some(path))?;
    let format =
        config.format.as_ref().map(|format| format.format_config.clone()).unwrap_or_default();
    let run_option = config.lint.as_ref().and_then(|lint| first_run_option(&lint.options));
    let config = into_config(config, Some(path))?;
    Ok(LoadedConfig { path: path.to_path_buf(), config, format, run_option })
}

fn first_run_option(options: &OptionsConfig) -> Option<&'static str> {
    let OptionsConfig { deny_warnings, max_warnings, report_unused_suppressions } = options;
    [
        ("lint.options.deny-warnings", deny_warnings.is_some()),
        ("lint.options.max-warnings", max_warnings.is_some()),
        ("lint.options.report-unused-suppressions", report_unused_suppressions.is_some()),
    ]
    .into_iter()
    .find_map(|(name, set)| set.then_some(name))
}

/// # Errors
///
/// Returns an error when the file cannot be read, its YAML is malformed, or
/// an option value is invalid.
pub fn load_path(path: &Path) -> Result<LoadedConfig, ConfigError> {
    let source = std::fs::read_to_string(path)
        .map_err(|source| ConfigError::Read { path: path.to_path_buf(), source })?;
    load_config_with_path(&source, path)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Config, ConfigError, load_config_with_path, load_path, parse_config};
    use crate::mode::to_yaml;
    use robusta_allocator::Allocator;
    use robusta_config::{IgnorePatterns, discover_config_path};
    use robusta_diagnostics::Severity;
    use robusta_formatter::{
        AfterClosingBrace, CommentLineStrategy, DescriptionWithDot, ImportGroup, ImportMatcher,
        ImportOrder, JavadocOptions, LineWrappingStyle, Options, SeparateTagGroups,
    };
    use robusta_linter::{
        ConfigStoreBuilder, LintFilter, LintFilterKind, Linter, RuleCategory, RunOptions, rules,
    };
    use robusta_semantic::SemanticBuilder;

    fn parse_options(source: &str) -> Result<Options, ConfigError> {
        parse_config(source).map(|config| config.options)
    }

    /// The enabled rules of `lint`, as `(id, severity)` pairs in registry order.
    fn enabled(lint: &ConfigStoreBuilder) -> Vec<(&'static str, Severity)> {
        lint.clone()
            .build(Path::new(""))
            .rules()
            .iter()
            .map(|configured| (configured.rule().id().as_str(), configured.severity()))
            .collect()
    }

    fn deny(name: &str) -> LintFilter {
        LintFilter::deny(LintFilterKind::from_name(name).expect("filter should name a rule"))
    }

    fn warn(name: &str) -> LintFilter {
        LintFilter::warn(LintFilterKind::from_name(name).expect("filter should name a rule"))
    }

    fn allow(name: &str) -> LintFilter {
        LintFilter::allow(LintFilterKind::from_name(name).expect("filter should name a rule"))
    }

    #[test]
    fn resolved_yaml_round_trips_the_typed_config() {
        let config = parse_config(
            r#"
format:
  line-width: 88
  indent-style: tab
  indent-width: 2
  after-closing-brace: new-line
  import-order:
    sections: [static, type, module]
    groups:
      - ["org", "com"]
  insert-final-newline: false
ignore-patterns: ["generated/", "!generated/Keep.java"]
lint:
  rules:
    modifier-order: warn
    uppercase-long-suffix: deny
"#,
        )
        .expect("source config should parse");

        let yaml = to_yaml(&config).expect("resolved config should serialize");
        let reparsed = parse_config(&yaml).expect("printed config should be valid robusta.yaml");

        assert_eq!(reparsed, config);
        assert!(yaml.contains("line-width: 88"));
        assert!(yaml.contains("indent-style: tab"));
        assert!(yaml.contains("after-closing-brace: new-line"));
        assert!(!yaml.contains("wrapping"));
        assert!(yaml.contains("javadoc: false"));
        assert!(yaml.contains("line-ending: lf"));
        assert!(yaml.contains("insert-final-newline: false"));
        assert!(yaml.contains("redundant-return: allow"));
    }

    #[test]
    fn parse_options_applies_format_overrides() {
        let options = parse_options(
            r"
format:
  line-width: 120
  indent-width: 4
",
        )
        .expect("config should parse");

        assert_eq!(options.line_width.value(), 120);
        assert_eq!(options.indent_width.value(), 4);
    }

    #[test]
    fn parse_options_applies_indent_style_and_final_newline_policy() {
        let defaults = parse_options("").expect("empty config should parse");
        let space = parse_options("format:\n  indent-style: space\n").expect("space should parse");
        let configured =
            parse_options("format:\n  indent-style: tab\n  insert-final-newline: false\n")
                .expect("indent and newline policy should parse");

        assert_eq!(defaults.indent_style, robusta_formatter::IndentStyle::Space);
        assert_eq!(space.indent_style, robusta_formatter::IndentStyle::Space);
        assert!(defaults.insert_final_newline);
        assert_eq!(configured.indent_style, robusta_formatter::IndentStyle::Tab);
        assert!(!configured.insert_final_newline);
    }

    #[test]
    fn parse_options_applies_javadoc_settings_and_defaults_to_disabled() {
        let default = parse_options("").expect("empty config should parse");
        let disabled =
            parse_options("format:\n  javadoc: false\n").expect("javadoc false should parse");
        let enabled =
            parse_options("format:\n  javadoc: true\n").expect("javadoc true should parse");
        let settings = parse_options(
            "format:\n  javadoc:\n    comment-line-strategy: keep\n    separate-tag-groups: always\n",
        )
        .expect("javadoc settings should parse");
        let all = parse_options(concat!(
            "format:\n  javadoc:\n",
            "    capitalize-descriptions: true\n",
            "    description-with-dot: always\n",
            "    comment-line-strategy: multiline\n",
            "    line-wrapping-style: keep\n",
            "    separate-tag-groups: before-return\n",
            "    paragraph-html-tags: true\n",
        ))
        .expect("every javadoc setting should parse");

        assert_eq!(default.javadoc, None);
        assert_eq!(disabled.javadoc, None);
        assert_eq!(enabled.javadoc, Some(JavadocOptions::default()));
        assert_eq!(
            settings.javadoc,
            Some(JavadocOptions {
                comment_line_strategy: CommentLineStrategy::Keep,
                separate_tag_groups: SeparateTagGroups::Always,
                ..JavadocOptions::default()
            })
        );
        assert_eq!(
            all.javadoc,
            Some(JavadocOptions {
                capitalize_descriptions: true,
                description_with_dot: DescriptionWithDot::Always,
                comment_line_strategy: CommentLineStrategy::Multiline,
                line_wrapping_style: LineWrappingStyle::Keep,
                separate_tag_groups: SeparateTagGroups::BeforeReturn,
                paragraph_html_tags: true,
            })
        );
    }

    #[test]
    fn javadoc_settings_round_trip_through_config_show() {
        let config = parse_config(
            "format:\n  javadoc:\n    line-wrapping-style: keep\n    description-with-dot: summary\n",
        )
        .expect("javadoc settings should parse");
        let yaml = to_yaml(&config).expect("resolved config should serialize");

        assert!(yaml.contains("line-wrapping-style: keep"), "{yaml}");
        assert!(yaml.contains("description-with-dot: summary"), "{yaml}");
        assert!(yaml.contains("comment-line-strategy: single-line"), "{yaml}");
        assert_eq!(parse_config(&yaml).expect("printed config should parse"), config);
    }

    #[test]
    fn parse_options_rejects_old_javadoc_modes_and_unknown_settings() {
        for mode in ["off", "normalize", "reflow"] {
            let error = parse_options(&format!("format:\n  javadoc: {mode}\n"))
                .expect_err("old javadoc modes should fail");
            assert!(
                error
                    .to_string()
                    .contains("expected true, false, or a mapping of Javadoc settings"),
                "{error}"
            );
        }

        let error = parse_options("format:\n  javadoc:\n    prefer-code-fences: true\n")
            .expect_err("unknown javadoc settings should fail");
        assert!(error.to_string().contains("unknown field `prefer-code-fences`"), "{error}");

        let error = parse_options("format:\n  javadoc:\n    line-wrapping-style: balance\n")
            .expect_err("unknown wrapping styles should fail");
        assert!(error.to_string().contains("unknown variant `balance`"), "{error}");
    }

    #[test]
    fn parse_options_applies_after_closing_brace_and_defaults_to_same_line() {
        let default = parse_options("").expect("empty config should parse");
        let null =
            parse_options("format:\n  after-closing-brace: null\n").expect("null should parse");
        let same_line = parse_options("format:\n  after-closing-brace: same-line\n")
            .expect("same-line should parse");
        let new_line = parse_options("format:\n  after-closing-brace: new-line\n")
            .expect("new-line should parse");

        assert_eq!(default.after_closing_brace, AfterClosingBrace::SameLine);
        assert_eq!(null.after_closing_brace, AfterClosingBrace::SameLine);
        assert_eq!(same_line.after_closing_brace, AfterClosingBrace::SameLine);
        assert_eq!(new_line.after_closing_brace, AfterClosingBrace::NewLine);
    }

    #[test]
    fn parse_options_rejects_unknown_after_closing_brace_values() {
        let error = parse_options("format:\n  after-closing-brace: next-line\n")
            .expect_err("unknown placement should fail");

        assert!(error.to_string().contains("unknown variant `next-line`"), "{error}");
    }

    #[test]
    fn parse_options_applies_line_ending_and_defaults_to_lf() {
        let default = parse_options("").expect("empty config should parse");
        let lf = parse_options("format:\n  line-ending: lf\n").expect("lf should parse");
        let crlf = parse_options("format:\n  line-ending: crlf\n").expect("crlf should parse");
        let cr = parse_options("format:\n  line-ending: cr\n").expect("cr should parse");

        assert_eq!(default.line_ending, robusta_formatter::LineEnding::Lf);
        assert_eq!(lf.line_ending, robusta_formatter::LineEnding::Lf);
        assert_eq!(crlf.line_ending, robusta_formatter::LineEnding::Crlf);
        assert_eq!(cr.line_ending, robusta_formatter::LineEnding::Cr);
    }

    #[test]
    fn parse_options_rejects_unknown_line_ending_values() {
        let error = parse_options("format:\n  line-ending: platform\n")
            .expect_err("unknown line ending should fail");

        assert!(error.to_string().contains("platform"), "{error}");
    }

    #[test]
    fn parse_options_rejects_unknown_root_keys() {
        for key in ["foramt", "fmt", "options", "overrides"] {
            let error = parse_options(&format!("{key}:\n  line-width: 120\n"))
                .expect_err("unknown root keys should fail");

            let expected =
                format!("unknown field `{key}`, expected one of format, ignore-patterns, lint");
            assert!(error.to_string().contains(&expected), "{error}");
        }
    }

    #[test]
    fn parse_config_builds_typed_linter_rules() {
        let config = parse_config(
            r"
lint:
  categories:
    correctness: allow
  rules:
    modifier-order: warn
    redundant-semicolons: deny
    uppercase-long-suffix: warn
",
        )
        .expect("config should parse");
        let lint = config.lint.build(Path::new(""));
        let configured = lint.rules();

        assert_eq!(configured.len(), 3);
        assert_eq!(configured[0].rule().id(), rules::REDUNDANT_SEMICOLONS_ID);
        assert_eq!(configured[0].severity(), Severity::Error);
        assert_eq!(configured[1].rule().id(), rules::UPPERCASE_LONG_SUFFIX_ID);
        assert_eq!(configured[1].severity(), Severity::Warning);
        assert_eq!(configured[2].rule().id(), rules::MODIFIER_ORDER_ID);
        assert_eq!(configured[2].severity(), Severity::Warning);

        let source = "class A { ;; static public long value = 10l; }";
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(source, &parsed.unit);
        let diagnostics = Linter::new(configured.iter().cloned()).lint(&semantic);

        assert_eq!(diagnostics.len(), 4);
        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.diagnostic().severity == Severity::Error)
                .count(),
            2
        );
        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.diagnostic().severity == Severity::Warning)
                .count(),
            2
        );
    }

    #[test]
    fn unused_suppression_config_accepts_only_canonical_severities() {
        let omitted = parse_config("").expect("empty config should parse");
        assert_eq!(omitted.lint.options().report_unused_suppressions, None);
        for (value, expected) in
            [("allow", None), ("warn", Some(Severity::Warning)), ("deny", Some(Severity::Error))]
        {
            let config = parse_config(&format!(
                "lint:\n  options:\n    report-unused-suppressions: {value}\n"
            ))
            .expect("severity should parse");
            assert_eq!(config.lint.options().report_unused_suppressions, expected);
            let yaml = to_yaml(&config).expect("serialize config");
            assert!(yaml.contains(&format!("report-unused-suppressions: {value}\n")));
            let reparsed = parse_config(&yaml).expect("round trip");
            assert_eq!(reparsed.lint.options(), config.lint.options());
        }
        for value in ["true", "false", "off", "error", "0", "1", "2", "info", "[]", "{}"] {
            let config = format!("lint:\n  options:\n    report-unused-suppressions: {value}\n");
            assert!(parse_config(&config).is_err(), "{config}");
            let config = format!("lint:\n  rules:\n    unused-import: {value}\n");
            assert!(parse_config(&config).is_err(), "{config}");
        }
        assert!(parse_config("lint:\n  report-unused-suppressions: true\n").is_err());
    }

    #[test]
    fn warning_limits_accept_a_boolean_and_a_non_negative_integer() {
        let omitted = parse_config("").expect("empty config should parse");
        assert!(!omitted.lint.options().deny_warnings);
        assert_eq!(omitted.lint.options().max_warnings, None);
        for (option, check) in [
            ("deny-warnings: true", RunOptions { deny_warnings: true, ..RunOptions::default() }),
            ("deny-warnings: false", RunOptions::default()),
            ("max-warnings: 0", RunOptions { max_warnings: Some(0), ..RunOptions::default() }),
            ("max-warnings: 10", RunOptions { max_warnings: Some(10), ..RunOptions::default() }),
        ] {
            let config = parse_config(&format!("lint:\n  options:\n    {option}\n"))
                .expect("option should parse");
            assert_eq!(config.lint.options(), check, "{option}");
            let yaml = to_yaml(&config).expect("serialize config");
            let reparsed = parse_config(&yaml).expect("round trip");
            assert_eq!(reparsed.lint.options(), check, "{yaml}");
        }
        for value in ["1", "\"true\"", "yes", "warn", "[]", "{}"] {
            let config = format!("lint:\n  options:\n    deny-warnings: {value}\n");
            assert!(parse_config(&config).is_err(), "{config}");
        }
        for value in ["-1", "1.5", "\"10\"", "true", "[]", "{}"] {
            let config = format!("lint:\n  options:\n    max-warnings: {value}\n");
            assert!(parse_config(&config).is_err(), "{config}");
        }
    }

    #[test]
    fn a_loaded_config_names_its_first_run_wide_option() {
        let path = std::path::Path::new("robusta.yaml");
        for (source, expected) in [
            ("", None),
            ("lint:\n  options: {}\n", None),
            ("lint:\n  options:\n    deny-warnings: null\n", None),
            ("lint:\n  options:\n    max-warnings: ~\n", None),
            ("lint:\n  options:\n    report-unused-suppressions:\n", None),
            ("lint:\n  options:\n    deny-warnings: false\n", Some("lint.options.deny-warnings")),
            ("lint:\n  options:\n    max-warnings: 3\n", Some("lint.options.max-warnings")),
            (
                "lint:\n  options:\n    report-unused-suppressions: allow\n",
                Some("lint.options.report-unused-suppressions"),
            ),
            (
                "lint:\n  options:\n    report-unused-suppressions: warn\n    max-warnings: 3\n",
                Some("lint.options.max-warnings"),
            ),
            (
                "lint:\n  options:\n    deny-warnings: null\n    max-warnings: 0\n",
                Some("lint.options.max-warnings"),
            ),
        ] {
            let loaded = load_config_with_path(source, path).expect("config should load");
            assert_eq!(loaded.run_option, expected, "{source}");
        }
    }

    #[test]
    fn parse_config_defaults_to_correctness_warnings_and_allow_disables_them() {
        let correctness = rules::builtin_rules()
            .iter()
            .filter(|rule| rule.meta().category == RuleCategory::Correctness)
            .map(|rule| rule.id().as_str())
            .collect::<Vec<_>>();
        let omitted = parse_config("").expect("empty config should parse");
        let disabled = correctness.iter().fold("lint:\n  rules:\n".to_owned(), |config, rule| {
            config + "    " + rule + ": allow\n"
        });
        let disabled = parse_config(&disabled).expect("config should parse");

        assert_eq!(omitted.lint, ConfigStoreBuilder::default());
        let mut defaults = enabled(&omitted.lint);
        defaults.sort_unstable();
        let mut expected =
            correctness.iter().map(|rule| (*rule, Severity::Warning)).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(defaults, expected);
        assert_eq!(enabled(&disabled.lint).len(), 0);
    }

    #[test]
    fn parse_config_sets_each_category_severity() {
        let config = parse_config(
            "lint:\n  categories:\n    correctness: deny\n    pedantic: warn\n    style: warn\n",
        )
        .expect("config should parse");
        let lint = config.lint.build(Path::new(""));
        let mut categories = Vec::new();
        for configured in lint.rules() {
            let category = configured.rule().meta().category;
            let expected = match category {
                RuleCategory::Correctness => Severity::Error,
                RuleCategory::Pedantic | RuleCategory::Style => Severity::Warning,
                RuleCategory::Suspicious => panic!("{} should not run", configured.rule().id()),
            };
            assert_eq!(configured.severity(), expected, "{}", configured.rule().id());
            categories.push(category);
        }

        assert!(categories.contains(&RuleCategory::Correctness));
        assert!(categories.contains(&RuleCategory::Pedantic));
        assert!(categories.contains(&RuleCategory::Style));
    }

    #[test]
    fn overrides_round_trip_through_config_show() {
        let config = parse_config(
            r#"
format:
  overrides:
    - files: ["**/src/test/**", "./Top.java"]
      options:
        javadoc:
          capitalize-descriptions: true
    - files: ["generated/**"]
      exclude-files: ["generated/Keep.java"]
      options:
        import-order:
          sections: [static]
lint:
  overrides:
    - files: ["./Top.java"]
      rules:
        unused-local-variable: [deny, { ignore-pattern: "^spare" }]
"#,
        )
        .expect("config should parse");
        let yaml = to_yaml(&config).expect("serialize config");

        assert!(yaml.contains("- ./Top.java\n"), "{yaml}");
        assert_eq!(parse_config(&yaml).expect("printed config should parse"), config);
    }

    #[test]
    fn parse_config_rejects_unknown_lint_rules() {
        let error = parse_config(
            r"
lint:
  rules:
    redundant-semicolon: warn
",
        )
        .expect_err("unknown rule should fail");

        assert!(error.to_string().contains("unknown rule \"redundant-semicolon\""));
    }

    const SPARE: &str = "local variable `spare` is never read";

    /// What `lint` reports in `A.java` for two unread locals, `unused` and `spare`.
    fn reported(lint: &ConfigStoreBuilder) -> Vec<String> {
        let source = "class A { void f() { String unused = g(); String spare = g(); } }";
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, source);
        let semantic = SemanticBuilder::new().build(source, &parsed.unit);
        lint.clone()
            .build(Path::new(""))
            .apply_overrides(Path::new("A.java"))
            .lint(&semantic)
            .iter()
            .map(|diagnostic| diagnostic.diagnostic().message.clone())
            .collect()
    }

    fn unused_local_variable(value: &str) -> Config {
        parse_config(&format!("lint:\n  rules:\n    unused-local-variable: {value}\n"))
            .expect("rule value should parse")
    }

    /// The printed config, after checking that it parses back to `config`.
    fn assert_round_trips(config: &Config) -> String {
        let yaml = to_yaml(config).expect("resolved config should serialize");
        assert_eq!(&parse_config(&yaml).expect("printed config should parse"), config);
        yaml
    }

    #[test]
    fn parse_config_sets_rule_severities_and_settings() {
        let config = parse_config(
            r#"
lint:
  rules:
    unused-import: deny
    modifier-order: warn
    unused-local-variable: [warn, { ignore-pattern: "^unused" }]
"#,
        )
        .expect("config should parse");

        assert_eq!(
            enabled(&config.lint),
            [
                ("redundant-semicolons", Severity::Warning),
                ("modifier-order", Severity::Warning),
                ("unused-import", Severity::Error),
                ("unused-local-variable", Severity::Warning),
                ("broken-null-check", Severity::Warning),
                ("duplicate-import", Severity::Warning),
            ]
        );
        assert_eq!(reported(&config.lint), [SPARE]);
    }

    #[test]
    fn rule_settings_configure_the_rule_through_filters_and_printing() {
        let config = unused_local_variable("[warn, { ignore-pattern: \"^unused\" }]");

        assert_eq!(reported(&config.lint), [SPARE]);
        let denied = config.lint.clone().with_filters(&[deny("unused-local-variable")]);
        assert_eq!(reported(&denied), [SPARE]);
        assert!(enabled(&denied).contains(&("unused-local-variable", Severity::Error)));
        let yaml = assert_round_trips(&config);
        assert!(
            yaml.contains("unused-local-variable:\n    - warn\n    - ignore-pattern: ^unused\n")
        );
        assert_eq!(unused_local_variable("[warn, {}]").lint, unused_local_variable("warn").lint);
    }

    #[test]
    fn an_allow_rule_keeps_its_settings_for_a_later_warn_or_deny() {
        let config = unused_local_variable("[allow, { ignore-pattern: \"^unused\" }]");
        let overridden = parse_config(
            r#"
lint:
  rules:
    unused-local-variable: [allow, { ignore-pattern: "^unused" }]
  overrides:
    - files: [A.java]
      rules:
        unused-import: deny
"#,
        )
        .expect("config should parse");

        assert_eq!(reported(&config.lint), Vec::<String>::new());
        assert_eq!(reported(&overridden.lint), Vec::<String>::new());
        for filter in [warn("unused-local-variable"), deny("unused-local-variable")] {
            assert_eq!(reported(&config.lint.clone().with_filters(&[filter])), [SPARE]);
        }
        let yaml = assert_round_trips(&config);
        assert!(
            yaml.contains("unused-local-variable:\n    - allow\n    - ignore-pattern: ^unused\n"),
            "{yaml}"
        );
    }

    #[test]
    fn an_allow_filter_drops_the_rule_settings() {
        let config = unused_local_variable("[warn, { ignore-pattern: \"^unused\" }]");
        let filters = [allow("unused-local-variable"), deny("unused-local-variable")];

        assert_eq!(
            reported(&config.lint.with_filters(&filters)),
            ["local variable `unused` is never read", SPARE]
        );
    }

    #[test]
    fn rule_settings_reject_unknown_settings_and_invalid_values() {
        for (rule, message) in [
            (
                "unused-local-variable: [warn, { ignore_pattern: x }]",
                "rule \"unused-local-variable\": unknown field `ignore_pattern`",
            ),
            ("unused-local-variable: [warn, { ignore-pattern: \"(\" }]", "regex parse error"),
            (
                "unused-import: [warn, { ignore-pattern: x }]",
                "rule \"unused-import\" has no settings",
            ),
            ("unused-import: [warn, {}]", "rule \"unused-import\" has no settings"),
            ("unused-local-variable: [warn]", "invalid length 1"),
            ("unused-local-variable: [warn, {}, {}]", "invalid length 3"),
        ] {
            let source = format!("lint:\n  rules:\n    {rule}\n");
            let error = load_config_with_path(&source, std::path::Path::new("robusta.yaml"))
                .expect_err("invalid rule value should fail");
            let error = error.to_string();
            assert!(error.starts_with("robusta.yaml: "), "{error}");
            assert!(error.contains(message), "{error}");
        }
    }

    #[test]
    fn parse_config_rejects_unknown_lint_keys_and_severities() {
        let unknown_key = parse_config(
            r"
lint:
  rule:
    redundant-semicolons: warn
",
        )
        .expect_err("unknown lint key should fail");
        let unknown_severity = parse_config(
            r"
lint:
  rules:
    redundant-semicolons: warning
",
        )
        .expect_err("unknown lint severity should fail");

        assert!(unknown_key.to_string().contains("unknown field `rule`"));
        assert!(unknown_severity.to_string().contains("warning"));
    }

    #[test]
    fn parse_options_rejects_unknown_format_keys() {
        let error = parse_options(
            r"
format:
  line_widht: 120
",
        )
        .expect_err("unknown format keys should fail");

        assert!(error.to_string().contains("line_widht"));
    }

    #[test]
    fn parse_options_rejects_zero_line_width() {
        let error = parse_options(
            r"
format:
  line-width: 0
",
        )
        .expect_err("zero line width should fail");

        let error = error.to_string();
        assert!(error.contains("format.line-width"));
        assert!(error.contains("between 1 and 320; got 0"));
    }

    #[test]
    fn parse_options_accepts_zero_indent_width() {
        let options = parse_options(
            r"
format:
  indent-width: 0
",
        )
        .expect("zero indent width should parse");

        assert_eq!(options.indent_width.value(), 0);
    }

    #[test]
    fn parse_options_accepts_width_boundaries_and_rejects_the_next_integer() {
        let options = parse_options(
            r"
format:
  line-width: 320
  indent-width: 24
",
        )
        .expect("upper width boundaries should parse");

        assert_eq!(options.line_width.value(), 320);
        assert_eq!(options.indent_width.value(), 24);

        for (key, value, range) in
            [("line-width", 321, "1 and 320"), ("indent-width", 25, "0 and 24")]
        {
            let source = format!("format:\n  {key}: {value}\n");
            let error = parse_options(&source)
                .expect_err("the first value beyond each width range should fail")
                .to_string();
            assert!(error.contains(&format!("format.{key}")), "{error}");
            assert!(error.contains(&format!("between {range}; got {value}")), "{error}");
        }
    }

    #[test]
    fn parse_options_reads_import_order_groups_and_appends_catch_all() {
        let options = parse_options(
            r#"
format:
  import-order:
    groups:
      - ["java", "javax"]
      - "org"
"#,
        )
        .expect("config should parse");

        assert_eq!(
            options.import_order,
            ImportOrder {
                groups: vec![
                    ImportGroup {
                        matchers: vec![
                            ImportMatcher::Prefix("java".to_string()),
                            ImportMatcher::Prefix("javax".to_string()),
                        ],
                    },
                    ImportGroup { matchers: vec![ImportMatcher::Prefix("org".to_string())] },
                    ImportGroup { matchers: vec![ImportMatcher::All] },
                ],
                ..ImportOrder::default()
            }
        );
    }

    #[test]
    fn parse_options_reads_import_groups_around_the_catch_all_with_sections() {
        use robusta_formatter::ImportSection::{Module, Static, Type};

        let options = parse_options(
            r#"
format:
  import-order:
    groups: [["java", "javax"], "*", ["com.example.platform", "com.example.billing"]]
    sections: [static, type, module]
"#,
        )
        .expect("config should parse");
        let prefixes = |names: &[&str]| ImportGroup {
            matchers: names.iter().map(|name| ImportMatcher::Prefix((*name).to_string())).collect(),
        };

        assert_eq!(
            options.import_order,
            ImportOrder {
                groups: vec![
                    prefixes(&["java", "javax"]),
                    ImportGroup { matchers: vec![ImportMatcher::All] },
                    prefixes(&["com.example.platform", "com.example.billing"]),
                ],
                sections: vec![Static, Type, Module],
            }
        );
    }

    #[test]
    fn parse_options_reads_complete_import_section_order() {
        let options = parse_options(
            r"
format:
  import-order:
    sections: [static, module, type]
",
        )
        .expect("complete section order should parse");

        assert_eq!(
            options.import_order.sections,
            [
                robusta_formatter::ImportSection::Static,
                robusta_formatter::ImportSection::Module,
                robusta_formatter::ImportSection::Type,
            ]
        );
    }

    #[test]
    fn parse_options_appends_unlisted_import_sections_in_default_order() {
        use robusta_formatter::ImportSection::{Module, Static, Type};

        for (sections, expected) in [
            ("[static]", [Static, Module, Type]),
            ("[static, module]", [Static, Module, Type]),
            ("[]", [Module, Type, Static]),
            ("[type]", [Type, Module, Static]),
            ("[module]", [Module, Type, Static]),
        ] {
            let source = format!("format:\n  import-order:\n    sections: {sections}\n");
            let options = parse_options(&source).expect("partial section order should parse");

            assert_eq!(options.import_order.sections, expected, "sections: {sections}");
        }
    }

    #[test]
    fn parse_options_rejects_duplicate_and_unknown_import_sections() {
        let duplicate = parse_options(
            r"
format:
  import-order:
    sections: [static, static]
",
        )
        .expect_err("duplicate import section should fail");
        let unknown = parse_options(
            r"
format:
  import-order:
    sections: [unknown]
",
        )
        .expect_err("unknown import section should fail");

        assert!(duplicate.to_string().contains("static is duplicated"));
        assert!(unknown.to_string().contains("unknown variant `unknown`"));
    }

    #[test]
    fn parse_options_rejects_import_section_matchers() {
        for matcher in ["<module>", "<static>"] {
            let source = format!("format:\n  import-order:\n    groups:\n      - {matcher:?}\n");
            let error = parse_options(&source).expect_err("section matchers should fail");

            assert!(error.to_string().contains("is not a dotted package prefix or `*`"));
        }
    }

    #[test]
    fn parse_options_rejects_duplicate_import_matchers() {
        let error = parse_options(
            r#"
format:
  import-order:
    groups:
      - "java"
      - ["javax", "java"]
"#,
        )
        .expect_err("duplicate import matchers should fail");

        assert!(error.to_string().contains("import group \"java\" is duplicated"));
    }

    #[test]
    fn parse_options_accepts_overlapping_import_groups_and_catch_all_anywhere() {
        for (groups, expected_groups) in [
            (r#"["*", "com.example"]"#, 2),
            (r#"["com", "com.example"]"#, 3),
            (r#"["com.example", "*", "com"]"#, 3),
            (r#"[["*", "com"], "com.example"]"#, 2),
        ] {
            let source = format!("format:\n  import-order:\n    groups: {groups}\n");
            let options = parse_options(&source).expect("overlapping import groups should parse");
            assert_eq!(options.import_order.groups.len(), expected_groups, "{groups}");
            assert_eq!(
                options
                    .import_order
                    .groups
                    .iter()
                    .flat_map(|group| &group.matchers)
                    .filter(|matcher| **matcher == ImportMatcher::All)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn parse_options_accepts_empty_import_groups_as_one_catch_all() {
        let options = parse_options("format:\n  import-order:\n    groups: []\n")
            .expect("empty import groups should parse");

        assert_eq!(
            options.import_order,
            ImportOrder {
                groups: vec![ImportGroup { matchers: vec![ImportMatcher::All] }],
                ..ImportOrder::default()
            }
        );
    }

    #[test]
    fn parse_options_rejects_empty_nested_import_groups() {
        let error = parse_options("format:\n  import-order:\n    groups: [[]]\n")
            .expect_err("an empty nested import group should fail");

        assert!(error.to_string().contains("import group must not be empty"));
    }

    #[test]
    fn parse_options_accepts_prefixes_that_share_only_textual_prefixes() {
        let options = parse_options(
            r#"
format:
  import-order:
    groups:
      - "java"
      - "javax"
"#,
        )
        .expect("sibling dotted prefixes should remain reachable");

        assert_eq!(
            options.import_order.groups[..2],
            [
                ImportGroup { matchers: vec![ImportMatcher::Prefix("java".to_string())] },
                ImportGroup { matchers: vec![ImportMatcher::Prefix("javax".to_string())] },
            ]
        );
    }

    #[test]
    fn parse_options_rejects_unsupported_import_matchers() {
        let error = parse_options(
            r#"
format:
  import-order:
    groups:
      - "java.**"
"#,
        )
        .expect_err("unsupported import matchers should fail");

        assert!(error.to_string().contains("unsupported glob/path syntax"));
    }

    #[test]
    fn parse_config_reads_ignore_patterns() {
        let config = parse_config(
            r#"
ignore-patterns:
  - "generated/"
  - "!generated/Keep.java"
format:
  indent-width: 2
  ignore-patterns: ["vendored/"]
lint:
  ignore-patterns: ["legacy/"]
"#,
        )
        .expect("config should parse");

        assert_eq!(config.options.indent_width.value(), 2);
        assert_eq!(
            config.ignore_patterns,
            IgnorePatterns::new(["generated/", "!generated/Keep.java"])
        );
        assert_eq!(config.format_ignore_patterns, IgnorePatterns::new(["vendored/"]));
        assert_eq!(config.lint_ignore_patterns, IgnorePatterns::new(["legacy/"]));
    }

    #[test]
    fn parse_config_defaults_to_no_ignore_patterns() {
        let config = parse_config("format:\n  indent-width: 2\n  ignore-patterns:\nlint: {}\n")
            .expect("config should parse");

        assert_eq!(config.ignore_patterns, IgnorePatterns::default());
        assert_eq!(config.format_ignore_patterns, IgnorePatterns::default());
        assert_eq!(config.lint_ignore_patterns, IgnorePatterns::default());
    }

    #[test]
    fn parse_config_rejects_the_removed_files_section() {
        let error = parse_config("files:\n  exclude:\n    - \"build/**\"\n")
            .expect_err("`files` is not a config key");

        assert!(error.to_string().contains("unknown field `files`"), "{error}");
    }

    #[test]
    fn parse_config_rejects_invalid_ignore_patterns() {
        for (pattern, reason) in
            [("../x", "`..` is not supported"), ("*.[java", "must end with `]`")]
        {
            for (section, indent, line) in
                [("", "", 1), ("format:\n", "  ", 2), ("lint:\n", "  ", 2)]
            {
                let error =
                    parse_config(&format!("{section}{indent}ignore-patterns: [{pattern:?}]\n"))
                        .expect_err("invalid pattern should fail the config");

                let message = error.to_string();
                assert!(message.contains(&format!("{pattern:?}")), "{message}");
                assert!(message.contains(reason), "{message}");
                assert!(message.contains(&format!("line {line} column")), "{message}");
            }
        }
    }

    #[test]
    fn discovered_config_loads_its_options_and_ignore_patterns() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let input_dir = temp.path().join("src");
        std::fs::create_dir_all(&input_dir).expect("failed to create input dir");
        let config_path = temp.path().join("robusta.yaml");
        std::fs::write(
            &config_path,
            r#"
format:
  line-width: 88
  indent-width: 2
ignore-patterns: ["generated/"]
"#,
        )
        .expect("failed to write config");

        let discovered = discover_config_path(&input_dir.join("Example.java"))
            .expect("config should be discovered");
        let loaded = load_path(&discovered).expect("loading should not fail");

        assert_eq!(loaded.path, config_path);
        assert_eq!(loaded.config.options.line_width.value(), 88);
        assert_eq!(loaded.config.options.indent_width.value(), 2);
        assert_eq!(loaded.config.ignore_patterns, IgnorePatterns::new(["generated/"]));
    }
}
