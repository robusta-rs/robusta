// Adapted from https://github.com/oxc-project/oxc/blob/566b11b2db6b664e787e67e2199d4ffcaa31c30a/crates/oxc_semantic/src/ast_types_bitset.rs

use robusta_ast::AstType;

const WORD_BITS: usize = usize::BITS as usize;
const NUM_WORDS: usize = AstType::COUNT.div_ceil(WORD_BITS);

/// A bit set with one bit per [`AstType`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstTypesBitset([usize; NUM_WORDS]);

impl AstTypesBitset {
    /// The empty set.
    pub const fn new() -> Self {
        Self([0; NUM_WORDS])
    }

    /// The set holding exactly `types`.
    pub const fn from_types(types: &[AstType]) -> Self {
        let mut bitset = Self::new();
        let mut i = 0;
        while i < types.len() {
            bitset.set(types[i]);
            i += 1;
        }
        bitset
    }

    /// Whether `ty` is in the set.
    pub const fn has(&self, ty: AstType) -> bool {
        let (index, mask) = Self::index_and_mask(ty);
        (self.0[index] & mask) != 0
    }

    /// Insert `ty` into the set.
    pub const fn set(&mut self, ty: AstType) {
        let (index, mask) = Self::index_and_mask(ty);
        self.0[index] |= mask;
    }

    /// Word index and bit mask for `ty`; the index is always in bounds
    /// because discriminants are dense in `0..AstType::COUNT`.
    const fn index_and_mask(ty: AstType) -> (usize, usize) {
        let n = ty as usize;
        (n / WORD_BITS, 1 << (n % WORD_BITS))
    }
}

impl Default for AstTypesBitset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_types_sets_exactly_the_given_kinds() {
        let bitset = AstTypesBitset::from_types(&[AstType::IntLiteral, AstType::LambdaExpression]);
        assert!(bitset.has(AstType::IntLiteral));
        assert!(bitset.has(AstType::LambdaExpression));
        let members = AstType::ALL.iter().filter(|ty| bitset.has(**ty)).count();
        assert_eq!(members, 2);
    }

    #[test]
    fn every_kind_is_representable() {
        let all = AstTypesBitset::from_types(&AstType::ALL);
        for ty in AstType::ALL {
            assert!(all.has(ty));
        }
        let empty = AstTypesBitset::new();
        for ty in AstType::ALL {
            assert!(!empty.has(ty));
        }
    }
}
