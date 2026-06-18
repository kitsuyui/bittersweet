//! Bit-manipulation predicates and operations for integer primitives.
//!
//! Position and range APIs use MSB-first indexing: position `0` is the most
//! significant bit, shown at the left edge of a binary literal or `bit_repr()`.
//! The last valid position is the least significant bit.
//!
//! See also: [`crate::matrix`] for bit matrix transposition over the same integer types.

mod base;
mod uints;
// re-export
pub use base::Bitline;
pub use uints::{Bitline128, Bitline16, Bitline32, Bitline64, Bitline8};
