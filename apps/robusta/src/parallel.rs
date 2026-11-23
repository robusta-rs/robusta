use std::num::NonZeroUsize;
use std::sync::OnceLock;

use rayon::prelude::*;
use robusta_diagnostics::Output;

/// A destination for one file's streamed output. The process-wide [`Output`]
/// writes straight to stdio; a [`Buffer`] collects a worker's output so the
/// main thread can flush results in input order. Both let the per-file work be
/// identical on the main thread (stdin) and on rayon workers.
pub trait Sink {
    /// Append to stdout without a trailing newline.
    fn stdout(&mut self, text: &str);
    /// Append one line to stdout.
    fn stdout_line(&mut self, line: &str);
    /// Append to stderr without a trailing newline. Per-file stderr is always a
    /// reporter's already-terminated diagnostic block, written in one go.
    fn stderr(&mut self, text: &str);
}

impl Sink for Output {
    fn stdout(&mut self, text: &str) {
        Output::stdout(self, text);
    }

    fn stdout_line(&mut self, line: &str) {
        Output::stdout_line(self, line);
    }

    fn stderr(&mut self, text: &str) {
        Output::stderr(self, text);
    }
}

/// One file's buffered stdout and stderr, flushed to the shared [`Output`] in
/// input order once the parallel map completes.
#[derive(Default)]
pub struct Buffer {
    stdout: String,
    stderr: String,
}

impl Buffer {
    /// Emit this file's captured output to the shared streams. Called on the
    /// main thread in input order, so the byte stream matches a serial run.
    pub fn flush_to(&self, out: &mut Output) {
        if !self.stdout.is_empty() {
            out.stdout(&self.stdout);
        }
        if !self.stderr.is_empty() {
            out.stderr(&self.stderr);
        }
    }
}

impl Sink for Buffer {
    fn stdout(&mut self, text: &str) {
        self.stdout.push_str(text);
    }

    fn stdout_line(&mut self, line: &str) {
        self.stdout.push_str(line);
        self.stdout.push('\n');
    }

    fn stderr(&mut self, text: &str) {
        self.stderr.push_str(text);
    }
}

/// Initialize the one worker pool used by source discovery and per-file
/// processing.
///
/// This must run before either phase: discovery sizes its walker from the
/// pool, and a late initialization would let the first parallel iterator
/// choose a different count from `--threads`.
pub fn initialize(threads: Option<usize>) {
    static POOL: OnceLock<()> = OnceLock::new();
    POOL.get_or_init(|| {
        let count = resolve_thread_count(threads);
        rayon::ThreadPoolBuilder::new()
            .num_threads(count)
            .build_global()
            .expect("rayon was initialized before the CLI runtime options");
    });
}

/// The worker count shared by every parallel phase of the run.
pub fn thread_count() -> usize {
    rayon::current_num_threads()
}

/// Map `work` over `files`, preserving input order in the returned vector.
///
/// [`initialize`] has already fixed the run's worker count before discovery.
/// Rayon's indexed `par_iter().collect()` keeps results in input order for
/// deterministic emission.
pub fn map_files<I, T, F>(files: &[I], work: F) -> Vec<T>
where
    I: Sync,
    T: Send,
    F: Fn(&I) -> T + Sync + Send,
{
    files.par_iter().map(work).collect()
}

/// Resolve the worker count: an explicit positive `--threads`, else the
/// available parallelism, else a single thread. `--threads 0` therefore means
/// the available parallelism too.
fn resolve_thread_count(threads: Option<usize>) -> usize {
    if let Some(count) = threads.filter(|&count| count > 0) {
        return count;
    }
    std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_explicit_thread_counts_and_zero_as_auto() {
        assert_eq!(resolve_thread_count(Some(8)), 8);
        assert_eq!(resolve_thread_count(Some(1)), 1);
        assert_eq!(resolve_thread_count(Some(0)), resolve_thread_count(None));
    }
}
