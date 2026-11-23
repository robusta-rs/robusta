//! Arena allocator for robusta's per-file data.
//!
//! Every node the parser produces is allocated in one bump arena that the file
//! holds while it is being processed, then freed all at once without running
//! per-node drop glue. This is what makes the abstract typed AST fast: no
//! per-node allocation or reference counting, and good cache locality.

mod accessor;
mod allocator;
mod allocator_api2;
#[cfg(not(feature = "testing"))]
mod arena;
#[cfg(feature = "testing")]
#[doc(hidden)]
pub mod arena;
mod boxed;
mod convert;
pub mod hash_map;
#[cfg(feature = "pool")]
mod pool;
mod take_in;
#[cfg(all(feature = "track_allocations", not(feature = "disable_track_allocations")))]
mod tracking;
mod vec;
mod vec2;

pub use self::allocator::Allocator;
pub use self::hash_map::HashMap;
#[cfg(feature = "pool")]
pub use self::pool::{AllocatorGuard, AllocatorPool};

/// Arena-backed collection types the AST is built from. AST list fields use
/// [`Vec`]; single owned children use [`Box`].
pub use self::{
    accessor::GetAllocator,
    boxed::Box,
    convert::{FromIn, IntoIn},
    take_in::{Dummy, TakeIn},
    vec::Vec,
};

#[cfg(test)]
mod tests {
    use super::*;

    struct Node<'alloc> {
        children: Vec<'alloc, Node<'alloc>>,
        child: Option<Box<'alloc, Node<'alloc>>>,
    }

    #[test]
    fn arena_containers_do_not_attach_recursive_drop_glue() {
        assert!(!std::mem::needs_drop::<Vec<'_, Node<'_>>>());
        assert!(!std::mem::needs_drop::<Box<'_, Node<'_>>>());
        assert!(!std::mem::needs_drop::<HashMap<'_, u32, Node<'_>>>());

        let allocator = &Allocator::new();
        let mut children = Vec::new_in(&allocator);
        children.push(Node { children: Vec::new_in(&allocator), child: None });
        let node = Box::new_in(Node { children, child: None }, &allocator);

        assert_eq!(node.children.len(), 1);
        assert!(node.child.is_none());
    }
}
