#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::prelude::v1::*;

use crate::bitline::base::Bitline;
use core::cmp;
use core::option::{
    Option,
    Option::{None, Some},
};

pub type Bitline8 = u8;
pub type Bitline16 = u16;
pub type Bitline32 = u32;
pub type Bitline64 = u64;
pub type Bitline128 = u128;

macro_rules! impl_Bitline {
    ($T:ty, $Size:literal) => {
        impl Bitline for $T {
            #[inline]
            fn as_empty() -> Self {
                0
            }
            #[inline]
            fn as_full() -> Self {
                Self::MAX
            }

            #[inline(always)]
            fn mask_01() -> Self {
                // constant 0b01010101...
                let mut mask = 0 as $T;
                let shifts = $Size / 2;
                let val = 1 as $T;
                for i in 0..shifts {
                    mask |= val << (i * 2);
                }
                mask
            }

            #[inline(always)]
            fn mask_10() -> Self {
                // constant 0b10101010...
                let mut mask = 0 as $T;
                let shifts = $Size / 2;
                let val = 2 as $T;
                for i in 0..shifts {
                    mask |= val << (i * 2);
                }
                mask
            }

            #[inline]
            fn by_range(begin: usize, end: usize) -> Self {
                let bits_size = Self::BITS as usize;
                let last_index = cmp::min(end, bits_size);
                let first_index = cmp::min(begin, last_index);
                let size = last_index - first_index;
                if (size <= 0) {
                    return Self::as_empty();
                }
                if (size >= bits_size) {
                    return Self::as_full();
                }
                let fill_bits = (1 << size) - 1;
                let right_pad = bits_size - last_index;
                fill_bits << right_pad
            }
            #[inline]
            fn is_empty(&self) -> bool {
                *self == Self::as_empty()
            }
            #[inline]
            fn is_not_empty(&self) -> bool {
                !self.is_empty()
            }
            #[inline]
            fn is_full(&self) -> bool {
                *self == Self::as_full()
            }
            #[inline]
            fn is_not_full(&self) -> bool {
                !self.is_full()
            }
            #[inline]
            fn first_index(&self) -> Option<usize> {
                let zeros = self.leading_zeros() as usize;
                if (zeros == Self::length()) {
                    return None;
                }
                Some(zeros)
            }
            #[inline]
            fn last_index(&self) -> Option<usize> {
                let zeros = (Self::length() - self.trailing_zeros() as usize);
                if (zeros < 1) {
                    return None;
                }
                Some(zeros - 1)
            }
            #[inline]
            fn radius(&self, n: usize) -> Self {
                (self << n) ^ (self >> n)
            }
            #[inline]
            fn around(&self, n: usize) -> Self {
                let mut a = 0;
                for m in 0..(n + 1) {
                    a |= self.radius(m);
                }
                a
            }
            #[inline]
            fn with_around(&self, n: usize) -> Self {
                self | self.around(n)
            }
            #[inline]
            fn first_bit(&self) -> Self {
                let zeros = self.leading_zeros() as usize;
                if (zeros == Self::length()) {
                    return Self::as_empty();
                }
                1 << (Self::length() - zeros - 1)
            }
            #[inline]
            fn last_bit(&self) -> Self {
                let zeros = self.trailing_zeros() as usize;
                if (zeros == Self::length()) {
                    return Self::as_empty();
                }
                1 << self.trailing_zeros()
            }
            #[inline]
            fn first_bits(&self) -> Self {
                self & !(self >> 1)
            }
            #[inline]
            fn last_bits(&self) -> Self {
                self & !(self << 1)
            }
            #[inline]
            fn filled_first_bit_to_last_bit(&self) -> Self {
                if (self.is_empty()) {
                    return Self::as_empty();
                }
                let first_index = self.first_index().unwrap();
                let last_index = self.last_index().unwrap();
                Self::by_range(first_index, last_index + 1)
            }
            #[inline]
            fn length() -> usize {
                Self::BITS as usize
            }
            #[inline]
            fn bytes_length() -> usize {
                (Self::BITS / 8) as usize
            }
            #[inline]
            fn num_bits(&self) -> usize {
                self.count_ones() as usize
            }
            #[inline]
            fn includes(&self, other: Self) -> bool {
                (self | other) - self == 0
            }
            #[inline]
            fn overlaps(&self, other: Self) -> bool {
                self & other != 0
            }
            #[inline]
            fn range(&self, begin: usize, end: usize) -> Self {
                self & Self::by_range(begin, end)
            }
            #[inline]
            fn remove(&self, other: Self) -> Self {
                self & !other
            }
            #[cfg(feature = "std")]
            #[inline]
            fn bit_repr(&self) -> String {
                let formatted = format!("{:b}", self);
                let lack_bits = Self::length() - formatted.len();
                "0".repeat(lack_bits) + &formatted
            }
            #[inline]
            fn left_rotate(&self, n: usize) -> Self {
                let n = n % Self::length();
                if (n == 0) {
                    return *self;
                }
                (self << n) | (self >> (Self::length() - n))
            }
            #[inline]
            fn right_rotate(&self, n: usize) -> Self {
                let n = n % Self::length();
                if (n == 0) {
                    return *self;
                }
                (self >> n) | (self << (Self::length() - n))
            }
            #[inline]
            fn bin_to_gray_code(&self) -> Self {
                *self ^ (*self >> 1)
            }
            #[inline]
            fn gray_code_to_bin(&self) -> Self {
                let g = *self;
                let mut n = g;
                let mut shift = 1;
                while (g >> shift > 0) {
                    n ^= g >> shift;
                    shift += 1;
                    if (shift >= Self::length()) {
                        break;
                    }
                }
                n
            }

            #[inline]
            fn bin_to_bit_reversal_permutation(&self) -> Self {
                let bits = Self::length();
                let mut reversed_index = 0;
                for i in 0..bits {
                    if (*self & (1 << i)) != 0 {
                        reversed_index |= 1 << (bits - 1 - i);
                    }
                }
                reversed_index
            }
            #[inline]
            fn bit_reversal_permutation_to_bin(&self) -> Self {
                self.bin_to_bit_reversal_permutation()
            }

            #[inline]
            fn two_bits_gray_code_rotation(&self) -> Self {
                let r = (self & Self::mask_01()) << 1;
                let l = ((!self) & Self::mask_10()) >> 1;
                r | l
            }
        }
    };
}

impl_Bitline!(Bitline8, 8);
impl_Bitline!(Bitline16, 16);
impl_Bitline!(Bitline32, 32);
impl_Bitline!(Bitline64, 64);
impl_Bitline!(Bitline128, 128);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_as_empty() {
        assert_eq!(u8::as_empty().bit_repr(), "0".repeat(8));
        assert_eq!(u16::as_empty().bit_repr(), "0".repeat(16));
        assert_eq!(u32::as_empty().bit_repr(), "0".repeat(32));
        assert_eq!(u64::as_empty().bit_repr(), "0".repeat(64));
        assert_eq!(u128::as_empty().bit_repr(), "0".repeat(128));
    }

    #[test]
    fn test_as_full() {
        assert_eq!(u8::as_full().bit_repr(), "1".repeat(8));
        assert_eq!(u16::as_full().bit_repr(), "1".repeat(16));
        assert_eq!(u32::as_full().bit_repr(), "1".repeat(32));
        assert_eq!(u64::as_full().bit_repr(), "1".repeat(64));
        assert_eq!(u128::as_full().bit_repr(), "1".repeat(128));
    }

    #[test]
    fn test_mask() {
        assert_eq!(u8::mask_01().bit_repr(), "01010101");
        assert_eq!(u8::mask_10().bit_repr(), "10101010");
        assert_eq!(u16::mask_01().bit_repr(), "0101010101010101");
        assert_eq!(u16::mask_10().bit_repr(), "1010101010101010");
        assert_eq!(
            u32::mask_01().bit_repr(),
            "01010101010101010101010101010101"
        );
        assert_eq!(
            u32::mask_10().bit_repr(),
            "10101010101010101010101010101010"
        );
        assert_eq!(
            u64::mask_01().bit_repr(),
            "0101010101010101010101010101010101010101010101010101010101010101"
        );
        assert_eq!(
            u64::mask_10().bit_repr(),
            "1010101010101010101010101010101010101010101010101010101010101010"
        );
        assert_eq!(u128::mask_01().bit_repr(), "01010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101");
        assert_eq!(u128::mask_10().bit_repr(), "10101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010");
    }

    #[test]
    fn test_by_range() {
        assert_eq!(u8::by_range(3, 4), 0b00010000);
        assert_eq!(u8::by_range(0, 8), 0b11111111);
        assert_eq!(u8::by_range(0, 0), 0b00000000);
    }

    #[test]
    fn test_first_index() {
        assert_eq!(0b01000000_u8.first_index(), Some(1));
        assert_eq!(0b00010000_u8.first_index(), Some(3));
        assert_eq!(0b00010100_u8.first_index(), Some(3));
        assert_eq!(0b00000100_u8.first_index(), Some(5));
        assert_eq!(0b00000001_u8.first_index(), Some(7));
        assert!(0b00000000_u8.first_index().is_none());
    }

    #[test]
    fn test_last_index() {
        assert_eq!(0b01000000_u8.last_index(), Some(1));
        assert_eq!(0b00010000_u8.last_index(), Some(3));
        assert_eq!(0b00010100_u8.last_index(), Some(5));
        assert_eq!(0b00000100_u8.last_index(), Some(5));
        assert_eq!(0b00000001_u8.last_index(), Some(7));
        assert!(0b00000000_u8.last_index().is_none());
    }

    #[test]
    fn test_radius() {
        assert_eq!(0b00010000_u8.radius(0), 0b00000000_u8);
        assert_eq!(0b00010000_u8.radius(1), 0b00101000_u8);
        assert_eq!(0b00010000_u8.radius(2), 0b01000100_u8);
        assert_eq!(0b00010000_u8.radius(3), 0b10000010_u8);
        assert_eq!(0b00010000_u8.radius(4), 0b00000001_u8);
        assert_eq!(0b00000000_u8.radius(0), 0b00000000_u8);
        assert_eq!(0b00000000_u8.radius(1), 0b00000000_u8);
        assert_eq!(0b00000000_u8.radius(2), 0b00000000_u8);
    }

    #[test]
    fn test_around() {
        assert_eq!(0b00010000_u8.around(0), 0b00000000_u8);
        assert_eq!(0b00010000_u8.around(1), 0b00101000_u8);
        assert_eq!(0b00010000_u8.around(2), 0b01101100_u8);
        assert_eq!(0b00010000_u8.around(3), 0b11101110_u8);
        assert_eq!(0b00010000_u8.around(4), 0b11101111_u8);
        assert_eq!(0b00000000_u8.around(0), 0b00000000_u8);
        assert_eq!(0b00000000_u8.around(1), 0b00000000_u8);
        assert_eq!(0b00000000_u8.around(2), 0b00000000_u8);
    }

    #[test]
    fn test_with_around() {
        assert_eq!(0b00010000_u8.with_around(0), 0b00010000_u8);
        assert_eq!(0b00010000_u8.with_around(1), 0b00111000_u8);
        assert_eq!(0b00010000_u8.with_around(2), 0b01111100_u8);
        assert_eq!(0b00010000_u8.with_around(3), 0b11111110_u8);
        assert_eq!(0b00010000_u8.with_around(4), 0b11111111_u8);
        assert_eq!(0b00000000_u8.with_around(0), 0b00000000_u8);
        assert_eq!(0b00000000_u8.with_around(1), 0b00000000_u8);
        assert_eq!(0b00000000_u8.with_around(2), 0b00000000_u8);
    }

    #[test]
    fn test_first_bit() {
        assert_eq!(0b01000000_u8.first_bit(), 0b01000000_u8);
        assert_eq!(0b00010000_u8.first_bit(), 0b00010000_u8);
        assert_eq!(0b00010100_u8.first_bit(), 0b00010000_u8);
        assert_eq!(0b00000100_u8.first_bit(), 0b00000100_u8);
        assert_eq!(0b00000001_u8.first_bit(), 0b00000001_u8);
        assert_eq!(0b00000000_u8.first_bit(), 0b00000000_u8);
    }

    #[test]
    fn test_last_bit() {
        assert_eq!(0b01000000_u8.last_bit(), 0b01000000_u8);
        assert_eq!(0b00010000_u8.last_bit(), 0b00010000_u8);
        assert_eq!(0b00010100_u8.last_bit(), 0b00000100_u8);
        assert_eq!(0b00000100_u8.last_bit(), 0b00000100_u8);
        assert_eq!(0b00000001_u8.last_bit(), 0b00000001_u8);
        assert_eq!(0b00000000_u8.last_bit(), 0b00000000_u8);
    }

    #[test]
    fn test_first_bits() {
        assert_eq!(0b11111111_u8.first_bits(), 0b10000000_u8);
        assert_eq!(0b01000000_u8.first_bits(), 0b01000000_u8);
        assert_eq!(0b01100110_u8.first_bits(), 0b01000100_u8);
    }

    #[test]
    fn test_last_bits() {
        assert_eq!(0b11111111_u8.last_bits(), 0b00000001_u8);
        assert_eq!(0b01000000_u8.last_bits(), 0b01000000_u8);
        assert_eq!(0b01100110_u8.last_bits(), 0b00100010_u8);
    }

    #[test]
    fn test_filled_first_bit_to_last_bit() {
        assert_eq!(0b01000000_u8.filled_first_bit_to_last_bit(), 0b01000000_u8);
        assert_eq!(0b00010000_u8.filled_first_bit_to_last_bit(), 0b00010000_u8);
        assert_eq!(0b00010100_u8.filled_first_bit_to_last_bit(), 0b00011100_u8);
        assert_eq!(0b00000100_u8.filled_first_bit_to_last_bit(), 0b00000100_u8);
        assert_eq!(0b00000001_u8.filled_first_bit_to_last_bit(), 0b00000001_u8);
        assert_eq!(0b00000000_u8.filled_first_bit_to_last_bit(), 0b00000000_u8);
    }

    #[test]
    fn test_length() {
        assert_eq!(u8::length(), 8);
        assert_eq!(u16::length(), 16);
        assert_eq!(u32::length(), 32);
        assert_eq!(u64::length(), 64);
        assert_eq!(u128::length(), 128);
    }

    #[test]
    fn test_bytes_length() {
        assert_eq!(u8::bytes_length(), 1);
        assert_eq!(u16::bytes_length(), 2);
        assert_eq!(u32::bytes_length(), 4);
        assert_eq!(u64::bytes_length(), 8);
        assert_eq!(u128::bytes_length(), 16);
    }

    #[test]
    fn test_is_empty() {
        assert!(u8::as_empty().is_empty());
        assert!(u16::as_empty().is_empty());
        assert!(u32::as_empty().is_empty());
        assert!(u64::as_empty().is_empty());
        assert!(u128::as_empty().is_empty());
        assert!(0b00000000_u8.is_empty());
        assert!(!0b00000001_u8.is_empty());
        assert!(!0b10000000_u8.is_empty());
        assert!(!0b00001000_u8.is_empty());
        assert!(!0b00000000_u8.is_not_empty());
        assert!(0b00000001_u8.is_not_empty());
        assert!(0b10000000_u8.is_not_empty());
        assert!(0b00001000_u8.is_not_empty());
    }

    #[test]
    fn test_is_full() {
        assert!(u8::as_full().is_full());
        assert!(u16::as_full().is_full());
        assert!(u32::as_full().is_full());
        assert!(u64::as_full().is_full());
        assert!(u128::as_full().is_full());
        assert!(0b11111111_u8.is_full());
        assert!(!0b11111110_u8.is_full());
        assert!(!0b01111111_u8.is_full());
        assert!(!0b11101111_u8.is_full());
        assert!(!0b11111111_u8.is_not_full());
        assert!(0b11111110_u8.is_not_full());
        assert!(0b01111111_u8.is_not_full());
        assert!(0b11101111_u8.is_not_full());
    }

    #[test]
    fn test_num_bits() {
        assert_eq!(0b00000000_u8.num_bits(), 0);
        assert_eq!(0b00001000_u8.num_bits(), 1);
        assert_eq!(0b01001000_u8.num_bits(), 2);
        assert_eq!(0b01101000_u8.num_bits(), 3);
        assert_eq!(0b11111111_u8.num_bits(), 8);
    }

    #[test]
    fn test_includes() {
        assert!(0b00000000_u8.includes(0b00000000_u8));
        assert!(0b00011110_u8.includes(0b00000110_u8));
    }

    #[test]
    fn test_overlaps() {
        assert!(!0b11110000_u8.overlaps(0b00001111_u8));
        assert!(0b00011110_u8.overlaps(0b00011000_u8));
    }

    #[test]
    fn test_range() {
        assert_eq!(0b11111111_u8.range(2, 6), 0b00111100_u8);
        assert_eq!(0b10101010_u8.range(2, 6), 0b00101000_u8);
        assert_eq!(0b01010101_u8.range(2, 6), 0b00010100_u8);
    }

    #[test]
    fn test_remove() {
        assert_eq!(0b11110000_u8.remove(0b00001111_u8), 0b11110000_u8);
        assert_eq!(0b11110000_u8.remove(0b00111100_u8), 0b11000000_u8);
    }

    #[test]
    fn test_bin_repr() {
        assert_eq!(0b11110000_u8.bit_repr(), "11110000");
    }

    #[test]
    fn test_left_rotate() {
        // 0 means no change
        assert_eq!(0b11110000_u8.left_rotate(0), 0b11110000_u8);

        assert_eq!(0b11110000_u8.left_rotate(1), 0b11100001_u8);
        assert_eq!(0b11110000_u8.left_rotate(2), 0b11000011_u8);
        assert_eq!(0b11110000_u8.left_rotate(3), 0b10000111_u8);
        assert_eq!(0b11110000_u8.left_rotate(4), 0b00001111_u8);
        assert_eq!(0b11110000_u8.left_rotate(5), 0b00011110_u8);
        assert_eq!(0b11110000_u8.left_rotate(6), 0b00111100_u8);
        assert_eq!(0b11110000_u8.left_rotate(7), 0b01111000_u8);
        assert_eq!(0b11110000_u8.left_rotate(8), 0b11110000_u8);
        assert_eq!(0b11110000_u8.left_rotate(9), 0b11100001_u8);
    }

    #[test]
    fn test_right_rotate() {
        // 0 means no change
        assert_eq!(0b11110000_u8.right_rotate(0), 0b11110000_u8);

        assert_eq!(0b11110000_u8.right_rotate(1), 0b01111000_u8);
        assert_eq!(0b11110000_u8.right_rotate(2), 0b00111100_u8);
        assert_eq!(0b11110000_u8.right_rotate(3), 0b00011110_u8);
        assert_eq!(0b11110000_u8.right_rotate(4), 0b00001111_u8);
        assert_eq!(0b11110000_u8.right_rotate(5), 0b10000111_u8);
        assert_eq!(0b11110000_u8.right_rotate(6), 0b11000011_u8);
        assert_eq!(0b11110000_u8.right_rotate(7), 0b11100001_u8);
        assert_eq!(0b11110000_u8.right_rotate(8), 0b11110000_u8);
        assert_eq!(0b11110000_u8.right_rotate(9), 0b01111000_u8);
    }

    #[test]
    fn test_right_rotate_and_left_rotate_is_reversible() {
        for bitline in 0..256 {
            for i in 0..128 {
                let bitline = bitline as u8;
                assert_eq!(bitline.right_rotate(i).left_rotate(i), bitline);
            }
        }
    }

    #[test]
    fn test_to_gray_code() {
        assert_eq!(0b00000000_u8.bin_to_gray_code(), 0b00000000_u8);
        assert_eq!(0b00000001_u8.bin_to_gray_code(), 0b00000001_u8);
        assert_eq!(0b00000010_u8.bin_to_gray_code(), 0b00000011_u8);
        assert_eq!(0b00000011_u8.bin_to_gray_code(), 0b00000010_u8);
        assert_eq!(0b00000100_u8.bin_to_gray_code(), 0b00000110_u8);
    }

    #[test]
    fn test_from_gray_code() {
        assert_eq!(0b00000000_u8.gray_code_to_bin(), 0b00000000_u8);
        assert_eq!(0b00000001_u8.gray_code_to_bin(), 0b00000001_u8);
        assert_eq!(0b00000011_u8.gray_code_to_bin(), 0b00000010_u8);
        assert_eq!(0b00000011_u8.gray_code_to_bin(), 0b00000010_u8);
        assert_eq!(0b00000010_u8.gray_code_to_bin(), 0b00000011_u8);
        assert_eq!(0b00000110_u8.gray_code_to_bin(), 0b00000100_u8);
    }

    #[test]
    fn test_to_gray_code_and_from_gray_code_is_reversible() {
        for bitline in 0..255 {
            let bitline = bitline as u8;
            assert_eq!(bitline.gray_code_to_bin().bin_to_gray_code(), bitline);
            assert_eq!(bitline.bin_to_gray_code().gray_code_to_bin(), bitline);
        }
    }

    #[test]
    fn test_gray_code_distance() {
        // sequential gray code has always 1 bit difference.
        for num in 0..255 {
            let num = num as u8;
            let gray_code = num.bin_to_gray_code();
            let next_gray_code = (num + 1).bin_to_gray_code();
            let distance = (gray_code ^ next_gray_code).num_bits();
            assert_eq!(distance, 1);
        }
    }

    #[test]
    fn test_bin_to_gray_code_is_bijection() {
        // gray code is bijection. no collision.
        assert_bijection(|bitline| bitline.bin_to_gray_code());
    }

    #[test]
    fn test_gray_code_to_bin_is_bijection() {
        // gray code is bijection. no collision.
        assert_bijection(|bitline| bitline.gray_code_to_bin());
    }

    #[test]
    fn test_bin_to_bit_reversal_permutation() {
        assert_eq!(
            0b00000000_u8.bin_to_bit_reversal_permutation(),
            0b00000000_u8
        );
        assert_eq!(
            0b00000001_u8.bin_to_bit_reversal_permutation(),
            0b10000000_u8
        );
        assert_eq!(
            0b00000010_u8.bin_to_bit_reversal_permutation(),
            0b01000000_u8
        );
        // ...
        for bitline in 0..256 {
            let bitline = bitline as u8;
            let tobe_reverse_str = bitline.bit_repr().chars().rev().collect::<String>();
            let asis_reversed_str = bitline.bin_to_bit_reversal_permutation().bit_repr();
            assert_eq!(tobe_reverse_str, asis_reversed_str,);
        }
    }

    #[test]
    fn test_bin_to_bit_reversal_permutation_and_bit_reversal_permutation_to_bin_is_reversible() {
        for bitline in 0..256 {
            let bitline = bitline as u8;
            assert_eq!(
                bitline
                    .bin_to_bit_reversal_permutation()
                    .bit_reversal_permutation_to_bin(),
                bitline
            );
        }
    }

    #[test]
    fn test_to_bit_reversal_permutation_is_bijection() {
        // bit reversal permutation is bijection. no collision.
        assert_bijection(|bitline| bitline.bin_to_bit_reversal_permutation());
    }

    #[test]
    fn test_from_bit_reversal_permutation_is_bijection() {
        // bit reversal permutation is bijection. no collision.
        assert_bijection(|bitline| bitline.bit_reversal_permutation_to_bin());
    }

    #[test]
    fn test_two_bits_gray_code_rotation() {
        assert_eq!(0b00000000_u8.two_bits_gray_code_rotation(), 0b01010101_u8);
        assert_eq!(0b01000001_u8.two_bits_gray_code_rotation(), 0b11010111_u8);
        assert_eq!(0b10100010_u8.two_bits_gray_code_rotation(), 0b00000100_u8);
        // 3 times rotation is the same as no rotation.
        assert_bijection(|bitline| {
            bitline
                .two_bits_gray_code_rotation()
                .two_bits_gray_code_rotation()
                .two_bits_gray_code_rotation()
        });
    }

    fn assert_bijection(function: fn(u8) -> u8) {
        // bijection means no collision.
        let mut counter = HashMap::new();
        for bitline in 0..256 {
            let bitline = bitline as u8;
            let result = function(bitline);
            let count = counter.entry(result).or_insert(0);
            *count += 1;
        }
        for (_, count) in counter.iter() {
            assert_eq!(*count, 1);
        }
        assert_eq!(counter.len(), 256);
    }
}
