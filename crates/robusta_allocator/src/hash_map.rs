// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_allocator/src/hash_map.rs

//! A hash map without `Drop` that stores data in arena allocator.
//!
//! See [`HashMap`] for more details.

use std::{
    fmt,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use rustc_hash::FxBuildHasher;

pub use hashbrown::hash_map::Entry;

use crate::{Allocator, arena::Arena};

type InnerHashMap<'alloc, K, V, S> = hashbrown::HashMap<K, V, S, &'alloc Arena>;

/// A hash map without `Drop` that stores data in arena allocator.
///
/// Uses [`FxHasher`] by default. The hasher can be customized via the `S` type parameter.
///
/// Just a thin wrapper around [`hashbrown::HashMap`], which disables the `Drop` implementation.
/// Create one with [`new_in`](HashMap::new_in) or [`with_capacity_in`](HashMap::with_capacity_in).
///
/// # No `Drop`s
///
/// Objects allocated into the arena are never [`Dropped`](Drop). Memory is released in bulk
/// when the allocator is dropped, so a key or value that owns memory outside the arena would leak.
/// Every method which creates a [`HashMap`] refuses to compile if either key or value is a
/// [`Drop`] type.
///
/// [`FxHasher`]: rustc_hash::FxHasher
pub struct HashMap<'alloc, K, V, S = FxBuildHasher>(ManuallyDrop<InnerHashMap<'alloc, K, V, S>>);

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for HashMap<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl<'alloc, K, V, S: Default> HashMap<'alloc, K, V, S> {
    /// Const assertions that `K` and `V` are not `Drop`.
    /// Must be referenced in all methods which create a `HashMap`.
    const ASSERT_K_AND_V_ARE_NOT_DROP: () = {
        assert!(
            !std::mem::needs_drop::<K>(),
            "Cannot create a HashMap<K, V> where K is a Drop type"
        );
        assert!(
            !std::mem::needs_drop::<V>(),
            "Cannot create a HashMap<K, V> where V is a Drop type"
        );
    };

    /// Creates an empty [`HashMap`]. It will be allocated with the given allocator.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate
    /// until it is first inserted into.
    #[inline]
    pub fn new_in(allocator: &'alloc Allocator) -> Self {
        const { Self::ASSERT_K_AND_V_ARE_NOT_DROP };

        let inner = InnerHashMap::with_hasher_in(S::default(), allocator.arena());
        Self(ManuallyDrop::new(inner))
    }

    /// Creates an empty [`HashMap`] with the specified capacity. It will be allocated with the given allocator.
    ///
    /// The hash map will be able to hold at least capacity elements without reallocating.
    /// If capacity is 0, the hash map will not allocate.
    #[inline]
    pub fn with_capacity_in(capacity: usize, allocator: &'alloc Allocator) -> Self {
        const { Self::ASSERT_K_AND_V_ARE_NOT_DROP };

        let inner =
            InnerHashMap::with_capacity_and_hasher_in(capacity, S::default(), allocator.arena());
        Self(ManuallyDrop::new(inner))
    }
}

// Provide access to all `hashbrown::HashMap`'s methods via deref
impl<'alloc, K, V, S> Deref for HashMap<'alloc, K, V, S> {
    type Target = InnerHashMap<'alloc, K, V, S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'alloc, K, V, S> DerefMut for HashMap<'alloc, K, V, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut InnerHashMap<'alloc, K, V, S> {
        &mut self.0
    }
}
