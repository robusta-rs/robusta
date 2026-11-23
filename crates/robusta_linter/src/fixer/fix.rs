// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/fixer/fix.rs
use std::{borrow::Cow, ops::Deref};

use bitflags::bitflags;

use robusta_span::Span;

bitflags! {
    /// Whether and how a rule-owned edit may be applied.
    ///
    /// One edit carries one kind. A union describes what a rule emits, or the
    /// ceiling a run applies up to.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FixKind: u8 {
        /// An edit that keeps the code compiling and behaving as before,
        /// applied by `--fix`.
        const SAFE_FIX = 1 << 0;
        /// An edit that may change what the code does or stop it compiling,
        /// applied with `--unsafe-fixes`.
        const UNSAFE_FIX = 1 << 1;

        /// No fix, as an unfixable rule declares.
        const NONE = 0;
        /// Every kind, as `--unsafe-fixes` permits.
        const ALL = Self::SAFE_FIX.bits() | Self::UNSAFE_FIX.bits();
    }
}

impl Default for FixKind {
    fn default() -> Self {
        Self::NONE
    }
}

impl FixKind {
    #[must_use]
    pub const fn is_none(self) -> bool {
        self.is_empty()
    }

    #[must_use]
    pub const fn is_some(self) -> bool {
        !self.is_empty()
    }

    /// Whether this set includes unsafe fixes.
    #[must_use]
    pub const fn is_unsafe(self) -> bool {
        self.contains(Self::UNSAFE_FIX)
    }

    /// # Panics
    /// Panics if the set contains unknown bits.
    pub fn emoji(self) -> &'static str {
        match self {
            Self::NONE => "",
            Self::SAFE_FIX => "🛠️",
            Self::UNSAFE_FIX => "⚠️🛠️️",
            Self::ALL => "🛠️⚠️🛠️️",
            _ => panic!("unknown fix kind bits"),
        }
    }

    /// Stable user-facing name used by configuration docs and CLI metadata.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NONE => "none",
            Self::SAFE_FIX => "fix",
            Self::UNSAFE_FIX => "unsafe_fix",
            Self::ALL => "safe_fix_or_unsafe_fix",
            _ => "unknown",
        }
    }

    /// Whether this ceiling holds every bit of `kind`.
    ///
    /// `--fix` alone rejects an unsafe fix.
    #[must_use]
    pub const fn can_apply(self, kind: Self) -> bool {
        self.contains(kind)
    }
}

/// A fix a rule returns from its fixer closure.
#[derive(Debug, Default)]
#[must_use = "Fixes must be used. If you don't need a fix, use `LintContext::diagnostic`, or create an empty fix using `RuleFixer::noop`."]
pub struct RuleFix {
    kind: FixKind,
    fix: CompositeFix,
}

macro_rules! impl_from {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for RuleFix {
                fn from(fix: $ty) -> Self {
                    Self { kind: FixKind::SAFE_FIX, fix: fix.into() }
                }
            }
        )*
    };
}
impl_from!(Fix, Option<Fix>, Vec<Fix>);

impl RuleFix {
    pub(super) const fn new(kind: FixKind, fix: CompositeFix) -> Self {
        Self { kind, fix }
    }

    pub const fn kind(&self) -> FixKind {
        self.kind
    }

    pub fn into_fix(self, source_text: &str) -> Fix {
        let mut fix = self.fix.normalize_fixes(source_text);
        fix.kind = self.kind;
        fix
    }

    pub fn extend<F: Into<CompositeFix>>(mut self, fix: F) -> Self {
        self.fix = self.fix.concat(fix.into());
        self
    }
}

impl Deref for RuleFix {
    type Target = CompositeFix;

    fn deref(&self) -> &Self::Target {
        &self.fix
    }
}

/// A completed, normalized fix ready to be applied to the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Fix {
    pub content: Cow<'static, str>,
    pub kind: FixKind,
    pub span: Span,
}

impl Fix {
    pub const fn delete(span: Span) -> Self {
        Self { content: Cow::Borrowed(""), kind: FixKind::NONE, span }
    }

    pub fn new<T: Into<Cow<'static, str>>>(content: T, span: Span) -> Self {
        Self { content: content.into(), kind: FixKind::NONE, span }
    }

    /// A fix that does not change the source code.
    pub const fn empty() -> Self {
        Self::delete(Span::empty(0))
    }

    #[must_use]
    pub const fn with_kind(mut self, kind: FixKind) -> Self {
        self.kind = kind;
        self
    }
}

#[derive(Debug, Default)]
pub enum CompositeFix {
    #[default]
    None,
    Single(Fix),
    /// Several fixes that will be merged into one, in order.
    Multiple(Vec<Fix>),
}

impl From<Fix> for CompositeFix {
    fn from(fix: Fix) -> Self {
        Self::Single(fix)
    }
}

impl From<Option<Fix>> for CompositeFix {
    fn from(fix: Option<Fix>) -> Self {
        fix.map_or(Self::None, Self::Single)
    }
}

impl From<Vec<Fix>> for CompositeFix {
    fn from(mut fixes: Vec<Fix>) -> Self {
        match fixes.len() {
            0 => Self::None,
            1 => Self::Single(fixes.pop().expect("one fix")),
            _ => Self::Multiple(fixes),
        }
    }
}

impl CompositeFix {
    #[must_use]
    pub fn concat(self, fix: Self) -> Self {
        match (self, fix) {
            (Self::None, f) | (f, Self::None) => f,
            (Self::Single(fix1), Self::Single(fix2)) => Self::Multiple(vec![fix1, fix2]),
            (Self::Single(fix), Self::Multiple(mut fixes)) => {
                fixes.insert(0, fix);
                Self::Multiple(fixes)
            }
            (Self::Multiple(mut fixes), Self::Single(fix)) => {
                fixes.push(fix);
                Self::Multiple(fixes)
            }
            (Self::Multiple(mut fixes1), Self::Multiple(fixes2)) => {
                fixes1.extend(fixes2);
                Self::Multiple(fixes1)
            }
        }
    }

    pub const fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Single(_) => false,
            Self::Multiple(fixes) => fixes.is_empty(),
        }
    }

    /// One fix for all of these, merging several in source order.
    pub fn normalize_fixes(self, source_text: &str) -> Fix {
        match self {
            Self::Single(fix) => fix,
            Self::Multiple(fixes) => Self::merge_fixes(fixes, source_text),
            Self::None => Fix::empty(),
        }
    }

    /// Merge fixes into one, keeping the source between them.
    ///
    /// Returns [`Fix::empty`] when fixes overlap or a span is not a valid range
    /// of `source_text`.
    ///
    /// # Panics
    ///
    /// In debug builds, when merging fails.
    pub fn merge_fixes(mut fixes: Vec<Fix>, source_text: &str) -> Fix {
        fixes.sort_by_key(|fix| (fix.span.start, fix.span.end));
        let (Some(first), Some(last)) = (fixes.first(), fixes.last()) else {
            return Fix::empty();
        };
        let span = first.span.cover(last.span);
        let mut content = String::new();
        let mut position = span.start;
        for fix in &fixes {
            if fix.span.start < position || super::validate_span(source_text, fix.span).is_err() {
                debug_assert!(false, "fixes overlap or leave the source: {fixes:?}");
                return Fix::empty();
            }
            content.push_str(&source_text[position as usize..fix.span.start as usize]);
            content.push_str(&fix.content);
            position = fix.span.end;
        }
        Fix::new(content, span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merged_fixes_preserve_intervening_unicode_source() {
        let fixes = vec![Fix::delete(Span::new(7, 8)), Fix::delete(Span::new(0, 1))];
        let fix = CompositeFix::merge_fixes(fixes, "a é b c");
        assert_eq!(fix, Fix::new(" é b ", Span::new(0, 8)));
    }

    #[test]
    #[should_panic(expected = "fixes overlap or leave the source")]
    fn merging_overlapping_fixes_panics() {
        let fixes = vec![Fix::delete(Span::new(0, 2)), Fix::delete(Span::new(1, 4))];
        let _ = CompositeFix::merge_fixes(fixes, "a é b");
    }

    #[test]
    #[should_panic(expected = "fixes overlap or leave the source")]
    fn merging_a_fix_inside_a_character_panics() {
        let fixes = vec![Fix::delete(Span::new(0, 1)), Fix::delete(Span::new(3, 4))];
        let _ = CompositeFix::merge_fixes(fixes, "a é b");
    }

    #[test]
    #[should_panic(expected = "fixes overlap or leave the source")]
    fn merging_a_fix_past_the_source_end_panics() {
        let fixes = vec![Fix::delete(Span::new(0, 1)), Fix::delete(Span::new(9, 10))];
        let _ = CompositeFix::merge_fixes(fixes, "a é b");
    }

    #[test]
    fn an_empty_rule_fix_declines() {
        assert!(RuleFix::from(None).is_empty());
        assert!(RuleFix::from(Vec::new()).is_empty());
        assert!(!RuleFix::from(Fix::delete(Span::new(0, 1))).is_empty());
    }

    #[test]
    fn fix_kind_emojis_describe_risk() {
        for (kind, emoji) in [
            (FixKind::NONE, ""),
            (FixKind::SAFE_FIX, "🛠️"),
            (FixKind::UNSAFE_FIX, "⚠️🛠️️"),
            (FixKind::ALL, "🛠️⚠️🛠️️"),
        ] {
            assert_eq!(kind.emoji(), emoji);
            assert_eq!(crate::RuleFixMeta::Fixable(kind).emoji(), Some(emoji));
            assert_eq!(crate::RuleFixMeta::Conditional(kind).emoji(), Some(emoji));
        }
        assert_eq!(crate::RuleFixMeta::None.emoji(), None);
    }

    #[test]
    fn fix_kinds_are_sets_of_two_independent_bits() {
        assert_eq!(FixKind::default(), FixKind::NONE);
        assert_eq!(FixKind::NONE.bits(), 0);
        assert_eq!(FixKind::SAFE_FIX.bits(), 0b01);
        assert_eq!(FixKind::UNSAFE_FIX.bits(), 0b10);
        assert_eq!(FixKind::ALL.bits(), 0b11);

        for left in 0..4u8 {
            let ceiling = FixKind::from_bits(left).expect("two defined bits");
            assert_eq!(ceiling.is_none(), left == 0);
            assert_eq!(ceiling.is_some(), left != 0);
            assert_eq!(ceiling.is_unsafe(), left & 0b10 != 0);
            for right in 0..4u8 {
                let kind = FixKind::from_bits(right).expect("two defined bits");
                assert_eq!(ceiling.union(kind).bits(), left | right);
                assert_eq!(ceiling.can_apply(kind), left & right == right);
            }
        }
    }
}
