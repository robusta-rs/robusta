use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::{Corpus, SourceFile, TestData, workspace_root};

const MISC_PATH: &str = "misc";

impl TestData {
    /// Load the misc fixtures, then each corpus root.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when a corpus cannot be walked or holds no
    /// selected `.java` file, or a source file cannot be read as UTF-8.
    pub fn load(filter: Option<&str>, roots: &[PathBuf]) -> io::Result<Self> {
        let misc = Corpus {
            name: MISC_PATH.to_string(),
            root: PathBuf::from("tasks/coverage"),
            files: load_misc(filter)?,
            snapshot: true,
        };
        let mut corpora = vec![misc];
        for root in roots {
            let files = load_corpus(root, filter)?;
            corpora.push(Corpus {
                name: corpus_name(root),
                root: root.clone(),
                files,
                snapshot: false,
            });
        }
        Ok(Self { corpora })
    }
}

/// The `misc/pass` and `misc/fail` fixtures, with paths such as
/// `misc/fail/example.java`.
fn load_misc(filter: Option<&str>) -> io::Result<Vec<SourceFile>> {
    let root = workspace_root();
    let mut files = read_java(root, &root.join(MISC_PATH), filter)?;
    let fail = Path::new(MISC_PATH).join("fail");
    for file in &mut files {
        file.should_fail = file.path.starts_with(&fail);
    }
    Ok(files)
}

fn load_corpus(root: &Path, filter: Option<&str>) -> io::Result<Vec<SourceFile>> {
    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not a directory", root.display()),
        ));
    }

    let files = read_java(root, root, filter)?;
    if files.is_empty() {
        let selection = filter.map_or_else(String::new, |value| format!(" matching {value:?}"));
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} contains no .java files{selection}", root.display()),
        ));
    }
    Ok(files)
}

/// The `.java` files under `dir` in path order, with paths relative to `root`.
fn read_java(root: &Path, dir: &Path, filter: Option<&str>) -> io::Result<Vec<SourceFile>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.map_err(|error| walk_error(&error))?;
        if entry.file_type().is_dir() || !is_java(entry.path()) {
            continue;
        }
        let relative = entry.path().strip_prefix(root).map_err(io::Error::other)?;
        if filter.is_none_or(|needle| relative.to_string_lossy().contains(needle)) {
            paths.push(entry.into_path());
        }
    }
    paths.sort();

    paths
        .into_par_iter()
        .map(|path| {
            let source = fs::read_to_string(&path).map_err(|error| {
                io::Error::new(error.kind(), format!("failed to read {}: {error}", path.display()))
            })?;
            let relative = path.strip_prefix(root).map_err(io::Error::other)?.to_path_buf();
            Ok(SourceFile { path: relative, source, should_fail: false })
        })
        .collect()
}

fn corpus_name(root: &Path) -> String {
    root.file_name()
        .filter(|name| !name.is_empty())
        .map_or_else(|| root.display().to_string(), |name| name.to_string_lossy().into_owned())
}

fn is_java(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("java"))
}

fn walk_error(error: &walkdir::Error) -> io::Error {
    let message = error
        .path()
        .map_or_else(|| error.to_string(), |path| format!("{}: {error}", path.display()));
    io::Error::other(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_java_recursively_in_path_order_and_filters() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("nested")).unwrap();
        fs::write(root.path().join("Z.java"), "class Z {}").unwrap();
        fs::write(root.path().join("nested/A.JAVA"), "class A {}").unwrap();
        fs::write(root.path().join("ignored.txt"), "not Java").unwrap();

        let files = load_corpus(root.path(), None).unwrap();
        assert_eq!(
            files.iter().map(|file| file.path.as_path()).collect::<Vec<_>>(),
            [Path::new("Z.java"), Path::new("nested/A.JAVA"),]
        );

        let filtered = load_corpus(root.path(), Some("nested")).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].path, Path::new("nested/A.JAVA"));
    }

    #[test]
    fn marks_misc_fail_fixtures_as_should_fail() {
        let files = load_misc(None).unwrap();

        assert!(files.iter().any(|file| file.should_fail));
        assert!(files.iter().any(|file| !file.should_fail));
        for file in files {
            assert_eq!(file.should_fail, file.path.starts_with("misc/fail"), "{:?}", file.path);
        }
    }

    #[test]
    fn rejects_a_selection_without_java_files() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("README.md"), "empty corpus").unwrap();

        let error = load_corpus(root.path(), None).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(error.to_string().contains("contains no .java files"));
    }
}
