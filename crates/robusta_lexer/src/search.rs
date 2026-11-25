// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_parser/src/lexer/search.rs
//! Searching source for combinations of byte values.
//!
//! * `SafeByteMatchTable` is a lookup table type for byte values.
//! * The `safe_byte_match_table!` macro creates those tables at compile time.
//! * [`Lexer::byte_search`] searches the rest of the source for the first byte
//!   matching a `SafeByteMatchTable`.

use crate::Lexer;

/// Batch size for searching
const SEARCH_BATCH_SIZE: usize = 16;

/// Safe byte matcher lookup table.
///
/// Create table at compile time as a `static` or `const` with `safe_byte_match_table!` macro.
/// Test bytes against table with `SafeByteMatchTable::matches`.
/// Or use [`Lexer::byte_search`] to search for first matching byte in source.
///
/// [`Lexer::byte_search`] using this table is guaranteed to leave the lexer positioned on a UTF-8
/// character boundary, provided that its starting position is already on a UTF-8 character
/// boundary.
///
/// To make this guarantee, one of the following must be true:
///
/// 1. Table contains `true` for all byte values 192 - 247
///    i.e. first byte of any multi-byte Unicode character matches.
///    (NB: 248 - 255 cannot occur in UTF-8 strings)
/// 2. Table contains `false` for all byte values 128 - 191
///    i.e. the continuation bytes of any multi-byte Unicode chars will be consumed in full.
///
/// This is statically checked by `SafeByteMatchTable::new`, and will fail to compile if match
/// pattern does not satisfy one of the above.
#[repr(C, align(64))]
pub struct SafeByteMatchTable([bool; 256]);

impl SafeByteMatchTable {
    /// Create new `SafeByteMatchTable`.
    pub const fn new(bytes: [bool; 256]) -> Self {
        let mut table = Self([false; 256]);

        // Check if contains either:
        // 1. `true` for all byte values 192..248
        // 2. `false` for all byte values 128..192
        let mut unicode_start_all_match = true;
        let mut unicode_cont_all_no_match = true;

        let mut i = 0;
        while i < 256 {
            let matches = bytes[i];
            table.0[i] = matches;

            if matches {
                if i >= 128 && i < 192 {
                    unicode_cont_all_no_match = false;
                }
            } else if i >= 192 && i < 248 {
                unicode_start_all_match = false;
            }

            i += 1;
        }

        assert!(
            unicode_start_all_match || unicode_cont_all_no_match,
            "Cannot create a `SafeByteMatchTable` with an unsafe pattern"
        );

        table
    }

    /// Test a value against this `SafeByteMatchTable`.
    #[inline]
    pub const fn matches(&self, b: u8) -> bool {
        self.0[b as usize]
    }
}

/// Macro to create a `SafeByteMatchTable` at compile time.
///
/// `safe_byte_match_table!(|b| matches!(b, b'"' | b'\\'))` evaluates the pattern for each of
/// the 256 byte values in a constant, so it costs nothing at runtime.
macro_rules! safe_byte_match_table {
    (|$byte:ident| $res:expr) => {{
        use crate::search::SafeByteMatchTable;
        const TABLE: SafeByteMatchTable = {
            let mut bytes = [false; 256];
            let mut i = 0;
            while i < 256 {
                #[expect(clippy::cast_possible_truncation)]
                let $byte = i as u8;
                bytes[i] = $res;
                i += 1;
            }
            SafeByteMatchTable::new(bytes)
        };
        TABLE
    }};
}
pub(crate) use safe_byte_match_table;

impl Lexer<'_> {
    /// Advance to the first byte at or after the current position that `table` matches, and
    /// return it. At the end of the source, return `None`, positioned at the end.
    ///
    /// Bytes are read in batches of `SEARCH_BATCH_SIZE`, so the compiler can unroll the inner loop.
    /// The table rules out stopping inside a multi-byte character.
    #[inline]
    pub fn byte_search(&mut self, table: &SafeByteMatchTable) -> Option<u8> {
        let (batches, rest) = self.src[self.pos..].as_chunks::<SEARCH_BATCH_SIZE>();
        let mut offset = 0;
        for batch in batches {
            for (i, &byte) in batch.iter().enumerate() {
                if table.matches(byte) {
                    self.pos += offset + i;
                    return Some(byte);
                }
            }
            offset += SEARCH_BATCH_SIZE;
        }
        for (i, &byte) in rest.iter().enumerate() {
            if table.matches(byte) {
                self.pos += offset + i;
                return Some(byte);
            }
        }
        self.pos = self.src.len();
        None
    }
}
