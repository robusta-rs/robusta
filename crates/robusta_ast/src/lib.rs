//! Arena-allocated Java syntax tree with source spans.
//!
//! Comments live on [`CompilationUnit`] in source order and are skipped by
//! visitors. The formatter recovers blank lines from gaps between spans.
//! Node names follow the JLS grammar.

mod ast;
mod ast_impl;
mod ast_kind_impl;
pub mod builder;
mod precedence;
mod span;
mod trivia;

mod generated {
    pub mod ast_kind;

    #[cfg(debug_assertions)]
    mod assert_layouts;
    mod ast_builder;
    mod derive_content_eq;
    mod derive_dummy;
    mod derive_get_address;
    mod derive_get_span;
    mod derive_unstable_address;
    mod get_id;
    mod inherit_variants;
}

pub use ast::*;
pub use ast_kind_impl::TypeDeclarationKind;
pub use generated::ast_kind::{AstKind, AstType};
pub use trivia::{comments_range, has_comments_between};

// Arena teardown skips destructors. Heap-owning fields would leak.
const _: () = assert!(
    !std::mem::needs_drop::<CompilationUnit<'static>>(),
    "AST nodes must not own values that need Drop"
);
