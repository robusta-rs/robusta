use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use cow_utils::CowUtils;
use indexmap::{IndexMap, map::Entry};
use robusta_diagnostics::Output;
use robusta_formatter::Options;
use robusta_linter::{ConfigStoreBuilder, LintFilter, Linter, RunOptions};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

use crate::{
    command::{ConfigOptions, IgnoreOptions},
    config_loader,
    core::{
        config::{FormatOverrides, FormatPolicy, editorconfig},
        format_config::FormatConfig,
    },
};

/// A file selected for processing, with only the command features resolved
/// from its governing config.
pub struct SelectedFile {
    pub path: PathBuf,
    /// `path` relative to the working directory when it lies below it, and
    /// absolute otherwise, however the input spelled it.
    pub display_path: PathBuf,
    format_options: Option<Arc<Options>>,
    /// The linter for this file's resolved lint config. Files sharing one
    /// resolved config share one linter, so its kind-dispatch buckets are built
    /// once per config rather than once per file.
    linter: Option<Arc<Linter>>,
}

impl SelectedFile {
    /// The formatter options, when the command formats this file.
    pub fn format_options(&self) -> Option<&Options> {
        self.format_options.as_deref()
    }

    /// The linter, when the command lints this file.
    pub fn linter(&self) -> Option<&Linter> {
        self.linter.as_deref()
    }
}

/// The files a command processes and the main configuration's run-wide options.
pub struct Selection {
    pub files: Vec<SelectedFile>,
    pub run_options: RunOptions,
    pub scope: SelectionScope,
}

/// Input roots and resolved exclusions, retained for deleted baseline paths.
pub struct SelectionScope {
    roots: Vec<PathBuf>,
    ignore_matcher: robusta_config::IgnoreMatcher,
    resolver: ConfigResolver,
}

impl SelectionScope {
    /// Whether a deleted `path` is in the run's lint scope: under a root, not
    /// excluded, and given a linter by its configuration. Its name is not checked,
    /// so a baseline entry for a deleted file can be pruned whatever it was called.
    pub fn lints(&mut self, out: &mut Output, path: &Path) -> Result<bool, ()> {
        if !self.roots.iter().any(|root| {
            path.starts_with(root) && !self.ignore_matcher.is_ignored_by_walk(path, root)
        }) {
            return Ok(false);
        }
        self.resolver
            .select(out, path)
            .map(|policy| policy.is_some_and(|policy| policy.linter.is_some()))
    }
}

/// Build the presentation-only path with forward-slash separators.
///
/// `display_path` is distinct from `path`, the native path used for all real
/// filesystem I/O; nothing performs structural path operations on it — every
/// consumer renders it with `.display()` or `to_string_lossy()`. Normalizing
/// its separators to `/` once, at construction, makes every downstream emission
/// (`--list-different`, `would reformat`, diff headers, error lines, and the
/// buffered reporters' `FileReport.path`) byte-identical across platforms.
///
/// `\` separates components only on Windows. Elsewhere it is part of a file name.
pub fn display_path(path: &Path) -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(path.to_string_lossy().cow_replace('\\', "/").into_owned())
    } else {
        path.to_path_buf()
    }
}

/// Config loaded once per config path: the requested command policy plus the
/// compiled ignore pattern lists, rooted at the config's directory.
pub struct LoadedSelection {
    format: Option<FormatPolicy>,
    lint: Option<robusta_linter::Config>,
    run_options: RunOptions,
    run_option: Option<&'static str>,
    /// Compiled once per config, like the run's command-line patterns, and
    /// consulted per selected file rather than during the walk: this scope's
    /// own nested configs are only discovered as their files are reached.
    ignore_patterns: robusta_config::IgnoreMatcher,
    format_ignore_patterns: robusta_config::IgnoreMatcher,
    lint_ignore_patterns: robusta_config::IgnoreMatcher,
}

/// The tools one command runs, and whether selection resolves their policies.
///
/// `format` resolves only formatter policy, `lint` only a linter, and `check`
/// both. Keeping that boundary here prevents one product tier from retaining
/// another tier's per-file state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionFeatures {
    format: bool,
    lint: bool,
    policies: bool,
}

impl SelectionFeatures {
    pub const FORMAT: Self = Self { format: true, lint: false, policies: true };
    pub const LINT: Self = Self { format: false, lint: true, policies: true };
    pub const CHECK: Self = Self { format: true, lint: true, policies: true };
    /// The files `check` selects without either policy, for `config --files`.
    pub const LIST: Self = Self { format: true, lint: true, policies: false };

    const fn resolves_format(self) -> bool {
        self.format && self.policies
    }

    const fn resolves_lint(self) -> bool {
        self.lint && self.policies
    }
}

pub struct SelectedPolicy {
    format_options: Option<Arc<Options>>,
    linter: Option<Arc<Linter>>,
}

/// Resolve configuration once per command while preserving per-file nested
/// discovery as the default mode.
pub struct ConfigResolver {
    mode: ConfigResolution,
    /// The main configuration's run-wide options, which apply to every file.
    run_options: RunOptions,
    features: SelectionFeatures,
    /// EditorConfig files keyed by the nearest one, loaded once per path for
    /// commands that format.
    editorconfigs: Option<ConfigCache<editorconfig::Resolver>>,
    /// EditorConfig values already reported as invalid, so each is reported once.
    rejected: FxHashSet<String>,
    /// The default formatter policy, retained only for commands that format.
    default_format: Option<FormatPolicy>,
    /// The default (no-config) rule set, shared by every file that resolves
    /// to no governing config. Formatter-only commands never construct it.
    default_lint: Option<robusta_linter::Config>,
    /// The run's ordered CLI severity filters, retained because a nested
    /// config is discovered and loaded during the walk and must receive the
    /// same filters as the ones resolved up front.
    filters: Vec<LintFilter>,
}

/// Resolve the configuration governing one input without walking or analyzing
/// source files. An explicit config has precedence; otherwise discovery starts
/// at the input. `config` uses the current directory as its input.
pub fn config_for_input(
    out: &mut Output,
    options: &ConfigOptions,
    input: &Path,
    features: SelectionFeatures,
) -> Result<config_loader::Config, ()> {
    let input = absolute_path(input).map_err(|err| {
        out.stderr_line(&format!("{}: {err}", input.display()));
    })?;
    let config_path = single_config_path(out, options, &input)?;
    let (mut config, format) = if let Some(config_path) = config_path {
        let loaded = config_loader::load_path(&config_path).map_err(|err| {
            out.stderr_line(&err.to_string());
        })?;
        (loaded.config, loaded.format)
    } else {
        (config_loader::Config::default(), FormatConfig::default())
    };
    if !features.resolves_format() {
        return Ok(config);
    }
    let editorconfig_resolver = editorconfig::Resolver::for_input(&input).map_err(|err| {
        out.stderr_line(&err.to_string());
    })?;
    let mut policy = FormatPolicy::new(format, config.options, FormatOverrides::default());
    let options = policy
        .resolve(editorconfig_resolver.as_ref(), &input)
        .map_err(|message| out.stderr_line(&message))?;
    config.options = Arc::unwrap_or_clone(options);
    Ok(config)
}

enum ConfigResolution {
    PerFile(ConfigCache<LoadedSelection>),
    Single(Option<Box<LoadedSelection>>),
}

struct ConfigCache<T> {
    /// Parsed files and failed loads, so each failure is reported once.
    configs: FxHashMap<PathBuf, Result<T, ()>>,
    /// Confirmed nearest scopes and misses, shared by sibling files.
    scopes: FxHashMap<PathBuf, Option<PathBuf>>,
}

impl<T> Default for ConfigCache<T> {
    fn default() -> Self {
        Self { configs: FxHashMap::default(), scopes: FxHashMap::default() }
    }
}

impl<T> ConfigCache<T> {
    /// The config nearest `path`, loaded on first use. `load` receives the
    /// cache, so a config can build on the cached configs above it.
    fn resolve(
        &mut self,
        path: &Path,
        file_name: &str,
        load: impl FnOnce(&mut Self, &Path) -> Result<T, ()>,
    ) -> Result<Option<&mut T>, ()> {
        let Some(config_path) = discover_file_config_path(path, file_name, &mut self.scopes) else {
            return Ok(None);
        };
        if !self.configs.contains_key(&config_path) {
            let loaded = load(self, &config_path);
            self.configs.insert(config_path.clone(), loaded);
        }
        match self.configs.get_mut(&config_path).expect("the config was loaded above") {
            Ok(config) => Ok(Some(config)),
            Err(()) => Err(()),
        }
    }
}

impl ConfigCache<editorconfig::Resolver> {
    /// The `.editorconfig` files for `path`. A file's resolver builds on the
    /// cached resolver of the files above it, so each file is read once.
    fn resolve_editorconfig(
        &mut self,
        out: &mut Output,
        path: &Path,
    ) -> Result<Option<&mut editorconfig::Resolver>, ()> {
        self.resolve(path, editorconfig::FILE_NAME, |cache, path| {
            cache.load_editorconfig(out, path)
        })
    }

    fn load_editorconfig(
        &mut self,
        out: &mut Output,
        path: &Path,
    ) -> Result<editorconfig::Resolver, ()> {
        let file = editorconfig::ConfigFile::load(path)
            .map_err(|err| out.stderr_line(&err.to_string()))?;
        // Discovery starts in the parent of the directory given here.
        let parent = match path.parent() {
            Some(directory) if !file.is_root() => self.resolve_editorconfig(out, directory)?,
            _ => None,
        };
        Ok(editorconfig::Resolver::new(file, parent.as_deref()))
    }
}

impl ConfigResolver {
    /// Build the resolver for a command. `config_root` anchors the one
    /// `robusta.yaml` lookup with `--no-nested-config`. An explicit `--config`
    /// takes precedence. EditorConfig discovery stays per file.
    pub fn new(
        out: &mut Output,
        options: &ConfigOptions,
        config_root: &Path,
        features: SelectionFeatures,
        filters: &[LintFilter],
    ) -> Result<Self, ()> {
        let config_root = absolute_path(config_root).map_err(|err| {
            out.stderr_line(&format!("{}: {err}", config_root.display()));
        })?;
        let main_path = single_config_path(out, options, &config_root)?;
        let main = main_path
            .as_deref()
            .map(|path| load_selection(out, path, features, filters))
            .transpose()?;
        let run_options = main.as_ref().map(|main| main.run_options).unwrap_or_default();
        let mode = if options.config.is_some() || options.no_nested_config {
            ConfigResolution::Single(main.map(Box::new))
        } else {
            let mut cache = ConfigCache::default();
            if let Some(path) = main_path {
                cache.configs.insert(path, Ok(main.expect("main config was loaded")));
            }
            ConfigResolution::PerFile(cache)
        };
        Ok(Self {
            mode,
            run_options,
            features,
            editorconfigs: features.resolves_format().then(ConfigCache::default),
            rejected: FxHashSet::default(),
            default_format: features.resolves_format().then(|| {
                FormatPolicy::new(
                    FormatConfig::default(),
                    Options::default(),
                    FormatOverrides::default(),
                )
            }),
            default_lint: features
                .resolves_lint()
                .then(|| ConfigStoreBuilder::default().with_filters(filters).build(Path::new(""))),
            filters: filters.to_vec(),
        })
    }

    /// Resolve policy and file selection for one absolute input path.
    pub fn select(&mut self, out: &mut Output, path: &Path) -> Result<Option<SelectedPolicy>, ()> {
        let features = self.features;
        let filters = &self.filters;
        let loaded = match &mut self.mode {
            ConfigResolution::Single(loaded) => loaded.as_deref_mut(),
            ConfigResolution::PerFile(cache) => {
                cache.resolve(path, robusta_config::CONFIG_FILE_NAME, |_, path| {
                    let loaded = load_selection(out, path, features, filters)?;
                    if let Some(option) = loaded.run_option {
                        out.stderr_line(&format!(
                            "{}: {option} is only supported in the main configuration; move the option to the main configuration",
                            path.display()
                        ));
                        return Err(());
                    }
                    Ok(loaded)
                })?
            }
        };
        // A tool's list cannot re-include a path the shared list skips.
        let skips =
            |patterns: &robusta_config::IgnoreMatcher| patterns.is_ignored(path, false, true);
        let (formats, lints) = match &loaded {
            Some(loaded) if skips(&loaded.ignore_patterns) => return Ok(None),
            Some(loaded) => (
                features.format && !skips(&loaded.format_ignore_patterns),
                features.lint && !skips(&loaded.lint_ignore_patterns),
            ),
            None => (features.format, features.lint),
        };
        if !formats && !lints {
            return Ok(None);
        }
        let (format, lint) = match loaded {
            Some(loaded) => (loaded.format.as_mut(), loaded.lint.as_mut()),
            None => (self.default_format.as_mut(), self.default_lint.as_mut()),
        };
        let (format, lint) = (format.filter(|_| formats), lint.filter(|_| lints));
        let editorconfig = match &mut self.editorconfigs {
            Some(cache) if format.is_some() => cache.resolve_editorconfig(out, path)?,
            _ => None,
        };
        let format_options = format
            .map(|format| format.resolve(editorconfig.as_deref(), path))
            .transpose()
            .map_err(|message| {
                if self.rejected.insert(message.clone()) {
                    out.stderr_line(&message);
                }
            })?;
        Ok(Some(SelectedPolicy {
            format_options,
            linter: lint.map(|lint| lint.apply_overrides(path)),
        }))
    }
}

/// Resolve one selected file's nearest config through the walk-wide directory
/// cache. Every probed directory receives the final scope, including `None`,
/// so sibling files stop at the first previously resolved ancestor. Directories
/// below an outer cached scope are still probed before that cache hit, which is
/// what lets a closer nested config win.
fn discover_file_config_path(
    path: &Path,
    file_name: &str,
    scopes: &mut FxHashMap<PathBuf, Option<PathBuf>>,
) -> Option<PathBuf> {
    debug_assert!(path.is_absolute(), "selected paths are normalized before config discovery");
    let mut probed = Vec::new();
    let mut resolved = None;
    for directory in path.parent().into_iter().flat_map(Path::ancestors) {
        if let Some(cached) = scopes.get(directory) {
            resolved.clone_from(cached);
            break;
        }

        probed.push(directory.to_path_buf());
        let candidate = directory.join(file_name);
        if candidate.is_file() {
            resolved = Some(candidate);
            break;
        }
    }

    for directory in probed {
        scopes.insert(directory, resolved.clone());
    }
    resolved
}

/// Build the run's one ignore matcher from its ignore files and patterns.
/// Every candidate path reaches this matcher, so the patterns bind explicit
/// file arguments and traversed entries alike.
pub fn ignore_matcher(
    out: &mut Output,
    ignore_options: &IgnoreOptions,
) -> Result<robusta_config::IgnoreMatcher, ()> {
    let invocation_root = working_directory(out)?;
    robusta_config::IgnoreMatcher::new(
        &ignore_options.ignore_path,
        &ignore_options.ignore_pattern,
        &invocation_root,
    )
    .map_err(|err| {
        out.stderr_line(&err.to_string());
    })
}

pub fn selected_files(
    out: &mut Output,
    paths: &[PathBuf],
    config_options: &ConfigOptions,
    ignore_matcher: &robusta_config::IgnoreMatcher,
    no_error_on_unmatched_pattern: bool,
    features: SelectionFeatures,
    filters: &[LintFilter],
    threads: usize,
) -> Result<Selection, ()> {
    // An omitted positional path means the current working directory. Keep the
    // fallback at the selection boundary so it follows the exact same config
    // discovery and `ignore-patterns` path as an explicit `.`.
    let default_path = PathBuf::from(".");
    let paths = if paths.is_empty() { std::slice::from_ref(&default_path) } else { paths };
    let mut failed = false;
    let working_directory = working_directory(out)?;
    let mut resolver =
        ConfigResolver::new(out, config_options, &working_directory, features, filters)?;
    // Overlapping inputs (a repeated path, a file plus its parent directory,
    // nested roots) must select each file once. Candidate paths are absolute
    // and lexically normalized, so key identity collapses equivalent
    // spellings; the first occurrence keeps its position, even when its config
    // excludes it.
    let mut selected: IndexMap<PathBuf, Option<SelectedFile>, FxBuildHasher> = IndexMap::default();

    for path in paths {
        match collect_candidate_files(path, ignore_matcher, threads) {
            Ok(candidates) => {
                for candidate in candidates {
                    let Entry::Vacant(entry) = selected.entry(candidate.clone()) else {
                        continue;
                    };
                    let file = select_file(out, candidate, &working_directory, &mut resolver);
                    entry.insert(file.unwrap_or_else(|()| {
                        failed = true;
                        None
                    }));
                }
            }
            Err(err) => {
                out.stderr_line(&format!("{}: {err}", path.display()));
                failed = true;
            }
        }
    }

    let files: Vec<_> = selected.into_values().flatten().collect();
    if failed {
        Err(())
    } else if files.is_empty() && !no_error_on_unmatched_pattern {
        out.stderr_line(
            "Expected at least one target file. All matched files may have been excluded by ignore rules.",
        );
        Err(())
    } else {
        let roots = paths
            .iter()
            .map(|path| absolute_path(path))
            .collect::<io::Result<_>>()
            .map_err(|error| out.stderr_line(&error.to_string()))?;
        Ok(Selection {
            files,
            run_options: resolver.run_options,
            scope: SelectionScope { roots, ignore_matcher: ignore_matcher.clone(), resolver },
        })
    }
}

/// Select the virtual path of a document read from standard input. It is
/// resolved like a selected file, but never read from disk, so the result is
/// empty when its configuration excludes it.
pub fn stdin_file(
    out: &mut Output,
    path: &Path,
    config_options: &ConfigOptions,
    features: SelectionFeatures,
    filters: &[LintFilter],
) -> Result<Selection, ()> {
    let absolute = absolute_path(path).map_err(|err| {
        out.stderr_line(&format!("{}: {err}", path.display()));
    })?;
    let working_directory = working_directory(out)?;
    let mut resolver =
        ConfigResolver::new(out, config_options, &working_directory, features, filters)?;
    let files = select_file(out, absolute.clone(), &working_directory, &mut resolver)?
        .into_iter()
        .collect();
    Ok(Selection {
        files,
        run_options: resolver.run_options,
        scope: SelectionScope {
            roots: vec![absolute],
            ignore_matcher: robusta_config::IgnoreMatcher::default(),
            resolver,
        },
    })
}

fn collect_candidate_files(
    path: &Path,
    ignore_matcher: &robusta_config::IgnoreMatcher,
    threads: usize,
) -> io::Result<Vec<PathBuf>> {
    let absolute = absolute_path(path)?;
    let metadata = std::fs::metadata(&absolute)?;
    if ignore_matcher.is_ignored(&absolute, metadata.is_dir(), true) {
        return Ok(Vec::new());
    }
    if metadata.is_file() && robusta_config::is_java_path(&absolute) {
        return Ok(vec![absolute]);
    }
    if metadata.is_dir() {
        let mut files = robusta_config::collect_java_files(&absolute, ignore_matcher, threads)?;
        files.sort();
        return Ok(files);
    }
    Ok(Vec::new())
}

fn select_file(
    out: &mut Output,
    path: PathBuf,
    working_directory: &Path,
    resolver: &mut ConfigResolver,
) -> Result<Option<SelectedFile>, ()> {
    let Some(policy) = resolver.select(out, &path)? else {
        return Ok(None);
    };
    let display_path = display_path(path.strip_prefix(working_directory).unwrap_or(&path));
    Ok(Some(SelectedFile {
        path,
        display_path,
        format_options: policy.format_options,
        linter: policy.linter,
    }))
}

/// The one configuration used when discovery is not per file: an explicit
/// `--config`, otherwise the nearest `robusta.yaml` at or above `root`.
fn single_config_path(
    out: &mut Output,
    options: &ConfigOptions,
    root: &Path,
) -> Result<Option<PathBuf>, ()> {
    let Some(config_path) = &options.config else {
        return Ok(robusta_config::discover_config_path(root));
    };
    absolute_path(config_path).map(Some).map_err(|err| {
        out.stderr_line(&format!("{}: {err}", config_path.display()));
    })
}

fn working_directory(out: &mut Output) -> Result<PathBuf, ()> {
    std::env::current_dir().map_err(|err| {
        out.stderr_line(&format!("failed to read the working directory: {err}"));
    })
}

pub fn load_selection(
    out: &mut Output,
    config_path: &Path,
    features: SelectionFeatures,
    filters: &[LintFilter],
) -> Result<LoadedSelection, ()> {
    let loaded = match config_loader::load_path(config_path) {
        Ok(loaded) => loaded,
        Err(err) => {
            out.stderr_line(&err.to_string());
            return Err(());
        }
    };
    let root = loaded.path.parent().unwrap_or_else(|| Path::new(""));
    let compile = |out: &mut Output, patterns: &robusta_config::IgnorePatterns| {
        robusta_config::IgnoreMatcher::new(&[], patterns.as_slice(), root).map_err(|err| {
            out.stderr_line(&format!("{}: {err}", loaded.path.display()));
        })
    };
    let ignore_patterns = compile(out, &loaded.config.ignore_patterns)?;
    let format_ignore_patterns = compile(out, &loaded.config.format_ignore_patterns)?;
    let lint_ignore_patterns = compile(out, &loaded.config.lint_ignore_patterns)?;
    let run_options = loaded.config.lint.options();
    let lint =
        features.resolves_lint().then(|| loaded.config.lint.with_filters(filters).build(root));
    let format = features.resolves_format().then(|| {
        FormatPolicy::new(
            loaded.format,
            loaded.config.options,
            FormatOverrides::new(loaded.config.format_overrides, root),
        )
    });
    Ok(LoadedSelection {
        format,
        lint,
        run_options,
        run_option: loaded.run_option,
        ignore_patterns,
        format_ignore_patterns,
        lint_ignore_patterns,
    })
}

/// Make `path` absolute against the current directory and lexically resolve
/// `.` and `..` components, so equivalent spellings of one path compare
/// equal. Normalization never touches the filesystem: symlinked spellings of
/// a file intentionally remain distinct paths.
pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    let absolute =
        if path.is_absolute() { path.to_path_buf() } else { std::env::current_dir()?.join(path) };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            // At the root, `..` stays at the root; `pop` then returns false.
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component),
        }
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_features_materialize_only_requested_policy() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let source = temp.path().join("Example.java");
        std::fs::write(&source, "class Example {}\n").expect("write source");
        let config_options = ConfigOptions { config: None, no_nested_config: true };

        for (features, has_format_options, has_linter) in [
            (SelectionFeatures::FORMAT, true, false),
            (SelectionFeatures::LINT, false, true),
            (SelectionFeatures::CHECK, true, true),
            (SelectionFeatures::LIST, false, false),
        ] {
            let mut out = Output::stdio();
            let mut resolver =
                ConfigResolver::new(&mut out, &config_options, temp.path(), features, &[])
                    .expect("build config resolver");
            let selected = resolver
                .select(&mut out, &source)
                .expect("resolve source")
                .expect("source is selected");

            assert_eq!(selected.format_options.is_some(), has_format_options);
            assert_eq!(selected.linter.is_some(), has_linter);
        }
    }

    #[test]
    fn formatter_policy_is_shared_when_editorconfig_is_absent() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let first = temp.path().join("First.java");
        let second = temp.path().join("Second.java");
        std::fs::write(&first, "class First {}\n").expect("write first source");
        std::fs::write(&second, "class Second {}\n").expect("write second source");

        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .expect("build config resolver");
        let first = resolver
            .select(&mut out, &first)
            .expect("resolve first source")
            .expect("first source is selected");
        let second = resolver
            .select(&mut out, &second)
            .expect("resolve second source")
            .expect("second source is selected");

        assert!(Arc::ptr_eq(
            first.format_options.as_ref().expect("format policy requested"),
            second.format_options.as_ref().expect("format policy requested"),
        ));
    }

    #[test]
    fn formatter_policy_is_shared_when_editorconfig_has_no_java_override() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let first = temp.path().join("First.java");
        let second = temp.path().join("Second.java");
        std::fs::write(&first, "class First {}\n").expect("write first source");
        std::fs::write(&second, "class Second {}\n").expect("write second source");
        std::fs::write(
            temp.path().join(".editorconfig"),
            "[*]\nindent_size = 2\n[*.md]\nindent_size = 8\n",
        )
        .expect("write editorconfig");

        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .expect("build config resolver");
        let first = resolver
            .select(&mut out, &first)
            .expect("resolve first source")
            .expect("first source is selected");
        let second = resolver
            .select(&mut out, &second)
            .expect("resolve second source")
            .expect("second source is selected");
        let first = first.format_options.as_ref().expect("format policy requested");
        let second = second.format_options.as_ref().expect("format policy requested");

        assert_eq!(first.indent_width.value(), 2);
        assert!(Arc::ptr_eq(first, second));
    }

    #[test]
    fn formatter_policy_is_owned_for_a_real_editorconfig_override() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let ordinary = temp.path().join("Ordinary.java");
        let generated = temp.path().join("generated/Generated.java");
        std::fs::create_dir_all(generated.parent().expect("generated source has parent"))
            .expect("create generated directory");
        std::fs::write(&ordinary, "class Ordinary {}\n").expect("write ordinary source");
        std::fs::write(&generated, "class Generated {}\n").expect("write generated source");
        std::fs::write(
            temp.path().join(".editorconfig"),
            "[*]\nindent_size = 2\n[generated/*.java]\nindent_size = 8\n",
        )
        .expect("write editorconfig");

        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .expect("build config resolver");
        let ordinary = resolver
            .select(&mut out, &ordinary)
            .expect("resolve ordinary source")
            .expect("ordinary source is selected")
            .format_options
            .expect("format policy requested");
        let generated = resolver
            .select(&mut out, &generated)
            .expect("resolve generated source")
            .expect("generated source is selected")
            .format_options
            .expect("format policy requested");

        assert_eq!(ordinary.indent_width.value(), 2);
        assert_eq!(generated.indent_width.value(), 8);
        assert!(!Arc::ptr_eq(&ordinary, &generated));
    }

    #[test]
    fn editorconfig_cache_reuses_parsed_files_and_shared_formatter_policies() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let config = temp.path().join(".editorconfig");
        let first = temp.path().join("First.java");
        let sibling = temp.path().join("nested/Sibling.java");
        std::fs::create_dir_all(sibling.parent().unwrap()).unwrap();
        std::fs::write(&config, "[*]\nindent_size = 2\n").unwrap();
        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .unwrap();

        let first = resolver.select(&mut out, &first).unwrap().unwrap().format_options.unwrap();
        std::fs::write(&config, "[*]\nindent_size = 8\n").unwrap();
        let sibling = resolver.select(&mut out, &sibling).unwrap().unwrap().format_options.unwrap();

        assert_eq!(sibling.indent_width.value(), 2);
        assert!(Arc::ptr_eq(&first, &sibling));
    }

    #[test]
    fn editorconfig_cache_reads_a_shared_parent_file_once() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let parent = temp.path().join(".editorconfig");
        for directory in ["a", "b"] {
            std::fs::create_dir_all(temp.path().join(directory)).unwrap();
            std::fs::write(
                temp.path().join(directory).join(".editorconfig"),
                "[*.java]\nmax_line_length = 90\n",
            )
            .unwrap();
        }
        std::fs::write(&parent, "root = true\n[*]\nindent_size = 4\n").unwrap();
        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .unwrap();

        let first = temp.path().join("a/First.java");
        let first = resolver.select(&mut out, &first).unwrap().unwrap().format_options.unwrap();
        std::fs::write(&parent, "root = true\n[*]\nindent_size = 8\n").unwrap();
        let second = temp.path().join("b/Second.java");
        let second = resolver.select(&mut out, &second).unwrap().unwrap().format_options.unwrap();

        assert_eq!((first.indent_width.value(), first.line_width.value()), (4, 90));
        assert_eq!((second.indent_width.value(), second.line_width.value()), (4, 90));
    }

    #[test]
    fn editorconfig_cache_retains_a_failed_read() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let config = temp.path().join(".editorconfig");
        std::fs::write(&config, [0xff]).unwrap();
        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: true },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .unwrap();

        assert!(resolver.select(&mut out, &temp.path().join("First.java")).is_err());
        std::fs::write(&config, "[*]\nindent_size = 2\n").unwrap();
        assert!(resolver.select(&mut out, &temp.path().join("Sibling.java")).is_err());
    }

    #[test]
    fn file_config_discovery_reuses_a_confirmed_parent_scope() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let config = temp.path().join("project/robusta.yaml");
        let first = temp.path().join("project/src/main/java/First.java");
        let sibling = temp.path().join("project/src/main/java/Sibling.java");
        std::fs::create_dir_all(first.parent().expect("source has parent"))
            .expect("create source directory");
        std::fs::write(&config, "").expect("write config");

        let mut scopes = FxHashMap::default();
        assert_eq!(
            discover_file_config_path(&first, robusta_config::CONFIG_FILE_NAME, &mut scopes),
            Some(config.clone())
        );
        std::fs::remove_file(&config).expect("remove config after its scope is confirmed");

        assert_eq!(
            discover_file_config_path(&sibling, robusta_config::CONFIG_FILE_NAME, &mut scopes),
            Some(config)
        );
    }

    #[test]
    fn per_file_resolution_preserves_the_nearest_scope_after_an_outer_cache_candidate() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let outer = temp.path().join("project/src/Outer.java");
        let nested = temp.path().join("project/src/nested/Nested.java");
        std::fs::create_dir_all(outer.parent().expect("outer source has parent"))
            .expect("create outer source directory");
        std::fs::create_dir_all(nested.parent().expect("nested source has parent"))
            .expect("create nested source directory");
        std::fs::write(&outer, "class Outer {}\n").expect("write outer source");
        std::fs::write(&nested, "class Nested {}\n").expect("write nested source");
        std::fs::write(temp.path().join("project/robusta.yaml"), "format:\n  line-width: 90\n")
            .expect("write outer config");
        std::fs::write(
            temp.path().join("project/src/nested/robusta.yaml"),
            "format:\n  line-width: 120\n",
        )
        .expect("write nested config");

        let mut out = Output::stdio();
        let mut resolver = ConfigResolver::new(
            &mut out,
            &ConfigOptions { config: None, no_nested_config: false },
            temp.path(),
            SelectionFeatures::FORMAT,
            &[],
        )
        .expect("build config resolver");

        let outer_policy = resolver
            .select(&mut out, &outer)
            .expect("resolve outer source")
            .expect("outer source is selected");
        let nested_policy = resolver
            .select(&mut out, &nested)
            .expect("resolve nested source")
            .expect("nested source is selected");
        let outer_options = outer_policy.format_options.expect("format policy requested");
        let nested_options = nested_policy.format_options.expect("format policy requested");

        assert_eq!(outer_options.line_width.value(), 90);
        assert_eq!(nested_options.line_width.value(), 120);
    }

    #[test]
    fn display_path_rewrites_backslashes_only_where_they_separate() {
        let expected = if cfg!(windows) { "src/nested/File.java" } else { r"src\nested\File.java" };
        let normalized = display_path(Path::new(r"src\nested\File.java"));
        assert_eq!(normalized.to_string_lossy(), expected);
    }

    #[test]
    fn display_path_leaves_forward_slashes_unchanged() {
        let normalized = display_path(Path::new("src/nested/File.java"));
        assert_eq!(normalized.to_string_lossy(), "src/nested/File.java");
    }
}
