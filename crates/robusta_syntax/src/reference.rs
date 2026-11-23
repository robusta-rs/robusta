use oxc_index::define_nonmax_u32_index_type;
use robusta_ast_macros::ast;

define_nonmax_u32_index_type! {
    /// Stable index of one resolved name reference in a compilation unit.
    #[ast]
    #[content_eq(skip)]
    pub struct ReferenceId;
}
