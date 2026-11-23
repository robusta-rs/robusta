use std::cmp::max;

/// The layout of a type.
#[derive(Clone, Default)]
pub struct Layout {
    /// Layout on 64-bit platforms.
    pub layout_64: PlatformLayout,
    /// Layout on 32-bit platforms.
    pub layout_32: PlatformLayout,
}

impl Layout {
    /// Create [`Layout`] from `size` and `align` pair, with no niche.
    ///
    /// Layout is same for both 64-bit and 32-bit platforms.
    #[must_use]
    pub fn from_size_align(size: u32, align: u32) -> Self {
        Self {
            layout_64: PlatformLayout::from_size_align(size, align),
            layout_32: PlatformLayout::from_size_align(size, align),
        }
    }

    /// Create [`Layout`] from `size` and `align` pair, and [`Niche`].
    ///
    /// Layout is same for both 64-bit and 32-bit platforms.
    #[must_use]
    pub fn from_size_align_niche(size: u32, align: u32, niche: &Niche) -> Self {
        Self {
            layout_64: PlatformLayout::from_size_align_niche(size, align, niche.clone()),
            layout_32: PlatformLayout::from_size_align_niche(size, align, niche.clone()),
        }
    }
}

/// The layout of a type on a specific platform type (64 bit or 32 bit).
#[derive(Clone, Default)]
pub struct PlatformLayout {
    pub size: u32,
    pub align: u32,
    pub niche: Option<Niche>,
}

impl PlatformLayout {
    /// Create [`PlatformLayout`] from `size` and `align` pair, with no niche.
    #[must_use]
    pub fn from_size_align(size: u32, align: u32) -> Self {
        Self { size, align, niche: None }
    }

    /// Create [`PlatformLayout`] from `size` and `align` pair, and [`Niche`].
    #[must_use]
    pub fn from_size_align_niche(size: u32, align: u32, niche: Niche) -> Self {
        Self { size, align, niche: Some(niche) }
    }
}

/// Niche that a type has.
#[derive(Clone)]
pub struct Niche {
    /// Number of values at start of range.
    pub count_start: u32,
    /// Number of values at end of range.
    pub count_end: u32,
}

impl Niche {
    /// Create new [`Niche`].
    #[must_use]
    pub fn new(count_start: u32, count_end: u32) -> Self {
        Self { count_start, count_end }
    }

    /// Get size of largest niche range (start or end).
    #[must_use]
    pub fn count_max(&self) -> u32 {
        max(self.count_start, self.count_end)
    }
}

/// Offset of a struct field.
#[derive(Clone, Copy, Default)]
#[expect(clippy::struct_field_names)]
pub struct Offset {
    /// Offset in bytes on 64-bit platforms.
    pub offset_64: u32,
    /// Offset in bytes on 32-bit platforms.
    pub offset_32: u32,
    /// Index of field in memory layout order.
    pub layout_index: u32,
}
