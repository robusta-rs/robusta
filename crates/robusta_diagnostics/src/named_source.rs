// Adapted from https://github.com/oxc-project/oxc/blob/82c924496c2e1c79e05b29e40377f6b703a2cd6a/crates/oxc_diagnostics/src/named_source.rs

use std::fmt;

use crate::SourceCode;

/// Names a [`SourceCode`] such as [`String`], or overrides its name.
#[derive(Clone)]
pub struct NamedSource<S: SourceCode> {
    source: S,
    name: String,
}

impl<S: SourceCode> fmt::Debug for NamedSource<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedSource")
            .field("name", &self.name)
            .field("source", &"<redacted>")
            .finish()
    }
}

impl<S: SourceCode> NamedSource<S> {
    /// Name a [`SourceCode`].
    #[must_use]
    pub fn new(name: impl AsRef<str>, source: S) -> Self {
        Self { source, name: name.as_ref().to_string() }
    }

    /// The name given to this source.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<S: SourceCode> SourceCode for NamedSource<S> {
    fn data(&self) -> &[u8] {
        self.source.data()
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}
