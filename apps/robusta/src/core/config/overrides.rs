use std::path::{Path, PathBuf};

use robusta_config::GlobSet;

use crate::config_file::FormatOverrideConfig;
use crate::core::format_config::FormatConfig;

/// The `format.overrides` entries, for per-file matching.
#[derive(Default)]
pub struct FormatOverrides {
    base_dir: PathBuf,
    entries: Vec<OverrideEntry>,
}

impl FormatOverrides {
    pub fn new(overrides: Vec<FormatOverrideConfig>, base_dir: &Path) -> Self {
        Self {
            base_dir: base_dir.to_path_buf(),
            entries: overrides
                .into_iter()
                .map(|config| OverrideEntry {
                    files: config.files,
                    exclude_files: config.exclude_files,
                    options: config.options,
                })
                .collect(),
        }
    }

    /// Collect the format settings of every entry matching `path`, in configuration order.
    pub fn matching(&self, path: &Path) -> Vec<&FormatConfig> {
        if self.entries.is_empty() {
            return Vec::new();
        }
        let relative = path.strip_prefix(&self.base_dir).unwrap_or(path).to_string_lossy();
        self.entries
            .iter()
            .filter(|entry| {
                entry.files.is_match(&relative) && !entry.exclude_files.is_match(&relative)
            })
            .map(|entry| &entry.options)
            .collect()
    }
}

/// One `format.overrides` entry.
struct OverrideEntry {
    files: GlobSet,
    exclude_files: GlobSet,
    options: FormatConfig,
}
