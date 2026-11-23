/// Configured order of the import section.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportOrder {
    pub sections: Vec<ImportSection>,
    pub groups: Vec<ImportGroup>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ImportSection {
    Static,
    Module,
    Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportGroup {
    pub matchers: Vec<ImportMatcher>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ImportMatcher {
    Prefix(String),
    All,
}

impl Default for ImportOrder {
    fn default() -> Self {
        Self {
            sections: vec![ImportSection::Module, ImportSection::Type, ImportSection::Static],
            groups: vec![
                ImportGroup {
                    matchers: vec![
                        ImportMatcher::Prefix("java".to_string()),
                        ImportMatcher::Prefix("javax".to_string()),
                    ],
                },
                ImportGroup { matchers: vec![ImportMatcher::All] },
            ],
        }
    }
}
