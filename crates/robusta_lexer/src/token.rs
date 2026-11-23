use robusta_ast::Comment;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenKind {
    // Names & literals
    Identifier,
    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,

    // Hard keywords
    Underscore,
    Package,
    Import,
    Class,
    Interface,
    Enum,
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Void,
    Const,
    Goto,
    Public,
    Protected,
    Private,
    Static,
    Final,
    Abstract,
    Native,
    Strictfp,
    Transient,
    Volatile,
    Extends,
    Implements,
    New,
    Super,
    Instanceof,
    Return,
    Throw,
    Assert,
    Try,
    Catch,
    Finally,
    Synchronized,
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    For,
    Break,
    Continue,
    Throws,
    This,
    True,
    False,
    Null,

    // Contextual keywords (JLS §3.9). Ordered as one contiguous block, and kept
    // that way: [`TokenKind::is_contextual_keyword`] is a range check over this
    // span, and scattering these variants turns it into a jump table at every
    // identifier-admission site.
    Exports,
    Module,
    /// Not a contextual keyword on its own — `non-sealed` is the keyword, and
    /// its hyphen makes it three tokens. Classifying `non` here lets the parser
    /// assemble that spelling from token kinds instead of source text.
    Non,
    Open,
    Opens,
    Permits,
    Provides,
    Record,
    Requires,
    Sealed,
    To,
    Transitive,
    Uses,
    Var,
    When,
    With,
    Yield,

    // Punctuation
    LBrace,     // {
    RBrace,     // }
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    Ellipsis,   // ...
    Question,   // ?
    Colon,      // :
    ColonColon, // ::
    At,         // @

    // Operators
    Assign,     // =
    Plus,       // +
    Minus,      // -
    PlusPlus,   // ++
    MinusMinus, // --
    Arrow,      // ->
    Star,       // *
    Slash,      // /
    Percent,    // %
    Bang,       // !
    Tilde,      // ~
    Amp,        // &
    AmpAmp,     // &&
    Pipe,       // |
    PipePipe,   // ||
    Caret,      // ^
    EqEq,       // ==
    BangEq,     // !=
    Lt,         // <
    Gt,         // > (never merged; the parser assembles `>>`-family operators)
    LtEq,       // <=
    GtEq,       // >=
    LtLt,       // <<
    LtLtEq,     // <<=
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    AmpEq,      // &=
    PipeEq,     // |=
    CaretEq,    // ^=

    /// Recovery placeholder for an unexpected character.
    Unknown,
    /// End of input.
    Eof,
}

impl TokenKind {
    /// Whether the token is one of the contextual keywords of JLS §3.9 — a word
    /// the lexer classifies, but which still stands as an ordinary identifier
    /// everywhere the specification does not restrict it.
    ///
    /// A range check over the contiguous block in the declaration above.
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u8) >= (Self::Exports as u8) && (self as u8) <= (Self::Yield as u8)
    }

    /// Whether the token may stand where the grammar expects an identifier.
    /// Every contextual keyword may: `int record = 0;` and `void yield() {}`
    /// are ordinary Java. `_` is excluded — Java 25 makes it a hard keyword,
    /// admitted only in the unnamed-variable binders that spell it out.
    #[inline]
    pub const fn is_identifier(self) -> bool {
        matches!(self, Self::Identifier) || self.is_contextual_keyword()
    }

    /// Whether the token is a literal (JLS §3.10), including the boolean and
    /// null literals the lexer groups with hard keywords.
    #[inline]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::IntLiteral
                | Self::FloatLiteral
                | Self::CharLiteral
                | Self::StringLiteral
                | Self::True
                | Self::False
                | Self::Null
        )
    }

    /// Whether the token is one of Java's eight primitive type keywords.
    /// `void` is not a type in the JLS type grammar.
    #[inline]
    pub const fn is_primitive_type(self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::Byte
                | Self::Char
                | Self::Double
                | Self::Float
                | Self::Int
                | Self::Long
                | Self::Short
        )
    }

    /// Whether the token can denote a keyword modifier in some Java grammar
    /// context. Contextual modifiers such as `sealed` remain identifiers.
    #[inline]
    pub const fn is_modifier_kind(self) -> bool {
        matches!(
            self,
            Self::Public
                | Self::Protected
                | Self::Private
                | Self::Abstract
                | Self::Default
                | Self::Static
                | Self::Final
                | Self::Native
                | Self::Strictfp
                | Self::Synchronized
                | Self::Transient
                | Self::Volatile
        )
    }

    /// Whether this single token is a complete assignment operator. The lexer
    /// leaves `>>=` and `>>>=` split for the parser to assemble contextually.
    #[inline]
    pub const fn is_assignment_operator(self) -> bool {
        matches!(
            self,
            Self::Assign
                | Self::PlusEq
                | Self::MinusEq
                | Self::StarEq
                | Self::SlashEq
                | Self::PercentEq
                | Self::AmpEq
                | Self::PipeEq
                | Self::CaretEq
                | Self::LtLtEq
        )
    }

    /// Whether the token participates in the binary-expression grammar. The
    /// lexer leaves `>>` and `>>>` split for the parser to assemble.
    #[inline]
    pub const fn is_binary_operator(self) -> bool {
        matches!(
            self,
            Self::PipePipe
                | Self::AmpAmp
                | Self::Pipe
                | Self::Caret
                | Self::Amp
                | Self::EqEq
                | Self::BangEq
                | Self::Lt
                | Self::Gt
                | Self::LtEq
                | Self::GtEq
                | Self::LtLt
                | Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Instanceof
        )
    }

    /// Whether the token is an increment or decrement operator.
    #[inline]
    pub const fn is_update_operator(self) -> bool {
        matches!(self, Self::PlusPlus | Self::MinusMinus)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct LexResult {
    /// The significant tokens, always terminated by [`TokenKind::Eof`].
    pub tokens: Vec<Token>,
    /// Identifier binding names changed by escape translation or ignorable removal.
    /// Keyed by the original token start.
    /// Ordinary identifiers borrow source text and need no entry here.
    pub identifier_names: rustc_hash::FxHashMap<u32, String>,
    /// The trivia side-table, in source order.
    pub comments: Vec<Comment>,
    /// Non-fatal lexical errors (unterminated string/comment, stray character).
    pub errors: Vec<RobustaDiagnostic>,
}

#[cfg(test)]
mod tests {
    use super::TokenKind::{self, *};

    #[test]
    fn token_kinds_classify_java_grammar_families() {
        assert!(
            [Boolean, Byte, Char, Double, Float, Int, Long, Short]
                .into_iter()
                .all(TokenKind::is_primitive_type)
        );
        assert!(!Void.is_primitive_type());

        assert!(
            [
                Public,
                Protected,
                Private,
                Abstract,
                Default,
                Static,
                Final,
                Native,
                Strictfp,
                Synchronized,
                Transient,
                Volatile,
            ]
            .into_iter()
            .all(TokenKind::is_modifier_kind)
        );
        assert!(!Class.is_modifier_kind());

        assert!(
            [Assign, PlusEq, MinusEq, StarEq, SlashEq, PercentEq, AmpEq, PipeEq, CaretEq, LtLtEq]
                .into_iter()
                .all(TokenKind::is_assignment_operator)
        );
        assert!(!Gt.is_assignment_operator());

        assert!(
            [
                PipePipe, AmpAmp, Pipe, Caret, Amp, EqEq, BangEq, Lt, Gt, LtEq, GtEq, LtLt, Plus,
                Minus, Star, Slash, Percent, Instanceof,
            ]
            .into_iter()
            .all(TokenKind::is_binary_operator)
        );
        assert!(!Assign.is_binary_operator());

        assert!([PlusPlus, MinusMinus].into_iter().all(TokenKind::is_update_operator));
        assert!(!Plus.is_update_operator());
    }

    #[test]
    fn contextual_keywords_are_one_contiguous_block() {
        let block = [
            Exports, Module, Non, Open, Opens, Permits, Provides, Record, Requires, Sealed, To,
            Transitive, Uses, Var, When, With, Yield,
        ];
        for (offset, kind) in block.iter().enumerate() {
            let expected = Exports as u8 + u8::try_from(offset).unwrap();
            assert_eq!(*kind as u8, expected, "{kind:?} is out of the contiguous block");
            assert!(kind.is_contextual_keyword(), "{kind:?}");
            assert!(kind.is_identifier(), "{kind:?}");
        }
        // Nothing outside the block may answer yes.
        assert!(!Identifier.is_contextual_keyword());
        assert!(!Class.is_contextual_keyword());
        assert!(!Null.is_contextual_keyword());
        assert!(!LBrace.is_contextual_keyword());
        assert!(!Class.is_identifier(), "a reserved word may not name things");
        assert!(!Underscore.is_identifier(), "`_` is admitted only by its own binders");
    }
}
