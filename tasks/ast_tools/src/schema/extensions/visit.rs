use quote::format_ident;
use syn::Ident;

/// Details of visiting on a struct.
#[derive(Default)]
pub struct VisitStruct {
    /// Name of `visit_*` method and `walk_*` function.
    /// `None` if this struct is not visited.
    pub visitor_names: Option<VisitorNames>,
    pub scope: Option<Scope>,
}

impl VisitStruct {
    /// Returns `true` if this struct has a visitor.
    #[must_use]
    pub const fn has_visitor(&self) -> bool {
        self.visitor_names.is_some()
    }
}

/// Details of visiting on an enum.
#[derive(Default)]
pub struct VisitEnum {
    /// Name of `visit_*` method and `walk_*` function.
    /// `None` if this enum is not visited.
    pub visitor_names: Option<VisitorNames>,
}

impl VisitEnum {
    /// Returns `true` if this enum has a visitor.
    #[must_use]
    pub const fn has_visitor(&self) -> bool {
        self.visitor_names.is_some()
    }
}

/// Details of visiting on a `Vec`.
#[derive(Default)]
pub struct VisitVec {
    /// Name of `visit_*` method and `walk_*` function.
    /// `None` if this `Vec` does not have a visitor.
    pub visitor_names: Option<VisitorNames>,
}

/// Names for visitor method and walk function.
pub struct VisitorNames {
    pub visit: String,
    pub walk: String,
}

impl VisitorNames {
    #[must_use]
    pub fn from_snake_name(snake_name: &str) -> Self {
        Self { visit: format!("visit_{snake_name}"), walk: format!("walk_{snake_name}") }
    }

    /// Get name of visitor method as an [`Ident`].
    ///
    /// [`Ident`]: struct@Ident
    #[must_use]
    pub fn visitor_ident(&self) -> Ident {
        format_ident!("{}", self.visit)
    }

    /// Get name of walk function as an [`Ident`].
    ///
    /// [`Ident`]: struct@Ident
    #[must_use]
    pub fn walk_ident(&self) -> Ident {
        format_ident!("{}", self.walk)
    }
}

/// Details of scope on a struct.
pub struct Scope {
    /// Field index before which scope is entered.
    pub enter_before_index: usize,
    /// Field index before which scope is exited.
    /// If scope is exited after last field, this is `struct_def.fields.len()`.
    pub exit_before_index: usize,
}
