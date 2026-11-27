use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use rustc_hash::FxHashMap;

use crate::gitignore::{add_line, decision, lines};

/// The `.gitignore` and `info/exclude` rules read during one walk.
///
/// A path's rules stop at its nearest repository, or else run up to the
/// filesystem root. Each directory's rules are read once, by whichever thread
/// reaches the directory first. A directory without rules shares its parent's
/// chain.
#[derive(Default)]
pub struct RepositoryRules {
    directories: RwLock<FxHashMap<PathBuf, Option<Arc<DirectoryRules>>>>,
}

struct DirectoryRules {
    gitignore: Option<Gitignore>,
    exclude: Option<Gitignore>,
    parent: Option<Arc<DirectoryRules>>,
}

impl RepositoryRules {
    /// Whether the rules of the directories above `path` exclude it.
    ///
    /// The closest `.gitignore` with an opinion decides, then the closest
    /// `info/exclude`. Excluded ancestors are the caller's to check.
    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        let Some(rules) = path.parent().and_then(|parent| self.directory(parent)) else {
            return false;
        };
        let matched = |matcher: &Option<Gitignore>| {
            matcher.as_ref().and_then(|matcher| decision(matcher, path, is_dir))
        };
        let mut exclude = None;
        let mut current = &*rules;
        loop {
            if let Some(ignored) = matched(&current.gitignore) {
                return ignored;
            }
            exclude = exclude.or_else(|| matched(&current.exclude));
            let Some(parent) = &current.parent else { return exclude == Some(true) };
            current = parent;
        }
    }

    fn directory(&self, directory: &Path) -> Option<Arc<DirectoryRules>> {
        if let Some(rules) = self.directories.read().unwrap().get(directory) {
            return rules.clone();
        }
        let git = directory.join(".git");
        let has_git = git.exists();
        let parent = if has_git || directory.join(".jj").exists() {
            None
        } else {
            directory.parent().and_then(|parent| self.directory(parent))
        };
        let gitignore = load(directory, &directory.join(".gitignore"));
        let exclude = if has_git {
            git_common_dir(&git).and_then(|common| load(directory, &common.join("info/exclude")))
        } else {
            None
        };
        let rules = if gitignore.is_none() && exclude.is_none() {
            parent
        } else {
            Some(Arc::new(DirectoryRules { gitignore, exclude, parent }))
        };
        self.directories.write().unwrap().insert(directory.to_path_buf(), rules.clone());
        rules
    }
}

/// Read a repository's ignore file, where an invalid line matches nothing and
/// an unreadable file has no rules, as in Git.
fn load(root: &Path, file: &Path) -> Option<Gitignore> {
    let contents = std::fs::read(file).ok()?;
    let mut builder = GitignoreBuilder::new(root);
    for line in lines(&String::from_utf8_lossy(&contents)) {
        let _ = add_line(&mut builder, line);
    }
    builder.build().ok().filter(|matcher| !matcher.is_empty())
}

/// The directory that holds `info/exclude` for a `.git` entry: the entry
/// itself or, in a linked worktree or submodule, the one its file names.
fn git_common_dir(git: &Path) -> Option<PathBuf> {
    let Ok(contents) = std::fs::read_to_string(git) else {
        return Some(git.to_path_buf());
    };
    let git_dir = git.parent()?.join(contents.lines().next()?.strip_prefix("gitdir: ")?);
    let common_dir = std::fs::read_to_string(git_dir.join("commondir"));
    Some(common_dir.map_or_else(|_| git_dir.clone(), |common| git_dir.join(common.trim_end())))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::RepositoryRules;

    fn write(path: &Path, contents: &str) {
        std::fs::create_dir_all(path.parent().expect("file has parent")).expect("create directory");
        std::fs::write(path, contents).expect("write file");
    }

    #[test]
    fn the_closest_gitignore_decides_before_the_exclude_file() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let root = temp.path();
        std::fs::create_dir_all(root.join(".git")).expect("create repository marker");
        write(&root.join(".git/info/exclude"), "excluded\nkept\n");
        write(&root.join(".gitignore"), "ignored\n!kept\nnested\n");
        write(&root.join("sub/.gitignore"), "!nested\n");

        let rules = RepositoryRules::default();

        assert!(rules.is_ignored(&root.join("excluded"), false));
        assert!(rules.is_ignored(&root.join("ignored"), false));
        assert!(!rules.is_ignored(&root.join("kept"), false));
        assert!(rules.is_ignored(&root.join("nested"), false));
        assert!(!rules.is_ignored(&root.join("sub/nested"), false));
        assert!(!rules.is_ignored(&root.join("other"), false));
    }

    #[test]
    fn rules_stop_at_a_repository() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let root = temp.path();
        std::fs::create_dir_all(root.join("repository/.jj")).expect("create repository marker");
        write(&root.join(".gitignore"), "outer\n");

        let rules = RepositoryRules::default();

        assert!(rules.is_ignored(&root.join("outer"), false));
        assert!(!rules.is_ignored(&root.join("repository/outer"), false));
    }

    #[test]
    fn a_git_file_names_the_directory_of_its_exclude_file() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let root = temp.path();
        write(&root.join("common/info/exclude"), "shared\n");
        write(&root.join("common/worktrees/linked/commondir"), "../..\n");
        write(&root.join("linked/.git"), "gitdir: ../common/worktrees/linked\n");
        write(&root.join("modules/one/info/exclude"), "own\n");
        write(&root.join("submodule/.git"), "gitdir: ../modules/one\n");

        let rules = RepositoryRules::default();

        assert!(rules.is_ignored(&root.join("linked/shared"), false));
        assert!(rules.is_ignored(&root.join("submodule/own"), false));
        assert!(!rules.is_ignored(&root.join("submodule/shared"), false));
    }

    #[test]
    fn an_invalid_line_in_a_repository_file_matches_nothing() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let root = temp.path();
        std::fs::create_dir_all(root.join(".git")).expect("create repository marker");
        write(&root.join(".gitignore"), "\u{feff}first\nOpen[.java\nend\\\n[[:word:]]\nlast\r\n");

        let rules = RepositoryRules::default();

        assert!(rules.is_ignored(&root.join("first"), false));
        assert!(rules.is_ignored(&root.join("last"), false));
        assert!(!rules.is_ignored(&root.join("Open[.java"), false));
    }
}
