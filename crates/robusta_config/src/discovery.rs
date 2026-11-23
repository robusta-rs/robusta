use std::path::{Path, PathBuf};

pub const CONFIG_FILE_NAME: &str = "robusta.yaml";

/// Find the nearest `file_name`, starting at a directory input itself or a
/// file input's containing directory and walking through its ancestors.
///
/// `input_path` must already be absolute. Callers normalize their inputs
/// before any lookup so that every spelling of one path resolves to the same
/// ancestors, which is also what lets this walk be a plain
/// [`Path::ancestors`] traversal: `..` components are already gone.
pub fn discover_ancestor_file(input_path: &Path, file_name: &str) -> Option<PathBuf> {
    debug_assert!(input_path.is_absolute(), "discovery inputs are normalized before the walk");
    search_root(input_path)
        .ancestors()
        .map(|directory| directory.join(file_name))
        .find(|candidate| candidate.is_file())
}

/// Find the nearest `robusta.yaml` governing an input path.
pub fn discover_config_path(input_path: &Path) -> Option<PathBuf> {
    discover_ancestor_file(input_path, CONFIG_FILE_NAME)
}

/// The directory the walk starts in. A directory input is searched itself; a
/// file input hands over to its parent. A path that does not exist is treated
/// as a file, which is what makes discovery for a not-yet-created file behave
/// like discovery for its siblings.
fn search_root(input_path: &Path) -> &Path {
    if input_path.is_dir() { input_path } else { input_path.parent().unwrap_or(input_path) }
}

#[cfg(test)]
mod tests {
    use super::{discover_ancestor_file, discover_config_path};

    #[test]
    fn finds_nearest_ancestor_config() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let package = temp.path().join("src/main/java/example");
        std::fs::create_dir_all(&package).expect("create package directory");
        std::fs::write(temp.path().join("robusta.yaml"), "").expect("write root config");
        let nearest = temp.path().join("src/main/robusta.yaml");
        std::fs::write(&nearest, "").expect("write nearest config");

        let discovered = discover_config_path(&package.join("Example.java"));

        assert_eq!(discovered, Some(nearest));
    }

    #[test]
    fn starts_discovery_in_an_input_directory() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let package = temp.path().join("src/main/java/example");
        std::fs::create_dir_all(&package).expect("create package directory");
        let nearest = package.join("robusta.yaml");
        std::fs::write(&nearest, "").expect("write directory config");

        let discovered = discover_config_path(&package);

        assert_eq!(discovered, Some(nearest));
    }

    #[test]
    fn returns_none_when_no_ancestor_has_config() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let input = temp.path().join("src/Example.java");
        std::fs::create_dir_all(input.parent().expect("input has parent"))
            .expect("create input directory");

        assert_eq!(discover_config_path(&input), None);
    }

    #[test]
    fn a_directory_named_like_the_file_is_not_a_hit() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let package = temp.path().join("src");
        std::fs::create_dir_all(package.join("robusta.yaml")).expect("create decoy directory");
        let nearest = temp.path().join("robusta.yaml");
        std::fs::write(&nearest, "").expect("write root config");

        let discovered = discover_config_path(&package.join("Example.java"));

        assert_eq!(discovered, Some(nearest));
    }

    #[test]
    fn any_file_name_walks_the_same_ancestors() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let package = temp.path().join("src/main/java");
        std::fs::create_dir_all(&package).expect("create package directory");
        let nearest = temp.path().join("src/.editorconfig");
        std::fs::write(&nearest, "").expect("write editorconfig");

        let discovered = discover_ancestor_file(&package.join("Example.java"), ".editorconfig");

        assert_eq!(discovered, Some(nearest));
    }
}
