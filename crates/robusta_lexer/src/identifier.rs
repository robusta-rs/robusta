use robusta_syntax::identifier::{
    IdentifierPart, identifier_part, is_ascii_ident_continue, is_ascii_ident_ignorable,
    remove_ignorable_characters,
};

use crate::{Lexer, TokenKind, offset32};

impl Lexer<'_> {
    pub(super) fn scan_ident_or_keyword(&mut self) -> TokenKind {
        let start = self.pos;
        let mut first_ignorable = None;
        while let Some(&byte) = self.src.get(self.pos) {
            if byte.is_ascii() {
                if !is_ascii_ident_continue(byte) {
                    break;
                }
                if is_ascii_ident_ignorable(byte) {
                    first_ignorable.get_or_insert(self.pos);
                }
                self.pos += 1;
            } else {
                let ch = self.peek_char().unwrap();
                match identifier_part(ch) {
                    Some(IdentifierPart::Ignorable) => {
                        first_ignorable.get_or_insert(self.pos);
                    }
                    Some(IdentifierPart::Significant) => {}
                    None => break,
                }
                self.pos += ch.len_utf8();
            }
        }
        if let Some(first) = first_ignorable {
            let name = remove_ignorable_characters(&self.text[start..self.pos], first - start);
            self.identifier_names.insert(offset32(start), name);
        }
        keyword_kind(&self.src[start..self.pos]).unwrap_or(TokenKind::Identifier)
    }
}

fn keyword_kind(text: &[u8]) -> Option<TokenKind> {
    Some(match text {
        b"package" => TokenKind::Package,
        b"_" => TokenKind::Underscore,
        b"import" => TokenKind::Import,
        b"class" => TokenKind::Class,
        b"interface" => TokenKind::Interface,
        b"enum" => TokenKind::Enum,
        b"boolean" => TokenKind::Boolean,
        b"byte" => TokenKind::Byte,
        b"char" => TokenKind::Char,
        b"double" => TokenKind::Double,
        b"float" => TokenKind::Float,
        b"int" => TokenKind::Int,
        b"long" => TokenKind::Long,
        b"short" => TokenKind::Short,
        b"void" => TokenKind::Void,
        b"const" => TokenKind::Const,
        b"goto" => TokenKind::Goto,
        b"public" => TokenKind::Public,
        b"protected" => TokenKind::Protected,
        b"private" => TokenKind::Private,
        b"static" => TokenKind::Static,
        b"final" => TokenKind::Final,
        b"abstract" => TokenKind::Abstract,
        b"native" => TokenKind::Native,
        b"strictfp" => TokenKind::Strictfp,
        b"transient" => TokenKind::Transient,
        b"volatile" => TokenKind::Volatile,
        b"extends" => TokenKind::Extends,
        b"implements" => TokenKind::Implements,
        b"new" => TokenKind::New,
        b"super" => TokenKind::Super,
        b"instanceof" => TokenKind::Instanceof,
        b"return" => TokenKind::Return,
        b"throw" => TokenKind::Throw,
        b"assert" => TokenKind::Assert,
        b"try" => TokenKind::Try,
        b"catch" => TokenKind::Catch,
        b"finally" => TokenKind::Finally,
        b"synchronized" => TokenKind::Synchronized,
        b"if" => TokenKind::If,
        b"else" => TokenKind::Else,
        b"switch" => TokenKind::Switch,
        b"case" => TokenKind::Case,
        b"default" => TokenKind::Default,
        b"while" => TokenKind::While,
        b"do" => TokenKind::Do,
        b"for" => TokenKind::For,
        b"break" => TokenKind::Break,
        b"continue" => TokenKind::Continue,
        b"throws" => TokenKind::Throws,
        b"this" => TokenKind::This,
        b"true" => TokenKind::True,
        b"false" => TokenKind::False,
        b"null" => TokenKind::Null,

        // Contextual keywords (JLS §3.9) use cooked text: `rec\u006frd` is `record`.
        // The parser admits them in identifier positions via `TokenKind::is_identifier`.
        b"exports" => TokenKind::Exports,
        b"module" => TokenKind::Module,
        b"non" => TokenKind::Non,
        b"open" => TokenKind::Open,
        b"opens" => TokenKind::Opens,
        b"permits" => TokenKind::Permits,
        b"provides" => TokenKind::Provides,
        b"record" => TokenKind::Record,
        b"requires" => TokenKind::Requires,
        b"sealed" => TokenKind::Sealed,
        b"to" => TokenKind::To,
        b"transitive" => TokenKind::Transitive,
        b"uses" => TokenKind::Uses,
        b"var" => TokenKind::Var,
        b"when" => TokenKind::When,
        b"with" => TokenKind::With,
        b"yield" => TokenKind::Yield,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use robusta_syntax::identifier::identifier_name;

    use crate::tokenize;

    #[test]
    fn identifier_identity_removes_bmp_java_ignorable_characters() {
        for ch in ('\u{0000}'..='\u{0008}')
            .chain('\u{000e}'..='\u{001b}')
            .chain('\u{007f}'..='\u{009f}')
            .chain(['\u{00ad}', '\u{200b}', '\u{200c}', '\u{200d}', '\u{2060}', '\u{feff}'])
        {
            let source = format!("co{ch}un{ch}t{ch}");
            let result = tokenize(&source);
            assert!(result.errors.is_empty(), "{:?}", result.errors);
            assert_eq!(result.identifier_names[&0], "count");
            assert_eq!(result.tokens[0].span.source_text(&source), source);
        }
    }

    #[test]
    fn unchanged_identifier_names_need_no_cooked_storage() {
        for name in [
            "count",
            "_",
            "$value2",
            "Á",
            "A\u{0301}",
            "α",
            "a\u{034f}",
            "a\u{fe0f}",
            "a\u{e0100}",
            "coun\u{e0001}t",
            "coun\u{1d173}t",
        ] {
            let result = tokenize(name);
            assert!(result.errors.is_empty(), "{:?}", result.errors);
            assert!(result.identifier_names.is_empty(), "{name:?}");
            assert!(matches!(identifier_name(name), Cow::Borrowed(value) if value == name));
        }
    }
}
