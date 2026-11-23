use super::options::{ImportGroup, ImportMatcher, ImportOrder, ImportSection};
use robusta_ast::{ImportDeclaration, ImportKind};

/// The section an import belongs to, matched against [`ImportSection`].
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum ImportKindKey {
    Module,
    Type,
    Static,
}

/// Sort by section, kind, group, entry, then name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ImportSortKey {
    pub(crate) section: usize,
    kind: ImportKindKey,
    pub(crate) group: usize,
    entry: usize,
    imported: String,
}

pub fn import_sort_key(order: &ImportOrder, import: &ImportDeclaration<'_>) -> ImportSortKey {
    let kind = match import.kind {
        ImportKind::SingleStatic | ImportKind::StaticOnDemand => ImportKindKey::Static,
        ImportKind::SingleModule => ImportKindKey::Module,
        ImportKind::SingleType | ImportKind::TypeOnDemand => ImportKindKey::Type,
    };
    let mut imported = import.name.dotted();
    if matches!(import.kind, ImportKind::TypeOnDemand | ImportKind::StaticOnDemand) {
        imported.push_str(".*");
    }
    let (group, entry) = if kind == ImportKindKey::Module {
        (0, 0)
    } else {
        order
            .groups
            .iter()
            .enumerate()
            .filter_map(|(index, group)| {
                group.match_entry(&imported).map(|(entry, length)| (index, entry, length))
            })
            .max_by_key(|&(_, _, length)| length)
            .map_or((order.groups.len(), 0), |(group, entry, _)| (group, entry))
    };
    let section = order
        .sections
        .iter()
        .position(|section| section.matches(kind))
        .unwrap_or(order.sections.len());
    ImportSortKey { section, kind, group, entry, imported }
}

impl ImportGroup {
    fn match_entry(&self, imported: &str) -> Option<(usize, usize)> {
        self.matchers
            .iter()
            .enumerate()
            .filter(|(_, matcher)| matcher.matches_import(imported))
            .map(|(entry, matcher)| {
                let length = match matcher {
                    ImportMatcher::Prefix(prefix) => prefix.len(),
                    ImportMatcher::All => 0,
                };
                (entry, length)
            })
            .max_by_key(|&(_, length)| length)
    }
}

impl ImportMatcher {
    /// Whether this matcher selects one dotted imported name.
    ///
    /// Prefixes match only on package-segment boundaries: `java` matches
    /// `java.util.List`, but not `javax.inject.Inject`.
    pub fn matches_import(&self, imported: &str) -> bool {
        match self {
            Self::Prefix(prefix) => prefix_matches_import(prefix, imported),
            Self::All => true,
        }
    }
}

impl ImportSection {
    fn matches(self, kind: ImportKindKey) -> bool {
        matches!(
            (self, kind),
            (Self::Static, ImportKindKey::Static)
                | (Self::Module, ImportKindKey::Module)
                | (Self::Type, ImportKindKey::Type)
        )
    }
}

fn prefix_matches_import(prefix: &str, imported: &str) -> bool {
    imported == prefix || imported.strip_prefix(prefix).is_some_and(|tail| tail.starts_with('.'))
}
