use memchr::memmem::Finder;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_span::Span;
use robusta_syntax::{
    identifier::{is_ascii_ident_start, is_horizontal_whitespace, is_identifier_start},
    line_terminator::is_line_terminator,
};

mod comment;
mod diagnostics;
mod identifier;
mod number;
mod punctuation;
mod search;
mod string;
mod token;
mod trivia_builder;
mod unicode;
mod whitespace;

pub use token::{LexResult, Token, TokenKind};
use trivia_builder::TriviaBuilder;
pub use unicode::{Translated, translate};

// Adapted from https://github.com/oxc-project/oxc/blob/83b64d85084d774a02e373d433d9181d536b5567/crates/oxc_parser/src/lib.rs
/// Maximum length of source which can be tokenized (in bytes).
/// ~4 GiB on 64-bit systems, ~2 GiB on 32-bit systems.
// Length is constrained by 2 factors:
// 1. `Span`'s `start` and `end` are `u32`s, which limits length to `u32::MAX` bytes.
// 2. Rust's allocator APIs limit allocations to `isize::MAX`.
pub const MAX_LEN: usize = if size_of::<usize>() >= 8 {
    // 64-bit systems
    u32::MAX as usize
} else {
    // 32-bit or 16-bit systems
    isize::MAX as usize
};

/// Do not extrapolate the corpus token-density estimate to unusually large,
/// sparse files. The token vector still grows normally beyond this reserve.
const TOKEN_RESERVE_SOURCE_LIMIT: usize = 1024 * 1024;

/// The UTF-8 byte-order mark accepted only at the start of a source file.
const UTF8_BOM: &str = "\u{feff}";

/// Convert a byte offset into span coordinates.
///
/// All offsets index into a source (or its cooked Unicode translation, which
/// is never longer) already admitted by the `MAX_LEN` check in [`tokenize`].
#[inline]
pub(crate) fn offset32(offset: usize) -> u32 {
    // Cannot overflow `u32` because of the `MAX_LEN` check in `tokenize`.
    #[expect(clippy::cast_possible_truncation)]
    let offset = offset as u32;
    offset
}

/// Build a span from byte offsets admitted by the `MAX_LEN` check.
#[inline]
pub(crate) fn span32(start: usize, end: usize) -> Span {
    Span::new(offset32(start), offset32(end))
}

/// Tokenize `source` into significant tokens plus a trivia side-table.
///
/// Unicode escapes are translated first (JLS §3.3); the byte-indexed lexer then
/// runs over the cooked text, and every span is mapped back to original-source
/// coordinates so downstream spellings stay verbatim.
///
/// A leading UTF-8 byte-order mark is skipped as an encoding marker. Token and
/// diagnostic spans still index the original source, including its three bytes.
///
/// A source longer than [`MAX_LEN`] is not tokenized at all: the result carries
/// only an end-of-file token and a fatal diagnostic, so span coordinates can
/// never overflow their `u32` fields.
pub fn tokenize(source: &str) -> LexResult {
    if source.len() > MAX_LEN {
        return overlong_source();
    }
    let initial_offset = if source.starts_with(UTF8_BOM) { UTF8_BOM.len() } else { 0 };
    let translated = unicode::translate(source);
    let mut result = Lexer::new(&translated.cooked, initial_offset).run();
    if translated.has_span_map() {
        let mut spans = translated.ordered_span_mapper();
        let mut identifier_names = std::mem::take(&mut result.identifier_names);
        for token in &mut result.tokens {
            let cooked = token.span;
            token.span = spans.map_span(cooked);
            // Cooked names use original offsets, including after preceding escapes.
            if token.kind.is_identifier() {
                let name = identifier_names.remove(&cooked.start).or_else(|| {
                    (cooked.len() != token.span.len())
                        .then(|| cooked.source_text(&translated.cooked).to_owned())
                });
                if let Some(name) = name {
                    result.identifier_names.insert(token.span.start, name);
                }
            }
        }
        translated.map_spans_in_order(result.comments.iter_mut().map(|comment| &mut comment.span));
        for comment in &mut result.comments {
            comment.attached_to = translated.map_offset(comment.attached_to);
        }
        for error in &mut result.errors {
            error.map_spans(|span| translated.map_span(span));
        }
    }
    // Translation errors already carry original spans. Merge and re-sort so the
    // combined list stays in source order.
    if !translated.errors.is_empty() {
        result.errors.extend(translated.errors);
        result.errors.sort_by_key(|error| error.primary_span().start);
    }
    result
}

#[cold]
fn overlong_source() -> LexResult {
    LexResult {
        tokens: vec![Token { kind: TokenKind::Eof, span: Span::empty(0) }],
        identifier_names: rustc_hash::FxHashMap::default(),
        comments: Vec::new(),
        errors: vec![diagnostics::source_too_long()],
    }
}

struct Lexer<'a> {
    text: &'a str,
    src: &'a [u8],
    pos: usize,
    tokens: Vec<Token>,
    identifier_names: rustc_hash::FxHashMap<u32, String>,
    trivia_builder: TriviaBuilder,
    errors: Vec<RobustaDiagnostic>,
    multi_line_comment_end_finder: Option<Finder<'static>>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str, initial_offset: usize) -> Self {
        debug_assert!(source.is_char_boundary(initial_offset));
        Self {
            text: source,
            src: source.as_bytes(),
            pos: initial_offset,
            // Corpus Java source averages one token per 7.5 bytes. Reserve
            // conservatively to reduce geometric growth, capped for large
            // sparse files.
            tokens: Vec::with_capacity(source.len().min(TOKEN_RESERVE_SOURCE_LIMIT) / 6 + 1),
            identifier_names: rustc_hash::FxHashMap::default(),
            trivia_builder: TriviaBuilder::new(),
            errors: Vec::new(),
            multi_line_comment_end_finder: None,
        }
    }

    fn run(mut self) -> LexResult {
        loop {
            self.skip_trivia();
            let start = self.pos;
            let Some(c) = self.peek() else {
                self.push_token(TokenKind::Eof, start);
                break;
            };
            let kind = if is_ascii_ident_start(c)
                || (!c.is_ascii() && is_identifier_start(self.peek_char().unwrap()))
            {
                self.scan_ident_or_keyword()
            } else if c.is_ascii_digit() {
                self.scan_number()
            } else if c == b'.' && matches!(self.peek2(), Some(b'0'..=b'9')) {
                self.scan_dot_decimal_float()
            } else if c == b'\'' {
                self.scan_char()
            } else if c == b'"' {
                self.scan_string()
            } else {
                self.scan_punct(c)
            };
            self.push_token(kind, start);
        }
        LexResult {
            tokens: self.tokens,
            identifier_names: self.identifier_names,
            comments: self.trivia_builder.comments,
            errors: self.errors,
        }
    }

    #[inline]
    fn push_token(&mut self, kind: TokenKind, start: usize) {
        let token = Token { kind, span: span32(start, self.pos) };
        self.tokens.push(token);
        self.trivia_builder.handle_token(token);
    }

    #[inline]
    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    #[inline]
    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(byte) if is_line_terminator(byte) => self.line_break_handler(),
                Some(byte) if is_horizontal_whitespace(byte) => self.pos += 1,
                // JLS §3.5 ignores SUB only at the end of the escaped input stream.
                Some(b'\x1a') if self.pos + 1 == self.src.len() => self.pos += 1,
                Some(b'/') if self.peek2() == Some(b'/') => self.skip_single_line_comment(),
                Some(b'/') if self.peek2() == Some(b'*') => self.skip_multi_line_comment(),
                _ => break,
            }
        }
    }

    fn bump_char(&mut self) {
        let ch = self.peek_char().unwrap();
        self.pos += ch.len_utf8();
    }

    fn peek_char(&self) -> Option<char> {
        self.text[self.pos..].chars().next()
    }
}
