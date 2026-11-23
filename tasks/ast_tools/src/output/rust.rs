// Adapted from https://github.com/oxc-project/oxc/blob/566b11b2db6b664e787e67e2199d4ffcaa31c30a/tasks/ast_tools/src/output/rust.rs

use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use lazy_regex::{Captures, Lazy, Regex, lazy_regex, regex::Replacer};

use crate::{invalid, workspace_root};

use super::add_header;

/// Format Rust code, and add header.
///
/// # Errors
///
/// Returns an error if `code` does not parse or `rustfmt` fails.
pub fn print_rust(code: &str, generator_path: &str) -> io::Result<String> {
    let file = syn::parse_file(code)
        .map_err(|error| invalid(format!("generated code does not parse: {error}\n{code}")))?;
    let code = prettyplease::unparse(&file);
    let code = COMMENT_REGEX.replace_all(&code, CommentReplacer).to_string();
    rust_fmt(&add_header(&code, generator_path))
}

/// Format Rust code with the workspace's pinned `rustfmt` and settings.
///
/// Does not format on disk - interfaces with `rustfmt` via stdin/stdout.
pub fn rust_fmt(source_text: &str) -> io::Result<String> {
    // `cargo +<toolchain>` sets `RUSTUP_TOOLCHAIN`, which would override the toolchain file.
    let mut rustfmt = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .current_dir(workspace_root())
        .env_remove("RUSTUP_TOOLCHAIN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| io::Error::new(error.kind(), format!("failed to run rustfmt: {error}")))?;

    // Report rustfmt's own error before a write it cut short.
    let written = rustfmt.stdin.take().expect("stdin is piped").write_all(source_text.as_bytes());
    let output = rustfmt.wait_with_output()?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(format!("rustfmt failed: {error}")));
    }
    written?;
    String::from_utf8(output.stdout).map_err(|error| invalid(error.to_string()))
}

/// Replace doc comments which start with `@` with plain comments or line breaks.
///
/// Original comment can be either `///@` or `//!@`.
///
/// * `///@ foo` becomes `// foo`.
/// * `//!@ foo` becomes `// foo`.
/// * `///@@line_break` is removed - i.e. line break.
/// * `//!@@line_break` is removed - i.e. line break.
///
/// Parsing discards plain comments, but these doc attributes survive until printing.
///
/// `//!@` form can be used to insert a line break in a position where `///@ ...`
/// is not valid syntax e.g. before an `#![expect(...)]`.
///
/// To dynamically generate a comment, use `format!("@ NOTE: {} doesn't exist!", name)`
/// as the value of an outer `#[doc = ...]` or inner `#![doc = ...]` attribute.
struct CommentReplacer;

impl Replacer for CommentReplacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
        assert_eq!(caps.len(), 2);
        let body = caps.get(1).unwrap().as_str();
        if body != "@line_break" {
            dst.push_str("//");
            dst.push_str(body);
        }
    }
}

static COMMENT_REGEX: Lazy<Regex> = lazy_regex!(r"[ \t]*//[/!]@(.*)");

#[cfg(test)]
mod tests {
    use super::print_rust;

    #[test]
    fn comment_markers_become_comments_and_line_breaks() {
        let code = r#"
            #![doc = "@ Inner comment"]
            #![doc = "@@line_break"]
            #![allow(dead_code)]
            #[doc = "@ Outer comment"]
            #[doc = "@@line_break"]
            fn generated() {}
        "#;
        let output = print_rust(code, "generator.rs").expect("generated code should format");
        assert_eq!(
            output,
            "// Auto-generated code, DO NOT EDIT DIRECTLY!\n\
             // To edit this generated file you have to edit `generator.rs`.\n\n\
             // Inner comment\n\n\
             #![allow(dead_code)]\n\
             // Outer comment\n\n\
             fn generated() {}\n"
        );
    }

    #[test]
    fn malformed_code_is_an_error() {
        let error = print_rust("fn broken(", "generator.rs").expect_err("code should not parse");
        assert!(error.to_string().contains("does not parse"), "{error}");
    }
}
