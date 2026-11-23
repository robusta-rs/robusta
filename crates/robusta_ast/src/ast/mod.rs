// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_ast/src/ast/mod.rs

//! AST definitions.
//!
//! # Enum inheritance
//!
//! A choice that includes another choice inherits its variants rather than holding it, through an
//! `INHERIT` marker variant:
//!
//! ```ignore
//! #[ast]
//! pub enum VariableInitializer<'a> {
//!     Array(Box<'a, ArrayInitializer<'a>>) = 64,
//!     INHERIT(Expression<'a>),
//! }
//! ```
//!
//! The `#[ast]` macro replaces the marker with `Expression`'s variants, with their discriminants,
//! so the two choices convert without cost:
//!
//! ```ignore
//! // Creation
//! let init = VariableInitializer::Lambda(lambda);
//! let init = VariableInitializer::from(expression);
//!
//! // Conversion
//! let expression = Expression::try_from(init).ok();
//! let expression = init.as_expression();
//!
//! // Testing
//! if init.is_expression() {}
//! if matches!(init, match_expression!(VariableInitializer)) {}
//!
//! // Branching
//! match init {
//!     VariableInitializer::Array(array) => {}
//!     it @ match_expression!(VariableInitializer) => {
//!         let expression = it.to_expression();
//!     }
//! }
//! ```
//!
//! A choice's own discriminants must leave those of what it inherits free. Each family keeps
//! its own range: `Expression` from 0, `ClassType` and `Type` from 64, `Pattern` from 72, and
//! the type declarations from 32.

pub mod comment;
pub mod java;
pub mod literal;

pub use comment::*;
pub use java::*;
pub use literal::*;
