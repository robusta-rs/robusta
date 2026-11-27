use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use editorconfig_parser::{
    EditorConfig, EditorConfigProperties, EditorConfigProperty, EndOfLine, IndentStyle,
    MaxLineLength,
};

use crate::core::format_config::{FormatConfig, IndentStyleConfig, LineEndingConfig};
use crate::core::options::{indent_width, line_width};

pub const FILE_NAME: &str = ".editorconfig";

#[derive(Debug)]
pub enum ReadError {
    Read { path: PathBuf, source: std::io::Error },
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(f, "{}: failed to read .editorconfig: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
        }
    }
}

/// One parsed `.editorconfig`.
#[derive(Debug)]
pub struct ConfigFile {
    path: PathBuf,
    config: EditorConfig,
}

impl ConfigFile {
    pub fn load(path: &Path) -> Result<Self, ReadError> {
        let source = std::fs::read_to_string(path)
            .map_err(|source| ReadError::Read { path: path.to_path_buf(), source })?;
        Ok(Self::from_source(path.to_path_buf(), &source))
    }

    fn from_source(path: PathBuf, source: &str) -> Self {
        let normalized = normalize(source);
        let cwd = path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
        Self { path, config: EditorConfig::parse(&normalized).with_cwd(&cwd) }
    }

    /// Whether the search for farther files stops at this one.
    pub fn is_root(&self) -> bool {
        self.config.root()
    }

    /// Apply this file's sections matching `path` over `properties`, the
    /// result of the farther files. `EditorConfig::resolve` starts from empty
    /// properties, so its `unset` could not remove a farther file's value.
    fn resolve_into(&self, path: &Path, properties: &mut EditorConfigProperties) {
        let relative =
            self.config.cwd().and_then(|cwd| path.strip_prefix(cwd).ok()).unwrap_or(path);
        for section in self.config.sections() {
            if section.matcher.as_ref().is_some_and(|matcher| matcher.is_match(relative)) {
                override_supported_properties(properties, &section.properties);
            }
        }
    }
}

/// The `.editorconfig` files that apply to a directory: the nearest one and
/// the files above it, up to the first with `root = true`.
#[derive(Debug)]
pub struct Resolver {
    /// Farthest first, so a closer file's sections apply later and win.
    files: Vec<Rc<ConfigFile>>,
    /// The `[*]` sections of every file, which apply to every path.
    root_properties: EditorConfigProperties,
}

impl Resolver {
    /// Discover and parse the `.editorconfig` files for `input`.
    pub fn for_input(input: &Path) -> Result<Option<Self>, ReadError> {
        let Some(path) = robusta_config::discover_ancestor_file(input, FILE_NAME) else {
            return Ok(None);
        };
        let file = ConfigFile::load(&path)?;
        let parent = match path.parent().and_then(Path::parent) {
            Some(directory) if !file.is_root() => Self::for_input(directory)?,
            _ => None,
        };
        Ok(Some(Self::new(file, parent.as_ref())))
    }

    /// `file` with the resolver for the files above it, if its search continues.
    pub fn new(file: ConfigFile, parent: Option<&Self>) -> Self {
        let mut root_properties = parent.map_or_default(|parent| parent.root_properties.clone());
        for section in file.config.sections().iter().filter(|section| section.name == "*") {
            override_supported_properties(&mut root_properties, &section.properties);
        }
        let mut files = parent.map_or_default(|parent| parent.files.clone());
        files.push(Rc::new(file));
        Self { files, root_properties }
    }

    /// The nearest file, which determines the rest.
    pub fn path(&self) -> &Path {
        &self.files.last().expect("a resolver has a file").path
    }

    pub fn root_properties(&self) -> &EditorConfigProperties {
        &self.root_properties
    }

    fn resolve(&self, path: &Path) -> EditorConfigProperties {
        let mut properties = EditorConfigProperties::default();
        for file in &self.files {
            file.resolve_into(path, &mut properties);
        }
        properties
    }

    /// Resolve `input`, returning the properties only when they change a
    /// supported property of the `[*]` sections.
    pub fn resolve_overrides(&self, input: &Path) -> Option<EditorConfigProperties> {
        let resolved = self.resolve(input);
        (resolved != self.root_properties).then_some(resolved)
    }

    /// The file that supplies `property` for `path`: the nearest one whose own
    /// matching sections leave it set.
    pub fn origin(&self, path: &Path, property: WidthProperty) -> &Path {
        self.files
            .iter()
            .rev()
            .find(|file| {
                let mut properties = EditorConfigProperties::default();
                file.resolve_into(path, &mut properties);
                property.is_set(&properties)
            })
            .map(|file| file.path.as_path())
            .expect("a width that fails its check was set by a file")
    }
}

/// Copy the properties Robusta maps, so a resolution holds nothing else.
fn override_supported_properties(
    properties: &mut EditorConfigProperties,
    section: &EditorConfigProperties,
) {
    override_property(&mut properties.max_line_length, &section.max_line_length);
    override_property(&mut properties.end_of_line, &section.end_of_line);
    override_property(&mut properties.indent_style, &section.indent_style);
    override_property(&mut properties.indent_size, &section.indent_size);
    override_property(&mut properties.tab_width, &section.tab_width);
    override_property(&mut properties.insert_final_newline, &section.insert_final_newline);
}

/// A later pair takes precedence, and `unset` removes an earlier value.
fn override_property<T: Copy>(
    property: &mut EditorConfigProperty<T>,
    later: &EditorConfigProperty<T>,
) {
    match later {
        EditorConfigProperty::Value(value) => *property = EditorConfigProperty::Value(*value),
        EditorConfigProperty::Unset => *property = EditorConfigProperty::None,
        EditorConfigProperty::None => {}
    }
}

/// A width property whose value is checked against the option it maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthProperty {
    MaxLineLength,
    IndentSize,
    TabWidth,
}

impl WidthProperty {
    fn name(self) -> &'static str {
        match self {
            Self::MaxLineLength => "max_line_length",
            Self::IndentSize => "indent_size",
            Self::TabWidth => "tab_width",
        }
    }

    fn is_set(self, properties: &EditorConfigProperties) -> bool {
        match self {
            Self::MaxLineLength => property_value(&properties.max_line_length).is_some(),
            Self::IndentSize => property_value(&properties.indent_size).is_some(),
            Self::TabWidth => property_value(&properties.tab_width).is_some(),
        }
    }
}

/// A width outside the range of the option it maps to.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidWidth {
    pub property: WidthProperty,
    message: String,
}

impl fmt::Display for InvalidWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}: {}", self.property.name(), self.message)
    }
}

/// Fill the fields `robusta.yaml` leaves unset from `.editorconfig` properties.
///
/// # Errors
///
/// Returns an error if a width it uses is outside the range of the option it maps to.
pub fn apply_editorconfig(
    config: &mut FormatConfig,
    properties: &EditorConfigProperties,
) -> Result<(), InvalidWidth> {
    if config.line_width.is_none()
        && let EditorConfigProperty::Value(MaxLineLength::Number(width)) =
            properties.max_line_length
    {
        line_width(width)
            .map_err(|message| InvalidWidth { property: WidthProperty::MaxLineLength, message })?;
        config.line_width = Some(width);
    }

    if config.indent_style.is_none()
        && let EditorConfigProperty::Value(style) = properties.indent_style
    {
        config.indent_style = Some(match style {
            IndentStyle::Space => IndentStyleConfig::Space,
            IndentStyle::Tab => IndentStyleConfig::Tab,
        });
    }

    if config.indent_width.is_none() {
        let tab_width = (WidthProperty::TabWidth, &properties.tab_width);
        let indent_size = (WidthProperty::IndentSize, &properties.indent_size);
        let indent_properties = match config.indent_style {
            Some(IndentStyleConfig::Tab) => [tab_width, indent_size],
            _ => [indent_size, tab_width],
        };
        if let Some((property, width)) = indent_properties
            .into_iter()
            .find_map(|(property, value)| property_value(value).map(|width| (property, width)))
        {
            indent_width(width).map_err(|message| InvalidWidth { property, message })?;
            config.indent_width = Some(width);
        }
    }

    if config.line_ending.is_none()
        && let EditorConfigProperty::Value(line_ending) = properties.end_of_line
    {
        config.line_ending = Some(match line_ending {
            EndOfLine::Lf => LineEndingConfig::Lf,
            EndOfLine::Cr => LineEndingConfig::Cr,
            EndOfLine::Crlf => LineEndingConfig::Crlf,
        });
    }

    if config.insert_final_newline.is_none()
        && let EditorConfigProperty::Value(insert) = properties.insert_final_newline
    {
        config.insert_final_newline = Some(insert);
    }
    Ok(())
}

fn property_value<T: Copy>(property: &EditorConfigProperty<T>) -> Option<T> {
    match property {
        EditorConfigProperty::Value(value) => Some(*value),
        EditorConfigProperty::None | EditorConfigProperty::Unset => None,
    }
}

/// Normalize case-insensitive keys and let `tab_width` supply `indent_size = tab`.
fn normalize(source: &str) -> String {
    let mut normalized = String::with_capacity(source.len());
    for line in source.lines() {
        if let Some((key, value)) = line.trim().split_once('=')
            && let Some(key) = canonical_property(key.trim())
        {
            let value = value.trim();
            let value = if key == "indent_size" && value.eq_ignore_ascii_case("tab") {
                "unset"
            } else {
                value
            };
            normalized.push_str(key);
            normalized.push_str(" = ");
            normalized.push_str(value);
        } else {
            normalized.push_str(line);
        }
        normalized.push('\n');
    }
    normalized
}

fn canonical_property(key: &str) -> Option<&'static str> {
    [
        "indent_style",
        "indent_size",
        "tab_width",
        "end_of_line",
        "charset",
        "trim_trailing_whitespace",
        "insert_final_newline",
        "max_line_length",
        "root",
    ]
    .into_iter()
    .find(|candidate| key.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use robusta_formatter::{IndentStyle, LineEnding, Options};

    use super::*;
    use crate::core::options::{to_robusta_formatter, validate};

    /// A resolver for one `/repo/.editorconfig` holding `source`.
    fn single(source: &str) -> Resolver {
        Resolver::new(ConfigFile::from_source(PathBuf::from("/repo/.editorconfig"), source), None)
    }

    /// The options `input` resolves to when `robusta.yaml` sets `config`.
    fn resolve(resolver: &Resolver, input: &Path, mut config: FormatConfig) -> Options {
        apply_editorconfig(&mut config, &resolver.resolve(input)).expect("widths are in range");
        to_robusta_formatter(&config, validate(&config, None).expect("filled config is valid"))
    }

    /// What `apply_editorconfig` returns for a lone `[*.java]` section.
    fn rejection(section: &str, mut config: FormatConfig) -> Result<(), String> {
        let resolver = single(&format!("[*.java]\n{section}"));
        apply_editorconfig(&mut config, &resolver.resolve(Path::new("/repo/Example.java")))
            .map_err(|err| err.to_string())
    }

    #[test]
    fn applies_matching_properties_and_keeps_yaml_fields() {
        let resolver = single(
            r"
[*]
max_line_length = 88
indent_style = tab
indent_size = 2
tab_width = 8
end_of_line = crlf
insert_final_newline = false

[generated/*.java]
indent_style = space
",
        );
        let yaml = FormatConfig { line_width: Some(100), ..FormatConfig::default() };

        let options = resolve(&resolver, Path::new("/repo/src/Example.java"), yaml);

        assert_eq!(options.line_width.value(), 100);
        assert_eq!(options.indent_style, IndentStyle::Tab);
        assert_eq!(options.indent_width.value(), 8);
        assert_eq!(options.line_ending, LineEnding::Crlf);
        assert!(!options.insert_final_newline);

        let generated =
            resolve(&resolver, Path::new("/repo/generated/File.java"), FormatConfig::default());
        assert_eq!(generated.indent_style, IndentStyle::Space);
        assert_eq!(generated.indent_width.value(), 2);
    }

    #[test]
    fn indent_size_tab_uses_tab_width_and_keys_are_case_insensitive() {
        let resolver = single("[*.java]\nINDENT_STYLE = TAB\nINDENT_SIZE = TAB\nTAB_WIDTH = 6\n");

        let options = resolve(&resolver, Path::new("/repo/Example.java"), FormatConfig::default());

        assert_eq!(options.indent_style, IndentStyle::Tab);
        assert_eq!(options.indent_width.value(), 6);
    }

    #[test]
    fn invalid_properties_leave_earlier_sections_in_effect() {
        let resolver = single(
            "[*]\nindent_size = 2\nindent_style = tab\nmax_line_length = 80\nend_of_line = crlf\ninsert_final_newline = false\n[*.java]\nindent_size = wide\nindent_style = narrow\nmax_line_length = many\nend_of_line = unknown\ninsert_final_newline = sometimes\n",
        );

        let options = resolve(&resolver, Path::new("/repo/Example.java"), FormatConfig::default());

        assert_eq!(options.indent_width.value(), 2);
        assert_eq!(options.indent_style, IndentStyle::Tab);
        assert_eq!(options.line_width.value(), 80);
        assert_eq!(options.line_ending, LineEnding::Crlf);
        assert!(!options.insert_final_newline);
        assert!(resolver.resolve_overrides(Path::new("/repo/Example.java")).is_none());
    }

    #[test]
    fn width_boundaries_apply() {
        let valid = single("[*.java]\nmax_line_length = 320\nindent_size = 24\n");

        let options = resolve(&valid, Path::new("/repo/Valid.java"), FormatConfig::default());
        assert_eq!(options.line_width.value(), 320);
        assert_eq!(options.indent_width.value(), 24);

        for (style, property, fallback) in
            [("space", "indent_size", "tab_width"), ("tab", "tab_width", "indent_size")]
        {
            let zero = single(&format!(
                "[*.java]\nindent_style = {style}\n{property} = 0\n{fallback} = 8\n"
            ));

            let options = resolve(&zero, Path::new("/repo/Zero.java"), FormatConfig::default());
            assert_eq!(options.indent_width.value(), 0, "{property}");
        }
    }

    #[test]
    fn rejects_used_widths_outside_their_option_range() {
        for (section, error) in [
            (
                "max_line_length = 321\n",
                "invalid max_line_length: must be between 1 and 320; got 321",
            ),
            ("max_line_length = 0\n", "invalid max_line_length: must be between 1 and 320; got 0"),
            (
                "indent_size = 25\ntab_width = 2\n",
                "invalid indent_size: must be between 0 and 24; got 25",
            ),
            (
                "indent_style = tab\ntab_width = 25\nindent_size = 2\n",
                "invalid tab_width: must be between 0 and 24; got 25",
            ),
            (
                "indent_size = tab\ntab_width = 25\n",
                "invalid tab_width: must be between 0 and 24; got 25",
            ),
            (
                "indent_size = wide\ntab_width = 25\n",
                "invalid tab_width: must be between 0 and 24; got 25",
            ),
        ] {
            assert_eq!(
                rejection(section, FormatConfig::default()),
                Err(error.to_owned()),
                "{section}"
            );
        }
    }

    #[test]
    fn widths_it_does_not_use_are_not_checked() {
        let yaml = FormatConfig {
            line_width: Some(100),
            indent_width: Some(4),
            ..FormatConfig::default()
        };
        assert_eq!(rejection("max_line_length = 1000\nindent_size = 25\n", yaml), Ok(()));

        for section in [
            "indent_size = 2\ntab_width = 25\n",
            "indent_style = tab\ntab_width = 2\nindent_size = 25\n",
            "max_line_length = off\nindent_size = unset\n",
        ] {
            assert_eq!(rejection(section, FormatConfig::default()), Ok(()), "{section}");
        }

        let resolver = single("[*.md]\nmax_line_length = 1000\n");
        let options = resolve(&resolver, Path::new("/repo/Example.java"), FormatConfig::default());
        assert_eq!(options.line_width.value(), 100);
    }

    #[test]
    fn resolved_line_widths_use_supported_values() {
        for (value, expected) in [("off", 100), ("18446744073709551616", 80), ("unset", 100)] {
            let resolver = single(&format!(
                "[*]\nmax_line_length = 80\n[*.java]\nmax_line_length = {value}\n"
            ));

            let options =
                resolve(&resolver, Path::new("/repo/Example.java"), FormatConfig::default());

            assert_eq!(options.line_width.value(), expected, "{value}");
        }
    }

    /// Resolvers for `/repo/.editorconfig` and `/repo/project/.editorconfig`.
    fn nested(outer: &str, inner: &str) -> Resolver {
        let outer = single(outer);
        let inner = ConfigFile::from_source(PathBuf::from("/repo/project/.editorconfig"), inner);
        Resolver::new(inner, Some(&outer))
    }

    #[test]
    fn reads_files_up_to_the_first_root() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let outer = temp.path().join("outer");
        let sub = outer.join("project/sub");
        std::fs::create_dir_all(&sub).expect("create nested directories");
        std::fs::write(temp.path().join(FILE_NAME), "[*]\ninsert_final_newline = false\n")
            .expect("write config above the root");
        std::fs::write(outer.join(FILE_NAME), "ROOT = True\n[*]\nindent_size = 8\n")
            .expect("write root config");
        std::fs::write(outer.join("project").join(FILE_NAME), "[*.java]\nmax_line_length = 20\n")
            .expect("write project config");
        std::fs::write(sub.join(FILE_NAME), "[*.java]\nindent_size = unset\n")
            .expect("write sub config");

        for (input, indent_width) in [
            (outer.join("project/A.java"), 8),
            (sub.join("B.java"), Options::default().indent_width.value()),
        ] {
            let resolver = Resolver::for_input(&input)
                .expect("discovery should not fail")
                .expect("an ancestor file exists");
            let options = resolve(&resolver, &input, FormatConfig::default());

            assert_eq!(options.indent_width.value(), indent_width, "{}", input.display());
            assert_eq!(options.line_width.value(), 20, "{}", input.display());
            assert!(options.insert_final_newline, "{}", input.display());
        }
    }

    #[test]
    fn closer_files_take_precedence_and_unset_farther_values() {
        let resolver = nested(
            "[*]\nindent_size = 2\nmax_line_length = 80\nend_of_line = crlf\n",
            "[*.java]\nindent_size = 4\nmax_line_length = unset\n",
        );

        let options =
            resolve(&resolver, Path::new("/repo/project/Example.java"), FormatConfig::default());

        assert_eq!(options.indent_width.value(), 4);
        assert_eq!(options.line_width.value(), Options::default().line_width.value());
        assert_eq!(options.line_ending, LineEnding::Crlf);
    }

    #[test]
    fn globs_are_relative_to_their_own_file() {
        let resolver = nested(
            "[project/generated/*.java]\nindent_size = 4\n",
            "[generated/*.java]\nmax_line_length = 60\n",
        );

        let generated = resolve(
            &resolver,
            Path::new("/repo/project/generated/Generated.java"),
            FormatConfig::default(),
        );
        assert_eq!(generated.indent_width.value(), 4);
        assert_eq!(generated.line_width.value(), 60);

        let ordinary =
            resolve(&resolver, Path::new("/repo/project/Ordinary.java"), FormatConfig::default());
        assert_eq!(ordinary.indent_width.value(), Options::default().indent_width.value());
        assert_eq!(ordinary.line_width.value(), Options::default().line_width.value());
    }

    #[test]
    fn star_sections_of_every_file_form_the_shared_properties() {
        let resolver = nested("[*]\nindent_size = 2\ntab_width = 8\n", "[*]\nindent_size = 4\n");
        let input = Path::new("/repo/project/Example.java");

        assert!(resolver.resolve_overrides(input).is_none());
        let options = resolve(&resolver, input, FormatConfig::default());
        assert_eq!(options.indent_width.value(), 4);
    }

    #[test]
    fn an_invalid_width_names_the_file_that_set_it() {
        let resolver = nested("[*.java]\nmax_line_length = 1000\n", "[*.java]\nindent_size = 4\n");
        let input = Path::new("/repo/project/Example.java");

        let err = apply_editorconfig(&mut FormatConfig::default(), &resolver.resolve(input))
            .expect_err("the width is out of range");

        assert_eq!(err.property, WidthProperty::MaxLineLength);
        assert_eq!(resolver.origin(input, err.property), Path::new("/repo/.editorconfig"));
        assert_eq!(
            resolver.origin(input, WidthProperty::IndentSize),
            Path::new("/repo/project/.editorconfig")
        );
    }

    #[test]
    fn detects_only_effective_supported_path_overrides() {
        let resolver = single(
            "[*]\nindent_size = 2\n[generated/*.java]\nindent_size = 8\n[*.md]\nindent_size = 4\n",
        );

        assert!(resolver.resolve_overrides(Path::new("/repo/src/Example.java")).is_none());
        assert!(resolver.resolve_overrides(Path::new("/repo/generated/Generated.java")).is_some());
        assert!(resolver.resolve_overrides(Path::new("/repo/README.md")).is_some());
    }

    #[test]
    fn unused_properties_do_not_force_owned_formatter_policy() {
        let resolver = single("[*]\nindent_size = 2\n[generated/*.java]\ncharset = latin1\n");
        let generated = Path::new("/repo/generated/Generated.java");

        assert!(resolver.resolve_overrides(generated).is_none());
        let options = resolve(&resolver, generated, FormatConfig::default());
        assert_eq!(options.indent_width.value(), 2);
    }
}
