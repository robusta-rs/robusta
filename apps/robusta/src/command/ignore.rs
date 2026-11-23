use std::path::PathBuf;

use bpaf::Bpaf;

/// Ignore options:
#[derive(Debug, Clone, Bpaf)]
pub struct IgnoreOptions {
    /// Path to an additional gitignore-syntax file. Can be specified multiple times.
    #[bpaf(long, argument("PATH"), many)]
    pub ignore_path: Vec<PathBuf>,
    /// Gitignore-syntax pattern to exclude, resolved from the working directory. Quote it so the
    /// shell does not expand it. These patterns are ordered and outrank `--ignore-path` files: the
    /// last one to match decides, and a leading `!` reincludes. Can be specified multiple times.
    #[bpaf(long, argument("PAT"), many)]
    pub ignore_pattern: Vec<String>,
}
