// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/lib.rs`.

use proc_macro2::TokenStream;
use quote::quote;

pub fn get_trait_crate_and_generics(trait_name: &str) -> Option<(TokenStream, TokenStream)> {
    let res = match trait_name {
        "Dummy" => (quote!(::robusta_allocator::Dummy), quote!(< 'static >)),
        "ContentEq" => (quote!(::robusta_span::ContentEq), TokenStream::new()),
        "GetSpan" => (quote!(::robusta_span::GetSpan), TokenStream::new()),
        _ => return None,
    };
    Some(res)
}
