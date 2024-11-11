#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::prelude::v1::*;

use core::option::Option;

pub trait Bitline {
    /// Return the bits all set to 0
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::as_empty(), 0b00000000);
    /// ```
    fn as_empty() -> Self;

    /// Return the bits all set to 1
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::as_full(), 0b11111111);
    /// ```
    fn as_full() -> Self;

    /// Return the bits pattern such as 01010101...
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::mask_01(), 0b01010101_u8);
    /// ```
    fn mask_01() -> Self;

    /// Return the bits pattern such as 10101010...
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::mask_10(), 0b10101010_u8);
    /// ```
    fn mask_10() -> Self;

    /// Return the bits standing in the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::by_range(2, 5), 0b00111000_u8);
    /// ```
    fn by_range(begin: usize, end: usize) -> Self;

    /// Return true if the bit is filled with zero.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!(!0b00111000_u8.is_empty());
    /// assert!(0b00000000_u8.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Return true if the bit is not filled with zero.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!(0b00111000_u8.is_not_empty());
    /// assert!(!0b00000000_u8.is_not_empty());
    /// ```
    fn is_not_empty(&self) -> bool;

    /// Return true if the bit is filled with one.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!((0b11111111 as Bitline8).is_full());
    /// assert!(!(0b01111111 as Bitline8).is_full());
    /// ```
    fn is_full(&self) -> bool;

    /// Return true if the bit is not filled with one.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!((0b01111111 as Bitline8).is_not_full());
    /// assert!(!(0b11111111 as Bitline8).is_not_full());
    /// ```
    fn is_not_full(&self) -> bool;

    /// Return the first bit index that is set to one.
    /// If there is no bit set to one, return None.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!((0b00111000 as Bitline8).first_index(), Some(2));
    /// assert_eq!((0b00000000 as Bitline8).first_index(), None);
    /// ```
    fn first_index(&self) -> Option<usize>;

    /// Return the last bit index that is set to one.
    /// If there is no bit set to one, return None.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!((0b00111000 as Bitline8).last_index(), Some(4));
    /// assert_eq!((0b00000000 as Bitline8).last_index(), None);
    /// ```
    fn last_index(&self) -> Option<usize>;

    /// Return the bits standing in n distance from the original starting bit.
    /// If there is no bit set to one, return None.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00001000 as Bitline8;
    /// assert_eq!(bitline.radius(0), 0b00000000);
    /// assert_eq!(bitline.radius(1), 0b00010100);
    /// assert_eq!(bitline.radius(2), 0b00100010);
    ///
    /// let bitline = 0b00000000 as Bitline8;
    /// assert_eq!(bitline.radius(0), 0b00000000);
    /// assert_eq!(bitline.radius(1), 0b00000000);
    /// assert_eq!(bitline.radius(2), 0b00000000);
    ///
    /// let bitline = 0b00100100 as Bitline8;
    /// assert_eq!(bitline.radius(0), 0b00000000);
    /// assert_eq!(bitline.radius(1), 0b01011010);
    /// assert_eq!(bitline.radius(2), 0b10011001);
    /// ```
    fn radius(&self, n: usize) -> Self;

    /// Return all bits standing between n distance from the standing bits (without original standing bits).
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00001000 as Bitline8;
    /// assert_eq!(bitline.around(0), 0b00000000);
    /// assert_eq!(bitline.around(1), 0b00010100);
    /// assert_eq!(bitline.around(2), 0b00110110);
    ///
    /// let bitline = 0b00000000 as Bitline8;
    /// assert_eq!(bitline.around(0), 0b00000000);
    /// assert_eq!(bitline.around(1), 0b00000000);
    /// assert_eq!(bitline.around(2), 0b00000000);
    /// ```
    fn around(&self, n: usize) -> Self;

    /// Return all bits standing between n distance from the standing bits (with original standing bits).
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00001000 as Bitline8;
    /// assert_eq!(bitline.with_around(0), 0b00001000);
    /// assert_eq!(bitline.with_around(1), 0b00011100);
    /// assert_eq!(bitline.with_around(2), 0b00111110);
    ///
    /// let bitline = 0b00000000 as Bitline8;
    /// assert_eq!(bitline.with_around(0), 0b00000000);
    /// assert_eq!(bitline.with_around(1), 0b00000000);
    /// assert_eq!(bitline.with_around(2), 0b00000000);
    /// ```
    fn with_around(&self, n: usize) -> Self;

    /// Return the first bit from the most significant bit. (last bit from the least significant bit)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01001000_u8;
    /// assert_eq!(bitline.first_bit(), 0b01000000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.first_bit(), 0b00000000_u8);
    /// ```
    fn first_bit(&self) -> Self;

    /// Return the last bit from the most significant bit. (first bit from the least significant bit)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01001000_u8;
    /// assert_eq!(bitline.last_bit(), 0b00001000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.last_bit(), 0b00000000_u8);
    /// ```
    fn last_bit(&self) -> Self;

    /// Return the first bits of each consecutive bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.first_bits(), 0b01001000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.first_bits(), 0b00000000_u8);
    /// ```
    fn first_bits(&self) -> Self;

    /// Return the last bits of each consecutive bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.last_bits(), 0b00100100_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.last_bits(), 0b00000000_u8);
    /// ```
    fn last_bits(&self) -> Self;

    /// Return the bits filled from the first bit to the last bit.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01000100_u8;
    /// assert_eq!(bitline.filled_first_bit_to_last_bit(), 0b01111100_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.filled_first_bit_to_last_bit(), 0b00000000_u8);
    /// ```
    fn filled_first_bit_to_last_bit(&self) -> Self;

    /// Return the bytes size of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8, Bitline16, Bitline32, Bitline64};
    /// assert_eq!(Bitline8::bytes_length(), 1);
    /// assert_eq!(Bitline16::bytes_length(), 2);
    /// assert_eq!(Bitline32::bytes_length(), 4);
    /// assert_eq!(Bitline64::bytes_length(), 8);
    /// ```
    fn bytes_length() -> usize;

    /// Return the bits size of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8, Bitline16, Bitline32, Bitline64};
    /// assert_eq!(Bitline8::length(), 8);
    /// assert_eq!(Bitline16::length(), 16);
    /// assert_eq!(Bitline32::length(), 32);
    /// assert_eq!(Bitline64::length(), 64);
    /// ```
    fn length() -> usize;

    /// Return the bits standing in the given range. (bit count)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.num_bits(), 4);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.num_bits(), 0);
    /// ```
    fn num_bits(&self) -> usize;

    /// Return true if every bits are standing in the given standing bits.
    /// empty bitlines are always included. (like a empty set in a set)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert!(bitline.includes(0b01100000_u8));
    /// assert!(!bitline.includes(0b01100001_u8));
    /// assert!(bitline.includes(0b00000000_u8));
    ///
    /// let bitline = 0b00000000_u8;
    /// assert!(bitline.includes(0b00000000_u8));
    /// assert!(!bitline.includes(0b00000001_u8));
    /// ```
    fn includes(&self, other: Self) -> bool;

    /// Return true if some bits are standing in the given standing bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert!(bitline.overlaps(0b01100000_u8));
    /// assert!(!bitline.overlaps(0b00000001_u8));
    /// assert!(!bitline.overlaps(0b00000000_u8));
    /// ```
    fn overlaps(&self, other: Self) -> bool;

    /// Return the standing bits by the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.range(0, 4), 0b01100000_u8);
    /// assert_eq!(bitline.range(4, 8), 0b00001100_u8);
    /// ```
    fn range(&self, begin: usize, end: usize) -> Self;

    /// Return the standing bits not included by the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.remove(0b01100000_u8), 0b00001100_u8);
    /// assert_eq!(bitline.remove(0b00001100_u8), 0b01100000_u8);
    /// ```
    fn remove(&self, other: Self) -> Self;

    /// Return the string representation of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(0b01101100_u8.bit_repr(), "01101100");
    /// ```
    #[cfg(feature = "std")]
    fn bit_repr(&self) -> String;

    /// Return the bits rotated to the left by n bits.
    /// Overflowed bits are moved to the right side.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.left_rotate(1), 0b11011000_u8);
    /// assert_eq!(bitline.left_rotate(2), 0b10110001_u8);
    /// ```
    fn left_rotate(&self, n: usize) -> Self;

    /// Return the bits rotated to the right by n bits.
    /// Overflowed bits are moved to the left side.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.right_rotate(1), 0b00110110_u8);
    /// assert_eq!(bitline.right_rotate(2), 0b00011011_u8);
    /// assert_eq!(bitline.right_rotate(3), 0b10001101_u8);
    /// ```
    fn right_rotate(&self, n: usize) -> Self;

    /// Return the Gray code of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00000010_u8;
    /// assert_eq!(bitline.bin_to_gray_code(), 0b00000011_u8);
    /// ```
    fn bin_to_gray_code(&self) -> Self;

    /// Return the bitline from the Gray code.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00000011_u8;
    /// assert_eq!(bitline.gray_code_to_bin(), 0b00000010_u8);
    /// ```
    fn gray_code_to_bin(&self) -> Self;

    /// Return the bitline from the binary to the bit reversal permutation index.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00000010_u8;
    /// assert_eq!(bitline.bin_to_bit_reversal_permutation(), 0b01000000_u8);
    /// ```
    fn bin_to_bit_reversal_permutation(&self) -> Self;

    /// Return the bitline from the bit reversal permutation index to the binary.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01000000_u8;
    /// assert_eq!(bitline.bit_reversal_permutation_to_bin(), 0b00000010_u8);
    /// ```
    fn bit_reversal_permutation_to_bin(&self) -> Self;

    /// Return the bitline rotating bits splitting by two bits.
    ///
    /// 0b00 -> 0b01
    /// 0b01 -> 0b11
    /// 0b10 -> 0b00
    /// 0b11 -> 0b10
    /// 0b10 -> 0b00
    ///
    /// This rotation is useful to mapping bits into polar coordinates spinning by 90 degrees.
    ///
    /// | 10 (3) | 11 (2) |
    /// |--------+--------|
    /// | 00 (0) | 01 (1) |
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.two_bits_gray_code_rotation(), 0b01111000_u8);
    /// ```
    fn two_bits_gray_code_rotation(&self) -> Self;

    /// Access the specified position in the bit sequence and get the value of the bit.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.access(0), false);
    /// assert_eq!(bitline.access(1), false);
    /// assert_eq!(bitline.access(2), false);
    /// assert_eq!(bitline.access(3), true);
    /// assert_eq!(bitline.access(4), true);
    /// ```
    fn access(&self, index: usize) -> bool;

    /// Count how many times 0 appears up to the index (i-th) position.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank_0(0), 0);
    /// assert_eq!(bitline.rank_0(1), 1);
    /// assert_eq!(bitline.rank_0(2), 2);
    /// assert_eq!(bitline.rank_0(5), 3);
    /// ```
    fn rank_0(&self, index: usize) -> usize;

    /// Count how many times 1 appears up to the index (i-th) position.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank_1(0), 0);
    /// assert_eq!(bitline.rank_1(1), 0);
    /// assert_eq!(bitline.rank_1(2), 0);
    /// assert_eq!(bitline.rank_1(3), 0);
    /// assert_eq!(bitline.rank_1(4), 1);
    /// assert_eq!(bitline.rank_1(5), 2);
    /// assert_eq!(bitline.rank_1(6), 3);
    /// assert_eq!(bitline.rank_1(7), 4);
    /// assert_eq!(bitline.rank_1(8), 4);
    /// ```
    fn rank_1(&self, index: usize) -> usize;

    /// Count how many times specified bit appears up to the index (i-th) position.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank(0, false), 0);
    /// assert_eq!(bitline.rank(1, false), 1);
    /// assert_eq!(bitline.rank(2, false), 2);
    /// assert_eq!(bitline.rank(3, true), 0);
    /// ```
    fn rank(&self, index: usize, bit: bool) -> usize;

    /// Count how many times 0 appears between the begin (i-th) and the end (j-th) positions.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank_range_0(0, 1), 1);
    /// assert_eq!(bitline.rank_range_0(0, 3), 3);
    /// assert_eq!(bitline.rank_range_0(0, 4), 3);
    /// ```
    fn rank_range_0(&self, begin: usize, end: usize) -> usize;

    /// Count how many times 1 appears between the begin (i-th) and the end (j-th) positions.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank_range_1(0, 1), 0);
    /// assert_eq!(bitline.rank_range_1(0, 3), 0);
    /// assert_eq!(bitline.rank_range_1(0, 4), 1);
    /// assert_eq!(bitline.rank_range_1(3, 5), 2);
    /// ```
    fn rank_range_1(&self, begin: usize, end: usize) -> usize;

    /// Count how many times specified bit appears between the begin (i-th) and the end (j-th) positions.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.rank_range(0, 1, false), 1);
    /// assert_eq!(bitline.rank_range(0, 3, false), 3);
    /// assert_eq!(bitline.rank_range(0, 4, false), 3);
    /// assert_eq!(bitline.rank_range(0, 4, true), 1);
    /// ```
    fn rank_range(&self, begin: usize, end: usize, bit: bool) -> usize;

    /// Find the position where the n-th 0 appears.
    /// If there is no n-th 0, return None.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.select_0(0), Some(0));
    /// assert_eq!(bitline.select_0(1), Some(1));
    /// assert_eq!(bitline.select_0(2), Some(2));
    /// assert_eq!(bitline.select_0(3), Some(7));
    /// assert_eq!(bitline.select_0(4), None);
    /// ```
    fn select_0(&self, nth: usize) -> Option<usize>;

    /// Find the position where the n-th 1 appears.
    /// If there is no n-th 1, return None.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.select_1(0), Some(3));
    /// assert_eq!(bitline.select_1(1), Some(4));
    /// assert_eq!(bitline.select_1(2), Some(5));
    /// assert_eq!(bitline.select_1(3), Some(6));
    /// assert_eq!(bitline.select_1(4), None);
    /// ```
    fn select_1(&self, nth: usize) -> Option<usize>;

    /// Find the position where the n-th specified bit appears.
    /// If there is no n-th specified bit, return None.
    ///
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b00011110_u8;
    /// assert_eq!(bitline.select(0, false), Some(0));
    /// assert_eq!(bitline.select(1, false), Some(1));
    /// assert_eq!(bitline.select(2, false), Some(2));
    /// assert_eq!(bitline.select(0, true), Some(3));
    /// ```
    fn select(&self, nth: usize, bit: bool) -> Option<usize>;
}
