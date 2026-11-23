use crate::{CoreFormatOptions, FormatContext};

/// Simple format context useful for testing.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct SimpleFormatContext<'src> {
    options: CoreFormatOptions,
    source_code: &'src str,
}

impl<'src> SimpleFormatContext<'src> {
    #[must_use]
    pub fn with_source_code(mut self, code: &'src str) -> Self {
        self.source_code = code;
        self
    }
}

impl FormatContext for SimpleFormatContext<'_> {
    type Options = CoreFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_code(&self) -> &str {
        self.source_code
    }
}
