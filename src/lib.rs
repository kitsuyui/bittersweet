// #![no_std]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod bitline;
pub mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn test_basis() {
        use crate::bitline::Bitline;
        assert_eq!(u8::as_empty().bit_repr(), "0".repeat(8));
    }
}
