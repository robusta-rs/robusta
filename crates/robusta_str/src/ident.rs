// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_str/src/ident.rs

//! Identifier string type with precomputed hash.

use std::{
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    slice, str,
};

use crate::ident_hasher::{IdentBuildHasher, ident_hash};

/// A packed representation of `len` and `hash` for `Ident` - 64-bit platforms version.
///
/// Stored as a single `u64`, with `len` in lower 32 bits, `hash` in upper 32 bits.
#[cfg(target_pointer_width = "64")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct LenAndHash(u64);

#[cfg(target_pointer_width = "64")]
#[expect(clippy::inline_always)] // All methods are trivial
impl LenAndHash {
    #[inline(always)]
    const fn new(len: u32, hash: u32) -> Self {
        Self((len as u64) | ((hash as u64) << 32))
    }

    #[expect(clippy::cast_possible_truncation)]
    #[inline(always)]
    const fn len(self) -> u32 {
        self.0 as u32
    }

    #[inline(always)]
    const fn to_u64(self) -> u64 {
        self.0
    }
}

/// A packed representation of `len` and `hash` for `Ident` - 32-bit platforms version.
///
/// Stored as 2 separate `u32`s for `len` and `hash`.
/// This is preferable on 32-bit platforms, because it has alignment 4, so `Ident` is 12 bytes, not 16.
#[cfg(not(target_pointer_width = "64"))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct LenAndHash {
    len: u32,
    hash: u32,
}

#[cfg(not(target_pointer_width = "64"))]
#[expect(clippy::inline_always)] // All methods are trivial
impl LenAndHash {
    #[inline(always)]
    const fn new(len: u32, hash: u32) -> Self {
        Self { len, hash }
    }

    #[inline(always)]
    const fn len(self) -> u32 {
        self.len
    }

    #[inline(always)]
    const fn to_u64(self) -> u64 {
        (self.len as u64) | ((self.hash as u64) << 32)
    }
}

/// An identifier string with a precomputed hash.
///
/// Stores a pointer to string data, length, and a precomputed hash for fast
/// HashMap lookups and equality comparisons.
///
/// On 64-bit platforms, length and hash are packed into a single `u64` for
/// maximum performance (16 bytes total, same as `&str`).
/// On 32-bit platforms, they are stored as separate `u32` fields (12 bytes total).
#[repr(C)]
pub struct Ident<'a> {
    ptr: NonNull<u8>,
    len_and_hash: LenAndHash,
    _marker: PhantomData<&'a str>,
}

impl<'a> Ident<'a> {
    /// Create a new [`Ident`] from a string slice, computing its hash.
    #[expect(clippy::inline_always, clippy::cast_possible_truncation)]
    #[inline(always)]
    const fn new(s: &'a str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len() as u32;
        let hash = ident_hash(bytes);
        let ptr = NonNull::from_ref(bytes).cast::<u8>();
        Self { ptr, len_and_hash: LenAndHash::new(len, hash), _marker: PhantomData }
    }

    /// Get the length of the identifier string.
    #[inline]
    const fn ident_len(&self) -> u32 {
        self.len_and_hash.len()
    }

    /// Borrow a string slice.
    #[expect(clippy::inline_always)]
    #[inline(always)] // Hot path — must be inlined
    pub fn as_str(&self) -> &'a str {
        let len = self.ident_len() as usize;
        // SAFETY: The pointer and length are valid because they came from a valid &str.
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.ptr.as_ptr(), len)) }
    }
}

// SAFETY: `Ident` is conceptually equivalent to `&str`, which is Send + Sync.
// `NonNull` is !Send/!Sync, but `Ident` only stores a pointer to borrowed data.
unsafe impl Send for Ident<'_> {}
// SAFETY: See above.
unsafe impl Sync for Ident<'_> {}

impl Deref for Ident<'_> {
    type Target = str;

    #[expect(clippy::inline_always)]
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

// We can't derive `Clone` or `Copy` because `NonNull` prevents it.
// The explicit impl is needed for `Copy` to work.
#[expect(clippy::expl_impl_clone_on_copy)]
impl Clone for Ident<'_> {
    #[expect(clippy::inline_always)]
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Ident<'_> {}

impl<'a> From<&'a str> for Ident<'a> {
    #[expect(clippy::inline_always)]
    #[inline(always)]
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

impl PartialEq for Ident<'_> {
    /// Fast-reject equality: compare packed len+hash first, then bytes.
    #[inline]
    fn eq(&self, other: &Ident<'_>) -> bool {
        self.len_and_hash == other.len_and_hash && self.as_str() == other.as_str()
    }
}

impl Eq for Ident<'_> {}

impl PartialEq<str> for Ident<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Ident<'_> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl Hash for Ident<'_> {
    /// Write the precomputed packed len+hash as a single u64.
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(self.len_and_hash.to_u64());
    }
}

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

/// Arena-allocated hash map keyed by [`Ident`], using precomputed ident hash.
pub type ArenaIdentHashMap<'alloc, V> =
    robusta_allocator::HashMap<'alloc, Ident<'alloc>, V, IdentBuildHasher>;

#[cfg(test)]
mod test {
    use std::hash::BuildHasher;

    use robusta_data_structures::types::implements;

    use super::*;

    #[test]
    fn ident_size() {
        #[cfg(target_pointer_width = "64")]
        assert_eq!(size_of::<Ident<'_>>(), 16);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(size_of::<Ident<'_>>(), 12);
    }

    #[test]
    fn ident_send_sync() {
        assert!(implements!(Ident: Send));
        assert!(implements!(Ident: Sync));
    }

    #[test]
    fn ident_copy() {
        assert!(implements!(Ident: Copy));
    }

    #[test]
    fn ident_from_str() {
        let s = "hello";
        let ident = Ident::from(s);
        assert_eq!(ident.as_str(), "hello");
    }

    #[test]
    fn ident_eq() {
        let a = Ident::from("foo");
        let b = Ident::from("foo");
        let c = Ident::from("bar");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn ident_eq_str() {
        let ident = Ident::from("hello");
        assert_eq!(ident, "hello");
        assert_ne!(ident, "world");
    }

    #[test]
    fn ident_hash_consistency() {
        let build_hasher = IdentBuildHasher;

        // Hash an Ident via BuildHasher::hash_one
        let ident = Ident::from("fooBar");
        let ident_hash_val = build_hasher.hash_one(ident);

        // Hash a &str through the same hasher
        let str_hash = build_hasher.hash_one("fooBar");

        assert_eq!(ident_hash_val, str_hash);
    }

    #[test]
    fn ident_deref() {
        let ident = Ident::from("hello");
        // Should be able to call str methods
        assert!(ident.starts_with("hel"));
        assert_eq!(ident.len(), 5);
    }

    #[test]
    fn ident_display() {
        let ident = Ident::from("test");
        assert_eq!(format!("{ident}"), "test");
    }

    #[test]
    fn ident_debug() {
        let ident = Ident::from("test");
        assert_eq!(format!("{ident:?}"), "\"test\"");
    }
}
