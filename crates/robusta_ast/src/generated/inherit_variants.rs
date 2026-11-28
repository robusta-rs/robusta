// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/inherit_variants.rs`.

// Some `TryFrom` impls have a single non-shared variant left for the catch-all arm
#![expect(clippy::match_wildcard_for_single_variants)]

use std::ptr::NonNull;

use crate::ast::*;

impl<'a> TypeDeclaration<'a> {
    /// Return if a [`TypeDeclaration`] is a [`LocalClassOrInterfaceDeclaration`].
    #[inline]
    pub fn is_local_class_or_interface_declaration(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Interface(_) | Self::Enum(_) | Self::Record(_))
    }

    /// Convert a [`TypeDeclaration`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_local_class_or_interface_declaration(self) -> LocalClassOrInterfaceDeclaration<'a> {
        LocalClassOrInterfaceDeclaration::try_from(self).unwrap()
    }

    /// Convert a [`&TypeDeclaration`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&TypeDeclaration`]: TypeDeclaration
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<&LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_ref(self).cast::<LocalClassOrInterfaceDeclaration>().as_ref()
            })
        } else {
            None
        }
    }

    /// Convert a [`&mut TypeDeclaration`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> Option<&mut LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_mut(self).cast::<LocalClassOrInterfaceDeclaration>().as_mut()
            })
        } else {
            None
        }
    }

    /// Convert a [`&TypeDeclaration`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&TypeDeclaration`]: TypeDeclaration
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration(&self) -> &LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration().unwrap()
    }

    /// Convert a [`&mut TypeDeclaration`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> &mut LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration_mut().unwrap()
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Convert a [`&LocalClassOrInterfaceDeclaration`] to a [`&TypeDeclaration`].
    ///
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration(&self) -> &TypeDeclaration<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<TypeDeclaration>().as_ref() }
    }
}

impl<'a> TryFrom<TypeDeclaration<'a>> for LocalClassOrInterfaceDeclaration<'a> {
    type Error = ();

    /// Convert a [`TypeDeclaration`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: TypeDeclaration<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            TypeDeclaration::Class(o) => Ok(LocalClassOrInterfaceDeclaration::Class(o)),
            TypeDeclaration::Interface(o) => Ok(LocalClassOrInterfaceDeclaration::Interface(o)),
            TypeDeclaration::Enum(o) => Ok(LocalClassOrInterfaceDeclaration::Enum(o)),
            TypeDeclaration::Record(o) => Ok(LocalClassOrInterfaceDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<LocalClassOrInterfaceDeclaration<'a>> for TypeDeclaration<'a> {
    /// Convert a [`LocalClassOrInterfaceDeclaration`] to a [`TypeDeclaration`].
    #[inline]
    fn from(value: LocalClassOrInterfaceDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            LocalClassOrInterfaceDeclaration::Class(o) => TypeDeclaration::Class(o),
            LocalClassOrInterfaceDeclaration::Interface(o) => TypeDeclaration::Interface(o),
            LocalClassOrInterfaceDeclaration::Enum(o) => TypeDeclaration::Enum(o),
            LocalClassOrInterfaceDeclaration::Record(o) => TypeDeclaration::Record(o),
        }
    }
}

impl<'a> AnnotationInterfaceMemberDeclaration<'a> {
    /// Return if an [`AnnotationInterfaceMemberDeclaration`] is a [`TypeDeclaration`].
    #[inline]
    pub fn is_type_declaration(&self) -> bool {
        matches!(
            self,
            Self::Empty(_)
                | Self::AnnotationInterface(_)
                | Self::Class(_)
                | Self::Interface(_)
                | Self::Enum(_)
                | Self::Record(_)
        )
    }

    /// Convert an [`AnnotationInterfaceMemberDeclaration`] to a [`TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type_declaration(self) -> TypeDeclaration<'a> {
        TypeDeclaration::try_from(self).unwrap()
    }

    /// Convert an [`&AnnotationInterfaceMemberDeclaration`] to a [`&TypeDeclaration`].
    ///
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration(&self) -> Option<&TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<TypeDeclaration>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut AnnotationInterfaceMemberDeclaration`] to a [`&mut TypeDeclaration`].
    ///
    /// [`&mut AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration_mut(&mut self) -> Option<&mut TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<TypeDeclaration>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&AnnotationInterfaceMemberDeclaration`] to a [`&TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration(&self) -> &TypeDeclaration<'a> {
        self.as_type_declaration().unwrap()
    }

    /// Convert an [`&mut AnnotationInterfaceMemberDeclaration`] to a [`&mut TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration_mut(&mut self) -> &mut TypeDeclaration<'a> {
        self.as_type_declaration_mut().unwrap()
    }
}

impl<'a> TypeDeclaration<'a> {
    /// Convert a [`&TypeDeclaration`] to an [`&AnnotationInterfaceMemberDeclaration`].
    ///
    /// [`&TypeDeclaration`]: TypeDeclaration
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    #[inline]
    pub fn as_annotation_interface_member_declaration(
        &self,
    ) -> &AnnotationInterfaceMemberDeclaration<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<AnnotationInterfaceMemberDeclaration>().as_ref() }
    }
}

impl<'a> TryFrom<AnnotationInterfaceMemberDeclaration<'a>> for TypeDeclaration<'a> {
    type Error = ();

    /// Convert an [`AnnotationInterfaceMemberDeclaration`] to a [`TypeDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: AnnotationInterfaceMemberDeclaration<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            AnnotationInterfaceMemberDeclaration::Empty(o) => Ok(TypeDeclaration::Empty(o)),
            AnnotationInterfaceMemberDeclaration::AnnotationInterface(o) => {
                Ok(TypeDeclaration::AnnotationInterface(o))
            }
            AnnotationInterfaceMemberDeclaration::Class(o) => Ok(TypeDeclaration::Class(o)),
            AnnotationInterfaceMemberDeclaration::Interface(o) => Ok(TypeDeclaration::Interface(o)),
            AnnotationInterfaceMemberDeclaration::Enum(o) => Ok(TypeDeclaration::Enum(o)),
            AnnotationInterfaceMemberDeclaration::Record(o) => Ok(TypeDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<TypeDeclaration<'a>> for AnnotationInterfaceMemberDeclaration<'a> {
    /// Convert a [`TypeDeclaration`] to an [`AnnotationInterfaceMemberDeclaration`].
    #[inline]
    fn from(value: TypeDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            TypeDeclaration::Empty(o) => AnnotationInterfaceMemberDeclaration::Empty(o),
            TypeDeclaration::AnnotationInterface(o) => {
                AnnotationInterfaceMemberDeclaration::AnnotationInterface(o)
            }
            TypeDeclaration::Class(o) => AnnotationInterfaceMemberDeclaration::Class(o),
            TypeDeclaration::Interface(o) => AnnotationInterfaceMemberDeclaration::Interface(o),
            TypeDeclaration::Enum(o) => AnnotationInterfaceMemberDeclaration::Enum(o),
            TypeDeclaration::Record(o) => AnnotationInterfaceMemberDeclaration::Record(o),
        }
    }
}

impl<'a> AnnotationInterfaceMemberDeclaration<'a> {
    /// Return if an [`AnnotationInterfaceMemberDeclaration`] is a [`LocalClassOrInterfaceDeclaration`].
    #[inline]
    pub fn is_local_class_or_interface_declaration(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Interface(_) | Self::Enum(_) | Self::Record(_))
    }

    /// Convert an [`AnnotationInterfaceMemberDeclaration`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_local_class_or_interface_declaration(self) -> LocalClassOrInterfaceDeclaration<'a> {
        LocalClassOrInterfaceDeclaration::try_from(self).unwrap()
    }

    /// Convert an [`&AnnotationInterfaceMemberDeclaration`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<&LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_ref(self).cast::<LocalClassOrInterfaceDeclaration>().as_ref()
            })
        } else {
            None
        }
    }

    /// Convert an [`&mut AnnotationInterfaceMemberDeclaration`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&mut AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> Option<&mut LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_mut(self).cast::<LocalClassOrInterfaceDeclaration>().as_mut()
            })
        } else {
            None
        }
    }

    /// Convert an [`&AnnotationInterfaceMemberDeclaration`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration(&self) -> &LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration().unwrap()
    }

    /// Convert an [`&mut AnnotationInterfaceMemberDeclaration`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> &mut LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration_mut().unwrap()
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Convert a [`&LocalClassOrInterfaceDeclaration`] to an [`&AnnotationInterfaceMemberDeclaration`].
    ///
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    /// [`&AnnotationInterfaceMemberDeclaration`]: AnnotationInterfaceMemberDeclaration
    #[inline]
    pub fn as_annotation_interface_member_declaration(
        &self,
    ) -> &AnnotationInterfaceMemberDeclaration<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<AnnotationInterfaceMemberDeclaration>().as_ref() }
    }
}

impl<'a> TryFrom<AnnotationInterfaceMemberDeclaration<'a>>
    for LocalClassOrInterfaceDeclaration<'a>
{
    type Error = ();

    /// Convert an [`AnnotationInterfaceMemberDeclaration`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: AnnotationInterfaceMemberDeclaration<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            AnnotationInterfaceMemberDeclaration::Class(o) => {
                Ok(LocalClassOrInterfaceDeclaration::Class(o))
            }
            AnnotationInterfaceMemberDeclaration::Interface(o) => {
                Ok(LocalClassOrInterfaceDeclaration::Interface(o))
            }
            AnnotationInterfaceMemberDeclaration::Enum(o) => {
                Ok(LocalClassOrInterfaceDeclaration::Enum(o))
            }
            AnnotationInterfaceMemberDeclaration::Record(o) => {
                Ok(LocalClassOrInterfaceDeclaration::Record(o))
            }
            _ => Err(()),
        }
    }
}

impl<'a> From<LocalClassOrInterfaceDeclaration<'a>> for AnnotationInterfaceMemberDeclaration<'a> {
    /// Convert a [`LocalClassOrInterfaceDeclaration`] to an [`AnnotationInterfaceMemberDeclaration`].
    #[inline]
    fn from(value: LocalClassOrInterfaceDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            LocalClassOrInterfaceDeclaration::Class(o) => {
                AnnotationInterfaceMemberDeclaration::Class(o)
            }
            LocalClassOrInterfaceDeclaration::Interface(o) => {
                AnnotationInterfaceMemberDeclaration::Interface(o)
            }
            LocalClassOrInterfaceDeclaration::Enum(o) => {
                AnnotationInterfaceMemberDeclaration::Enum(o)
            }
            LocalClassOrInterfaceDeclaration::Record(o) => {
                AnnotationInterfaceMemberDeclaration::Record(o)
            }
        }
    }
}

impl<'a> ClassMember<'a> {
    /// Return if a [`ClassMember`] is a [`TypeDeclaration`].
    #[inline]
    pub fn is_type_declaration(&self) -> bool {
        matches!(
            self,
            Self::Empty(_)
                | Self::AnnotationInterface(_)
                | Self::Class(_)
                | Self::Interface(_)
                | Self::Enum(_)
                | Self::Record(_)
        )
    }

    /// Convert a [`ClassMember`] to a [`TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type_declaration(self) -> TypeDeclaration<'a> {
        TypeDeclaration::try_from(self).unwrap()
    }

    /// Convert a [`&ClassMember`] to a [`&TypeDeclaration`].
    ///
    /// [`&ClassMember`]: ClassMember
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration(&self) -> Option<&TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<TypeDeclaration>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut ClassMember`] to a [`&mut TypeDeclaration`].
    ///
    /// [`&mut ClassMember`]: ClassMember
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration_mut(&mut self) -> Option<&mut TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<TypeDeclaration>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&ClassMember`] to a [`&TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&ClassMember`]: ClassMember
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration(&self) -> &TypeDeclaration<'a> {
        self.as_type_declaration().unwrap()
    }

    /// Convert a [`&mut ClassMember`] to a [`&mut TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut ClassMember`]: ClassMember
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration_mut(&mut self) -> &mut TypeDeclaration<'a> {
        self.as_type_declaration_mut().unwrap()
    }
}

impl<'a> TypeDeclaration<'a> {
    /// Convert a [`&TypeDeclaration`] to a [`&ClassMember`].
    ///
    /// [`&TypeDeclaration`]: TypeDeclaration
    /// [`&ClassMember`]: ClassMember
    #[inline]
    pub fn as_class_member(&self) -> &ClassMember<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<ClassMember>().as_ref() }
    }
}

impl<'a> TryFrom<ClassMember<'a>> for TypeDeclaration<'a> {
    type Error = ();

    /// Convert a [`ClassMember`] to a [`TypeDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: ClassMember<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            ClassMember::Empty(o) => Ok(TypeDeclaration::Empty(o)),
            ClassMember::AnnotationInterface(o) => Ok(TypeDeclaration::AnnotationInterface(o)),
            ClassMember::Class(o) => Ok(TypeDeclaration::Class(o)),
            ClassMember::Interface(o) => Ok(TypeDeclaration::Interface(o)),
            ClassMember::Enum(o) => Ok(TypeDeclaration::Enum(o)),
            ClassMember::Record(o) => Ok(TypeDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<TypeDeclaration<'a>> for ClassMember<'a> {
    /// Convert a [`TypeDeclaration`] to a [`ClassMember`].
    #[inline]
    fn from(value: TypeDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            TypeDeclaration::Empty(o) => ClassMember::Empty(o),
            TypeDeclaration::AnnotationInterface(o) => ClassMember::AnnotationInterface(o),
            TypeDeclaration::Class(o) => ClassMember::Class(o),
            TypeDeclaration::Interface(o) => ClassMember::Interface(o),
            TypeDeclaration::Enum(o) => ClassMember::Enum(o),
            TypeDeclaration::Record(o) => ClassMember::Record(o),
        }
    }
}

impl<'a> ClassMember<'a> {
    /// Return if a [`ClassMember`] is a [`LocalClassOrInterfaceDeclaration`].
    #[inline]
    pub fn is_local_class_or_interface_declaration(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Interface(_) | Self::Enum(_) | Self::Record(_))
    }

    /// Convert a [`ClassMember`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_local_class_or_interface_declaration(self) -> LocalClassOrInterfaceDeclaration<'a> {
        LocalClassOrInterfaceDeclaration::try_from(self).unwrap()
    }

    /// Convert a [`&ClassMember`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&ClassMember`]: ClassMember
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<&LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_ref(self).cast::<LocalClassOrInterfaceDeclaration>().as_ref()
            })
        } else {
            None
        }
    }

    /// Convert a [`&mut ClassMember`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&mut ClassMember`]: ClassMember
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> Option<&mut LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_mut(self).cast::<LocalClassOrInterfaceDeclaration>().as_mut()
            })
        } else {
            None
        }
    }

    /// Convert a [`&ClassMember`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&ClassMember`]: ClassMember
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration(&self) -> &LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration().unwrap()
    }

    /// Convert a [`&mut ClassMember`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut ClassMember`]: ClassMember
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> &mut LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration_mut().unwrap()
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Convert a [`&LocalClassOrInterfaceDeclaration`] to a [`&ClassMember`].
    ///
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    /// [`&ClassMember`]: ClassMember
    #[inline]
    pub fn as_class_member(&self) -> &ClassMember<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<ClassMember>().as_ref() }
    }
}

impl<'a> TryFrom<ClassMember<'a>> for LocalClassOrInterfaceDeclaration<'a> {
    type Error = ();

    /// Convert a [`ClassMember`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: ClassMember<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            ClassMember::Class(o) => Ok(LocalClassOrInterfaceDeclaration::Class(o)),
            ClassMember::Interface(o) => Ok(LocalClassOrInterfaceDeclaration::Interface(o)),
            ClassMember::Enum(o) => Ok(LocalClassOrInterfaceDeclaration::Enum(o)),
            ClassMember::Record(o) => Ok(LocalClassOrInterfaceDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<LocalClassOrInterfaceDeclaration<'a>> for ClassMember<'a> {
    /// Convert a [`LocalClassOrInterfaceDeclaration`] to a [`ClassMember`].
    #[inline]
    fn from(value: LocalClassOrInterfaceDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            LocalClassOrInterfaceDeclaration::Class(o) => ClassMember::Class(o),
            LocalClassOrInterfaceDeclaration::Interface(o) => ClassMember::Interface(o),
            LocalClassOrInterfaceDeclaration::Enum(o) => ClassMember::Enum(o),
            LocalClassOrInterfaceDeclaration::Record(o) => ClassMember::Record(o),
        }
    }
}

impl<'a> InterfaceMember<'a> {
    /// Return if an [`InterfaceMember`] is a [`TypeDeclaration`].
    #[inline]
    pub fn is_type_declaration(&self) -> bool {
        matches!(
            self,
            Self::Empty(_)
                | Self::AnnotationInterface(_)
                | Self::Class(_)
                | Self::Interface(_)
                | Self::Enum(_)
                | Self::Record(_)
        )
    }

    /// Convert an [`InterfaceMember`] to a [`TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type_declaration(self) -> TypeDeclaration<'a> {
        TypeDeclaration::try_from(self).unwrap()
    }

    /// Convert an [`&InterfaceMember`] to a [`&TypeDeclaration`].
    ///
    /// [`&InterfaceMember`]: InterfaceMember
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration(&self) -> Option<&TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<TypeDeclaration>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut InterfaceMember`] to a [`&mut TypeDeclaration`].
    ///
    /// [`&mut InterfaceMember`]: InterfaceMember
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn as_type_declaration_mut(&mut self) -> Option<&mut TypeDeclaration<'a>> {
        if self.is_type_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<TypeDeclaration>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&InterfaceMember`] to a [`&TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&InterfaceMember`]: InterfaceMember
    /// [`&TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration(&self) -> &TypeDeclaration<'a> {
        self.as_type_declaration().unwrap()
    }

    /// Convert an [`&mut InterfaceMember`] to a [`&mut TypeDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut InterfaceMember`]: InterfaceMember
    /// [`&mut TypeDeclaration`]: TypeDeclaration
    #[inline]
    pub fn to_type_declaration_mut(&mut self) -> &mut TypeDeclaration<'a> {
        self.as_type_declaration_mut().unwrap()
    }
}

impl<'a> TypeDeclaration<'a> {
    /// Convert a [`&TypeDeclaration`] to an [`&InterfaceMember`].
    ///
    /// [`&TypeDeclaration`]: TypeDeclaration
    /// [`&InterfaceMember`]: InterfaceMember
    #[inline]
    pub fn as_interface_member(&self) -> &InterfaceMember<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<InterfaceMember>().as_ref() }
    }
}

impl<'a> TryFrom<InterfaceMember<'a>> for TypeDeclaration<'a> {
    type Error = ();

    /// Convert an [`InterfaceMember`] to a [`TypeDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: InterfaceMember<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            InterfaceMember::Empty(o) => Ok(TypeDeclaration::Empty(o)),
            InterfaceMember::AnnotationInterface(o) => Ok(TypeDeclaration::AnnotationInterface(o)),
            InterfaceMember::Class(o) => Ok(TypeDeclaration::Class(o)),
            InterfaceMember::Interface(o) => Ok(TypeDeclaration::Interface(o)),
            InterfaceMember::Enum(o) => Ok(TypeDeclaration::Enum(o)),
            InterfaceMember::Record(o) => Ok(TypeDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<TypeDeclaration<'a>> for InterfaceMember<'a> {
    /// Convert a [`TypeDeclaration`] to an [`InterfaceMember`].
    #[inline]
    fn from(value: TypeDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            TypeDeclaration::Empty(o) => InterfaceMember::Empty(o),
            TypeDeclaration::AnnotationInterface(o) => InterfaceMember::AnnotationInterface(o),
            TypeDeclaration::Class(o) => InterfaceMember::Class(o),
            TypeDeclaration::Interface(o) => InterfaceMember::Interface(o),
            TypeDeclaration::Enum(o) => InterfaceMember::Enum(o),
            TypeDeclaration::Record(o) => InterfaceMember::Record(o),
        }
    }
}

impl<'a> InterfaceMember<'a> {
    /// Return if an [`InterfaceMember`] is a [`LocalClassOrInterfaceDeclaration`].
    #[inline]
    pub fn is_local_class_or_interface_declaration(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Interface(_) | Self::Enum(_) | Self::Record(_))
    }

    /// Convert an [`InterfaceMember`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_local_class_or_interface_declaration(self) -> LocalClassOrInterfaceDeclaration<'a> {
        LocalClassOrInterfaceDeclaration::try_from(self).unwrap()
    }

    /// Convert an [`&InterfaceMember`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&InterfaceMember`]: InterfaceMember
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<&LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_ref(self).cast::<LocalClassOrInterfaceDeclaration>().as_ref()
            })
        } else {
            None
        }
    }

    /// Convert an [`&mut InterfaceMember`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&mut InterfaceMember`]: InterfaceMember
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> Option<&mut LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_mut(self).cast::<LocalClassOrInterfaceDeclaration>().as_mut()
            })
        } else {
            None
        }
    }

    /// Convert an [`&InterfaceMember`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&InterfaceMember`]: InterfaceMember
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration(&self) -> &LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration().unwrap()
    }

    /// Convert an [`&mut InterfaceMember`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut InterfaceMember`]: InterfaceMember
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> &mut LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration_mut().unwrap()
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Convert a [`&LocalClassOrInterfaceDeclaration`] to an [`&InterfaceMember`].
    ///
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    /// [`&InterfaceMember`]: InterfaceMember
    #[inline]
    pub fn as_interface_member(&self) -> &InterfaceMember<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<InterfaceMember>().as_ref() }
    }
}

impl<'a> TryFrom<InterfaceMember<'a>> for LocalClassOrInterfaceDeclaration<'a> {
    type Error = ();

    /// Convert an [`InterfaceMember`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: InterfaceMember<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            InterfaceMember::Class(o) => Ok(LocalClassOrInterfaceDeclaration::Class(o)),
            InterfaceMember::Interface(o) => Ok(LocalClassOrInterfaceDeclaration::Interface(o)),
            InterfaceMember::Enum(o) => Ok(LocalClassOrInterfaceDeclaration::Enum(o)),
            InterfaceMember::Record(o) => Ok(LocalClassOrInterfaceDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<LocalClassOrInterfaceDeclaration<'a>> for InterfaceMember<'a> {
    /// Convert a [`LocalClassOrInterfaceDeclaration`] to an [`InterfaceMember`].
    #[inline]
    fn from(value: LocalClassOrInterfaceDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            LocalClassOrInterfaceDeclaration::Class(o) => InterfaceMember::Class(o),
            LocalClassOrInterfaceDeclaration::Interface(o) => InterfaceMember::Interface(o),
            LocalClassOrInterfaceDeclaration::Enum(o) => InterfaceMember::Enum(o),
            LocalClassOrInterfaceDeclaration::Record(o) => InterfaceMember::Record(o),
        }
    }
}

impl<'a> Statement<'a> {
    /// Return if a [`Statement`] is a [`LocalClassOrInterfaceDeclaration`].
    #[inline]
    pub fn is_local_class_or_interface_declaration(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Interface(_) | Self::Enum(_) | Self::Record(_))
    }

    /// Convert a [`Statement`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_local_class_or_interface_declaration(self) -> LocalClassOrInterfaceDeclaration<'a> {
        LocalClassOrInterfaceDeclaration::try_from(self).unwrap()
    }

    /// Convert a [`&Statement`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&Statement`]: Statement
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<&LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_ref(self).cast::<LocalClassOrInterfaceDeclaration>().as_ref()
            })
        } else {
            None
        }
    }

    /// Convert a [`&mut Statement`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// [`&mut Statement`]: Statement
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn as_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> Option<&mut LocalClassOrInterfaceDeclaration<'a>> {
        if self.is_local_class_or_interface_declaration() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe {
                NonNull::from_mut(self).cast::<LocalClassOrInterfaceDeclaration>().as_mut()
            })
        } else {
            None
        }
    }

    /// Convert a [`&Statement`] to a [`&LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&Statement`]: Statement
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration(&self) -> &LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration().unwrap()
    }

    /// Convert a [`&mut Statement`] to a [`&mut LocalClassOrInterfaceDeclaration`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut Statement`]: Statement
    /// [`&mut LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    #[inline]
    pub fn to_local_class_or_interface_declaration_mut(
        &mut self,
    ) -> &mut LocalClassOrInterfaceDeclaration<'a> {
        self.as_local_class_or_interface_declaration_mut().unwrap()
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Convert a [`&LocalClassOrInterfaceDeclaration`] to a [`&Statement`].
    ///
    /// [`&LocalClassOrInterfaceDeclaration`]: LocalClassOrInterfaceDeclaration
    /// [`&Statement`]: Statement
    #[inline]
    pub fn as_statement(&self) -> &Statement<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<Statement>().as_ref() }
    }
}

impl<'a> TryFrom<Statement<'a>> for LocalClassOrInterfaceDeclaration<'a> {
    type Error = ();

    /// Convert a [`Statement`] to a [`LocalClassOrInterfaceDeclaration`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: Statement<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            Statement::Class(o) => Ok(LocalClassOrInterfaceDeclaration::Class(o)),
            Statement::Interface(o) => Ok(LocalClassOrInterfaceDeclaration::Interface(o)),
            Statement::Enum(o) => Ok(LocalClassOrInterfaceDeclaration::Enum(o)),
            Statement::Record(o) => Ok(LocalClassOrInterfaceDeclaration::Record(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<LocalClassOrInterfaceDeclaration<'a>> for Statement<'a> {
    /// Convert a [`LocalClassOrInterfaceDeclaration`] to a [`Statement`].
    #[inline]
    fn from(value: LocalClassOrInterfaceDeclaration<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            LocalClassOrInterfaceDeclaration::Class(o) => Statement::Class(o),
            LocalClassOrInterfaceDeclaration::Interface(o) => Statement::Interface(o),
            LocalClassOrInterfaceDeclaration::Enum(o) => Statement::Enum(o),
            LocalClassOrInterfaceDeclaration::Record(o) => Statement::Record(o),
        }
    }
}

impl<'a> SwitchRuleBody<'a> {
    /// Return if a [`SwitchRuleBody`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert a [`SwitchRuleBody`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert a [`&SwitchRuleBody`] to an [`&Expression`].
    ///
    /// [`&SwitchRuleBody`]: SwitchRuleBody
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut SwitchRuleBody`] to an [`&mut Expression`].
    ///
    /// [`&mut SwitchRuleBody`]: SwitchRuleBody
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&SwitchRuleBody`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&SwitchRuleBody`]: SwitchRuleBody
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert a [`&mut SwitchRuleBody`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut SwitchRuleBody`]: SwitchRuleBody
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to a [`&SwitchRuleBody`].
    ///
    /// [`&Expression`]: Expression
    /// [`&SwitchRuleBody`]: SwitchRuleBody
    #[inline]
    pub fn as_switch_rule_body(&self) -> &SwitchRuleBody<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<SwitchRuleBody>().as_ref() }
    }
}

impl<'a> TryFrom<SwitchRuleBody<'a>> for Expression<'a> {
    type Error = ();

    /// Convert a [`SwitchRuleBody`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: SwitchRuleBody<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            SwitchRuleBody::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            SwitchRuleBody::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            SwitchRuleBody::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            SwitchRuleBody::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            SwitchRuleBody::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            SwitchRuleBody::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            SwitchRuleBody::This(o) => Ok(Expression::This(o)),
            SwitchRuleBody::Super(o) => Ok(Expression::Super(o)),
            SwitchRuleBody::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            SwitchRuleBody::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            SwitchRuleBody::Identifier(o) => Ok(Expression::Identifier(o)),
            SwitchRuleBody::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            SwitchRuleBody::MethodCall(o) => Ok(Expression::MethodCall(o)),
            SwitchRuleBody::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            SwitchRuleBody::ClassInstanceCreation(o) => Ok(Expression::ClassInstanceCreation(o)),
            SwitchRuleBody::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            SwitchRuleBody::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            SwitchRuleBody::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            SwitchRuleBody::Update(o) => Ok(Expression::Update(o)),
            SwitchRuleBody::Unary(o) => Ok(Expression::Unary(o)),
            SwitchRuleBody::Cast(o) => Ok(Expression::Cast(o)),
            SwitchRuleBody::Instanceof(o) => Ok(Expression::Instanceof(o)),
            SwitchRuleBody::Binary(o) => Ok(Expression::Binary(o)),
            SwitchRuleBody::Assignment(o) => Ok(Expression::Assignment(o)),
            SwitchRuleBody::Conditional(o) => Ok(Expression::Conditional(o)),
            SwitchRuleBody::Switch(o) => Ok(Expression::Switch(o)),
            SwitchRuleBody::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            SwitchRuleBody::Lambda(o) => Ok(Expression::Lambda(o)),
            SwitchRuleBody::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for SwitchRuleBody<'a> {
    /// Convert an [`Expression`] to a [`SwitchRuleBody`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => SwitchRuleBody::IntLiteral(o),
            Expression::FloatLiteral(o) => SwitchRuleBody::FloatLiteral(o),
            Expression::CharLiteral(o) => SwitchRuleBody::CharLiteral(o),
            Expression::StringLiteral(o) => SwitchRuleBody::StringLiteral(o),
            Expression::BooleanLiteral(o) => SwitchRuleBody::BooleanLiteral(o),
            Expression::NullLiteral(o) => SwitchRuleBody::NullLiteral(o),
            Expression::This(o) => SwitchRuleBody::This(o),
            Expression::Super(o) => SwitchRuleBody::Super(o),
            Expression::QualifiedThis(o) => SwitchRuleBody::QualifiedThis(o),
            Expression::QualifiedSuper(o) => SwitchRuleBody::QualifiedSuper(o),
            Expression::Identifier(o) => SwitchRuleBody::Identifier(o),
            Expression::FieldAccess(o) => SwitchRuleBody::FieldAccess(o),
            Expression::MethodCall(o) => SwitchRuleBody::MethodCall(o),
            Expression::ClassLiteral(o) => SwitchRuleBody::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => SwitchRuleBody::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                SwitchRuleBody::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => SwitchRuleBody::ArrayCreation(o),
            Expression::ArrayAccess(o) => SwitchRuleBody::ArrayAccess(o),
            Expression::Update(o) => SwitchRuleBody::Update(o),
            Expression::Unary(o) => SwitchRuleBody::Unary(o),
            Expression::Cast(o) => SwitchRuleBody::Cast(o),
            Expression::Instanceof(o) => SwitchRuleBody::Instanceof(o),
            Expression::Binary(o) => SwitchRuleBody::Binary(o),
            Expression::Assignment(o) => SwitchRuleBody::Assignment(o),
            Expression::Conditional(o) => SwitchRuleBody::Conditional(o),
            Expression::Switch(o) => SwitchRuleBody::Switch(o),
            Expression::Parenthesized(o) => SwitchRuleBody::Parenthesized(o),
            Expression::Lambda(o) => SwitchRuleBody::Lambda(o),
            Expression::MethodReference(o) => SwitchRuleBody::MethodReference(o),
        }
    }
}

impl<'a> Resource<'a> {
    /// Return if a [`Resource`] is a [`ResourceAccess`].
    #[inline]
    pub fn is_resource_access(&self) -> bool {
        matches!(self, Self::Identifier(_) | Self::Field(_))
    }

    /// Convert a [`Resource`] to a [`ResourceAccess`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_resource_access(self) -> ResourceAccess<'a> {
        ResourceAccess::try_from(self).unwrap()
    }

    /// Convert a [`&Resource`] to a [`&ResourceAccess`].
    ///
    /// [`&Resource`]: Resource
    /// [`&ResourceAccess`]: ResourceAccess
    #[inline]
    pub fn as_resource_access(&self) -> Option<&ResourceAccess<'a>> {
        if self.is_resource_access() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ResourceAccess>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut Resource`] to a [`&mut ResourceAccess`].
    ///
    /// [`&mut Resource`]: Resource
    /// [`&mut ResourceAccess`]: ResourceAccess
    #[inline]
    pub fn as_resource_access_mut(&mut self) -> Option<&mut ResourceAccess<'a>> {
        if self.is_resource_access() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ResourceAccess>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&Resource`] to a [`&ResourceAccess`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&Resource`]: Resource
    /// [`&ResourceAccess`]: ResourceAccess
    #[inline]
    pub fn to_resource_access(&self) -> &ResourceAccess<'a> {
        self.as_resource_access().unwrap()
    }

    /// Convert a [`&mut Resource`] to a [`&mut ResourceAccess`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut Resource`]: Resource
    /// [`&mut ResourceAccess`]: ResourceAccess
    #[inline]
    pub fn to_resource_access_mut(&mut self) -> &mut ResourceAccess<'a> {
        self.as_resource_access_mut().unwrap()
    }
}

impl<'a> ResourceAccess<'a> {
    /// Convert a [`&ResourceAccess`] to a [`&Resource`].
    ///
    /// [`&ResourceAccess`]: ResourceAccess
    /// [`&Resource`]: Resource
    #[inline]
    pub fn as_resource(&self) -> &Resource<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<Resource>().as_ref() }
    }
}

impl<'a> TryFrom<Resource<'a>> for ResourceAccess<'a> {
    type Error = ();

    /// Convert a [`Resource`] to a [`ResourceAccess`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: Resource<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            Resource::Identifier(o) => Ok(ResourceAccess::Identifier(o)),
            Resource::Field(o) => Ok(ResourceAccess::Field(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ResourceAccess<'a>> for Resource<'a> {
    /// Convert a [`ResourceAccess`] to a [`Resource`].
    #[inline]
    fn from(value: ResourceAccess<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ResourceAccess::Identifier(o) => Resource::Identifier(o),
            ResourceAccess::Field(o) => Resource::Field(o),
        }
    }
}

impl<'a> CatchType<'a> {
    /// Return if a [`CatchType`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`CatchType`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&CatchType`] to a [`&ClassType`].
    ///
    /// [`&CatchType`]: CatchType
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut CatchType`] to a [`&mut ClassType`].
    ///
    /// [`&mut CatchType`]: CatchType
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&CatchType`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&CatchType`]: CatchType
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut CatchType`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut CatchType`]: CatchType
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&CatchType`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&CatchType`]: CatchType
    #[inline]
    pub fn as_catch_type(&self) -> &CatchType<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<CatchType>().as_ref() }
    }
}

impl<'a> TryFrom<CatchType<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`CatchType`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: CatchType<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            CatchType::Simple(o) => Ok(ClassType::Simple(o)),
            CatchType::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for CatchType<'a> {
    /// Convert a [`ClassType`] to a [`CatchType`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => CatchType::Simple(o),
            ClassType::Qualified(o) => CatchType::Qualified(o),
        }
    }
}

impl<'a> VariableInitializer<'a> {
    /// Return if a [`VariableInitializer`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert a [`VariableInitializer`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert a [`&VariableInitializer`] to an [`&Expression`].
    ///
    /// [`&VariableInitializer`]: VariableInitializer
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut VariableInitializer`] to an [`&mut Expression`].
    ///
    /// [`&mut VariableInitializer`]: VariableInitializer
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&VariableInitializer`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&VariableInitializer`]: VariableInitializer
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert a [`&mut VariableInitializer`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut VariableInitializer`]: VariableInitializer
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to a [`&VariableInitializer`].
    ///
    /// [`&Expression`]: Expression
    /// [`&VariableInitializer`]: VariableInitializer
    #[inline]
    pub fn as_variable_initializer(&self) -> &VariableInitializer<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<VariableInitializer>().as_ref() }
    }
}

impl<'a> TryFrom<VariableInitializer<'a>> for Expression<'a> {
    type Error = ();

    /// Convert a [`VariableInitializer`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: VariableInitializer<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            VariableInitializer::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            VariableInitializer::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            VariableInitializer::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            VariableInitializer::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            VariableInitializer::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            VariableInitializer::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            VariableInitializer::This(o) => Ok(Expression::This(o)),
            VariableInitializer::Super(o) => Ok(Expression::Super(o)),
            VariableInitializer::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            VariableInitializer::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            VariableInitializer::Identifier(o) => Ok(Expression::Identifier(o)),
            VariableInitializer::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            VariableInitializer::MethodCall(o) => Ok(Expression::MethodCall(o)),
            VariableInitializer::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            VariableInitializer::ClassInstanceCreation(o) => {
                Ok(Expression::ClassInstanceCreation(o))
            }
            VariableInitializer::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            VariableInitializer::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            VariableInitializer::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            VariableInitializer::Update(o) => Ok(Expression::Update(o)),
            VariableInitializer::Unary(o) => Ok(Expression::Unary(o)),
            VariableInitializer::Cast(o) => Ok(Expression::Cast(o)),
            VariableInitializer::Instanceof(o) => Ok(Expression::Instanceof(o)),
            VariableInitializer::Binary(o) => Ok(Expression::Binary(o)),
            VariableInitializer::Assignment(o) => Ok(Expression::Assignment(o)),
            VariableInitializer::Conditional(o) => Ok(Expression::Conditional(o)),
            VariableInitializer::Switch(o) => Ok(Expression::Switch(o)),
            VariableInitializer::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            VariableInitializer::Lambda(o) => Ok(Expression::Lambda(o)),
            VariableInitializer::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for VariableInitializer<'a> {
    /// Convert an [`Expression`] to a [`VariableInitializer`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => VariableInitializer::IntLiteral(o),
            Expression::FloatLiteral(o) => VariableInitializer::FloatLiteral(o),
            Expression::CharLiteral(o) => VariableInitializer::CharLiteral(o),
            Expression::StringLiteral(o) => VariableInitializer::StringLiteral(o),
            Expression::BooleanLiteral(o) => VariableInitializer::BooleanLiteral(o),
            Expression::NullLiteral(o) => VariableInitializer::NullLiteral(o),
            Expression::This(o) => VariableInitializer::This(o),
            Expression::Super(o) => VariableInitializer::Super(o),
            Expression::QualifiedThis(o) => VariableInitializer::QualifiedThis(o),
            Expression::QualifiedSuper(o) => VariableInitializer::QualifiedSuper(o),
            Expression::Identifier(o) => VariableInitializer::Identifier(o),
            Expression::FieldAccess(o) => VariableInitializer::FieldAccess(o),
            Expression::MethodCall(o) => VariableInitializer::MethodCall(o),
            Expression::ClassLiteral(o) => VariableInitializer::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => VariableInitializer::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                VariableInitializer::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => VariableInitializer::ArrayCreation(o),
            Expression::ArrayAccess(o) => VariableInitializer::ArrayAccess(o),
            Expression::Update(o) => VariableInitializer::Update(o),
            Expression::Unary(o) => VariableInitializer::Unary(o),
            Expression::Cast(o) => VariableInitializer::Cast(o),
            Expression::Instanceof(o) => VariableInitializer::Instanceof(o),
            Expression::Binary(o) => VariableInitializer::Binary(o),
            Expression::Assignment(o) => VariableInitializer::Assignment(o),
            Expression::Conditional(o) => VariableInitializer::Conditional(o),
            Expression::Switch(o) => VariableInitializer::Switch(o),
            Expression::Parenthesized(o) => VariableInitializer::Parenthesized(o),
            Expression::Lambda(o) => VariableInitializer::Lambda(o),
            Expression::MethodReference(o) => VariableInitializer::MethodReference(o),
        }
    }
}

impl<'a> InstanceofRhs<'a> {
    /// Return if an [`InstanceofRhs`] is a [`Type`].
    #[inline]
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Primitive(_) | Self::Array(_) | Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert an [`InstanceofRhs`] to a [`Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type(self) -> Type<'a> {
        Type::try_from(self).unwrap()
    }

    /// Convert an [`&InstanceofRhs`] to a [`&Type`].
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> Option<&Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut Type`].
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut Type`]: Type
    #[inline]
    pub fn as_type_mut(&mut self) -> Option<&mut Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Type>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&InstanceofRhs`] to a [`&Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&Type`]: Type
    #[inline]
    pub fn to_type(&self) -> &Type<'a> {
        self.as_type().unwrap()
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut Type`]: Type
    #[inline]
    pub fn to_type_mut(&mut self) -> &mut Type<'a> {
        self.as_type_mut().unwrap()
    }
}

impl<'a> Type<'a> {
    /// Convert a [`&Type`] to an [`&InstanceofRhs`].
    ///
    /// [`&Type`]: Type
    /// [`&InstanceofRhs`]: InstanceofRhs
    #[inline]
    pub fn as_instanceof_rhs(&self) -> &InstanceofRhs<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<InstanceofRhs>().as_ref() }
    }
}

impl<'a> TryFrom<InstanceofRhs<'a>> for Type<'a> {
    type Error = ();

    /// Convert an [`InstanceofRhs`] to a [`Type`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: InstanceofRhs<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            InstanceofRhs::Primitive(o) => Ok(Type::Primitive(o)),
            InstanceofRhs::Array(o) => Ok(Type::Array(o)),
            InstanceofRhs::Simple(o) => Ok(Type::Simple(o)),
            InstanceofRhs::Qualified(o) => Ok(Type::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Type<'a>> for InstanceofRhs<'a> {
    /// Convert a [`Type`] to an [`InstanceofRhs`].
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Type::Primitive(o) => InstanceofRhs::Primitive(o),
            Type::Array(o) => InstanceofRhs::Array(o),
            Type::Simple(o) => InstanceofRhs::Simple(o),
            Type::Qualified(o) => InstanceofRhs::Qualified(o),
        }
    }
}

impl<'a> InstanceofRhs<'a> {
    /// Return if an [`InstanceofRhs`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert an [`InstanceofRhs`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert an [`&InstanceofRhs`] to a [`&ClassType`].
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut ClassType`].
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&InstanceofRhs`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to an [`&InstanceofRhs`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&InstanceofRhs`]: InstanceofRhs
    #[inline]
    pub fn as_instanceof_rhs(&self) -> &InstanceofRhs<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<InstanceofRhs>().as_ref() }
    }
}

impl<'a> TryFrom<InstanceofRhs<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert an [`InstanceofRhs`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: InstanceofRhs<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            InstanceofRhs::Simple(o) => Ok(ClassType::Simple(o)),
            InstanceofRhs::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for InstanceofRhs<'a> {
    /// Convert a [`ClassType`] to an [`InstanceofRhs`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => InstanceofRhs::Simple(o),
            ClassType::Qualified(o) => InstanceofRhs::Qualified(o),
        }
    }
}

impl<'a> InstanceofRhs<'a> {
    /// Return if an [`InstanceofRhs`] is a [`Pattern`].
    #[inline]
    pub fn is_pattern(&self) -> bool {
        matches!(self, Self::TypePattern(_) | Self::RecordPattern(_))
    }

    /// Convert an [`InstanceofRhs`] to a [`Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_pattern(self) -> Pattern<'a> {
        Pattern::try_from(self).unwrap()
    }

    /// Convert an [`&InstanceofRhs`] to a [`&Pattern`].
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&Pattern`]: Pattern
    #[inline]
    pub fn as_pattern(&self) -> Option<&Pattern<'a>> {
        if self.is_pattern() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Pattern>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut Pattern`].
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut Pattern`]: Pattern
    #[inline]
    pub fn as_pattern_mut(&mut self) -> Option<&mut Pattern<'a>> {
        if self.is_pattern() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Pattern>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&InstanceofRhs`] to a [`&Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&InstanceofRhs`]: InstanceofRhs
    /// [`&Pattern`]: Pattern
    #[inline]
    pub fn to_pattern(&self) -> &Pattern<'a> {
        self.as_pattern().unwrap()
    }

    /// Convert an [`&mut InstanceofRhs`] to a [`&mut Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut InstanceofRhs`]: InstanceofRhs
    /// [`&mut Pattern`]: Pattern
    #[inline]
    pub fn to_pattern_mut(&mut self) -> &mut Pattern<'a> {
        self.as_pattern_mut().unwrap()
    }
}

impl<'a> Pattern<'a> {
    /// Convert a [`&Pattern`] to an [`&InstanceofRhs`].
    ///
    /// [`&Pattern`]: Pattern
    /// [`&InstanceofRhs`]: InstanceofRhs
    #[inline]
    pub fn as_instanceof_rhs(&self) -> &InstanceofRhs<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<InstanceofRhs>().as_ref() }
    }
}

impl<'a> TryFrom<InstanceofRhs<'a>> for Pattern<'a> {
    type Error = ();

    /// Convert an [`InstanceofRhs`] to a [`Pattern`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: InstanceofRhs<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            InstanceofRhs::TypePattern(o) => Ok(Pattern::TypePattern(o)),
            InstanceofRhs::RecordPattern(o) => Ok(Pattern::RecordPattern(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Pattern<'a>> for InstanceofRhs<'a> {
    /// Convert a [`Pattern`] to an [`InstanceofRhs`].
    #[inline]
    fn from(value: Pattern<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Pattern::TypePattern(o) => InstanceofRhs::TypePattern(o),
            Pattern::RecordPattern(o) => InstanceofRhs::RecordPattern(o),
        }
    }
}

impl<'a> ComponentPattern<'a> {
    /// Return if a [`ComponentPattern`] is a [`Pattern`].
    #[inline]
    pub fn is_pattern(&self) -> bool {
        matches!(self, Self::TypePattern(_) | Self::RecordPattern(_))
    }

    /// Convert a [`ComponentPattern`] to a [`Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_pattern(self) -> Pattern<'a> {
        Pattern::try_from(self).unwrap()
    }

    /// Convert a [`&ComponentPattern`] to a [`&Pattern`].
    ///
    /// [`&ComponentPattern`]: ComponentPattern
    /// [`&Pattern`]: Pattern
    #[inline]
    pub fn as_pattern(&self) -> Option<&Pattern<'a>> {
        if self.is_pattern() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Pattern>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut ComponentPattern`] to a [`&mut Pattern`].
    ///
    /// [`&mut ComponentPattern`]: ComponentPattern
    /// [`&mut Pattern`]: Pattern
    #[inline]
    pub fn as_pattern_mut(&mut self) -> Option<&mut Pattern<'a>> {
        if self.is_pattern() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Pattern>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&ComponentPattern`] to a [`&Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&ComponentPattern`]: ComponentPattern
    /// [`&Pattern`]: Pattern
    #[inline]
    pub fn to_pattern(&self) -> &Pattern<'a> {
        self.as_pattern().unwrap()
    }

    /// Convert a [`&mut ComponentPattern`] to a [`&mut Pattern`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut ComponentPattern`]: ComponentPattern
    /// [`&mut Pattern`]: Pattern
    #[inline]
    pub fn to_pattern_mut(&mut self) -> &mut Pattern<'a> {
        self.as_pattern_mut().unwrap()
    }
}

impl<'a> Pattern<'a> {
    /// Convert a [`&Pattern`] to a [`&ComponentPattern`].
    ///
    /// [`&Pattern`]: Pattern
    /// [`&ComponentPattern`]: ComponentPattern
    #[inline]
    pub fn as_component_pattern(&self) -> &ComponentPattern<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<ComponentPattern>().as_ref() }
    }
}

impl<'a> TryFrom<ComponentPattern<'a>> for Pattern<'a> {
    type Error = ();

    /// Convert a [`ComponentPattern`] to a [`Pattern`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: ComponentPattern<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            ComponentPattern::TypePattern(o) => Ok(Pattern::TypePattern(o)),
            ComponentPattern::RecordPattern(o) => Ok(Pattern::RecordPattern(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Pattern<'a>> for ComponentPattern<'a> {
    /// Convert a [`Pattern`] to a [`ComponentPattern`].
    #[inline]
    fn from(value: Pattern<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Pattern::TypePattern(o) => ComponentPattern::TypePattern(o),
            Pattern::RecordPattern(o) => ComponentPattern::RecordPattern(o),
        }
    }
}

impl<'a> LambdaBody<'a> {
    /// Return if a [`LambdaBody`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert a [`LambdaBody`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert a [`&LambdaBody`] to an [`&Expression`].
    ///
    /// [`&LambdaBody`]: LambdaBody
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut LambdaBody`] to an [`&mut Expression`].
    ///
    /// [`&mut LambdaBody`]: LambdaBody
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&LambdaBody`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&LambdaBody`]: LambdaBody
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert a [`&mut LambdaBody`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut LambdaBody`]: LambdaBody
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to a [`&LambdaBody`].
    ///
    /// [`&Expression`]: Expression
    /// [`&LambdaBody`]: LambdaBody
    #[inline]
    pub fn as_lambda_body(&self) -> &LambdaBody<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<LambdaBody>().as_ref() }
    }
}

impl<'a> TryFrom<LambdaBody<'a>> for Expression<'a> {
    type Error = ();

    /// Convert a [`LambdaBody`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: LambdaBody<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            LambdaBody::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            LambdaBody::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            LambdaBody::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            LambdaBody::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            LambdaBody::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            LambdaBody::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            LambdaBody::This(o) => Ok(Expression::This(o)),
            LambdaBody::Super(o) => Ok(Expression::Super(o)),
            LambdaBody::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            LambdaBody::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            LambdaBody::Identifier(o) => Ok(Expression::Identifier(o)),
            LambdaBody::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            LambdaBody::MethodCall(o) => Ok(Expression::MethodCall(o)),
            LambdaBody::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            LambdaBody::ClassInstanceCreation(o) => Ok(Expression::ClassInstanceCreation(o)),
            LambdaBody::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            LambdaBody::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            LambdaBody::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            LambdaBody::Update(o) => Ok(Expression::Update(o)),
            LambdaBody::Unary(o) => Ok(Expression::Unary(o)),
            LambdaBody::Cast(o) => Ok(Expression::Cast(o)),
            LambdaBody::Instanceof(o) => Ok(Expression::Instanceof(o)),
            LambdaBody::Binary(o) => Ok(Expression::Binary(o)),
            LambdaBody::Assignment(o) => Ok(Expression::Assignment(o)),
            LambdaBody::Conditional(o) => Ok(Expression::Conditional(o)),
            LambdaBody::Switch(o) => Ok(Expression::Switch(o)),
            LambdaBody::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            LambdaBody::Lambda(o) => Ok(Expression::Lambda(o)),
            LambdaBody::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for LambdaBody<'a> {
    /// Convert an [`Expression`] to a [`LambdaBody`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => LambdaBody::IntLiteral(o),
            Expression::FloatLiteral(o) => LambdaBody::FloatLiteral(o),
            Expression::CharLiteral(o) => LambdaBody::CharLiteral(o),
            Expression::StringLiteral(o) => LambdaBody::StringLiteral(o),
            Expression::BooleanLiteral(o) => LambdaBody::BooleanLiteral(o),
            Expression::NullLiteral(o) => LambdaBody::NullLiteral(o),
            Expression::This(o) => LambdaBody::This(o),
            Expression::Super(o) => LambdaBody::Super(o),
            Expression::QualifiedThis(o) => LambdaBody::QualifiedThis(o),
            Expression::QualifiedSuper(o) => LambdaBody::QualifiedSuper(o),
            Expression::Identifier(o) => LambdaBody::Identifier(o),
            Expression::FieldAccess(o) => LambdaBody::FieldAccess(o),
            Expression::MethodCall(o) => LambdaBody::MethodCall(o),
            Expression::ClassLiteral(o) => LambdaBody::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => LambdaBody::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                LambdaBody::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => LambdaBody::ArrayCreation(o),
            Expression::ArrayAccess(o) => LambdaBody::ArrayAccess(o),
            Expression::Update(o) => LambdaBody::Update(o),
            Expression::Unary(o) => LambdaBody::Unary(o),
            Expression::Cast(o) => LambdaBody::Cast(o),
            Expression::Instanceof(o) => LambdaBody::Instanceof(o),
            Expression::Binary(o) => LambdaBody::Binary(o),
            Expression::Assignment(o) => LambdaBody::Assignment(o),
            Expression::Conditional(o) => LambdaBody::Conditional(o),
            Expression::Switch(o) => LambdaBody::Switch(o),
            Expression::Parenthesized(o) => LambdaBody::Parenthesized(o),
            Expression::Lambda(o) => LambdaBody::Lambda(o),
            Expression::MethodReference(o) => LambdaBody::MethodReference(o),
        }
    }
}

impl<'a> MethodReferenceTarget<'a> {
    /// Return if a [`MethodReferenceTarget`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert a [`MethodReferenceTarget`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert a [`&MethodReferenceTarget`] to an [`&Expression`].
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut MethodReferenceTarget`] to an [`&mut Expression`].
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&MethodReferenceTarget`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert a [`&mut MethodReferenceTarget`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to a [`&MethodReferenceTarget`].
    ///
    /// [`&Expression`]: Expression
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    #[inline]
    pub fn as_method_reference_target(&self) -> &MethodReferenceTarget<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<MethodReferenceTarget>().as_ref() }
    }
}

impl<'a> TryFrom<MethodReferenceTarget<'a>> for Expression<'a> {
    type Error = ();

    /// Convert a [`MethodReferenceTarget`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: MethodReferenceTarget<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            MethodReferenceTarget::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            MethodReferenceTarget::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            MethodReferenceTarget::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            MethodReferenceTarget::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            MethodReferenceTarget::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            MethodReferenceTarget::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            MethodReferenceTarget::This(o) => Ok(Expression::This(o)),
            MethodReferenceTarget::Super(o) => Ok(Expression::Super(o)),
            MethodReferenceTarget::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            MethodReferenceTarget::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            MethodReferenceTarget::Identifier(o) => Ok(Expression::Identifier(o)),
            MethodReferenceTarget::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            MethodReferenceTarget::MethodCall(o) => Ok(Expression::MethodCall(o)),
            MethodReferenceTarget::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            MethodReferenceTarget::ClassInstanceCreation(o) => {
                Ok(Expression::ClassInstanceCreation(o))
            }
            MethodReferenceTarget::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            MethodReferenceTarget::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            MethodReferenceTarget::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            MethodReferenceTarget::Update(o) => Ok(Expression::Update(o)),
            MethodReferenceTarget::Unary(o) => Ok(Expression::Unary(o)),
            MethodReferenceTarget::Cast(o) => Ok(Expression::Cast(o)),
            MethodReferenceTarget::Instanceof(o) => Ok(Expression::Instanceof(o)),
            MethodReferenceTarget::Binary(o) => Ok(Expression::Binary(o)),
            MethodReferenceTarget::Assignment(o) => Ok(Expression::Assignment(o)),
            MethodReferenceTarget::Conditional(o) => Ok(Expression::Conditional(o)),
            MethodReferenceTarget::Switch(o) => Ok(Expression::Switch(o)),
            MethodReferenceTarget::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            MethodReferenceTarget::Lambda(o) => Ok(Expression::Lambda(o)),
            MethodReferenceTarget::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for MethodReferenceTarget<'a> {
    /// Convert an [`Expression`] to a [`MethodReferenceTarget`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => MethodReferenceTarget::IntLiteral(o),
            Expression::FloatLiteral(o) => MethodReferenceTarget::FloatLiteral(o),
            Expression::CharLiteral(o) => MethodReferenceTarget::CharLiteral(o),
            Expression::StringLiteral(o) => MethodReferenceTarget::StringLiteral(o),
            Expression::BooleanLiteral(o) => MethodReferenceTarget::BooleanLiteral(o),
            Expression::NullLiteral(o) => MethodReferenceTarget::NullLiteral(o),
            Expression::This(o) => MethodReferenceTarget::This(o),
            Expression::Super(o) => MethodReferenceTarget::Super(o),
            Expression::QualifiedThis(o) => MethodReferenceTarget::QualifiedThis(o),
            Expression::QualifiedSuper(o) => MethodReferenceTarget::QualifiedSuper(o),
            Expression::Identifier(o) => MethodReferenceTarget::Identifier(o),
            Expression::FieldAccess(o) => MethodReferenceTarget::FieldAccess(o),
            Expression::MethodCall(o) => MethodReferenceTarget::MethodCall(o),
            Expression::ClassLiteral(o) => MethodReferenceTarget::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => MethodReferenceTarget::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                MethodReferenceTarget::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => MethodReferenceTarget::ArrayCreation(o),
            Expression::ArrayAccess(o) => MethodReferenceTarget::ArrayAccess(o),
            Expression::Update(o) => MethodReferenceTarget::Update(o),
            Expression::Unary(o) => MethodReferenceTarget::Unary(o),
            Expression::Cast(o) => MethodReferenceTarget::Cast(o),
            Expression::Instanceof(o) => MethodReferenceTarget::Instanceof(o),
            Expression::Binary(o) => MethodReferenceTarget::Binary(o),
            Expression::Assignment(o) => MethodReferenceTarget::Assignment(o),
            Expression::Conditional(o) => MethodReferenceTarget::Conditional(o),
            Expression::Switch(o) => MethodReferenceTarget::Switch(o),
            Expression::Parenthesized(o) => MethodReferenceTarget::Parenthesized(o),
            Expression::Lambda(o) => MethodReferenceTarget::Lambda(o),
            Expression::MethodReference(o) => MethodReferenceTarget::MethodReference(o),
        }
    }
}

impl<'a> MethodReferenceTarget<'a> {
    /// Return if a [`MethodReferenceTarget`] is a [`Type`].
    #[inline]
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Primitive(_) | Self::Array(_) | Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`MethodReferenceTarget`] to a [`Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type(self) -> Type<'a> {
        Type::try_from(self).unwrap()
    }

    /// Convert a [`&MethodReferenceTarget`] to a [`&Type`].
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> Option<&Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut MethodReferenceTarget`] to a [`&mut Type`].
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut Type`]: Type
    #[inline]
    pub fn as_type_mut(&mut self) -> Option<&mut Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Type>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&MethodReferenceTarget`] to a [`&Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&Type`]: Type
    #[inline]
    pub fn to_type(&self) -> &Type<'a> {
        self.as_type().unwrap()
    }

    /// Convert a [`&mut MethodReferenceTarget`] to a [`&mut Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut Type`]: Type
    #[inline]
    pub fn to_type_mut(&mut self) -> &mut Type<'a> {
        self.as_type_mut().unwrap()
    }
}

impl<'a> Type<'a> {
    /// Convert a [`&Type`] to a [`&MethodReferenceTarget`].
    ///
    /// [`&Type`]: Type
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    #[inline]
    pub fn as_method_reference_target(&self) -> &MethodReferenceTarget<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<MethodReferenceTarget>().as_ref() }
    }
}

impl<'a> TryFrom<MethodReferenceTarget<'a>> for Type<'a> {
    type Error = ();

    /// Convert a [`MethodReferenceTarget`] to a [`Type`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: MethodReferenceTarget<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            MethodReferenceTarget::Primitive(o) => Ok(Type::Primitive(o)),
            MethodReferenceTarget::Array(o) => Ok(Type::Array(o)),
            MethodReferenceTarget::Simple(o) => Ok(Type::Simple(o)),
            MethodReferenceTarget::Qualified(o) => Ok(Type::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Type<'a>> for MethodReferenceTarget<'a> {
    /// Convert a [`Type`] to a [`MethodReferenceTarget`].
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Type::Primitive(o) => MethodReferenceTarget::Primitive(o),
            Type::Array(o) => MethodReferenceTarget::Array(o),
            Type::Simple(o) => MethodReferenceTarget::Simple(o),
            Type::Qualified(o) => MethodReferenceTarget::Qualified(o),
        }
    }
}

impl<'a> MethodReferenceTarget<'a> {
    /// Return if a [`MethodReferenceTarget`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`MethodReferenceTarget`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&MethodReferenceTarget`] to a [`&ClassType`].
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut MethodReferenceTarget`] to a [`&mut ClassType`].
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&MethodReferenceTarget`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut MethodReferenceTarget`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut MethodReferenceTarget`]: MethodReferenceTarget
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&MethodReferenceTarget`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&MethodReferenceTarget`]: MethodReferenceTarget
    #[inline]
    pub fn as_method_reference_target(&self) -> &MethodReferenceTarget<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<MethodReferenceTarget>().as_ref() }
    }
}

impl<'a> TryFrom<MethodReferenceTarget<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`MethodReferenceTarget`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: MethodReferenceTarget<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            MethodReferenceTarget::Simple(o) => Ok(ClassType::Simple(o)),
            MethodReferenceTarget::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for MethodReferenceTarget<'a> {
    /// Convert a [`ClassType`] to a [`MethodReferenceTarget`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => MethodReferenceTarget::Simple(o),
            ClassType::Qualified(o) => MethodReferenceTarget::Qualified(o),
        }
    }
}

impl<'a> VariableType<'a> {
    /// Return if a [`VariableType`] is a [`Type`].
    #[inline]
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Primitive(_) | Self::Array(_) | Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`VariableType`] to a [`Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type(self) -> Type<'a> {
        Type::try_from(self).unwrap()
    }

    /// Convert a [`&VariableType`] to a [`&Type`].
    ///
    /// [`&VariableType`]: VariableType
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> Option<&Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut VariableType`] to a [`&mut Type`].
    ///
    /// [`&mut VariableType`]: VariableType
    /// [`&mut Type`]: Type
    #[inline]
    pub fn as_type_mut(&mut self) -> Option<&mut Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Type>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&VariableType`] to a [`&Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&VariableType`]: VariableType
    /// [`&Type`]: Type
    #[inline]
    pub fn to_type(&self) -> &Type<'a> {
        self.as_type().unwrap()
    }

    /// Convert a [`&mut VariableType`] to a [`&mut Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut VariableType`]: VariableType
    /// [`&mut Type`]: Type
    #[inline]
    pub fn to_type_mut(&mut self) -> &mut Type<'a> {
        self.as_type_mut().unwrap()
    }
}

impl<'a> Type<'a> {
    /// Convert a [`&Type`] to a [`&VariableType`].
    ///
    /// [`&Type`]: Type
    /// [`&VariableType`]: VariableType
    #[inline]
    pub fn as_variable_type(&self) -> &VariableType<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<VariableType>().as_ref() }
    }
}

impl<'a> TryFrom<VariableType<'a>> for Type<'a> {
    type Error = ();

    /// Convert a [`VariableType`] to a [`Type`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: VariableType<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            VariableType::Primitive(o) => Ok(Type::Primitive(o)),
            VariableType::Array(o) => Ok(Type::Array(o)),
            VariableType::Simple(o) => Ok(Type::Simple(o)),
            VariableType::Qualified(o) => Ok(Type::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Type<'a>> for VariableType<'a> {
    /// Convert a [`Type`] to a [`VariableType`].
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Type::Primitive(o) => VariableType::Primitive(o),
            Type::Array(o) => VariableType::Array(o),
            Type::Simple(o) => VariableType::Simple(o),
            Type::Qualified(o) => VariableType::Qualified(o),
        }
    }
}

impl<'a> VariableType<'a> {
    /// Return if a [`VariableType`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`VariableType`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&VariableType`] to a [`&ClassType`].
    ///
    /// [`&VariableType`]: VariableType
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut VariableType`] to a [`&mut ClassType`].
    ///
    /// [`&mut VariableType`]: VariableType
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&VariableType`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&VariableType`]: VariableType
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut VariableType`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut VariableType`]: VariableType
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&VariableType`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&VariableType`]: VariableType
    #[inline]
    pub fn as_variable_type(&self) -> &VariableType<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<VariableType>().as_ref() }
    }
}

impl<'a> TryFrom<VariableType<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`VariableType`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: VariableType<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            VariableType::Simple(o) => Ok(ClassType::Simple(o)),
            VariableType::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for VariableType<'a> {
    /// Convert a [`ClassType`] to a [`VariableType`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => VariableType::Simple(o),
            ClassType::Qualified(o) => VariableType::Qualified(o),
        }
    }
}

impl<'a> Type<'a> {
    /// Return if a [`Type`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`Type`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&Type`] to a [`&ClassType`].
    ///
    /// [`&Type`]: Type
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut Type`] to a [`&mut ClassType`].
    ///
    /// [`&mut Type`]: Type
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&Type`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&Type`]: Type
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut Type`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut Type`]: Type
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&Type`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> &Type<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() }
    }
}

impl<'a> TryFrom<Type<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`Type`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: Type<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            Type::Simple(o) => Ok(ClassType::Simple(o)),
            Type::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for Type<'a> {
    /// Convert a [`ClassType`] to a [`Type`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => Type::Simple(o),
            ClassType::Qualified(o) => Type::Qualified(o),
        }
    }
}

impl<'a> MethodResult<'a> {
    /// Return if a [`MethodResult`] is a [`Type`].
    #[inline]
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Primitive(_) | Self::Array(_) | Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`MethodResult`] to a [`Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type(self) -> Type<'a> {
        Type::try_from(self).unwrap()
    }

    /// Convert a [`&MethodResult`] to a [`&Type`].
    ///
    /// [`&MethodResult`]: MethodResult
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> Option<&Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut MethodResult`] to a [`&mut Type`].
    ///
    /// [`&mut MethodResult`]: MethodResult
    /// [`&mut Type`]: Type
    #[inline]
    pub fn as_type_mut(&mut self) -> Option<&mut Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Type>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&MethodResult`] to a [`&Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&MethodResult`]: MethodResult
    /// [`&Type`]: Type
    #[inline]
    pub fn to_type(&self) -> &Type<'a> {
        self.as_type().unwrap()
    }

    /// Convert a [`&mut MethodResult`] to a [`&mut Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut MethodResult`]: MethodResult
    /// [`&mut Type`]: Type
    #[inline]
    pub fn to_type_mut(&mut self) -> &mut Type<'a> {
        self.as_type_mut().unwrap()
    }
}

impl<'a> Type<'a> {
    /// Convert a [`&Type`] to a [`&MethodResult`].
    ///
    /// [`&Type`]: Type
    /// [`&MethodResult`]: MethodResult
    #[inline]
    pub fn as_method_result(&self) -> &MethodResult<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<MethodResult>().as_ref() }
    }
}

impl<'a> TryFrom<MethodResult<'a>> for Type<'a> {
    type Error = ();

    /// Convert a [`MethodResult`] to a [`Type`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: MethodResult<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            MethodResult::Primitive(o) => Ok(Type::Primitive(o)),
            MethodResult::Array(o) => Ok(Type::Array(o)),
            MethodResult::Simple(o) => Ok(Type::Simple(o)),
            MethodResult::Qualified(o) => Ok(Type::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Type<'a>> for MethodResult<'a> {
    /// Convert a [`Type`] to a [`MethodResult`].
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Type::Primitive(o) => MethodResult::Primitive(o),
            Type::Array(o) => MethodResult::Array(o),
            Type::Simple(o) => MethodResult::Simple(o),
            Type::Qualified(o) => MethodResult::Qualified(o),
        }
    }
}

impl<'a> MethodResult<'a> {
    /// Return if a [`MethodResult`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`MethodResult`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&MethodResult`] to a [`&ClassType`].
    ///
    /// [`&MethodResult`]: MethodResult
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut MethodResult`] to a [`&mut ClassType`].
    ///
    /// [`&mut MethodResult`]: MethodResult
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&MethodResult`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&MethodResult`]: MethodResult
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut MethodResult`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut MethodResult`]: MethodResult
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&MethodResult`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&MethodResult`]: MethodResult
    #[inline]
    pub fn as_method_result(&self) -> &MethodResult<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<MethodResult>().as_ref() }
    }
}

impl<'a> TryFrom<MethodResult<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`MethodResult`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: MethodResult<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            MethodResult::Simple(o) => Ok(ClassType::Simple(o)),
            MethodResult::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for MethodResult<'a> {
    /// Convert a [`ClassType`] to a [`MethodResult`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => MethodResult::Simple(o),
            ClassType::Qualified(o) => MethodResult::Qualified(o),
        }
    }
}

impl<'a> TypeArgument<'a> {
    /// Return if a [`TypeArgument`] is a [`Type`].
    #[inline]
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Primitive(_) | Self::Array(_) | Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`TypeArgument`] to a [`Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_type(self) -> Type<'a> {
        Type::try_from(self).unwrap()
    }

    /// Convert a [`&TypeArgument`] to a [`&Type`].
    ///
    /// [`&TypeArgument`]: TypeArgument
    /// [`&Type`]: Type
    #[inline]
    pub fn as_type(&self) -> Option<&Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Type>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut TypeArgument`] to a [`&mut Type`].
    ///
    /// [`&mut TypeArgument`]: TypeArgument
    /// [`&mut Type`]: Type
    #[inline]
    pub fn as_type_mut(&mut self) -> Option<&mut Type<'a>> {
        if self.is_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Type>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&TypeArgument`] to a [`&Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&TypeArgument`]: TypeArgument
    /// [`&Type`]: Type
    #[inline]
    pub fn to_type(&self) -> &Type<'a> {
        self.as_type().unwrap()
    }

    /// Convert a [`&mut TypeArgument`] to a [`&mut Type`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut TypeArgument`]: TypeArgument
    /// [`&mut Type`]: Type
    #[inline]
    pub fn to_type_mut(&mut self) -> &mut Type<'a> {
        self.as_type_mut().unwrap()
    }
}

impl<'a> Type<'a> {
    /// Convert a [`&Type`] to a [`&TypeArgument`].
    ///
    /// [`&Type`]: Type
    /// [`&TypeArgument`]: TypeArgument
    #[inline]
    pub fn as_type_argument(&self) -> &TypeArgument<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<TypeArgument>().as_ref() }
    }
}

impl<'a> TryFrom<TypeArgument<'a>> for Type<'a> {
    type Error = ();

    /// Convert a [`TypeArgument`] to a [`Type`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: TypeArgument<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            TypeArgument::Primitive(o) => Ok(Type::Primitive(o)),
            TypeArgument::Array(o) => Ok(Type::Array(o)),
            TypeArgument::Simple(o) => Ok(Type::Simple(o)),
            TypeArgument::Qualified(o) => Ok(Type::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Type<'a>> for TypeArgument<'a> {
    /// Convert a [`Type`] to a [`TypeArgument`].
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Type::Primitive(o) => TypeArgument::Primitive(o),
            Type::Array(o) => TypeArgument::Array(o),
            Type::Simple(o) => TypeArgument::Simple(o),
            Type::Qualified(o) => TypeArgument::Qualified(o),
        }
    }
}

impl<'a> TypeArgument<'a> {
    /// Return if a [`TypeArgument`] is a [`ClassType`].
    #[inline]
    pub fn is_class_type(&self) -> bool {
        matches!(self, Self::Simple(_) | Self::Qualified(_))
    }

    /// Convert a [`TypeArgument`] to a [`ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_class_type(self) -> ClassType<'a> {
        ClassType::try_from(self).unwrap()
    }

    /// Convert a [`&TypeArgument`] to a [`&ClassType`].
    ///
    /// [`&TypeArgument`]: TypeArgument
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn as_class_type(&self) -> Option<&ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ClassType>().as_ref() })
        } else {
            None
        }
    }

    /// Convert a [`&mut TypeArgument`] to a [`&mut ClassType`].
    ///
    /// [`&mut TypeArgument`]: TypeArgument
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn as_class_type_mut(&mut self) -> Option<&mut ClassType<'a>> {
        if self.is_class_type() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ClassType>().as_mut() })
        } else {
            None
        }
    }

    /// Convert a [`&TypeArgument`] to a [`&ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&TypeArgument`]: TypeArgument
    /// [`&ClassType`]: ClassType
    #[inline]
    pub fn to_class_type(&self) -> &ClassType<'a> {
        self.as_class_type().unwrap()
    }

    /// Convert a [`&mut TypeArgument`] to a [`&mut ClassType`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut TypeArgument`]: TypeArgument
    /// [`&mut ClassType`]: ClassType
    #[inline]
    pub fn to_class_type_mut(&mut self) -> &mut ClassType<'a> {
        self.as_class_type_mut().unwrap()
    }
}

impl<'a> ClassType<'a> {
    /// Convert a [`&ClassType`] to a [`&TypeArgument`].
    ///
    /// [`&ClassType`]: ClassType
    /// [`&TypeArgument`]: TypeArgument
    #[inline]
    pub fn as_type_argument(&self) -> &TypeArgument<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<TypeArgument>().as_ref() }
    }
}

impl<'a> TryFrom<TypeArgument<'a>> for ClassType<'a> {
    type Error = ();

    /// Convert a [`TypeArgument`] to a [`ClassType`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: TypeArgument<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            TypeArgument::Simple(o) => Ok(ClassType::Simple(o)),
            TypeArgument::Qualified(o) => Ok(ClassType::Qualified(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ClassType<'a>> for TypeArgument<'a> {
    /// Convert a [`ClassType`] to a [`TypeArgument`].
    #[inline]
    fn from(value: ClassType<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ClassType::Simple(o) => TypeArgument::Simple(o),
            ClassType::Qualified(o) => TypeArgument::Qualified(o),
        }
    }
}

impl<'a> AnnotationArgument<'a> {
    /// Return if an [`AnnotationArgument`] is an [`ElementValue`].
    #[inline]
    pub fn is_element_value(&self) -> bool {
        matches!(
            self,
            Self::Annotation(_)
                | Self::Array(_)
                | Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert an [`AnnotationArgument`] to an [`ElementValue`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_element_value(self) -> ElementValue<'a> {
        ElementValue::try_from(self).unwrap()
    }

    /// Convert an [`&AnnotationArgument`] to an [`&ElementValue`].
    ///
    /// [`&AnnotationArgument`]: AnnotationArgument
    /// [`&ElementValue`]: ElementValue
    #[inline]
    pub fn as_element_value(&self) -> Option<&ElementValue<'a>> {
        if self.is_element_value() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<ElementValue>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut AnnotationArgument`] to an [`&mut ElementValue`].
    ///
    /// [`&mut AnnotationArgument`]: AnnotationArgument
    /// [`&mut ElementValue`]: ElementValue
    #[inline]
    pub fn as_element_value_mut(&mut self) -> Option<&mut ElementValue<'a>> {
        if self.is_element_value() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<ElementValue>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&AnnotationArgument`] to an [`&ElementValue`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&AnnotationArgument`]: AnnotationArgument
    /// [`&ElementValue`]: ElementValue
    #[inline]
    pub fn to_element_value(&self) -> &ElementValue<'a> {
        self.as_element_value().unwrap()
    }

    /// Convert an [`&mut AnnotationArgument`] to an [`&mut ElementValue`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut AnnotationArgument`]: AnnotationArgument
    /// [`&mut ElementValue`]: ElementValue
    #[inline]
    pub fn to_element_value_mut(&mut self) -> &mut ElementValue<'a> {
        self.as_element_value_mut().unwrap()
    }
}

impl<'a> ElementValue<'a> {
    /// Convert an [`&ElementValue`] to an [`&AnnotationArgument`].
    ///
    /// [`&ElementValue`]: ElementValue
    /// [`&AnnotationArgument`]: AnnotationArgument
    #[inline]
    pub fn as_annotation_argument(&self) -> &AnnotationArgument<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<AnnotationArgument>().as_ref() }
    }
}

impl<'a> TryFrom<AnnotationArgument<'a>> for ElementValue<'a> {
    type Error = ();

    /// Convert an [`AnnotationArgument`] to an [`ElementValue`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: AnnotationArgument<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            AnnotationArgument::Annotation(o) => Ok(ElementValue::Annotation(o)),
            AnnotationArgument::Array(o) => Ok(ElementValue::Array(o)),
            AnnotationArgument::IntLiteral(o) => Ok(ElementValue::IntLiteral(o)),
            AnnotationArgument::FloatLiteral(o) => Ok(ElementValue::FloatLiteral(o)),
            AnnotationArgument::CharLiteral(o) => Ok(ElementValue::CharLiteral(o)),
            AnnotationArgument::StringLiteral(o) => Ok(ElementValue::StringLiteral(o)),
            AnnotationArgument::BooleanLiteral(o) => Ok(ElementValue::BooleanLiteral(o)),
            AnnotationArgument::NullLiteral(o) => Ok(ElementValue::NullLiteral(o)),
            AnnotationArgument::This(o) => Ok(ElementValue::This(o)),
            AnnotationArgument::Super(o) => Ok(ElementValue::Super(o)),
            AnnotationArgument::QualifiedThis(o) => Ok(ElementValue::QualifiedThis(o)),
            AnnotationArgument::QualifiedSuper(o) => Ok(ElementValue::QualifiedSuper(o)),
            AnnotationArgument::Identifier(o) => Ok(ElementValue::Identifier(o)),
            AnnotationArgument::FieldAccess(o) => Ok(ElementValue::FieldAccess(o)),
            AnnotationArgument::MethodCall(o) => Ok(ElementValue::MethodCall(o)),
            AnnotationArgument::ClassLiteral(o) => Ok(ElementValue::ClassLiteral(o)),
            AnnotationArgument::ClassInstanceCreation(o) => {
                Ok(ElementValue::ClassInstanceCreation(o))
            }
            AnnotationArgument::QualifiedClassInstanceCreation(o) => {
                Ok(ElementValue::QualifiedClassInstanceCreation(o))
            }
            AnnotationArgument::ArrayCreation(o) => Ok(ElementValue::ArrayCreation(o)),
            AnnotationArgument::ArrayAccess(o) => Ok(ElementValue::ArrayAccess(o)),
            AnnotationArgument::Update(o) => Ok(ElementValue::Update(o)),
            AnnotationArgument::Unary(o) => Ok(ElementValue::Unary(o)),
            AnnotationArgument::Cast(o) => Ok(ElementValue::Cast(o)),
            AnnotationArgument::Instanceof(o) => Ok(ElementValue::Instanceof(o)),
            AnnotationArgument::Binary(o) => Ok(ElementValue::Binary(o)),
            AnnotationArgument::Assignment(o) => Ok(ElementValue::Assignment(o)),
            AnnotationArgument::Conditional(o) => Ok(ElementValue::Conditional(o)),
            AnnotationArgument::Switch(o) => Ok(ElementValue::Switch(o)),
            AnnotationArgument::Parenthesized(o) => Ok(ElementValue::Parenthesized(o)),
            AnnotationArgument::Lambda(o) => Ok(ElementValue::Lambda(o)),
            AnnotationArgument::MethodReference(o) => Ok(ElementValue::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<ElementValue<'a>> for AnnotationArgument<'a> {
    /// Convert an [`ElementValue`] to an [`AnnotationArgument`].
    #[inline]
    fn from(value: ElementValue<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            ElementValue::Annotation(o) => AnnotationArgument::Annotation(o),
            ElementValue::Array(o) => AnnotationArgument::Array(o),
            ElementValue::IntLiteral(o) => AnnotationArgument::IntLiteral(o),
            ElementValue::FloatLiteral(o) => AnnotationArgument::FloatLiteral(o),
            ElementValue::CharLiteral(o) => AnnotationArgument::CharLiteral(o),
            ElementValue::StringLiteral(o) => AnnotationArgument::StringLiteral(o),
            ElementValue::BooleanLiteral(o) => AnnotationArgument::BooleanLiteral(o),
            ElementValue::NullLiteral(o) => AnnotationArgument::NullLiteral(o),
            ElementValue::This(o) => AnnotationArgument::This(o),
            ElementValue::Super(o) => AnnotationArgument::Super(o),
            ElementValue::QualifiedThis(o) => AnnotationArgument::QualifiedThis(o),
            ElementValue::QualifiedSuper(o) => AnnotationArgument::QualifiedSuper(o),
            ElementValue::Identifier(o) => AnnotationArgument::Identifier(o),
            ElementValue::FieldAccess(o) => AnnotationArgument::FieldAccess(o),
            ElementValue::MethodCall(o) => AnnotationArgument::MethodCall(o),
            ElementValue::ClassLiteral(o) => AnnotationArgument::ClassLiteral(o),
            ElementValue::ClassInstanceCreation(o) => AnnotationArgument::ClassInstanceCreation(o),
            ElementValue::QualifiedClassInstanceCreation(o) => {
                AnnotationArgument::QualifiedClassInstanceCreation(o)
            }
            ElementValue::ArrayCreation(o) => AnnotationArgument::ArrayCreation(o),
            ElementValue::ArrayAccess(o) => AnnotationArgument::ArrayAccess(o),
            ElementValue::Update(o) => AnnotationArgument::Update(o),
            ElementValue::Unary(o) => AnnotationArgument::Unary(o),
            ElementValue::Cast(o) => AnnotationArgument::Cast(o),
            ElementValue::Instanceof(o) => AnnotationArgument::Instanceof(o),
            ElementValue::Binary(o) => AnnotationArgument::Binary(o),
            ElementValue::Assignment(o) => AnnotationArgument::Assignment(o),
            ElementValue::Conditional(o) => AnnotationArgument::Conditional(o),
            ElementValue::Switch(o) => AnnotationArgument::Switch(o),
            ElementValue::Parenthesized(o) => AnnotationArgument::Parenthesized(o),
            ElementValue::Lambda(o) => AnnotationArgument::Lambda(o),
            ElementValue::MethodReference(o) => AnnotationArgument::MethodReference(o),
        }
    }
}

impl<'a> AnnotationArgument<'a> {
    /// Return if an [`AnnotationArgument`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert an [`AnnotationArgument`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert an [`&AnnotationArgument`] to an [`&Expression`].
    ///
    /// [`&AnnotationArgument`]: AnnotationArgument
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut AnnotationArgument`] to an [`&mut Expression`].
    ///
    /// [`&mut AnnotationArgument`]: AnnotationArgument
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&AnnotationArgument`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&AnnotationArgument`]: AnnotationArgument
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert an [`&mut AnnotationArgument`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut AnnotationArgument`]: AnnotationArgument
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to an [`&AnnotationArgument`].
    ///
    /// [`&Expression`]: Expression
    /// [`&AnnotationArgument`]: AnnotationArgument
    #[inline]
    pub fn as_annotation_argument(&self) -> &AnnotationArgument<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<AnnotationArgument>().as_ref() }
    }
}

impl<'a> TryFrom<AnnotationArgument<'a>> for Expression<'a> {
    type Error = ();

    /// Convert an [`AnnotationArgument`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: AnnotationArgument<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            AnnotationArgument::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            AnnotationArgument::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            AnnotationArgument::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            AnnotationArgument::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            AnnotationArgument::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            AnnotationArgument::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            AnnotationArgument::This(o) => Ok(Expression::This(o)),
            AnnotationArgument::Super(o) => Ok(Expression::Super(o)),
            AnnotationArgument::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            AnnotationArgument::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            AnnotationArgument::Identifier(o) => Ok(Expression::Identifier(o)),
            AnnotationArgument::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            AnnotationArgument::MethodCall(o) => Ok(Expression::MethodCall(o)),
            AnnotationArgument::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            AnnotationArgument::ClassInstanceCreation(o) => {
                Ok(Expression::ClassInstanceCreation(o))
            }
            AnnotationArgument::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            AnnotationArgument::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            AnnotationArgument::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            AnnotationArgument::Update(o) => Ok(Expression::Update(o)),
            AnnotationArgument::Unary(o) => Ok(Expression::Unary(o)),
            AnnotationArgument::Cast(o) => Ok(Expression::Cast(o)),
            AnnotationArgument::Instanceof(o) => Ok(Expression::Instanceof(o)),
            AnnotationArgument::Binary(o) => Ok(Expression::Binary(o)),
            AnnotationArgument::Assignment(o) => Ok(Expression::Assignment(o)),
            AnnotationArgument::Conditional(o) => Ok(Expression::Conditional(o)),
            AnnotationArgument::Switch(o) => Ok(Expression::Switch(o)),
            AnnotationArgument::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            AnnotationArgument::Lambda(o) => Ok(Expression::Lambda(o)),
            AnnotationArgument::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for AnnotationArgument<'a> {
    /// Convert an [`Expression`] to an [`AnnotationArgument`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => AnnotationArgument::IntLiteral(o),
            Expression::FloatLiteral(o) => AnnotationArgument::FloatLiteral(o),
            Expression::CharLiteral(o) => AnnotationArgument::CharLiteral(o),
            Expression::StringLiteral(o) => AnnotationArgument::StringLiteral(o),
            Expression::BooleanLiteral(o) => AnnotationArgument::BooleanLiteral(o),
            Expression::NullLiteral(o) => AnnotationArgument::NullLiteral(o),
            Expression::This(o) => AnnotationArgument::This(o),
            Expression::Super(o) => AnnotationArgument::Super(o),
            Expression::QualifiedThis(o) => AnnotationArgument::QualifiedThis(o),
            Expression::QualifiedSuper(o) => AnnotationArgument::QualifiedSuper(o),
            Expression::Identifier(o) => AnnotationArgument::Identifier(o),
            Expression::FieldAccess(o) => AnnotationArgument::FieldAccess(o),
            Expression::MethodCall(o) => AnnotationArgument::MethodCall(o),
            Expression::ClassLiteral(o) => AnnotationArgument::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => AnnotationArgument::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                AnnotationArgument::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => AnnotationArgument::ArrayCreation(o),
            Expression::ArrayAccess(o) => AnnotationArgument::ArrayAccess(o),
            Expression::Update(o) => AnnotationArgument::Update(o),
            Expression::Unary(o) => AnnotationArgument::Unary(o),
            Expression::Cast(o) => AnnotationArgument::Cast(o),
            Expression::Instanceof(o) => AnnotationArgument::Instanceof(o),
            Expression::Binary(o) => AnnotationArgument::Binary(o),
            Expression::Assignment(o) => AnnotationArgument::Assignment(o),
            Expression::Conditional(o) => AnnotationArgument::Conditional(o),
            Expression::Switch(o) => AnnotationArgument::Switch(o),
            Expression::Parenthesized(o) => AnnotationArgument::Parenthesized(o),
            Expression::Lambda(o) => AnnotationArgument::Lambda(o),
            Expression::MethodReference(o) => AnnotationArgument::MethodReference(o),
        }
    }
}

impl<'a> ElementValue<'a> {
    /// Return if an [`ElementValue`] is an [`Expression`].
    #[inline]
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            Self::IntLiteral(_)
                | Self::FloatLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::This(_)
                | Self::Super(_)
                | Self::QualifiedThis(_)
                | Self::QualifiedSuper(_)
                | Self::Identifier(_)
                | Self::FieldAccess(_)
                | Self::MethodCall(_)
                | Self::ClassLiteral(_)
                | Self::ClassInstanceCreation(_)
                | Self::QualifiedClassInstanceCreation(_)
                | Self::ArrayCreation(_)
                | Self::ArrayAccess(_)
                | Self::Update(_)
                | Self::Unary(_)
                | Self::Cast(_)
                | Self::Instanceof(_)
                | Self::Binary(_)
                | Self::Assignment(_)
                | Self::Conditional(_)
                | Self::Switch(_)
                | Self::Parenthesized(_)
                | Self::Lambda(_)
                | Self::MethodReference(_)
        )
    }

    /// Convert an [`ElementValue`] to an [`Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    #[inline]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::try_from(self).unwrap()
    }

    /// Convert an [`&ElementValue`] to an [`&Expression`].
    ///
    /// [`&ElementValue`]: ElementValue
    /// [`&Expression`]: Expression
    #[inline]
    pub fn as_expression(&self) -> Option<&Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_ref(self).cast::<Expression>().as_ref() })
        } else {
            None
        }
    }

    /// Convert an [`&mut ElementValue`] to an [`&mut Expression`].
    ///
    /// [`&mut ElementValue`]: ElementValue
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn as_expression_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is_expression() {
            // SAFETY: Transmute is safe because discriminants + types are identical between
            // `parent` and `child` for the shared variants
            Some(unsafe { NonNull::from_mut(self).cast::<Expression>().as_mut() })
        } else {
            None
        }
    }

    /// Convert an [`&ElementValue`] to an [`&Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&ElementValue`]: ElementValue
    /// [`&Expression`]: Expression
    #[inline]
    pub fn to_expression(&self) -> &Expression<'a> {
        self.as_expression().unwrap()
    }

    /// Convert an [`&mut ElementValue`] to an [`&mut Expression`].
    ///
    /// # Panics
    /// Panics if not convertible.
    ///
    /// [`&mut ElementValue`]: ElementValue
    /// [`&mut Expression`]: Expression
    #[inline]
    pub fn to_expression_mut(&mut self) -> &mut Expression<'a> {
        self.as_expression_mut().unwrap()
    }
}

impl<'a> Expression<'a> {
    /// Convert an [`&Expression`] to an [`&ElementValue`].
    ///
    /// [`&Expression`]: Expression
    /// [`&ElementValue`]: ElementValue
    #[inline]
    pub fn as_element_value(&self) -> &ElementValue<'a> {
        // SAFETY: Transmute is safe because discriminants + types are identical between
        // `parent` and `child` for the shared variants
        unsafe { NonNull::from_ref(self).cast::<ElementValue>().as_ref() }
    }
}

impl<'a> TryFrom<ElementValue<'a>> for Expression<'a> {
    type Error = ();

    /// Convert an [`ElementValue`] to an [`Expression`].
    ///
    /// # Errors
    /// Returns `Err` if not convertible.
    #[inline]
    fn try_from(value: ElementValue<'a>) -> Result<Self, Self::Error> {
        // Compiler should implement this as a check of discriminant and then zero-cost transmute,
        // as discriminants for `parent` and `child` are aligned
        match value {
            ElementValue::IntLiteral(o) => Ok(Expression::IntLiteral(o)),
            ElementValue::FloatLiteral(o) => Ok(Expression::FloatLiteral(o)),
            ElementValue::CharLiteral(o) => Ok(Expression::CharLiteral(o)),
            ElementValue::StringLiteral(o) => Ok(Expression::StringLiteral(o)),
            ElementValue::BooleanLiteral(o) => Ok(Expression::BooleanLiteral(o)),
            ElementValue::NullLiteral(o) => Ok(Expression::NullLiteral(o)),
            ElementValue::This(o) => Ok(Expression::This(o)),
            ElementValue::Super(o) => Ok(Expression::Super(o)),
            ElementValue::QualifiedThis(o) => Ok(Expression::QualifiedThis(o)),
            ElementValue::QualifiedSuper(o) => Ok(Expression::QualifiedSuper(o)),
            ElementValue::Identifier(o) => Ok(Expression::Identifier(o)),
            ElementValue::FieldAccess(o) => Ok(Expression::FieldAccess(o)),
            ElementValue::MethodCall(o) => Ok(Expression::MethodCall(o)),
            ElementValue::ClassLiteral(o) => Ok(Expression::ClassLiteral(o)),
            ElementValue::ClassInstanceCreation(o) => Ok(Expression::ClassInstanceCreation(o)),
            ElementValue::QualifiedClassInstanceCreation(o) => {
                Ok(Expression::QualifiedClassInstanceCreation(o))
            }
            ElementValue::ArrayCreation(o) => Ok(Expression::ArrayCreation(o)),
            ElementValue::ArrayAccess(o) => Ok(Expression::ArrayAccess(o)),
            ElementValue::Update(o) => Ok(Expression::Update(o)),
            ElementValue::Unary(o) => Ok(Expression::Unary(o)),
            ElementValue::Cast(o) => Ok(Expression::Cast(o)),
            ElementValue::Instanceof(o) => Ok(Expression::Instanceof(o)),
            ElementValue::Binary(o) => Ok(Expression::Binary(o)),
            ElementValue::Assignment(o) => Ok(Expression::Assignment(o)),
            ElementValue::Conditional(o) => Ok(Expression::Conditional(o)),
            ElementValue::Switch(o) => Ok(Expression::Switch(o)),
            ElementValue::Parenthesized(o) => Ok(Expression::Parenthesized(o)),
            ElementValue::Lambda(o) => Ok(Expression::Lambda(o)),
            ElementValue::MethodReference(o) => Ok(Expression::MethodReference(o)),
            _ => Err(()),
        }
    }
}

impl<'a> From<Expression<'a>> for ElementValue<'a> {
    /// Convert an [`Expression`] to an [`ElementValue`].
    #[inline]
    fn from(value: Expression<'a>) -> Self {
        // Compiler should implement this as zero-cost transmute as discriminants
        // for `child` and `parent` are aligned
        match value {
            Expression::IntLiteral(o) => ElementValue::IntLiteral(o),
            Expression::FloatLiteral(o) => ElementValue::FloatLiteral(o),
            Expression::CharLiteral(o) => ElementValue::CharLiteral(o),
            Expression::StringLiteral(o) => ElementValue::StringLiteral(o),
            Expression::BooleanLiteral(o) => ElementValue::BooleanLiteral(o),
            Expression::NullLiteral(o) => ElementValue::NullLiteral(o),
            Expression::This(o) => ElementValue::This(o),
            Expression::Super(o) => ElementValue::Super(o),
            Expression::QualifiedThis(o) => ElementValue::QualifiedThis(o),
            Expression::QualifiedSuper(o) => ElementValue::QualifiedSuper(o),
            Expression::Identifier(o) => ElementValue::Identifier(o),
            Expression::FieldAccess(o) => ElementValue::FieldAccess(o),
            Expression::MethodCall(o) => ElementValue::MethodCall(o),
            Expression::ClassLiteral(o) => ElementValue::ClassLiteral(o),
            Expression::ClassInstanceCreation(o) => ElementValue::ClassInstanceCreation(o),
            Expression::QualifiedClassInstanceCreation(o) => {
                ElementValue::QualifiedClassInstanceCreation(o)
            }
            Expression::ArrayCreation(o) => ElementValue::ArrayCreation(o),
            Expression::ArrayAccess(o) => ElementValue::ArrayAccess(o),
            Expression::Update(o) => ElementValue::Update(o),
            Expression::Unary(o) => ElementValue::Unary(o),
            Expression::Cast(o) => ElementValue::Cast(o),
            Expression::Instanceof(o) => ElementValue::Instanceof(o),
            Expression::Binary(o) => ElementValue::Binary(o),
            Expression::Assignment(o) => ElementValue::Assignment(o),
            Expression::Conditional(o) => ElementValue::Conditional(o),
            Expression::Switch(o) => ElementValue::Switch(o),
            Expression::Parenthesized(o) => ElementValue::Parenthesized(o),
            Expression::Lambda(o) => ElementValue::Lambda(o),
            Expression::MethodReference(o) => ElementValue::MethodReference(o),
        }
    }
}

/// Macro for matching [`TypeDeclaration`]'s variants.
///
/// Includes variants inherited from [`LocalClassOrInterfaceDeclaration`].
#[macro_export]
macro_rules! match_type_declaration {
    ($ty:ident) => {
        $ty::Empty(_)
            | $ty::AnnotationInterface(_)
            | $ty::Class(_)
            | $ty::Interface(_)
            | $ty::Enum(_)
            | $ty::Record(_)
    };
}

/// Macro for matching [`LocalClassOrInterfaceDeclaration`]'s variants.
#[macro_export]
macro_rules! match_local_class_or_interface_declaration {
    ($ty:ident) => {
        $ty::Class(_) | $ty::Interface(_) | $ty::Enum(_) | $ty::Record(_)
    };
}

/// Macro for matching [`ResourceAccess`]'s variants.
#[macro_export]
macro_rules! match_resource_access {
    ($ty:ident) => {
        $ty::Identifier(_) | $ty::Field(_)
    };
}

/// Macro for matching [`Expression`]'s variants.
#[macro_export]
macro_rules! match_expression {
    ($ty:ident) => {
        $ty::IntLiteral(_)
            | $ty::FloatLiteral(_)
            | $ty::CharLiteral(_)
            | $ty::StringLiteral(_)
            | $ty::BooleanLiteral(_)
            | $ty::NullLiteral(_)
            | $ty::This(_)
            | $ty::Super(_)
            | $ty::QualifiedThis(_)
            | $ty::QualifiedSuper(_)
            | $ty::Identifier(_)
            | $ty::FieldAccess(_)
            | $ty::MethodCall(_)
            | $ty::ClassLiteral(_)
            | $ty::ClassInstanceCreation(_)
            | $ty::QualifiedClassInstanceCreation(_)
            | $ty::ArrayCreation(_)
            | $ty::ArrayAccess(_)
            | $ty::Update(_)
            | $ty::Unary(_)
            | $ty::Cast(_)
            | $ty::Instanceof(_)
            | $ty::Binary(_)
            | $ty::Assignment(_)
            | $ty::Conditional(_)
            | $ty::Switch(_)
            | $ty::Parenthesized(_)
            | $ty::Lambda(_)
            | $ty::MethodReference(_)
    };
}

/// Macro for matching [`Pattern`]'s variants.
#[macro_export]
macro_rules! match_pattern {
    ($ty:ident) => {
        $ty::TypePattern(_) | $ty::RecordPattern(_)
    };
}

/// Macro for matching [`Type`]'s variants.
///
/// Includes variants inherited from [`ClassType`].
#[macro_export]
macro_rules! match_type {
    ($ty:ident) => {
        $ty::Primitive(_) | $ty::Array(_) | $ty::Simple(_) | $ty::Qualified(_)
    };
}

/// Macro for matching [`ClassType`]'s variants.
#[macro_export]
macro_rules! match_class_type {
    ($ty:ident) => {
        $ty::Simple(_) | $ty::Qualified(_)
    };
}

/// Macro for matching [`ElementValue`]'s variants.
///
/// Includes variants inherited from [`Expression`].
#[macro_export]
macro_rules! match_element_value {
    ($ty:ident) => {
        $ty::Annotation(_)
            | $ty::Array(_)
            | $ty::IntLiteral(_)
            | $ty::FloatLiteral(_)
            | $ty::CharLiteral(_)
            | $ty::StringLiteral(_)
            | $ty::BooleanLiteral(_)
            | $ty::NullLiteral(_)
            | $ty::This(_)
            | $ty::Super(_)
            | $ty::QualifiedThis(_)
            | $ty::QualifiedSuper(_)
            | $ty::Identifier(_)
            | $ty::FieldAccess(_)
            | $ty::MethodCall(_)
            | $ty::ClassLiteral(_)
            | $ty::ClassInstanceCreation(_)
            | $ty::QualifiedClassInstanceCreation(_)
            | $ty::ArrayCreation(_)
            | $ty::ArrayAccess(_)
            | $ty::Update(_)
            | $ty::Unary(_)
            | $ty::Cast(_)
            | $ty::Instanceof(_)
            | $ty::Binary(_)
            | $ty::Assignment(_)
            | $ty::Conditional(_)
            | $ty::Switch(_)
            | $ty::Parenthesized(_)
            | $ty::Lambda(_)
            | $ty::MethodReference(_)
    };
}
