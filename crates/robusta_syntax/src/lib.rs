//! Java syntax vocabulary that sits *beneath* the AST.
//!
//! Operators and the semantic index types are facts about Java, not about any
//! one representation of it. Keeping them here means a consumer can name an
//! operator without depending on the tree, and name a symbol without depending
//! on the analyser that populates the symbol table.

use std::num::NonZeroU32;

use robusta_ast_macros::ast;

pub mod identifier;
pub mod line_terminator;
pub mod node;
pub mod operator;
pub mod precedence;
pub mod reference;
pub mod scope;
pub mod symbol;
pub mod unicode_escape;

mod generated {
    mod derive_content_eq;
}

// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_syntax/src/lib.rs
/// Dummy type to communicate the content of `nonmax::NonMaxU32` to `robusta_ast_tools`.
#[ast(foreign = NonMaxU32)]
#[expect(dead_code)]
struct NonMaxU32Alias(NonZeroU32);
