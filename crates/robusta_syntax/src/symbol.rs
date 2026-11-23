use oxc_index::define_nonmax_u32_index_type;
use robusta_ast_macros::ast;

define_nonmax_u32_index_type! {
    /// Stable index of one declared symbol in a compilation unit's symbol table.
    #[ast]
    #[content_eq(skip)]
    pub struct SymbolId;
}
