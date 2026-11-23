use std::ops::Range;

use super::group_matcher::ImportSortKey;

pub struct FormattedImports {
    pub start: usize,
    pub items: Vec<SortableImport>,
}

pub struct SortableImport {
    pub key: ImportSortKey,
    pub elements: Range<usize>,
}
