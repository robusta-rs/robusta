//! The [`GraphicalReportHandler`] type and its builder API.
//!
//! This module holds the handler's theme and terminal width. The actual
//! rendering lives in the sibling modules (`report`, `snippet`, …).

use crate::GraphicalTheme;

#[derive(Debug, Clone)]
pub struct GraphicalReportHandler {
    /// Terminal width to wrap at.
    pub(super) termwidth: usize,
    /// How to style reports.
    pub(super) theme: GraphicalTheme,
}

impl GraphicalReportHandler {
    /// Create a new `GraphicalReportHandler` with a given [`GraphicalTheme`].
    #[must_use]
    pub fn new_themed(theme: GraphicalTheme) -> Self {
        Self { termwidth: 200, theme }
    }

    /// Sets the width to wrap the report at.
    #[must_use]
    pub fn with_width(mut self, width: usize) -> Self {
        self.termwidth = width;
        self
    }
}
