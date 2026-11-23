// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_str/src/lib.rs

//! String types for robusta.
//!
//! This crate provides [`Ident`], a UTF-8 identifier with a precomputed hash,
//! the [`IdentBuildHasher`] that maps keyed by it use, and the
//! [`ArenaIdentHashMap`] built on both.

mod ident;
mod ident_hasher;

pub use ident::{ArenaIdentHashMap, Ident};
pub use ident_hasher::{IdentBuildHasher, IdentHasher};
