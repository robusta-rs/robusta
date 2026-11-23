//! Test infrastructure shared by formatter crates.
//!
//! A formatter crate uses `harness` from its `tests/fixtures/mod.rs` through
//! `[dev-dependencies]`. This crate never depends on a language crate, so the
//! language crate's tests can depend on it without a cycle.

mod harness;

pub use harness::{FixtureFormatter, fixture_output, stable_output};
