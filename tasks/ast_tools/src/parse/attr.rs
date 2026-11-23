use std::fmt::{self, Display};

use crate::schema::{Def, EnumDef, StructDef};

/// Attribute location.
pub enum AttrLocation<'d> {
    /// Attribute on a struct
    Struct(&'d mut StructDef),
    /// Attribute on an enum
    Enum(&'d mut EnumDef),
    /// Attribute on a struct field.
    /// Comprises [`StructDef`] and field index.
    StructField(&'d mut StructDef, usize),
    /// Attribute on an enum variant.
    /// Comprises [`EnumDef`] and variant index.
    EnumVariant(&'d mut EnumDef, usize),
    /// Part of `#[ast]` attr on a struct
    StructAstAttr(&'d mut StructDef),
    /// Part of `#[ast]` attr on an enum
    EnumAstAttr(&'d mut EnumDef),
}

impl AttrLocation<'_> {
    /// Convert `&mut AttrLocation` to `AttrLocation`.
    pub fn unpack(&mut self) -> AttrLocation<'_> {
        match self {
            AttrLocation::Struct(struct_def) => AttrLocation::Struct(struct_def),
            AttrLocation::Enum(enum_def) => AttrLocation::Enum(enum_def),
            AttrLocation::StructField(struct_def, field_index) => {
                AttrLocation::StructField(struct_def, *field_index)
            }
            AttrLocation::EnumVariant(enum_def, variant_index) => {
                AttrLocation::EnumVariant(enum_def, *variant_index)
            }
            AttrLocation::StructAstAttr(struct_def) => AttrLocation::StructAstAttr(struct_def),
            AttrLocation::EnumAstAttr(enum_def) => AttrLocation::EnumAstAttr(enum_def),
        }
    }
}

impl Display for AttrLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrLocation::Struct(struct_def) | AttrLocation::StructAstAttr(struct_def) => {
                f.write_str(struct_def.name())
            }
            AttrLocation::Enum(enum_def) | AttrLocation::EnumAstAttr(enum_def) => {
                f.write_str(enum_def.name())
            }
            AttrLocation::StructField(struct_def, field_index) => {
                write!(f, "{}::{}", struct_def.name(), struct_def.fields[*field_index].name)
            }
            AttrLocation::EnumVariant(enum_def, variant_index) => {
                write!(f, "{}::{}", enum_def.name(), enum_def.variants[*variant_index].name)
            }
        }
    }
}

/// Part of an attribute.
///
/// e.g.:
///
/// * `#[scope]` translates to a single `AttrPart::None`.
/// * `#[scope(enter_before)]` translates to a single `AttrPart::Tag("enter_before")`.
#[derive(Debug)]
pub enum AttrPart<'p> {
    /// No parts in attribute.
    /// e.g. `#[scope]`.
    None,
    /// Named part.
    /// e.g. `#[scope(enter_before)]`.
    Tag(&'p str),
}
