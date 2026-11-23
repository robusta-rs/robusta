// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/utils.rs

use phf_codegen::Map as PhfMapGen;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, LitInt, parse_str};

/// Convert integer to [`LitInt`].
///
/// This prints without a `usize` postfix. i.e. `123` not `123usize`.
///
/// [`LitInt`]: struct@LitInt
pub fn number_lit<N: Into<u64>>(n: N) -> LitInt {
    LitInt::new(&n.into().to_string(), Span::call_site())
}

/// Convert type name to snake case.
///
/// e.g. `ExpressionStatement` -> `expression_statement`.
#[must_use]
pub fn snake_case(name: &str) -> String {
    let mut output = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && index > 0 {
            output.push('_');
        }
        output.extend(ch.to_lowercase());
    }
    output
}

/// Get the plural of a type name.
///
/// e.g. `CatchClause` -> `CatchClauses`.
#[must_use]
pub fn pluralize(name: &str) -> String {
    match name.as_bytes().last() {
        Some(b's') => format!("{name}es"),
        Some(b'y') => format!("{}ies", &name[..name.len() - 1]),
        _ => format!("{name}s"),
    }
}

/// Get the indefinite article for a type name, by the sound its first letter makes.
#[must_use]
pub fn article_for(word: &str) -> &'static str {
    match word.as_bytes().first().map(u8::to_ascii_uppercase) {
        Some(b'A' | b'E' | b'I' | b'O') => "an",
        _ => "a",
    }
}

/// Generate a `phf::Map` from string keys to values.
pub fn generate_phf_map<'e>(entries: impl Iterator<Item = (&'e str, TokenStream)>) -> TokenStream {
    let mut map = PhfMapGen::new();

    for (entry_name, tokens) in entries {
        map.entry(entry_name, tokens.to_string());
    }

    let map =
        parse_str::<Expr>(&map.build().to_string()).expect("phf_codegen prints an expression");
    quote!( #map )
}
