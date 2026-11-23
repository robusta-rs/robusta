// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_ast_macros/src/lib.rs

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Item, parse_macro_input};

mod ast;
mod generated {
    pub mod derived_traits;
    pub mod enums;
    pub mod structs;
}
mod variant;

/// This attribute serves two purposes:
///
/// Firstly, it is a marker for `robusta_ast_tools`, to understand that a type is part of the AST.
/// `#[ast(visit)]` marks a node or choice, which the walk visits.
///
/// Secondly, it adds the following code around the type:
///
/// ### `#[repr(C)]`
///
/// `#[repr]` attribute is added to the type, to make the memory layout of the type predictable:
///
/// * Structs: `#[repr(C)]`. e.g. `#[repr(C)] struct S { x: X, y: Y }`
/// * Fieldless enums: `#[repr(u8)]` e.g. `#[repr(u8)] enum E { X, Y, Z, }`.
/// * Fieldful enums: `#[repr(C, u8)]` e.g. `#[repr(C, u8)] enum E { X(X), Y(Y) }`
///
/// A struct's fields are re-ordered in memory, in the order `robusta_ast_tools` calculates.
///
/// ### Inherited variants
///
/// An `INHERIT(Choice<'a>)` marker variant in an enum is replaced by the variants of `Choice`,
/// with the same discriminants, so converting between the two enums costs nothing.
///
/// ### Derive `Ast` trait
///
/// `#[derive(robusta_ast_macros::Ast)]` is added to the type.
///
/// `Ast` derive macro is a no-op (see below) but allows custom attributes on AST types.
///
/// ### Trait assertions
///
/// `robusta_ast_tools` generates code for trait impls where those traits are specified with
/// `#[generate_derive(SomeTrait)]`. This is similar to `#[derive(SomeTrait)]`, but the code is
/// generated ahead-of-time, rather than in a proc macro.
///
/// Add assertions that traits used in `#[generate_derive(...)]` are in scope.
#[proc_macro_attribute]
pub fn ast(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Item);
    let expanded = ast::ast(&mut input, TokenStream2::from(args));
    TokenStream::from(expanded)
}

/// Dummy derive macro for a non-existent trait `Ast`.
///
/// Does not generate any code.
/// Its only purpose is to allow the occurrence of helper attributes used in `tasks/ast_tools`.
///
/// See [`macro@ast`] for further details.
#[proc_macro_derive(Ast, attributes(content_eq, generate_derive, plural, scope, visit))]
pub fn ast_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
