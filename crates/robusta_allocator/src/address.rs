// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_allocator/src/address.rs

// All methods are 1 instruction or less
#![expect(clippy::inline_always)]

use std::ptr::NonNull;

use crate::Box;

/// Memory address of an AST node in arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(usize);

/// Trait for getting the memory address of an AST node.
pub trait GetAddress {
    /// Get the memory address of a value allocated in the arena.
    fn address(&self) -> Address;
}

impl<T> GetAddress for Box<'_, T> {
    /// Get the memory address of a value allocated in the arena.
    ///
    /// AST nodes in a `Box` in an arena are guaranteed to never move in memory,
    /// so this address acts as a unique identifier for the duration of the arena's existence.
    #[inline(always)] // Because it's only 1 instruction
    fn address(&self) -> Address {
        let ptr = Box::as_non_null(self);
        Address(ptr.addr().get())
    }
}

/// Trait for getting the memory address of an AST node which is not necessarily stable.
///
/// See [`UnstableAddress::unstable_address`] for more details.
///
/// This trait is implemented for all AST node struct types.
///
/// *DO NOT* implement this trait on any other type.
pub trait UnstableAddress {
    /// Get the memory [`Address`] of a reference to an AST node in arena, which is not necessarily stable.
    ///
    /// # Stable addresses
    ///
    /// Types which have a stable address:
    /// * [`Box<T>`]
    /// * AST enums where all variants are `Box`es (e.g. `Statement`, `Expression`)
    /// * `robusta_ast::AstKind` - guaranteed stable address while `Semantic` is alive.
    ///
    /// The above types all implement [`GetAddress::address`]. If you have access to one of these types,
    /// it's better to use `GetAddress::address` instead of this method.
    ///
    /// # When a type does not move in memory
    ///
    /// If the AST is immutable (e.g. in `Visit` trait), then any node in the AST is statically positioned in memory.
    /// Therefore, in semantic analysis and the linter, any reference to an AST node is guaranteed to have
    /// a stable `Address`.
    ///
    /// If an AST node is in a `Vec`, then it'll remain at same memory address, but only as long as the `Vec`
    /// does not reallocate (e.g. by `Vec::push`, `Vec::extend`).
    ///
    /// When using this method, it is recommended to make a comment at the call site, explaining how you can prove
    /// that the `Address` will remain accurate (the AST node will not move) for the time period that you need it to be.
    ///
    /// # Common pitfalls
    ///
    /// Ensure that the reference passed to this method is to a node which is in the arena, *not* on the stack.
    /// An owned node moved onto the stack has an `Address` which is meaningless.
    ///
    /// [`Box<T>`]: crate::Box
    #[inline(always)] // Because it's a no-op
    fn unstable_address(&self) -> Address {
        let p = NonNull::from_ref(self);
        Address(p.addr().get())
    }
}
