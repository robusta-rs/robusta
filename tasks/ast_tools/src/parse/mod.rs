// Directory adapted from https://github.com/oxc-project/oxc/tree/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/parse

//! Create schema of types from Rust source files.
//!
//! Parsing proceeds in 2 phases:
//!
//! 1. Find the structs and enums marked `#[ast]`, which gives each a [`TypeId`].
//! 2. Parse those types and link them to each other, which also creates a [`TypeDef`] for each
//!    primitive, `Option`, `Box`, `Vec` and `Cell` they hold.
//!
//! [`TypeId`]: crate::schema::TypeId
//! [`TypeDef`]: crate::schema::TypeDef

use std::{io, path::Path};

use oxc_index::IndexVec;

use crate::schema::{File, FileId, Schema};

pub mod attr;
mod load;
#[expect(clippy::module_inception)]
mod parse;
mod skeleton;
use load::{load_file, parse_item};
use parse::parse;

/// Create a [`Schema`] from the `#[ast]` types in the files at `file_paths`, in order.
///
/// `file_paths` are relative to `root_path`.
///
/// # Errors
///
/// Returns an error if a file cannot be read, does not parse, or carries a
/// marker this generator does not understand.
pub fn parse_files(file_paths: &[String], root_path: &Path) -> io::Result<Schema> {
    let mut files = IndexVec::with_capacity(file_paths.len());
    let mut sources = Vec::with_capacity(file_paths.len());
    for file_path in file_paths {
        files.push(File::new(file_path));
        sources.push(load_file(&root_path.join(file_path))?);
    }
    parse_loaded_files(files, sources)
}

/// Create a [`Schema`] from the `#[ast]` types in parsed source files, in order, all of them
/// taken to be in `robusta_ast`.
///
/// # Errors
///
/// Returns an error if a type carries a marker this generator does not understand.
#[cfg(test)]
pub fn parse_sources(sources: Vec<syn::File>) -> io::Result<Schema> {
    let files = sources.iter().map(|_| File::new("crates/robusta_ast/src/lib.rs")).collect();
    parse_loaded_files(files, sources)
}

fn parse_loaded_files(
    files: IndexVec<FileId, File>,
    sources: Vec<syn::File>,
) -> io::Result<Schema> {
    let mut skeletons = Vec::new();
    for (file_id, source) in files.indices().zip(sources) {
        for item in source.items {
            skeletons.extend(parse_item(item, file_id)?);
        }
    }
    parse(skeletons, files)
}
