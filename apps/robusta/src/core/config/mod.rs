// Directory adapted from https://github.com/oxc-project/oxc/tree/0e6498f7e218420153910074d5a55bbef7c1c1d4/apps/oxfmt/src/core/config

pub mod editorconfig;
mod overrides;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use robusta_formatter::Options;
use rustc_hash::FxHashMap;

pub use self::overrides::FormatOverrides;

use self::editorconfig::{Resolver, apply_editorconfig};
use super::format_config::FormatConfig;
use super::options::{to_robusta_formatter, validate};

/// Formatter options for the files one `robusta.yaml` governs.
///
/// Matching `format.overrides` replace settings, then `.editorconfig` fills
/// the fields left unset.
pub struct FormatPolicy {
    /// The format section as written, before `.editorconfig` fills it.
    config: FormatConfig,
    /// `config` translated on its own, for files without an `.editorconfig`.
    options: Arc<Options>,
    overrides: FormatOverrides,
    /// Shared root policies keyed by the nearest EditorConfig path.
    roots: FxHashMap<PathBuf, Arc<Options>>,
}

impl FormatPolicy {
    /// `options` is `config` passed through the gate, as loading does.
    pub fn new(config: FormatConfig, options: Options, overrides: FormatOverrides) -> Self {
        Self { config, options: Arc::new(options), overrides, roots: FxHashMap::default() }
    }

    /// Resolve the options for `path`.
    ///
    /// Fast path: a file no override or per-file `.editorconfig` section
    /// changes shares the translation of its `.editorconfig` files' `[*]` sections.
    ///
    /// # Errors
    ///
    /// Returns an error if `.editorconfig` supplies a width outside its option's range.
    pub fn resolve(
        &mut self,
        editorconfig: Option<&Resolver>,
        path: &Path,
    ) -> Result<Arc<Options>, String> {
        let overrides = self.overrides.matching(path);
        let editorconfig_overrides =
            editorconfig.and_then(|resolver| resolver.resolve_overrides(path));
        let shared = overrides.is_empty() && editorconfig_overrides.is_none();
        if shared {
            let Some(resolver) = editorconfig else {
                return Ok(Arc::clone(&self.options));
            };
            if let Some(root) = self.roots.get(resolver.path()) {
                return Ok(Arc::clone(root));
            }
        }

        let mut config = self.config.clone();
        for format in overrides {
            config.merge(format);
        }
        if let Some(resolver) = editorconfig {
            let properties =
                editorconfig_overrides.as_ref().unwrap_or_else(|| resolver.root_properties());
            apply_editorconfig(&mut config, properties).map_err(|err| {
                format!("{}: {err}", resolver.origin(path, err.property).display())
            })?;
        }
        let validated = validate(&config, None).expect(
            "robusta.yaml and its overrides passed the gate and .editorconfig fills only in-range values",
        );
        let resolved = Arc::new(to_robusta_formatter(&config, validated));
        if shared && let Some(resolver) = editorconfig {
            self.roots.insert(resolver.path().to_path_buf(), Arc::clone(&resolved));
        }
        Ok(resolved)
    }
}
