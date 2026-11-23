//! Unit tests for `Arena`.

// Note: Only tests which require private types, fields, or methods should be in here.
// Anything that can just be tested via public API surface should be in `tests/arena/*`.

use std::{alloc::Layout, ptr::NonNull};

use allocator_api2::alloc::Allocator;

use super::{
    Arena, DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER,
    create::{OVERHEAD, TYPICAL_PAGE_SIZE},
};

// Uses private `DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER`, `OVERHEAD`, and `TYPICAL_PAGE_SIZE`
#[test]
fn allocated_and_used_bytes() {
    fn chunk_count<const MIN_ALIGN: usize>(b: &mut Arena<MIN_ALIGN>) -> usize {
        b.iter_allocated_chunks().count()
    }

    // Each new chunk is sized to twice the previous chunk's allocatable space, rounded up to a page
    // boundary (after accounting for `OVERHEAD` = 16-byte malloc overhead + footer).
    // Mirrors the formula in `Arena::new_chunk`.
    const fn next_chunk_size(prev: usize) -> usize {
        let raw = 2 * prev + OVERHEAD;
        let aligned = (raw + TYPICAL_PAGE_SIZE - 1) & !(TYPICAL_PAGE_SIZE - 1);
        aligned - OVERHEAD
    }
    const SECOND_CHUNK: usize = next_chunk_size(DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
    const THIRD_CHUNK: usize = next_chunk_size(SECOND_CHUNK);

    let mut b = Arena::new();

    // Empty arena owns no chunks
    assert_eq!(b.allocated_bytes(), 0);
    assert_eq!(b.used_bytes(), 0);
    assert_eq!(chunk_count(&mut b), 0);

    // First allocation creates the first chunk
    b.alloc(0u8);
    assert_eq!(b.allocated_bytes(), DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
    assert_eq!(b.used_bytes(), 1);
    assert_eq!(chunk_count(&mut b), 1);

    // Reset with a single chunk: The chunk is retained (capacity unchanged), but the cursor is reset
    b.reset();
    assert_eq!(b.allocated_bytes(), DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
    assert_eq!(b.used_bytes(), 0);
    assert_eq!(chunk_count(&mut b), 1);

    // Allocations that fit in the current chunk don't grow the allocated total, but do grow `used_bytes`
    let small = Layout::from_size_align(64, 1).unwrap();
    b.alloc_layout(small);
    b.alloc_layout(small);
    assert_eq!(b.allocated_bytes(), DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
    assert_eq!(b.used_bytes(), small.size() * 2);
    assert_eq!(chunk_count(&mut b), 1);

    // A request which cannot be served in first chunk forces creation of a second chunk.
    // Chunk 1 is retired with its current `used_bytes` (`2 * 64`); chunk 2 holds the `big` allocation.
    let big = Layout::from_size_align(DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER, 1).unwrap();
    b.alloc_layout(big);
    let two_chunks = b.allocated_bytes();
    assert_eq!(two_chunks, DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER + SECOND_CHUNK);
    assert_eq!(b.used_bytes(), small.size() * 2 + big.size());
    assert_eq!(chunk_count(&mut b), 2);

    // A request larger than every chunk seen so far forces a third chunk.
    // (The previous `big` allocation might fit alongside another `big` in chunk 2,
    // so use something definitely too big to fit anywhere existing.)
    let huge = Layout::from_size_align(DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER * 2, 1).unwrap();
    b.alloc_layout(huge);
    let three_chunks = b.allocated_bytes();
    assert_eq!(three_chunks, DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER + SECOND_CHUNK + THIRD_CHUNK);
    assert_eq!(b.used_bytes(), small.size() * 2 + big.size() + huge.size());
    assert_eq!(chunk_count(&mut b), 3);

    // `reset` with multiple chunks deallocates every chunk except the most recent (which is also the largest),
    // so `allocated_bytes` after reset equals just the size of that surviving chunk
    b.reset();
    let after_reset = b.allocated_bytes();
    assert_eq!(after_reset, THIRD_CHUNK);
    assert_eq!(b.used_bytes(), 0);
    assert_eq!(chunk_count(&mut b), 1);
}

/// Grow or shrink `ptr` to `new_size` bytes through the `Allocator` impl, keeping its alignment.
unsafe fn realloc(arena: &Arena, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> NonNull<u8> {
    let new_layout = Layout::from_size_align(new_size, layout.align()).unwrap();
    let result = if new_size <= layout.size() {
        unsafe { Allocator::shrink(&arena, ptr, layout, new_layout) }
    } else {
        unsafe { Allocator::grow(&arena, ptr, layout, new_layout) }
    };
    result.unwrap().cast::<u8>()
}

// Uses private `DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER` and `with_min_align_and_capacity`
#[test]
fn test_realloc() {
    unsafe {
        const CAPACITY: usize = DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER;
        let mut b = Arena::<1>::with_min_align_and_capacity(CAPACITY);

        // `realloc` doesn't shrink allocations that aren't "worth it"
        let layout = Layout::from_size_align(100, 1).unwrap();
        let p = b.alloc_layout(layout);
        let q = realloc(&b, p, layout, 51);
        assert_eq!(p, q);
        b.reset();

        // `realloc` will shrink allocations that are "worth it"
        let layout = Layout::from_size_align(100, 1).unwrap();
        let p = b.alloc_layout(layout);
        let q = realloc(&b, p, layout, 50);
        assert_ne!(p, q);
        b.reset();

        // `realloc` will reuse the last allocation when growing
        let layout = Layout::from_size_align(10, 1).unwrap();
        let p = b.alloc_layout(layout);
        let q = realloc(&b, p, layout, 11);
        assert_eq!(q.as_ptr().addr(), p.as_ptr().addr() - 1);
        b.reset();

        // `realloc` will allocate a new chunk when growing the last allocation, if need be
        let layout = Layout::from_size_align(1, 1).unwrap();
        let p = b.alloc_layout(layout);
        let q = realloc(&b, p, layout, CAPACITY + 1);
        assert_ne!(q.as_ptr().addr(), p.as_ptr().addr() - CAPACITY);
        b.reset();

        // `realloc` will allocate and copy when reallocating anything that wasn't the last allocation
        let layout = Layout::from_size_align(1, 1).unwrap();
        let p = b.alloc_layout(layout);
        let _ = b.alloc_layout(layout);
        let q = realloc(&b, p, layout, 2);
        assert_ne!(q.as_ptr().addr(), p.as_ptr().addr() - 1);
        b.reset();
    }
}

// Growing an allocation that is no longer the most recent one must copy it, not read past its end
#[test]
fn invalid_read() {
    let b = &Arena::new();

    unsafe {
        let l1 = Layout::from_size_align(12000, 4).unwrap();
        let p1 = Allocator::allocate(&b, l1).unwrap().cast::<u8>();

        let l2 = Layout::from_size_align(1000, 4).unwrap();
        Allocator::allocate(&b, l2).unwrap();

        let l3 = Layout::from_size_align(24000, 4).unwrap();
        let p1 = Allocator::grow(&b, p1, l1, l3).unwrap().cast::<u8>();
        let l4 = Layout::from_size_align(48000, 4).unwrap();
        Allocator::grow(&b, p1, l3, l4).unwrap();
    }
}
