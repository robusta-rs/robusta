// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_data_structures/src/lib.rs

//! Data structures used across other robusta crates.

#[cfg(feature = "assert_unchecked")]
mod assert_unchecked;

#[cfg(feature = "code_buffer")]
pub mod code_buffer;

#[cfg(feature = "types")]
pub mod types;
