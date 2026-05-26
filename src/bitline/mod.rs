//! Bit-manipulation predicates and operations for integer primitives.
//!
//! See also: [`crate::matrix`] for bit matrix transposition over the same integer types.

mod base;
mod uints;
// re-export
pub use base::Bitline;
pub use uints::{Bitline128, Bitline16, Bitline32, Bitline64, Bitline8};
