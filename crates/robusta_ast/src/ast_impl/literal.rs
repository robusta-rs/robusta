use crate::ast::*;

impl StringLiteral<'_> {
    /// Whether this is written as a text block (JLS §3.10.6).
    pub fn is_text_block(&self) -> bool {
        self.raw.starts_with("\"\"\"")
    }
}
