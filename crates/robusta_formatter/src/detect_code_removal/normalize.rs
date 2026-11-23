use std::{fmt::Write as _, ops::Range};

use robusta_lexer::TokenKind;

use super::TokenFingerprint;

/// Reduce a lexed token stream to the sequence formatting must preserve.
///
/// Imports sort first, since a trailing comma is recognized by what follows it.
pub(super) fn normalize_tokens(lexed: &[TokenFingerprint]) -> Vec<TokenFingerprint> {
    normalize_trailing_separators(&normalize_import_regions(lexed))
}

/// Drop a trailing comma, which the formatter may have inserted or omitted.
///
/// Only an enum constant list, an array initializer, and an annotation
/// element-value array allow one (JLS SE 25 §19 `EnumBody`, `ArrayInitializer`,
/// `ElementValueArrayInitializer`), so in parsed source a comma before `}` or
/// the enum body's `;` is always one of these.
fn normalize_trailing_separators(tokens: &[TokenFingerprint]) -> Vec<TokenFingerprint> {
    let mut normalized = Vec::with_capacity(tokens.len());
    for (idx, token) in tokens.iter().enumerate() {
        let is_trailing_separator = token.kind == TokenKind::Comma
            && matches!(
                tokens.get(idx + 1).map(|next| next.kind),
                Some(TokenKind::RBrace | TokenKind::Semicolon)
            );
        if !is_trailing_separator {
            normalized.push(token.clone());
        }
    }
    normalized
}

/// Sort each run of `import` statements.
fn normalize_import_regions(tokens: &[TokenFingerprint]) -> Vec<TokenFingerprint> {
    let mut normalized = Vec::with_capacity(tokens.len());
    let mut idx = 0usize;
    while idx < tokens.len() {
        if tokens[idx].kind != TokenKind::Import {
            normalized.push(tokens[idx].clone());
            idx += 1;
            continue;
        }

        let mut imports: Vec<Range<usize>> = Vec::new();
        while idx < tokens.len() && tokens[idx].kind == TokenKind::Import {
            let start = idx;
            while idx < tokens.len() && tokens[idx].kind != TokenKind::Semicolon {
                idx += 1;
            }
            if idx < tokens.len() {
                idx += 1;
            }
            imports.push(start..idx);
        }
        imports.sort_by_key(|import| import_token_key(&tokens[import.clone()]));

        for import in imports {
            normalized.extend_from_slice(&tokens[import]);
        }
    }
    normalized
}

fn import_token_key(tokens: &[TokenFingerprint]) -> (u8, String) {
    let is_static = tokens.get(1).is_some_and(|token| token.kind == TokenKind::Static);
    let is_module = !is_static
        && tokens.get(1).is_some_and(|token| token.kind == TokenKind::Module)
        && tokens.get(2).is_some_and(|token| token.kind.is_identifier());
    let name_start = if is_static || is_module { 2 } else { 1 };
    let block = if is_static {
        0
    } else if is_module {
        1
    } else {
        2
    };
    let mut imported = String::new();
    for token in
        tokens.iter().skip(name_start).take_while(|token| token.kind != TokenKind::Semicolon)
    {
        match token.kind {
            kind if kind.is_identifier() || kind == TokenKind::Underscore => {
                match &token.spelling {
                    Some(spelling) => imported.push_str(spelling),
                    // A contextual keyword has no spelling. Its kind keeps
                    // `com.open.A` and `com.to.A` apart.
                    None => {
                        let _ = write!(imported, "{:?}", token.kind);
                    }
                }
            }
            TokenKind::Dot => imported.push('.'),
            TokenKind::Star => imported.push('*'),
            _ => {}
        }
    }
    (block, imported)
}
