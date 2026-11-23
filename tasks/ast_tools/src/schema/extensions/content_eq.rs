/// Details for `ContentEq` derive on a struct or enum.
#[derive(Default)]
pub struct ContentEqType {
    /// `true` if type should ignored by `ContentEq`
    pub skip: bool,
}

/// Details for `ContentEq` derive on a struct field.
#[derive(Default)]
pub struct ContentEqStructField {
    /// `true` if field should ignored by `ContentEq`
    pub skip: bool,
}
