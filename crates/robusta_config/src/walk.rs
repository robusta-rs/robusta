// Adapted from https://github.com/oxc-project/oxc/blob/d1dbd7187a33c743ba7ec7d6210fc4c09ad1e960/crates/oxc_config/src/walk.rs

use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};

use ignore::{
    DirEntry, WalkBuilder, WalkState,
    gitignore::{Gitignore, GitignoreBuilder},
};

use crate::{gitignore, repository_rules::RepositoryRules};

/// Version-control metadata directories pruned below explicit roots.
pub const PRUNED_TRAVERSAL_DIRS: &[&str] = &[".git", ".hg", ".jj", ".sl", ".svn"];

/// The run's explicit exclusions: `--ignore-path` files and `--ignore-pattern`
/// arguments.
///
/// Each ignore file uses gitignore syntax rooted at the directory containing
/// that file, and the files compose as a union: a path ignored by any of them
/// is ignored overall. The command-line patterns form one further ordered
/// gitignore layer rooted at the invocation directory. That layer is consulted
/// first and its decision is final, so a later pattern overrides an earlier
/// one and a `!` pattern re-includes a path an `--ignore-path` file excluded.
/// As in Git, reinclusion cannot reach below an excluded directory, because
/// the walk never descends into one.
///
/// The repository's own `.gitignore` and `.git/info/exclude` are applied by the
/// walk rather than through this matcher, so a `!` pattern narrows what this
/// matcher excludes and never widens the walk past a Git ignore.
///
/// That layer is a gitignore matcher rather than one of the ignore crate's
/// overrides, whose polarity is reversed: raw patterns given to an override
/// builder mean the opposite of what they say, and negating each one to
/// compensate would leave a user's own leading `!` as a literal character.
///
/// A configuration's `ignore-patterns` compile to a matcher of their own,
/// with no ignore files and rooted at the configuration directory.
#[derive(Clone, Debug, Default)]
pub struct IgnoreMatcher {
    files: Arc<[Gitignore]>,
    patterns: Option<Arc<Gitignore>>,
}

impl IgnoreMatcher {
    /// Load and validate the run's ignore files and command-line patterns.
    ///
    /// `root` is the invocation directory: relative ignore paths resolve
    /// against it and the patterns are rooted at it. The caller supplies it so
    /// that one directory answers for path normalization and for matching
    /// alike. An absent, non-file, unreadable, or malformed ignore path is an
    /// error, as is a pattern the gitignore syntax cannot express.
    ///
    /// # Errors
    ///
    /// Returns an I/O error describing the first invalid ignore path or
    /// pattern.
    pub fn new(paths: &[PathBuf], patterns: &[String], root: &Path) -> io::Result<Self> {
        if paths.is_empty() && patterns.is_empty() {
            return Ok(Self::default());
        }
        Ok(Self {
            files: Self::load_files(paths, root)?,
            patterns: Self::compile_patterns(patterns, root)?,
        })
    }

    fn load_files(paths: &[PathBuf], root: &Path) -> io::Result<Arc<[Gitignore]>> {
        let mut matchers = Vec::with_capacity(paths.len());
        for path in paths {
            let absolute = normalize_absolute_path(path, root);
            let metadata = std::fs::metadata(&absolute).map_err(|err| {
                io::Error::new(err.kind(), format!("ignore file {}: {err}", path.display()))
            })?;
            if !metadata.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("ignore file {} is not a file", path.display()),
                ));
            }
            let root = absolute.parent().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("ignore file {} has no parent directory", path.display()),
                )
            })?;
            let parse_error = |error: String| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("failed to parse ignore file {}: {error}", path.display()),
                )
            };
            let contents = std::fs::read_to_string(&absolute)
                .map_err(|error| parse_error(error.to_string()))?;
            let mut builder = GitignoreBuilder::new(root);
            gitignore::lines(&contents)
                .try_for_each(|line| gitignore::add_line(&mut builder, line))
                .map_err(parse_error)?;
            let matcher = builder.build().map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("failed to build ignore file {}: {error}", path.display()),
                )
            })?;
            matchers.push(matcher);
        }
        Ok(Arc::from(matchers))
    }

    /// Compile the command-line patterns into one matcher, once per run, so
    /// the traversal filter and the explicit-path check share it instead of
    /// rebuilding the glob set per candidate path.
    fn compile_patterns(patterns: &[String], root: &Path) -> io::Result<Option<Arc<Gitignore>>> {
        if patterns.is_empty() {
            return Ok(None);
        }
        let mut builder = GitignoreBuilder::new(root);
        for pattern in patterns {
            gitignore::add_line(&mut builder, pattern).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse ignore pattern {pattern}: {error}"),
                )
            })?;
        }
        let matcher = builder.build().map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("failed to build ignore patterns: {error}"),
            )
        })?;
        Ok(Some(Arc::new(matcher)))
    }

    /// Whether traversal from `root` would exclude `path`, including missing files.
    pub fn is_ignored_by_walk(&self, path: &Path, root: &Path) -> bool {
        self.is_ignored(path, false, true)
            || is_gitignored(path)
            || path
                .ancestors()
                .skip(1)
                .take_while(|parent| parent.starts_with(root) && *parent != root)
                .any(|parent| {
                    is_pruned_path(parent)
                        || parent.symlink_metadata().is_ok_and(|metadata| metadata.is_symlink())
                })
    }

    /// Return whether a path is excluded by the run's ignore files or patterns.
    pub fn is_ignored(&self, path: &Path, is_dir: bool, check_ancestors: bool) -> bool {
        if check_ancestors && (self.patterns.is_some() || !self.files.is_empty()) {
            return self.is_ignored(path, is_dir, false)
                || path.ancestors().skip(1).any(|parent| self.is_ignored(parent, true, false));
        }
        if let Some(patterns) = &self.patterns
            && let Some(ignored) = gitignore::decision(patterns, path, is_dir)
        {
            return ignored;
        }
        self.files.iter().any(|matcher| gitignore::decision(matcher, path, is_dir) == Some(true))
    }
}

fn normalize_absolute_path(path: &Path, root: &Path) -> PathBuf {
    use std::path::Component;

    let absolute = if path.is_absolute() { path.to_path_buf() } else { root.join(path) };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component),
        }
    }
    normalized
}

/// Collect every `.java` file under `root`, in unspecified order, using the
/// run's resolved worker count.
///
/// An explicit symlink root is followed; symlinked directories discovered
/// during the walk are not. The first traversal error fails the whole root.
///
/// # Errors
///
/// Returns the first traversal error encountered by the parallel walker.
pub fn collect_java_files(
    root: &Path,
    ignore_matcher: &IgnoreMatcher,
    threads: usize,
) -> io::Result<Vec<PathBuf>> {
    if ignore_matcher.is_ignored(root, true, true) || is_gitignored(root) {
        return Ok(Vec::new());
    }

    let ignore_matcher = ignore_matcher.clone();
    let walk_root = root.to_path_buf();
    // The rules above a symlinked root are those of the directory it names.
    let canonical_root = root.canonicalize().unwrap_or_else(|_| walk_root.clone());
    let is_symlinked = canonical_root != walk_root;
    let rules = RepositoryRules::new(find_vcs_boundary(&canonical_root).is_some());
    let walker = WalkBuilder::new(root)
        // The walker's own ignore filters read a different pattern grammar.
        .standard_filters(false)
        .threads(threads)
        .filter_entry(move |entry| {
            let path = entry.path();
            let is_dir = entry.file_type().is_some_and(|file_type| file_type.is_dir());
            (is_dir || is_java_file(entry))
                && !is_pruned_traversal_dir(entry)
                && !ignore_matcher.is_ignored(path, is_dir, false)
                && !if is_symlinked {
                    let relative = path.strip_prefix(&walk_root).unwrap_or(path);
                    rules.is_ignored(&canonical_root.join(relative), is_dir)
                } else {
                    rules.is_ignored(path, is_dir)
                }
        })
        .build_parallel();
    let (sender, receiver) = mpsc::channel();
    walker.run(|| {
        let sender = sender.clone();
        Box::new(move |entry| match entry {
            Ok(entry) => {
                if is_java_file(&entry) && sender.send(Ok(entry.into_path())).is_err() {
                    return WalkState::Quit;
                }
                WalkState::Continue
            }
            Err(err) => {
                let _ = sender.send(Err(err));
                WalkState::Quit
            }
        })
    });
    drop(sender);
    let mut files = Vec::new();
    for result in receiver {
        match result {
            Ok(path) => files.push(path),
            Err(err) => {
                let message = err.to_string();
                return Err(err.into_io_error().unwrap_or_else(|| io::Error::other(message)));
            }
        }
    }
    Ok(files)
}

/// Whether Git ignore rules exclude a path or its ancestors below the repository boundary.
///
/// A walk never filters its root, and prunes an ignored directory only while
/// descending into it, so a root inside an ignored directory would otherwise
/// be walked.
fn is_gitignored(path: &Path) -> bool {
    let Ok(path) = std::path::absolute(path) else { return false };
    let is_dir = path.symlink_metadata().is_ok_and(|metadata| metadata.is_dir());
    let vcs_root = find_vcs_boundary(&path);
    // Without a repository, rules run up to the filesystem root.
    let base = vcs_root.or_else(|| path.ancestors().last()).unwrap_or(&path);
    if path == base {
        return false;
    }
    let rules = RepositoryRules::new(vcs_root.is_some());
    let mut ancestors: Vec<_> =
        path.ancestors().skip(1).take_while(|ancestor| *ancestor != base).collect();
    ancestors.reverse();
    ancestors.into_iter().any(|ancestor| rules.is_ignored(ancestor, true))
        || rules.is_ignored(&path, is_dir)
}

fn find_vcs_boundary(path: &Path) -> Option<&Path> {
    let start = if path.is_file() { path.parent().unwrap_or(path) } else { path };
    start
        .ancestors()
        .find(|directory| directory.join(".git").exists() || directory.join(".jj").exists())
}

fn is_pruned_traversal_dir(entry: &DirEntry) -> bool {
    entry.depth() > 0
        && entry.file_type().is_some_and(|file_type| file_type.is_dir())
        && is_pruned_path(entry.path())
}

fn is_pruned_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| PRUNED_TRAVERSAL_DIRS.contains(&name))
}

fn is_java_file(entry: &DirEntry) -> bool {
    entry.file_type().is_some_and(|file_type| !file_type.is_dir()) && is_java_path(entry.path())
}

/// Whether a file's name makes it Java source.
pub fn is_java_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "java")
}

#[cfg(test)]
mod tests {
    use super::{IgnoreMatcher, collect_java_files};

    #[test]
    fn collects_only_java_files() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let source = temp.path().join("src/Example.java");
        let non_java = temp.path().join("src/README.md");
        std::fs::create_dir_all(source.parent().expect("source has parent"))
            .expect("create source directory");
        std::fs::write(&source, "class Example {}").expect("write Java source");
        std::fs::write(non_java, "readme").expect("write non-Java file");

        let files = collect_java_files(temp.path(), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert_eq!(files, vec![source]);
    }

    #[test]
    fn prunes_version_control_directories_below_the_root() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let source = temp.path().join("Example.java");
        std::fs::write(&source, "class Example {}").expect("write Java source");
        for directory in [".git", ".hg", ".jj", ".sl", ".svn"] {
            let metadata = temp.path().join(directory).join("Metadata.java");
            std::fs::create_dir_all(metadata.parent().expect("metadata has parent"))
                .expect("create metadata directory");
            std::fs::write(metadata, "class Metadata {}").expect("write metadata source");
        }

        let files = collect_java_files(temp.path(), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");
        let explicit = collect_java_files(&temp.path().join(".svn"), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert_eq!(files, vec![source]);
        assert_eq!(explicit, vec![temp.path().join(".svn/Metadata.java")]);
    }

    #[test]
    fn respects_gitignore_without_requiring_a_repository() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let included = temp.path().join("Included.java");
        let ignored = temp.path().join("Ignored.java");
        std::fs::write(&included, "class Included {}").expect("write included source");
        std::fs::write(ignored, "class Ignored {}").expect("write ignored source");
        std::fs::write(temp.path().join(".gitignore"), "Ignored.java\n").expect("write gitignore");

        let files = collect_java_files(temp.path(), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert_eq!(files, vec![included]);
    }

    #[test]
    fn respects_repository_local_git_exclude() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let included = temp.path().join("Included.java");
        let ignored = temp.path().join("Ignored.java");
        let exclude = temp.path().join(".git/info/exclude");
        std::fs::create_dir_all(exclude.parent().expect("exclude file has parent"))
            .expect("create Git metadata directory");
        std::fs::write(&included, "class Included {}").expect("write included source");
        std::fs::write(ignored, "class Ignored {}").expect("write ignored source");
        std::fs::write(exclude, "Ignored.java\n").expect("write Git exclude");

        let files = collect_java_files(temp.path(), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert_eq!(files, vec![included]);
    }

    #[test]
    fn does_not_read_generic_ignore_files() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let included = temp.path().join("Included.java");
        let ignored = temp.path().join("Ignored.java");
        std::fs::write(&included, "class Included {}").expect("write included source");
        std::fs::write(&ignored, "class Ignored {}").expect("write second source");
        std::fs::write(temp.path().join(".ignore"), "Ignored.java\n")
            .expect("write generic ignore");

        let mut files = collect_java_files(temp.path(), &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");
        files.sort();
        let mut expected = vec![included, ignored];
        expected.sort();

        assert_eq!(files, expected);
    }

    #[test]
    fn explicit_ignore_files_are_rooted_at_their_directory() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let included = temp.path().join("config/Included.java");
        let ignored = temp.path().join("config/generated/Ignored.java");
        let ignore_path = temp.path().join("config/robusta.ignore");
        std::fs::create_dir_all(included.parent().expect("included source has parent"))
            .expect("create source directory");
        std::fs::create_dir_all(ignored.parent().expect("ignored source has parent"))
            .expect("create generated directory");
        std::fs::create_dir_all(ignore_path.parent().expect("ignore file has parent"))
            .expect("create config directory");
        std::fs::write(&included, "class Included {}").expect("write included source");
        std::fs::write(ignored, "class Ignored {}").expect("write ignored source");
        std::fs::write(&ignore_path, "generated/\n").expect("write ignore file");
        let matcher =
            IgnoreMatcher::new(&[ignore_path], &[], temp.path()).expect("load ignore file");

        let files = collect_java_files(temp.path(), &matcher, 1).expect("walk should succeed");

        assert_eq!(files, vec![included]);
    }

    #[test]
    fn malformed_explicit_ignore_file_is_rejected() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let ignore_path = temp.path().join("robusta.ignore");
        std::fs::write(&ignore_path, "Open[.java\n").expect("write ignore file");

        let error = IgnoreMatcher::new(&[ignore_path], &[], temp.path())
            .expect_err("ignore must be invalid");

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("failed to parse ignore file"));
    }

    #[test]
    fn command_line_patterns_apply_in_order_with_reinclusion() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let kept = temp.path().join("src/Kept.java");
        let excluded = temp.path().join("src/Excluded.java");
        std::fs::create_dir_all(kept.parent().expect("source has parent"))
            .expect("create source directory");
        std::fs::write(&kept, "class Kept {}").expect("write kept source");
        std::fs::write(excluded, "class Excluded {}").expect("write excluded source");
        let patterns = [String::from("*.java"), String::from("!src/Kept.java")];
        let matcher =
            IgnoreMatcher::new(&[], &patterns, temp.path()).expect("compile ignore patterns");

        let files = collect_java_files(temp.path(), &matcher, 1).expect("walk should succeed");

        assert_eq!(files, vec![kept]);
    }

    #[test]
    fn command_line_patterns_decide_over_ignore_files() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let kept = temp.path().join("vendor/Kept.java");
        let excluded = temp.path().join("vendor/Excluded.java");
        let ignore_path = temp.path().join("robusta.ignore");
        std::fs::create_dir_all(kept.parent().expect("source has parent"))
            .expect("create vendor directory");
        std::fs::write(&kept, "class Kept {}").expect("write kept source");
        std::fs::write(excluded, "class Excluded {}").expect("write excluded source");
        std::fs::write(&ignore_path, "vendor/*.java\n").expect("write ignore file");
        let patterns = [String::from("!vendor/Kept.java")];
        let matcher = IgnoreMatcher::new(&[ignore_path], &patterns, temp.path())
            .expect("compile ignore inputs");

        let files = collect_java_files(temp.path(), &matcher, 1).expect("walk should succeed");

        assert_eq!(files, vec![kept]);
    }

    #[test]
    fn command_line_patterns_cannot_reinclude_below_a_pruned_directory() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let hidden = temp.path().join("vendor/Kept.java");
        std::fs::create_dir_all(hidden.parent().expect("source has parent"))
            .expect("create vendor directory");
        std::fs::write(hidden, "class Kept {}").expect("write source");
        let patterns = [String::from("vendor/"), String::from("!vendor/Kept.java")];
        let matcher =
            IgnoreMatcher::new(&[], &patterns, temp.path()).expect("compile ignore patterns");

        let files = collect_java_files(temp.path(), &matcher, 1).expect("walk should succeed");

        // Gitignore semantics: the walk never descends into an excluded
        // directory, so nothing below it can be reincluded.
        assert!(files.is_empty(), "{files:?}");
    }

    #[test]
    fn command_line_patterns_exclude_an_explicit_file_through_its_parent() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let nested = temp.path().join("src/generated/Nested.java");
        std::fs::create_dir_all(nested.parent().expect("source has parent"))
            .expect("create generated directory");
        std::fs::write(&nested, "class Nested {}").expect("write nested source");
        let patterns = [String::from("generated/")];
        let matcher =
            IgnoreMatcher::new(&[], &patterns, temp.path()).expect("compile ignore patterns");

        assert!(matcher.is_ignored(&nested, false, true));
        assert!(!matcher.is_ignored(&nested, false, false));
    }

    #[test]
    fn excluded_ancestors_override_negated_paths() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let ignore_path = temp.path().join("robusta.ignore");
        for (file_patterns, cli_patterns) in [
            ("", vec!["vendor/", "!vendor/Kept.java", "!vendor/sub/"]),
            ("vendor/\n!vendor/Kept.java\n!vendor/sub/\n", vec![]),
            ("vendor/\n", vec!["!vendor/Kept.java", "!vendor/sub/"]),
        ] {
            std::fs::write(&ignore_path, file_patterns).expect("write ignore file");
            let patterns: Vec<_> = cli_patterns.into_iter().map(String::from).collect();
            let matcher =
                IgnoreMatcher::new(std::slice::from_ref(&ignore_path), &patterns, temp.path())
                    .expect("compile ignore inputs");

            for (relative, is_dir) in
                [("vendor/Kept.java", false), ("vendor/sub", true), ("vendor/sub/X.java", false)]
            {
                let path = temp.path().join(relative);
                assert!(
                    matcher.is_ignored(&path, is_dir, true),
                    "{file_patterns:?} {patterns:?} {relative}"
                );
            }
        }

        let root = temp.path().join("vendor");
        std::fs::create_dir_all(&root).expect("create matcher root");
        let ignore_path = root.join("robusta.ignore");
        std::fs::write(&ignore_path, "sub/\n").expect("write ignore file");
        let patterns = [String::from("*"), String::from("!vendor/"), String::from("!vendor/**")];
        let matcher = IgnoreMatcher::new(&[ignore_path], &patterns, temp.path())
            .expect("compile ignore inputs");

        assert!(!matcher.is_ignored(&root.join("sub/X.java"), false, true));
        let outside =
            temp.path().parent().expect("temporary directory has parent").join("Outside.java");
        assert!(!matcher.is_ignored(&outside, false, true));
    }

    #[test]
    fn command_line_patterns_do_not_reach_outside_their_root() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let outside = temp.path().join("Outside.java");
        std::fs::write(&outside, "class Outside {}").expect("write outside source");
        let root = temp.path().join("root");
        std::fs::create_dir_all(&root).expect("create pattern root");
        let patterns = [String::from("*.java")];
        let matcher = IgnoreMatcher::new(&[], &patterns, &root).expect("compile ignore patterns");

        assert!(!matcher.is_ignored(&outside, false, true));
    }

    #[test]
    fn malformed_command_line_pattern_is_rejected() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let patterns = [String::from("[")];

        let error =
            IgnoreMatcher::new(&[], &patterns, temp.path()).expect_err("pattern must be invalid");

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains("failed to parse ignore pattern ["));
    }

    /// `{First,Second}.java` names one file, with the three other files beside it.
    fn brace_fixture() -> (tempfile::TempDir, Vec<std::path::PathBuf>) {
        let temp = tempfile::tempdir().expect("create temp directory");
        std::fs::create_dir_all(temp.path().join(".git")).expect("create repository marker");
        std::fs::create_dir_all(temp.path().join(".git/info")).expect("create Git metadata");
        for name in ["{First,Second}.java", "First.java", "Second.java", "Keep.java"] {
            std::fs::write(temp.path().join(name), "class A {}").expect("write Java source");
        }
        let selected = ["First.java", "Keep.java", "Second.java"]
            .iter()
            .map(|name| temp.path().join(name))
            .collect();
        (temp, selected)
    }

    fn walk_sorted(root: &std::path::Path, matcher: &IgnoreMatcher) -> Vec<std::path::PathBuf> {
        let mut files = collect_java_files(root, matcher, 1).expect("walk should succeed");
        files.sort();
        files
    }

    #[test]
    fn braces_are_literal_in_every_ignore_source() {
        let pattern = "{First,Second}.java\n";

        let (temp, selected) = brace_fixture();
        std::fs::write(temp.path().join(".gitignore"), pattern).expect("write gitignore");
        assert_eq!(walk_sorted(temp.path(), &IgnoreMatcher::default()), selected);

        let (temp, selected) = brace_fixture();
        std::fs::write(temp.path().join(".git/info/exclude"), pattern).expect("write exclude");
        assert_eq!(walk_sorted(temp.path(), &IgnoreMatcher::default()), selected);

        let (temp, selected) = brace_fixture();
        let ignore_path = temp.path().join("robusta.ignore");
        std::fs::write(&ignore_path, pattern).expect("write ignore file");
        let matcher = IgnoreMatcher::new(&[ignore_path], &[], temp.path()).expect("load file");
        assert_eq!(walk_sorted(temp.path(), &matcher), selected);

        let (temp, selected) = brace_fixture();
        let patterns = [pattern.trim_end().to_string()];
        let matcher = IgnoreMatcher::new(&[], &patterns, temp.path()).expect("compile pattern");
        assert_eq!(walk_sorted(temp.path(), &matcher), selected);
    }

    #[test]
    fn a_brace_directory_is_literal_for_walk_roots_and_baseline_paths() {
        let temp = tempfile::tempdir().expect("create temp directory");
        std::fs::create_dir_all(temp.path().join(".git")).expect("create repository marker");
        std::fs::write(temp.path().join(".gitignore"), "{a,b}/\n").expect("write gitignore");
        let excluded = temp.path().join("{a,b}/child/Excluded.java");
        let kept = temp.path().join("a/Kept.java");
        for file in [&excluded, &kept] {
            std::fs::create_dir_all(file.parent().expect("file has parent"))
                .expect("create directory");
            std::fs::write(file, "class A {}").expect("write Java source");
        }
        let matcher = IgnoreMatcher::default();

        assert_eq!(walk_sorted(temp.path(), &matcher), vec![kept.clone()]);
        assert_eq!(walk_sorted(&temp.path().join("a"), &matcher), vec![kept.clone()]);
        for root in ["{a,b}", "{a,b}/child"] {
            let files = walk_sorted(&temp.path().join(root), &matcher);
            assert!(files.is_empty(), "{root}: {files:?}");
        }
        assert!(matcher.is_ignored_by_walk(&excluded, temp.path()));
        assert!(!matcher.is_ignored_by_walk(&kept, temp.path()));
    }

    #[test]
    fn a_negated_brace_pattern_reincludes_only_its_literal_name() {
        let (temp, _) = brace_fixture();
        std::fs::write(temp.path().join(".gitignore"), "*.java\n!{First,Second}.java\n")
            .expect("write gitignore");

        let files = walk_sorted(temp.path(), &IgnoreMatcher::default());

        assert_eq!(files, vec![temp.path().join("{First,Second}.java")]);
    }

    #[cfg(unix)]
    #[test]
    fn gitignore_rules_follow_a_symlinked_root_to_its_target() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let target = temp.path().join("repository/source");
        std::fs::create_dir_all(temp.path().join("repository/.git"))
            .expect("create repository marker");
        std::fs::create_dir_all(&target).expect("create target");
        std::fs::write(temp.path().join("repository/.gitignore"), "{a}.java\n")
            .expect("write gitignore");
        std::fs::write(target.join("{a}.java"), "class A {}").expect("write source");
        std::fs::write(target.join("a.java"), "class A {}").expect("write source");
        let link = temp.path().join("link");
        std::os::unix::fs::symlink(&target, &link).expect("create symlink");

        let files = walk_sorted(&link, &IgnoreMatcher::default());

        assert_eq!(files, vec![link.join("a.java")]);
    }

    #[test]
    fn parent_gitignore_does_not_cross_repository_boundary() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let repository = temp.path().join("repo");
        let included = repository.join("Included.java");
        std::fs::create_dir_all(repository.join(".git")).expect("create repository marker");
        std::fs::write(temp.path().join(".gitignore"), "*\n").expect("write parent gitignore");
        std::fs::write(&included, "class Included {}").expect("write included source");

        let files = collect_java_files(&repository, &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert_eq!(files, vec![included]);
    }

    #[test]
    fn skips_a_root_inside_a_gitignored_directory() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let included = temp.path().join("src/Included.java");
        let deep = temp.path().join("src/deep/nested");
        std::fs::create_dir_all(temp.path().join(".git")).expect("create repository marker");
        std::fs::create_dir_all(&deep).expect("create ignored directory");
        std::fs::write(&included, "class Included {}").expect("write included source");
        std::fs::write(deep.join("Ignored.java"), "class Ignored {}")
            .expect("write ignored source");
        std::fs::write(temp.path().join("src/.gitignore"), "deep/\n").expect("write gitignore");
        let matcher = IgnoreMatcher::default();

        let from_repository = collect_java_files(temp.path(), &matcher, 1).expect("walk root");
        let ignored = collect_java_files(&temp.path().join("src/deep"), &matcher, 1)
            .expect("walk ignored directory");
        let below_ignored = collect_java_files(&deep, &matcher, 1).expect("walk nested directory");

        assert_eq!(from_repository, vec![included]);
        assert!(ignored.is_empty(), "{ignored:?}");
        assert!(below_ignored.is_empty(), "{below_ignored:?}");
    }

    #[test]
    fn skips_a_gitignored_root_without_a_repository() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let ignored = temp.path().join("generated");
        std::fs::create_dir_all(&ignored).expect("create ignored directory");
        std::fs::write(ignored.join("Ignored.java"), "class Ignored {}").expect("write source");
        std::fs::write(temp.path().join(".gitignore"), "generated\n").expect("write gitignore");

        let files = collect_java_files(&ignored, &IgnoreMatcher::default(), 1)
            .expect("walk should succeed");

        assert!(files.is_empty(), "{files:?}");
    }
}
