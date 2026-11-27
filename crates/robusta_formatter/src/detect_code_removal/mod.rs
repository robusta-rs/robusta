mod documentation;
mod normalize;

use robusta_lexer::{TokenKind, tokenize};

use documentation::{
    Documented, declaration_starts, documented_declarations, is_before_declaration,
};
use normalize::normalize_tokens;

use crate::print::text_block::{text_before_closing, text_block_lines};

/// One lexed revision, with imports sorted and trailing commas dropped.
#[derive(Debug)]
struct SourceSnapshot {
    tokens: Vec<TokenFingerprint>,
    block_comments: usize,
    /// Every line comment but a `///` comment before a declaration, in source order.
    line_comments: Vec<String>,
    documented: Vec<Documented>,
}

impl SourceSnapshot {
    /// Build a verification snapshot with one lexer pass, and a parse when the
    /// source has a documentation comment.
    fn new(source: &str) -> Self {
        let lexed = tokenize(source);
        let tokens: Vec<TokenFingerprint> = lexed
            .tokens
            .iter()
            .map(|token| TokenFingerprint {
                kind: token.kind,
                spelling: preserves_source_spelling(token.kind)
                    .then(|| spelling(token.kind, token.span.source_text(source))),
            })
            .collect();
        let starts = declaration_starts(source, &lexed.comments);
        Self {
            tokens: normalize_tokens(&tokens),
            block_comments: lexed.comments.iter().filter(|comment| comment.is_block()).count(),
            line_comments: lexed
                .comments
                .iter()
                .filter(|comment| comment.is_line() && !is_before_declaration(comment, &starts))
                .map(|comment| comment.span.source_text(source).to_string())
                .collect(),
            documented: documented_declarations(source, &lexed.comments, &starts),
        }
    }

    /// Compare significant tokens.
    fn compare_tokens(&self, after: &Self) -> Result<(), String> {
        if self.tokens == after.tokens {
            Ok(())
        } else {
            Err(token_mismatch_message(&self.tokens, &after.tokens))
        }
    }

    // Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_formatter/src/detect_code_removal/mod.rs
    /// Compare block comments by count and line comments by text, and which
    /// declaration each documentation comment documents.
    ///
    /// A line comment may lose trailing whitespace, or merge into another:
    /// `a // b\n; // c` prints `a; // b // c`. The text of a documentation comment
    /// before a declaration is not compared, since Javadoc formatting rewrites it.
    fn compare_comments(&self, after: &Self) -> Result<(), String> {
        if self.block_comments != after.block_comments {
            return Err(format!(
                "BlockComment count mismatch: {} -> {}",
                self.block_comments, after.block_comments
            ));
        }
        if self.line_comments != after.line_comments {
            for line in &self.line_comments {
                let line = line.trim_end();
                if !after.line_comments.iter().any(|comment| comment.contains(line)) {
                    return Err(format!("LineComment mismatch: `{line}` not found"));
                }
            }
        }
        if self.documented != after.documented {
            return Err(documented_mismatch_message(&self.documented, &after.documented));
        }
        Ok(())
    }
}

/// Check that formatting kept every significant token and comment, and what
/// each documentation comment documents.
///
/// # Errors
///
/// Returns the first difference.
pub fn detect_code_removal(input: &str, output: &str) -> Result<(), String> {
    let before = SourceSnapshot::new(input);
    let after = SourceSnapshot::new(output);
    before.compare_tokens(&after)?;
    before.compare_comments(&after)
}

/// A token's kind, and its spelling where formatting must not change it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenFingerprint {
    kind: TokenKind,
    spelling: Option<String>,
}

fn token_mismatch_message(before: &[TokenFingerprint], after: &[TokenFingerprint]) -> String {
    let first = before.iter().zip(after.iter()).position(|(left, right)| left != right);
    match first {
        Some(index) => {
            let before = &before[index];
            let after = &after[index];
            if before.kind == after.kind {
                format!(
                    "token spelling changed at index {index}: {:?} input {:?}, output {:?}",
                    before.kind, before.spelling, after.spelling
                )
            } else {
                format!(
                    "token kind changed at index {index}: input {:?}, output {:?}",
                    before.kind, after.kind
                )
            }
        }
        None => format!(
            "token stream changed length: input has {} tokens, output has {} tokens",
            before.len(),
            after.len()
        ),
    }
}

fn documented_mismatch_message(before: &[Documented], after: &[Documented]) -> String {
    let first = before
        .iter()
        .zip(after)
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| before.len().min(after.len()));
    format!(
        "documentation comment {first} changed: input {:?}, output {:?}",
        before.get(first),
        after.get(first)
    )
}

/// Whether formatting must keep the token's spelling, not just its kind.
///
/// Keywords print canonically, contextual ones included: `record` -> `record`.
fn preserves_source_spelling(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Identifier | TokenKind::Underscore) || kind.is_literal()
}

/// The spelling formatting must keep: a text block's without the indentation
/// the compiler removes, which moves with the text block, and with a closing
/// `"""` after text on that text's line, from which it may move to the next
/// line after a `\`.
fn spelling(kind: TokenKind, source: &str) -> String {
    let is_text_block = kind == TokenKind::StringLiteral && source.starts_with(r#"""""#);
    let Some(lines) = text_block_lines(source).filter(|_| is_text_block) else {
        return source.to_string();
    };
    let mut lines = lines.collect::<Vec<_>>();
    let closing = lines.pop().expect("a text block has a closing line");
    if let Some(text) = text_before_closing(closing) {
        lines.push(text);
    } else {
        let last = lines.pop().expect("a text block has an opening line");
        match strip_line_continuation(last) {
            Some(text) => lines.push(text),
            None => lines.extend([last, ""]),
        }
    }
    lines.join("\n") + r#"""""#
}

/// `line` without the `\` that ends it and the line break after it (JLS
/// §3.10.7), or `None` when it does not end with one.
fn strip_line_continuation(line: &str) -> Option<&str> {
    let text = line.strip_suffix('\\')?;
    let escaped_backslashes = text.len() - text.trim_end_matches('\\').len();
    (escaped_backslashes % 2 == 0).then_some(text)
}

#[cfg(test)]
mod tests;
