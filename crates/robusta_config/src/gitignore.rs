use std::path::Path;
use std::str::{Chars, Lines};

use ignore::{
    Match,
    gitignore::{Gitignore, GitignoreBuilder},
};

/// Add one line of Git's pattern format to `builder`, whose own glob grammar
/// expands `{a,b}`, keeps an unterminated `[` literal, and reads classes without
/// escapes or POSIX names.
///
/// # Errors
///
/// Returns why the pattern is invalid. Git accepts such a line and never matches it.
pub fn add_line(builder: &mut GitignoreBuilder, line: &str) -> Result<(), String> {
    if let Some(glob) = to_glob(line)? {
        builder.add_line(None, &glob).map_err(|error| error.to_string())?;
    }
    Ok(())
}

/// The lines of a Git ignore file, without its byte order mark. A final `\r`
/// ends a line as `\r\n` does.
pub fn lines(contents: &str) -> Lines<'_> {
    let contents = contents.strip_prefix('\u{feff}').unwrap_or(contents);
    contents.strip_suffix('\r').unwrap_or(contents).lines()
}

/// Rewrite a line into the builder's grammar, or `None` when it can match nothing.
fn to_glob(line: &str) -> Result<Option<String>, String> {
    if line.starts_with('#') {
        return Ok(None);
    }
    let line = trim_trailing_spaces(line);
    let (negation, line) = line.strip_prefix('!').map_or(("", line), |rest| ("!", rest));
    let (pattern, directory) = line.strip_suffix('/').map_or((line, ""), |rest| (rest, "/"));

    let mut glob = String::with_capacity(pattern.len());
    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('/') => glob.push('/'),
                Some(escaped) => push_literal(&mut glob, escaped),
                None => return Err("a pattern cannot end with a backslash".to_string()),
            },
            '{' | '}' => push_literal(&mut glob, c),
            _ if c != ' ' && c.is_whitespace() => push_literal(&mut glob, c),
            '[' => {
                if !Class::parse(&mut chars)?.write_to(&mut glob) {
                    return Ok(None);
                }
            }
            // Any run of `*` beyond two reads as `**`.
            '*' if glob.ends_with("**") => {}
            _ => glob.push(c),
        }
    }

    if glob.is_empty() {
        return Ok(None);
    }
    // The builder anchors a pattern by whether it holds a `/`, which a class can add or remove.
    let anchor = match (pattern.contains('/'), glob.contains('/')) {
        (true, false) => "/",
        (false, true) => "**/",
        _ => "",
    };
    Ok(Some(format!("{negation}{anchor}{glob}{directory}")))
}

/// Drop trailing spaces, except one a backslash quotes.
fn trim_trailing_spaces(line: &str) -> &str {
    let trimmed = line.trim_end_matches(' ');
    let backslashes = trimmed.bytes().rev().take_while(|byte| *byte == b'\\').count();
    if trimmed.len() < line.len() && backslashes % 2 == 1 {
        &line[..=trimmed.len()]
    } else {
        trimmed
    }
}

/// Write `c` so that it matches itself. The builder trims a line's trailing
/// whitespace and a backslash before its final `/`, which a group and a class survive.
fn push_literal(glob: &mut String, c: char) {
    if c == '\\' {
        glob.push_str(r"[\]");
    } else if c != ' ' && c.is_whitespace() {
        glob.push('{');
        glob.push(c);
        glob.push('}');
    } else {
        glob.push('\\');
        glob.push(c);
    }
}

/// A bracket expression read in Git's grammar.
#[derive(Default)]
struct Class {
    negated: bool,
    /// One bit per ASCII member.
    ascii: u128,
    /// The non-ASCII members, in the builder's grammar.
    other: String,
}

impl Class {
    /// Read the expression after its `[`.
    fn parse(chars: &mut Chars<'_>) -> Result<Self, String> {
        const UNTERMINATED: &str = "a character class must end with `]`";

        let mut class = Self::default();
        if matches!(chars.clone().next(), Some('!' | '^')) {
            class.negated = true;
            chars.next();
        }
        let mut previous = None;
        let mut first = true;
        loop {
            let c = chars.next().ok_or(UNTERMINATED)?;
            match (c, previous) {
                (']', _) if !first => return Ok(class),
                ('\\', _) => {
                    let escaped = chars.next().ok_or(UNTERMINATED)?;
                    class.add(escaped, escaped);
                    previous = Some(escaped);
                }
                ('-', Some(start)) if chars.clone().next().is_some_and(|next| next != ']') => {
                    let mut end = chars.next().ok_or(UNTERMINATED)?;
                    if end == '\\' {
                        end = chars.next().ok_or(UNTERMINATED)?;
                    }
                    class.add(start, end);
                    previous = None;
                }
                ('[', _) if chars.as_str().starts_with(':') => {
                    let rest = &chars.as_str()[1..];
                    let end = rest.find(']').ok_or(UNTERMINATED)?;
                    if let Some(name) = rest[..end].strip_suffix(':') {
                        class.add_named(name)?;
                        *chars = rest[end + 1..].chars();
                        previous = None;
                    } else {
                        class.add('[', '[');
                        previous = Some('[');
                    }
                }
                _ => {
                    class.add(c, c);
                    previous = Some(c);
                }
            }
            first = false;
        }
    }

    /// Add a range, where a reversed one has no members.
    fn add(&mut self, start: char, end: char) {
        if start > end {
            return;
        }
        for byte in u32::from(start)..=u32::from(end).min(0x7f) {
            self.ascii |= 1 << byte;
        }
        if end > '\x7f' {
            self.other.push(start.max('\u{80}'));
            self.other.push('-');
            self.other.push(end);
        }
    }

    fn add_named(&mut self, name: &str) -> Result<(), String> {
        let ranges: &[(char, char)] = match name {
            "alnum" => &[('0', '9'), ('A', 'Z'), ('a', 'z')],
            "alpha" => &[('A', 'Z'), ('a', 'z')],
            "blank" => &[(' ', ' '), ('\t', '\t')],
            "cntrl" => &[('\0', '\x1f'), ('\x7f', '\x7f')],
            "digit" => &[('0', '9')],
            "graph" => &[('!', '~')],
            "lower" => &[('a', 'z')],
            "print" => &[(' ', '~')],
            "punct" => &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')],
            "space" => &[(' ', ' '), ('\t', '\n'), ('\r', '\r')],
            "upper" => &[('A', 'Z')],
            "xdigit" => &[('0', '9'), ('A', 'F'), ('a', 'f')],
            _ => return Err(format!("`[:{name}:]` is not a character class")),
        };
        for (start, end) in ranges {
            self.add(*start, *end);
        }
        Ok(())
    }

    /// Write the class in the builder's grammar, or return `false` when it has no member.
    ///
    /// There, `]` is a member only first and `-` only first or last, and a
    /// leading `!` or `^` negates. Git's classes never match `/`.
    fn write_to(&self, glob: &mut String) -> bool {
        let slash = 1 << b'/';
        let ascii = if self.negated { self.ascii | slash } else { self.ascii & !slash };
        let has = |c: u8| ascii & (1 << c) != 0;

        let mut members = String::new();
        if has(b']') {
            members.push(']');
        }
        members.extend((0..0x80).filter(|c| has(*c) && !b"]!^-".contains(c)).map(char::from));
        members.push_str(&self.other);
        if !self.negated && members.is_empty() {
            if ascii == 0 {
                return false;
            }
            // No path holds NUL, and it keeps a leading `!` or `^` from negating.
            members.push('\0');
        }
        members.extend(b"!^-".iter().filter(|c| has(**c)).map(|c| char::from(*c)));

        glob.push('[');
        if self.negated {
            glob.push('!');
        }
        glob.push_str(&members);
        glob.push(']');
        true
    }
}

/// Evaluate one gitignore layer, returning `None` when it has no opinion.
///
/// Patterns apply only below the layer's root, which is not itself a target.
pub fn decision(matcher: &Gitignore, path: &Path, is_dir: bool) -> Option<bool> {
    if path == matcher.path() || !path.starts_with(matcher.path()) {
        return None;
    }
    match matcher.matched(path, is_dir) {
        Match::None => None,
        Match::Ignore(_) => Some(true),
        Match::Whitelist(_) => Some(false),
    }
}

#[cfg(test)]
mod tests {
    use ignore::gitignore::GitignoreBuilder;

    use super::{add_line, lines};

    fn matches(pattern: &str, path: &str) -> bool {
        let mut builder = GitignoreBuilder::new("");
        add_line(&mut builder, pattern).expect("pattern should be valid");
        let matcher = builder.build().expect("rewritten pattern should compile");
        matcher.matched(path, false).is_ignore()
    }

    #[test]
    fn braces_are_literal() {
        assert!(matches("{First,Second}.java", "{First,Second}.java"));
        assert!(!matches("{First,Second}.java", "First.java"));
        assert!(matches("Open{.java", "Open{.java"));
        assert!(matches("Close}.java", "Close}.java"));
        assert!(matches(r"\{a\}.java", "{a}.java"));
        assert!(!matches(r"\{a\}.java", "a.java"));
        assert!(matches("[{]a}.java", "{a}.java"));
    }

    #[test]
    fn classes_read_git_grammar() {
        for (pattern, matching, other) in [
            ("[ab].java", "b.java", "c.java"),
            ("[a-c].java", "b.java", "d.java"),
            ("[!a].java", "b.java", "a.java"),
            ("[^a].java", "b.java", "a.java"),
            ("[]a].java", "].java", "b.java"),
            ("[!]].java", "a.java", "].java"),
            (r"[\]].java", "].java", r"\.java"),
            (r"[a\-c].java", "-.java", "b.java"),
            ("[a-].java", "-.java", "b.java"),
            ("[-a].java", "-.java", "b.java"),
            ("[]-a].java", "^.java", "b.java"),
            (r"[\!^].java", "!.java", "a.java"),
            (r"[\!].java", "!.java", "a.java"),
            ("[{,}].java", ",.java", "a.java"),
            ("[[:digit:]].java", "7.java", "a.java"),
            ("[[:upper:]x].java", "x.java", "a.java"),
            ("[![:alpha:]].java", "7.java", "a.java"),
            ("[[:alpha:]-z].java", "-.java", "7.java"),
            ("[[:].java", ":.java", "a.java"),
        ] {
            assert!(matches(pattern, matching), "{pattern} should match {matching}");
            assert!(!matches(pattern, other), "{pattern} should not match {other}");
        }
    }

    #[test]
    fn a_class_never_matches_a_slash() {
        assert!(!matches("a[!x]b", "a/b"));
        assert!(!matches("*[!x]b", "a/b"));
        assert!(matches("*[!x]b", "dir/aab"));
        assert!(!matches("a[/]b", "a/b"));
        assert!(!matches("a[+-9]b", "a/b"));
        assert!(matches("a[+-9]b", "a0b"));
    }

    #[test]
    fn a_slash_in_a_class_still_anchors_the_pattern() {
        assert!(matches("a[/x]b", "axb"));
        assert!(!matches("a[/x]b", "dir/axb"));
    }

    #[test]
    fn a_negated_class_without_a_slash_matches_at_any_depth() {
        assert!(matches("[!x].java", "dir/a.java"));
    }

    #[test]
    fn invalid_patterns_are_reported() {
        for (pattern, reason) in [
            ("Open[.java", "must end with `]`"),
            ("[]", "must end with `]`"),
            ("[!", "must end with `]`"),
            ("[a-", "must end with `]`"),
            (r"[a\", "must end with `]`"),
            ("[[:alpha:]", "must end with `]`"),
            ("[[:alpha", "must end with `]`"),
            ("[[:word:]]", "`[:word:]` is not a character class"),
            ("end\\", "cannot end with a backslash"),
            ("dir\\/", "cannot end with a backslash"),
        ] {
            let error = add_line(&mut GitignoreBuilder::new(""), pattern)
                .expect_err("pattern should be invalid");
            assert!(error.contains(reason), "{pattern}: {error}");
        }
    }

    #[test]
    fn a_reversed_range_has_no_members() {
        assert!(!matches("[z-a].java", "b.java"));
        assert!(matches("[!z-a].java", "b.java"));
        assert!(matches("[z-ab].java", "b.java"));
    }

    #[test]
    fn line_syntax_follows_git() {
        assert!(matches("a.java  ", "a.java"));
        assert!(matches(r"a.java\ ", "a.java "));
        assert!(matches(r"a.java\  ", "a.java "));
        assert!(matches("a.java\t", "a.java\t"));
        assert!(!matches("a.java\t", "a.java"));
        assert!(matches(r"\#a.java", "#a.java"));
        assert!(!matches("#a.java", "#a.java"));
        assert!(matches(r"\!a.java", "!a.java"));
        assert!(!matches("!", "a.java"));
        assert!(!matches("/", "a.java"));
    }

    #[test]
    fn whitespace_outside_ascii_is_literal() {
        for name in ["a\u{a0}b.java", "a\u{3000}", "\u{3000}"] {
            assert!(matches(name, name), "{name}");
            assert!(!matches(name, "a.java"), "{name}");
        }
        assert!(matches("a\\\u{3000}", "a\u{3000}"));
        assert!(!matches("a\u{3000}", "a"));
    }

    // A Windows path cannot hold a backslash.
    #[cfg(unix)]
    #[test]
    fn an_escaped_backslash_is_literal() {
        assert!(matches("back\\\\", "back\\"));
    }

    #[test]
    fn lines_follow_git() {
        let split = |contents| lines(contents).collect::<Vec<_>>();

        assert_eq!(split("\u{feff}a\r\nb\n"), ["a", "b"]);
        assert_eq!(split("a\nb\r"), ["a", "b"]);
        assert_eq!(split("a\r\r"), ["a\r"]);
    }

    #[test]
    fn consecutive_asterisks_follow_git() {
        assert!(matches("a/***/b", "a/x/y/b"));
        assert!(matches(r"a/**\/b", "a/x/y/b"));
        assert!(!matches("a/**b", "a/x/b"));
        assert!(matches("a/**b", "a/xb"));
    }
}
