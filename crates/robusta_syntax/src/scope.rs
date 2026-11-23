use oxc_index::define_nonmax_u32_index_type;
use robusta_ast_macros::ast;

define_nonmax_u32_index_type! {
    /// Stable index of one lexical scope in a compilation unit's scope tree.
    #[ast]
    #[content_eq(skip)]
    pub struct ScopeId;
}

impl ScopeId {
    pub const ROOT: Self = Self::new(0);
}
