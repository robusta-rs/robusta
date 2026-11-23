/// A Rust source file.
pub struct File {
    /// Crate file is in e.g. `robusta_ast`
    pub krate: String,
    /// Import path excluding crate e.g. `::ast::java::names`
    pub import_path: String,
}

impl File {
    /// Create new [`File`] from a source path.
    ///
    /// # Panics
    ///
    /// Panics if `file_path` is not in the `src` directory of a crate under `crates/`.
    #[must_use]
    pub fn new(file_path: &str) -> Self {
        // Convert file path to crate and import path.
        // `crates/robusta_ast/src/ast/java/names.rs` -> `robusta_ast`, `::ast::java::names`.
        // `crates/robusta_ast/src/ast/java/mod.rs` -> `robusta_ast`, `::ast::java`.
        // `crates/robusta_syntax/src/lib.rs` -> `robusta_syntax`, ``.
        let path = file_path.trim_end_matches(".rs").trim_end_matches("/mod");

        let mut parts = path.split('/');
        assert_eq!(
            parts.next(),
            Some("crates"),
            "Expected path beginning with `crates/`: `{path}`"
        );
        let krate = parts.next().expect("a crate name").to_string();
        assert_eq!(parts.next(), Some("src"));

        let mut import_path = format!("::{}", parts.collect::<Vec<_>>().join("::"));
        if import_path == "::lib" {
            import_path.clear();
        }

        Self { krate, import_path }
    }

    /// Get name of crate this [`File`] is in.
    #[must_use]
    pub fn krate(&self) -> &str {
        &self.krate
    }

    /// Get import path for this [`File`].
    #[must_use]
    pub fn import_path(&self) -> &str {
        &self.import_path
    }
}

#[cfg(test)]
mod test {
    use super::File;

    #[test]
    fn test_file_new() {
        let cases = [
            ("crates/robusta_ast/src/ast/java/names.rs", "robusta_ast", "::ast::java::names"),
            ("crates/robusta_ast/src/ast/java/mod.rs", "robusta_ast", "::ast::java"),
            ("crates/robusta_syntax/src/lib.rs", "robusta_syntax", ""),
        ];

        for (file_path, krate, import_path) in cases {
            let file = File::new(file_path);
            assert_eq!(file.krate(), krate);
            assert_eq!(file.import_path(), import_path);
        }
    }
}
