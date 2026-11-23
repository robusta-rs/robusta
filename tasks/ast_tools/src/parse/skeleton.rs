use syn::{ItemEnum, ItemStruct};

use crate::schema::FileId;

/// "Skeleton" parsed from type definition in source file.
///
/// The bare minimum required to be able to link up the types in the second pass.
pub enum Skeleton {
    Struct(StructSkeleton),
    Enum(EnumSkeleton),
}

impl Skeleton {
    pub fn name(&self) -> &str {
        match self {
            Self::Struct(skeleton) => &skeleton.name,
            Self::Enum(skeleton) => &skeleton.name,
        }
    }
}

pub struct StructSkeleton {
    pub name: String,
    pub file_id: FileId,
    pub is_foreign: bool,
    pub item: ItemStruct,
}

pub struct EnumSkeleton {
    pub name: String,
    pub file_id: FileId,
    pub item: ItemEnum,
}
