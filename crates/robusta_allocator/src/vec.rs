use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr, slice,
};

use crate::{GetAllocator, vec2::RawVec};

/// A vector whose buffer and elements live in an arena and are not dropped
/// individually.
///
/// The element restriction makes bulk arena teardown sound: every resource an
/// element owns must itself belong to the arena or require no destruction.
///
/// The header is 24 bytes on a 64-bit target, which
/// `the_header_stays_packed_at_three_words` pins. That width is repeated in
/// every list field of every syntax node, so it is part of the type's contract
/// rather than an implementation detail.
#[repr(transparent)]
pub struct Vec<'alloc, T: 'alloc>(RawVec<'alloc, T>);

impl<'alloc, T: 'alloc> Vec<'alloc, T> {
    // Adapted from https://github.com/oxc-project/oxc/blob/66e1a25c0ec3bbf5a840830f3fc6698fa559dacc/crates/oxc_allocator/src/vec.rs
    const ASSERT_T_IS_NOT_DROP: () =
        assert!(!std::mem::needs_drop::<T>(), "cannot arena-allocate a type that needs Drop");

    /// Construct an empty arena-backed vector.
    #[inline]
    pub fn new_in<A: GetAllocator<'alloc>>(allocator: &A) -> Self {
        const { Self::ASSERT_T_IS_NOT_DROP };
        Self(RawVec::new_in(allocator.allocator().arena()))
    }

    /// Construct an empty arena-backed vector with at least the given capacity.
    #[inline]
    pub fn with_capacity_in<A: GetAllocator<'alloc>>(capacity: usize, allocator: &A) -> Self {
        const { Self::ASSERT_T_IS_NOT_DROP };
        Self(RawVec::with_capacity_in(capacity, allocator.allocator().arena()))
    }

    /// Construct an arena-backed vector holding the iterator's elements.
    #[inline]
    pub fn from_iter_in<I: IntoIterator<Item = T>, A: GetAllocator<'alloc>>(
        iter: I,
        allocator: &A,
    ) -> Self {
        const { Self::ASSERT_T_IS_NOT_DROP };
        let iter = iter.into_iter();
        let mut vector = Self::with_capacity_in(iter.size_hint().0, allocator);
        vector.extend(iter);
        vector
    }

    /// Consume the vector, returning its contents as an arena-owned slice.
    ///
    /// The slice lives for the arena's lifetime, so the caller can keep it
    /// without keeping the vector.
    #[inline]
    pub fn into_arena_slice(self) -> &'alloc [T] {
        // SAFETY: the buffer is owned by the arena rather than by `self`, so it
        // stays valid for `'alloc` once the vector is gone, and its first `len`
        // elements are initialized.
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.0.len()) }
    }

    /// Append an element to the vector.
    #[inline]
    pub fn push(&mut self, value: T) {
        let len = self.0.len_u32();
        if len == self.0.capacity() {
            self.0.grow_one();
        }

        // SAFETY: growth above guarantees room for one more element, so the
        // slot at `len` is allocated and uninitialized, and `len + 1` elements
        // are initialized once it is written.
        unsafe {
            ptr::write(self.0.as_ptr().add(len as usize), value);
            self.0.set_len(len + 1);
        }
    }

    /// Remove and return the final element, if any.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let len = self.0.len_u32().checked_sub(1)?;

        // SAFETY: the element at `len` is initialized, and shortening to `len`
        // gives up ownership of it before it is moved out.
        unsafe {
            self.0.set_len(len);
            Some(ptr::read(self.0.as_ptr().add(len as usize)))
        }
    }

    /// Shorten the vector to `len` elements, dropping nothing (elements never
    /// carry drop glue).
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if len < self.0.len() {
            #[expect(clippy::cast_possible_truncation, reason = "below the current `u32` length")]
            let len = len as u32;
            // SAFETY: `len` is below the current length, so it is within
            // capacity and every element under it is still initialized.
            unsafe { self.0.set_len(len) }
        }
    }

    /// Make room for at least `additional` more elements.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// A pointer to the buffer, valid for writes up to the capacity.
    ///
    /// Shadows the slice method, whose pointer would cover only the elements
    /// below the length.
    #[expect(clippy::needless_pass_by_ref_mut, reason = "writes need exclusive access")]
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_ptr()
    }

    /// Force the length to `len`.
    ///
    /// # Safety
    ///
    /// `len` must not exceed the capacity, and every element below it must be
    /// initialized.
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        #[expect(clippy::cast_possible_truncation, reason = "within the `u32` capacity")]
        let len = len as u32;
        // SAFETY: the caller upholds the same contract.
        unsafe { self.0.set_len(len) }
    }
}

impl<'alloc, T: 'alloc> Extend<T> for Vec<'alloc, T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.0.reserve(iter.size_hint().0);
        for value in iter {
            self.push(value);
        }
    }
}

impl<'alloc, T: 'alloc> Deref for Vec<'alloc, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: the first `len` elements of the buffer are initialized, and
        // the buffer outlives the borrow of `self`.
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.0.len()) }
    }
}

impl<'alloc, T: 'alloc> DerefMut for Vec<'alloc, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: as for `deref`, and `&mut self` makes the borrow exclusive.
        unsafe { slice::from_raw_parts_mut(self.0.as_ptr(), self.0.len()) }
    }
}

impl<'a, 'alloc, T: 'alloc> IntoIterator for &'a Vec<'alloc, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, 'alloc, T: 'alloc> IntoIterator for &'a mut Vec<'alloc, T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'alloc, T: 'alloc> IntoIterator for Vec<'alloc, T> {
    type Item = T;
    type IntoIter = IntoIter<'alloc, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { elements: self.into_arena_slice(), taken: 0, owned: PhantomData }
    }
}

/// Moves elements out of an arena vector one at a time.
///
/// The buffer itself stays with the arena, so this walks a slice and reads each
/// element out of it. Anything left unread is simply abandoned, which is sound
/// only because arena elements never carry drop glue.
pub struct IntoIter<'alloc, T> {
    elements: &'alloc [T],
    taken: usize,
    owned: PhantomData<T>,
}

impl<T> Iterator for IntoIter<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let element = self.elements.get(self.taken)?;
        self.taken += 1;
        // SAFETY: each index is read at most once, because `taken` only ever
        // moves forward, so no element is duplicated out of the buffer.
        Some(unsafe { ptr::read(element) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.elements.len() - self.taken;
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<'_, T> {}

impl<T: Debug> Debug for IntoIter<'_, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.elements[self.taken..], formatter)
    }
}

impl<T: Debug> Debug for Vec<'_, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Allocator;

    /// A non-`Copy`, non-`Drop` element, so the tests exercise moves rather
    /// than the compiler's freedom to duplicate a `Copy` value.
    #[derive(Debug, PartialEq, Eq)]
    struct Moved(u32);

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn the_header_stays_packed_at_three_words() {
        assert_eq!(size_of::<Vec<'_, u8>>(), 24, "a wider header is repeated in every list field");
        assert_eq!(align_of::<Vec<'_, u8>>(), 8);
        assert_eq!(size_of::<Vec<'_, [u8; 37]>>(), 24, "the header does not vary with the element");
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn the_header_stays_packed_at_four_words() {
        assert_eq!(size_of::<Vec<'_, u8>>(), 16);
        assert_eq!(align_of::<Vec<'_, u8>>(), 4);
    }

    #[test]
    fn a_new_vector_is_empty() {
        let allocator = &Allocator::new();
        let vector = Vec::<u32>::new_in(&allocator);

        assert_eq!(vector.len(), 0);
        assert!(vector.is_empty());
        assert_eq!(&*vector, &[] as &[u32]);
    }

    #[test]
    fn requested_capacity_absorbs_that_many_pushes_without_moving_the_buffer() {
        let allocator = &Allocator::new();
        let mut vector = Vec::with_capacity_in(8, &allocator);
        let buffer = vector.as_ptr();

        for value in 0..8_u32 {
            vector.push(Moved(value));
        }

        assert_eq!(vector.as_ptr(), buffer, "a reserved buffer must absorb its own capacity");
        assert_eq!(vector.len(), 8);
    }

    #[test]
    fn reserved_room_absorbs_that_many_pushes_without_moving_the_buffer() {
        let allocator = &Allocator::new();
        let mut vector = Vec::new_in(&allocator);
        vector.reserve(8);
        let buffer = vector.as_ptr();

        for value in 0..8_u32 {
            vector.push(Moved(value));
        }

        assert_eq!(vector.as_ptr(), buffer);
    }

    #[test]
    fn set_len_moves_a_tail_copied_through_the_raw_buffer() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in((0..4_u32).map(Moved), &allocator);
        let mut tail = Vec::with_capacity_in(2, &allocator);

        // SAFETY: `tail` has room for the two elements past index 2, which move
        // into it while `vector` forgets them.
        unsafe {
            std::ptr::copy_nonoverlapping(vector.as_ptr().add(2), tail.as_mut_ptr(), 2);
            tail.set_len(2);
            vector.set_len(2);
        }

        assert_eq!(&*vector, &[Moved(0), Moved(1)]);
        assert_eq!(&*tail, &[Moved(2), Moved(3)]);
    }

    #[test]
    fn zero_capacity_is_a_legal_request() {
        let allocator = &Allocator::new();
        let mut vector = Vec::with_capacity_in(0, &allocator);

        assert!(vector.is_empty());
        vector.push(Moved(1));
        assert_eq!(&*vector, &[Moved(1)]);
    }

    #[test]
    fn from_iter_in_keeps_iteration_order() {
        let allocator = &Allocator::new();
        let vector = Vec::from_iter_in((0..5_u32).map(Moved), &allocator);

        assert_eq!(&*vector, &[Moved(0), Moved(1), Moved(2), Moved(3), Moved(4)]);
    }

    #[test]
    fn growth_across_many_reallocations_preserves_every_element() {
        let allocator = &Allocator::new();
        let mut vector = Vec::new_in(&allocator);

        for value in 0..1_000_u32 {
            vector.push(Moved(value));
        }

        assert_eq!(vector.len(), 1_000);
        assert!(vector.iter().map(|element| element.0).eq(0..1_000));
    }

    #[test]
    fn pop_returns_elements_in_reverse_and_then_none() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in((0..3_u32).map(Moved), &allocator);

        assert_eq!(vector.pop(), Some(Moved(2)));
        assert_eq!(vector.pop(), Some(Moved(1)));
        assert_eq!(vector.pop(), Some(Moved(0)));
        assert_eq!(vector.pop(), None);
        assert!(vector.is_empty());
    }

    #[test]
    fn truncate_shortens_and_ignores_a_length_it_already_satisfies() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in((0..5_u32).map(Moved), &allocator);

        vector.truncate(2);
        assert_eq!(&*vector, &[Moved(0), Moved(1)]);

        vector.truncate(9);
        assert_eq!(&*vector, &[Moved(0), Moved(1)], "truncating past the end is a no-op");

        vector.truncate(0);
        assert!(vector.is_empty());
    }

    #[test]
    fn extend_appends_after_the_existing_elements() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in((0..2_u32).map(Moved), &allocator);

        vector.extend((2..5_u32).map(Moved));

        assert_eq!(&*vector, &[Moved(0), Moved(1), Moved(2), Moved(3), Moved(4)]);
    }

    #[test]
    fn deref_mut_exposes_the_slice_for_in_place_edits() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in([Moved(3), Moved(1), Moved(2)], &allocator);

        vector.sort_by_key(|element| element.0);
        vector[0] = Moved(9);

        assert_eq!(&*vector, &[Moved(9), Moved(2), Moved(3)]);
        assert_eq!(vector.last(), Some(&Moved(3)));
    }

    #[test]
    fn the_three_iteration_forms_all_visit_every_element() {
        let allocator = &Allocator::new();
        let mut vector = Vec::from_iter_in((0..4_u32).map(Moved), &allocator);

        let borrowed: std::vec::Vec<u32> = (&vector).into_iter().map(|element| element.0).collect();
        assert_eq!(borrowed, [0, 1, 2, 3]);

        for element in &mut vector {
            element.0 *= 2;
        }

        let owned: std::vec::Vec<u32> = vector.into_iter().map(|element| element.0).collect();
        assert_eq!(owned, [0, 2, 4, 6]);
    }

    #[test]
    fn into_arena_slice_outlives_the_vector_it_came_from() {
        let allocator = &Allocator::new();

        let slice: &[Moved] = {
            let vector = Vec::from_iter_in((0..3_u32).map(Moved), &allocator);
            vector.into_arena_slice()
        };

        assert_eq!(slice, &[Moved(0), Moved(1), Moved(2)]);
    }

    #[test]
    fn an_empty_vector_still_yields_a_usable_arena_slice() {
        let allocator = &Allocator::new();
        let slice = Vec::<Moved>::new_in(&allocator).into_arena_slice();

        assert_eq!(slice, []);
    }

    #[test]
    fn zero_sized_elements_are_counted_without_being_stored() {
        let allocator = &Allocator::new();
        let mut vector = Vec::new_in(&allocator);

        for _ in 0..1_000 {
            vector.push(());
        }

        assert_eq!(vector.len(), 1_000);
        assert_eq!(vector.pop(), Some(()));
        assert_eq!(vector.len(), 999);
        assert_eq!(vector.into_arena_slice().len(), 999);
    }

    #[test]
    fn an_over_aligned_element_stays_aligned_across_growth() {
        #[derive(Debug, PartialEq, Eq)]
        #[repr(align(64))]
        struct OverAligned(u64);

        let allocator = &Allocator::new();
        let mut vector = Vec::new_in(&allocator);

        for value in 0..64_u64 {
            vector.push(OverAligned(value));
            assert_eq!(vector.as_ptr() as usize % 64, 0, "growth must preserve element alignment");
        }

        assert_eq!(vector.last(), Some(&OverAligned(63)));
    }

    /// The counters are `u32`, and `robusta_lexer::MAX_LEN` caps an admitted
    /// source at `u32::MAX` bytes, so no list built from one can reach this.
    /// A request that does must still fail rather than wrap around to a
    /// capacity smaller than what the caller asked for. Only a `usize` wider
    /// than the counter can express such a request.
    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(expected = "arena vector capacity overflow")]
    fn a_capacity_past_the_counter_is_rejected_rather_than_truncated() {
        let allocator = &Allocator::new();
        let _ = Vec::<u64>::with_capacity_in(u32::MAX as usize + 1, &allocator);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(expected = "arena vector capacity overflow")]
    fn a_zero_sized_element_is_length_checked_even_though_it_never_allocates() {
        let allocator = &Allocator::new();
        let _ = Vec::<()>::with_capacity_in(u32::MAX as usize + 1, &allocator);
    }

    #[test]
    fn debug_renders_as_the_slice_does() {
        let allocator = &Allocator::new();
        let vector = Vec::from_iter_in([1_u32, 2, 3], &allocator);

        assert_eq!(format!("{vector:?}"), "[1, 2, 3]");
    }

    #[test]
    fn vectors_from_one_arena_do_not_disturb_each_other_while_growing() {
        let allocator = &Allocator::new();
        let mut first = Vec::new_in(&allocator);
        let mut second = Vec::new_in(&allocator);

        for value in 0..200_u32 {
            first.push(Moved(value));
            second.push(Moved(value + 1_000));
        }

        assert!(first.iter().map(|element| element.0).eq(0..200));
        assert!(second.iter().map(|element| element.0).eq(1_000..1_200));
    }
}
