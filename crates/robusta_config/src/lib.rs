//! Shared configuration infrastructure for Robusta command-line tools.

mod discovery;
mod gitignore;
mod glob_set;
mod ignore_patterns;
mod repository_rules;
mod scalar;
mod walk;

pub use discovery::{CONFIG_FILE_NAME, discover_ancestor_file, discover_config_path};
pub use glob_set::GlobSet;
pub use ignore_patterns::IgnorePatterns;
pub use scalar::deserialize_scalar;
pub use walk::{IgnoreMatcher, PRUNED_TRAVERSAL_DIRS, collect_java_files, is_java_path};
