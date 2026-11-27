//! Java grammar productions

mod grammar;

mod annotations;
mod class;
mod declaration;
mod expression;
mod lambda;
mod method;
mod module;
mod operator;
mod statement;
mod types;

// Copied from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/js/mod.rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VariableDeclarationParent {
    For,
    Statement,
}
