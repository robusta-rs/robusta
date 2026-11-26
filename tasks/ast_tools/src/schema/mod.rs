// Directory adapted from https://github.com/oxc-project/oxc/tree/732a8e42617296af7aed3b0b374cb76863ac0efc/tasks/ast_tools/src/schema

use oxc_index::{IndexVec, define_index_type};
use rustc_hash::FxHashMap;

mod defs;
mod file;
pub use defs::*;
pub use file::File;

/// Extensions to schema for specific generators.
pub mod extensions {
    pub mod content_eq;
    pub mod dummy;
    pub mod kind;
    pub mod layout;
    pub mod visit;
}

define_index_type! {
    /// ID of a type in the AST.
    pub struct TypeId = u32;
    MAX_INDEX = u32::MAX as usize;
}

define_index_type! {
    /// ID of a source file.
    pub struct FileId = u32;
    MAX_INDEX = u32::MAX as usize;
}

/// Schema of all AST types.
///
/// Fields stay in source order here. Their order in memory is on [`FieldDef::offset`].
pub struct Schema {
    /// Type definitions. Structs and enums come first, in source order.
    pub types: IndexVec<TypeId, TypeDef>,
    /// Mapping from type name to [`TypeId`].
    pub type_names: FxHashMap<String, TypeId>,
    /// Source files.
    pub files: IndexVec<FileId, File>,
}

impl Schema {
    /// Get reference to [`TypeDef`] for a type name.
    ///
    /// # Panics
    /// Panics if no type with supplied name.
    #[must_use]
    pub fn type_by_name(&self, name: &str) -> &TypeDef {
        &self.types[self.type_names[name]]
    }

    /// Get reference to [`StructDef`] for a [`TypeId`].
    ///
    /// # Panics
    /// Panics if type [`TypeId`] refers to is not a struct.
    #[must_use]
    pub fn struct_def(&self, type_id: TypeId) -> &StructDef {
        self.types[type_id].as_struct().expect("type is a struct")
    }

    /// Get mutable reference to [`StructDef`] for a [`TypeId`].
    ///
    /// # Panics
    /// Panics if type [`TypeId`] refers to is not a struct.
    pub fn struct_def_mut(&mut self, type_id: TypeId) -> &mut StructDef {
        self.types[type_id].as_struct_mut().expect("type is a struct")
    }

    /// Get reference to [`EnumDef`] for a [`TypeId`].
    ///
    /// # Panics
    /// Panics if type [`TypeId`] refers to is not an enum.
    #[must_use]
    pub fn enum_def(&self, type_id: TypeId) -> &EnumDef {
        self.types[type_id].as_enum().expect("type is an enum")
    }

    /// Get mutable reference to [`EnumDef`] for a [`TypeId`].
    ///
    /// # Panics
    /// Panics if type [`TypeId`] refers to is not an enum.
    pub fn enum_def_mut(&mut self, type_id: TypeId) -> &mut EnumDef {
        self.types[type_id].as_enum_mut().expect("type is an enum")
    }

    /// Get iterator over all structs and enums.
    pub fn structs_and_enums(&self) -> impl Iterator<Item = StructOrEnum<'_>> {
        self.types.iter().filter_map(TypeDef::as_struct_or_enum)
    }

    /// Get iterator over all structs.
    pub fn structs(&self) -> impl Iterator<Item = &StructDef> {
        self.types.iter().filter_map(TypeDef::as_struct)
    }

    /// Get iterator over all structs as mutable references.
    pub fn structs_mut(&mut self) -> impl Iterator<Item = &mut StructDef> {
        self.types.iter_mut().filter_map(TypeDef::as_struct_mut)
    }

    /// Get iterator over all enums.
    pub fn enums(&self) -> impl Iterator<Item = &EnumDef> {
        self.types.iter().filter_map(TypeDef::as_enum)
    }

    /// Get iterator over the nodes and choices, which the walk visits.
    pub fn visited(&self) -> impl Iterator<Item = StructOrEnum<'_>> {
        self.structs_and_enums().filter(|type_def| type_def.is_visited())
    }
}
