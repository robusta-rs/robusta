// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/output/mod.rs

use std::{fs, io, path::Path};

use cow_utils::CowUtils;
use proc_macro2::TokenStream;

mod rust;

pub use rust::print_rust;
use rust::rust_fmt;

/// Get path for an output.
#[must_use]
pub fn output_path(krate: &str, path: &str) -> String {
    format!("{krate}/src/generated/{path}")
}

/// Add a generated file warning to top of file.
fn add_header(code: &str, generator_path: &str) -> String {
    format!(
        "// Auto-generated code, DO NOT EDIT DIRECTLY!\n\
        // To edit this generated file you have to edit `{generator_path}`.\n\n\
        {code}"
    )
}

/// An output from codegen: Rust source yet to be printed and formatted.
pub struct Output {
    pub path: String,
    pub code: String,
}

impl Output {
    /// Create an [`Output`] from Rust tokens.
    #[must_use]
    pub fn from_tokens(path: String, tokens: &TokenStream) -> Self {
        Self { path, code: tokens.to_string() }
    }

    /// Convert [`Output`] to [`RawOutput`], which prints and formats the output.
    ///
    /// # Errors
    ///
    /// Returns an error if the output does not parse or cannot be formatted.
    pub fn into_raw(self, generator_path: &str) -> io::Result<RawOutput> {
        let generator_path = generator_path.cow_replace('\\', "/");
        let content = print_rust(&self.code, &generator_path)?;
        Ok(RawOutput { path: self.path, content })
    }
}

/// A raw output from codegen: formatted, ready to write to file.
pub struct RawOutput {
    pub path: String,
    pub content: String,
}

impl RawOutput {
    /// Create a [`RawOutput`] from a whole Rust source file, which has no header.
    ///
    /// # Errors
    ///
    /// Returns an error if the source cannot be formatted.
    pub fn from_rust_string(path: String, code: &str) -> io::Result<Self> {
        Ok(Self { path, content: rust_fmt(code)? })
    }

    /// Write [`RawOutput`] to file under `root_path`.
    ///
    /// If contents haven't changed, the file isn't touched, so it does not trigger a rebuild.
    ///
    /// # Errors
    ///
    /// Returns an error if the file or its directory cannot be written.
    pub fn write_to_file(&self, root_path: &Path) -> io::Result<()> {
        let path = root_path.join(&self.path);
        if fs::read_to_string(&path).is_ok_and(|existing| existing == self.content) {
            return Ok(());
        }
        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path)?;
        }
        fs::write(path, &self.content)
    }
}
