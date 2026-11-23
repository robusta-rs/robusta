use proc_macro2::TokenStream;

use super::{
    super::extensions::layout::Layout, BoxDef, CellDef, Def, EnumDef, OptionDef, PrimitiveDef,
    Schema, StructDef, StructOrEnum, TypeId, VecDef,
};

/// Type definition for a type.
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
    Primitive(PrimitiveDef),
    Option(OptionDef),
    Box(BoxDef),
    Vec(VecDef),
    Cell(CellDef),
}

impl Def for TypeDef {
    fn id(&self) -> TypeId {
        match self {
            Self::Struct(def) => def.id(),
            Self::Enum(def) => def.id(),
            Self::Primitive(def) => def.id(),
            Self::Option(def) => def.id(),
            Self::Box(def) => def.id(),
            Self::Vec(def) => def.id(),
            Self::Cell(def) => def.id(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Struct(def) => def.name(),
            Self::Enum(def) => def.name(),
            Self::Primitive(def) => def.name(),
            Self::Option(def) => def.name(),
            Self::Box(def) => def.name(),
            Self::Vec(def) => def.name(),
            Self::Cell(def) => def.name(),
        }
    }

    fn has_lifetime(&self, schema: &Schema) -> bool {
        match self {
            Self::Struct(def) => def.has_lifetime(schema),
            Self::Enum(def) => def.has_lifetime(schema),
            Self::Primitive(def) => def.has_lifetime(schema),
            Self::Option(def) => def.has_lifetime(schema),
            Self::Box(def) => def.has_lifetime(schema),
            Self::Vec(def) => def.has_lifetime(schema),
            Self::Cell(def) => def.has_lifetime(schema),
        }
    }

    fn ty_with_lifetime(&self, schema: &Schema, anon: bool) -> TokenStream {
        match self {
            Self::Struct(def) => def.ty_with_lifetime(schema, anon),
            Self::Enum(def) => def.ty_with_lifetime(schema, anon),
            Self::Primitive(def) => def.ty_with_lifetime(schema, anon),
            Self::Option(def) => def.ty_with_lifetime(schema, anon),
            Self::Box(def) => def.ty_with_lifetime(schema, anon),
            Self::Vec(def) => def.ty_with_lifetime(schema, anon),
            Self::Cell(def) => def.ty_with_lifetime(schema, anon),
        }
    }

    fn maybe_inner_type<'s>(&self, schema: &'s Schema) -> Option<&'s TypeDef> {
        match self {
            Self::Struct(def) => def.maybe_inner_type(schema),
            Self::Enum(def) => def.maybe_inner_type(schema),
            Self::Primitive(def) => def.maybe_inner_type(schema),
            Self::Option(def) => def.maybe_inner_type(schema),
            Self::Box(def) => def.maybe_inner_type(schema),
            Self::Vec(def) => def.maybe_inner_type(schema),
            Self::Cell(def) => def.maybe_inner_type(schema),
        }
    }
}

impl TypeDef {
    /// Get the layout of this type, once `AssertLayoutsGenerator` has calculated it.
    #[must_use]
    pub fn layout(&self) -> &Layout {
        match self {
            Self::Struct(def) => &def.layout,
            Self::Enum(def) => &def.layout,
            Self::Primitive(def) => &def.layout,
            Self::Option(def) => &def.layout,
            Self::Box(def) => &def.layout,
            Self::Vec(def) => &def.layout,
            Self::Cell(def) => &def.layout,
        }
    }

    /// Get mutable reference to the layout of this type.
    pub fn layout_mut(&mut self) -> &mut Layout {
        match self {
            Self::Struct(def) => &mut def.layout,
            Self::Enum(def) => &mut def.layout,
            Self::Primitive(def) => &mut def.layout,
            Self::Option(def) => &mut def.layout,
            Self::Box(def) => &mut def.layout,
            Self::Vec(def) => &mut def.layout,
            Self::Cell(def) => &mut def.layout,
        }
    }

    /// Get whether this type is or holds a type the walk visits.
    #[must_use]
    pub fn contains_visited(&self, schema: &Schema) -> bool {
        self.innermost_type(schema).as_struct_or_enum().is_some_and(StructOrEnum::is_visited)
    }

    #[must_use]
    pub fn as_struct(&self) -> Option<&StructDef> {
        match self {
            Self::Struct(def) => Some(def),
            _ => None,
        }
    }

    pub fn as_struct_mut(&mut self) -> Option<&mut StructDef> {
        match self {
            Self::Struct(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_enum(&self) -> Option<&EnumDef> {
        match self {
            Self::Enum(def) => Some(def),
            _ => None,
        }
    }

    /// Get mutable reference to the [`EnumDef`], if this type is an enum.
    pub fn as_enum_mut(&mut self) -> Option<&mut EnumDef> {
        match self {
            Self::Enum(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_struct_or_enum(&self) -> Option<StructOrEnum<'_>> {
        match self {
            Self::Struct(def) => Some(StructOrEnum::Struct(def)),
            Self::Enum(def) => Some(StructOrEnum::Enum(def)),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_vec(&self) -> Option<&VecDef> {
        match self {
            Self::Vec(def) => Some(def),
            _ => None,
        }
    }

    /// Get mutable reference to the [`VecDef`], if this type is a `Vec`.
    pub fn as_vec_mut(&mut self) -> Option<&mut VecDef> {
        match self {
            Self::Vec(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_box(&self) -> Option<&BoxDef> {
        match self {
            Self::Box(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_cell(&self) -> Option<&CellDef> {
        match self {
            Self::Cell(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_option(&self) -> Option<&OptionDef> {
        match self {
            Self::Option(def) => Some(def),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_box(&self) -> bool {
        matches!(self, Self::Box(_))
    }

    #[must_use]
    pub fn is_option(&self) -> bool {
        matches!(self, Self::Option(_))
    }

    #[must_use]
    pub fn is_cell(&self) -> bool {
        matches!(self, Self::Cell(_))
    }
}
