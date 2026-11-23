// Adapted from https://github.com/oxc-project/oxc/blob/9cd28b36ba1930f3ee042e5caa18c0a8fd57c635/apps/oxlint/src/mode/debug_files.rs
use robusta_diagnostics::Output;

use crate::command::{ConfigOptions, FileListing};
use crate::result::CliRunResult;
use crate::{parallel, walk};

pub fn run_debug_files(
    out: &mut Output,
    listing: &FileListing,
    options: &ConfigOptions,
) -> CliRunResult {
    parallel::initialize(None);
    let Ok(ignore_matcher) = walk::ignore_matcher(out, &listing.ignore_options) else {
        return CliRunResult::FileSelectionFailed;
    };
    let Ok(walk::Selection { mut files, .. }) = walk::selected_files(
        out,
        &listing.paths,
        options,
        &ignore_matcher,
        false,
        walk::SelectionFeatures::LIST,
        &[],
        parallel::thread_count(),
    ) else {
        return CliRunResult::FileSelectionFailed;
    };
    files.sort_unstable_by(|a, b| a.path.cmp(&b.path));
    for file in files {
        out.stdout_line(&file.display_path.to_string_lossy());
    }
    CliRunResult::FilesListed
}
