use std::cmp;

pub type Bitline8 = u8;
pub type Bitline16 = u16;
pub type Bitline32 = u32;
pub type Bitline64 = u64;
pub type Bitline128 = u128;

pub trait Bitline {
    /// return the bits all set to 0
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::as_empty(), 0b00000000);
    /// ```
    fn as_empty() -> Self;

    /// return the bits all set to 1
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::as_full(), 0b11111111);
    /// ```
    fn as_full() -> Self;

    /// return the bits standing in the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(Bitline8::by_range(2, 5), 0b00111000_u8);
    /// ```
    fn by_range(begin: usize, end: usize) -> Self;

    /// return true if the bit is filled with zero.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!(!0b00111000_u8.is_empty());
    /// assert!(0b00000000_u8.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// return true if the bit is not filled with zero.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!(0b00111000_u8.is_not_empty());
    /// assert!(!0b00000000_u8.is_not_empty());
    /// ```
    fn is_not_empty(&self) -> bool;

    /// return true if the bit is filled with one.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!((0b11111111 as Bitline8).is_full());
    /// assert!(!(0b01111111 as Bitline8).is_full());
    /// ```
    fn is_full(&self) -> bool;

    /// return true if the bit is not filled with one.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert!((0b01111111 as Bitline8).is_not_full());
    /// assert!(!(0b11111111 as Bitline8).is_not_full());
    /// ```
    fn is_not_full(&self) -> bool;

    /// return the first bit index that is set to one.
    /// If there is no bit set to one, return None.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!((0b00111000 as Bitline8).first_index(), Some(2));
    /// assert_eq!((0b00000000 as Bitline8).first_index(), None);
    /// ```

    fn first_index(&self) -> Option<usize>;
    /// return the last bit index that is set to one.
    /// If there is no bit set to one, return None.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!((0b00111000 as Bitline8).last_index(), Some(4));
    /// assert_eq!((0b00000000 as Bitline8).last_index(), None);
    /// ```
    fn last_index(&self) -> Option<usize>;

    /// return the bits standing in n distance from the original starting bit.
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

    /// return all bits standing between n distance from the standing bits (without original standing bits).
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

    /// return all bits standing between n distance from the standing bits (with original standing bits).
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

    /// return the first bit from the most significant bit. (last bit from the least significant bit)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01001000_u8;
    /// assert_eq!(bitline.first_bit(), 0b01000000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.first_bit(), 0b00000000_u8);
    /// ```
    fn first_bit(&self) -> Self;

    /// return the last bit from the most significant bit. (first bit from the least significant bit)
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01001000_u8;
    /// assert_eq!(bitline.last_bit(), 0b00001000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.last_bit(), 0b00000000_u8);
    /// ```
    fn last_bit(&self) -> Self;

    /// return the first bits of each consecutive bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.first_bits(), 0b01001000_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.first_bits(), 0b00000000_u8);
    /// ```
    fn first_bits(&self) -> Self;

    /// return the last bits of each consecutive bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.last_bits(), 0b00100100_u8);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.last_bits(), 0b00000000_u8);
    /// ```
    fn last_bits(&self) -> Self;

    /// return the first and last bit of each consecutive bits.
    fn first_and_last(&self) -> Self;

    /// return the bytes size of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8, Bitline16, Bitline32, Bitline64};
    /// assert_eq!(Bitline8::bytes_length(), 1);
    /// assert_eq!(Bitline16::bytes_length(), 2);
    /// assert_eq!(Bitline32::bytes_length(), 4);
    /// assert_eq!(Bitline64::bytes_length(), 8);
    /// ```
    fn bytes_length() -> usize;

    /// return the bits size of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8, Bitline16, Bitline32, Bitline64};
    /// assert_eq!(Bitline8::length(), 8);
    /// assert_eq!(Bitline16::length(), 16);
    /// assert_eq!(Bitline32::length(), 32);
    /// assert_eq!(Bitline64::length(), 64);
    /// ```
    fn length() -> usize;

    /// return the bits standing in the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.num_bits(), 4);
    /// let bitline = 0b00000000_u8;
    /// assert_eq!(bitline.num_bits(), 0);
    /// ```
    fn num_bits(&self) -> usize;

    /// return true if every bits are standing in the given standing bits.
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

    /// return true if some bits are standing in the given standing bits.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert!(bitline.overlaps(0b01100000_u8));
    /// assert!(!bitline.overlaps(0b00000001_u8));
    /// assert!(!bitline.overlaps(0b00000000_u8));
    /// ```
    fn overlaps(&self, other: Self) -> bool;

    /// return the standing bits by the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.range(0, 4), 0b01100000_u8);
    /// assert_eq!(bitline.range(4, 8), 0b00001100_u8);
    /// ```
    fn range(&self, begin: usize, end: usize) -> Self;

    /// return the standing bits not included by the given range.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// let bitline = 0b01101100_u8;
    /// assert_eq!(bitline.remove(0b01100000_u8), 0b00001100_u8);
    /// assert_eq!(bitline.remove(0b00001100_u8), 0b01100000_u8);
    /// ```
    fn remove(&self, other: Self) -> Self;

    /// return the string representation of the bitline.
    /// # Examples
    /// ```
    /// use bittersweet::bitline::{Bitline, Bitline8};
    /// assert_eq!(0b01101100_u8.bit_repr(), "01101100");
    /// ```
    fn bit_repr(&self) -> String;
}

macro_rules! impl_Bitline {
    ($T:ty) => {
        impl Bitline for $T {
            #[inline]
            fn as_empty() -> Self {
                0
            }
            #[inline]
            fn as_full() -> Self {
                Self::max_value()
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
            fn first_and_last(&self) -> Self {
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
            #[inline]
            fn bit_repr(&self) -> String {
                let formatted = format!("{:b}", self);
                let lack_bits = Self::length() - formatted.len();
                "0".repeat(lack_bits) + &formatted
            }
        }
    };
}

impl_Bitline!(Bitline8);
impl_Bitline!(Bitline16);
impl_Bitline!(Bitline32);
impl_Bitline!(Bitline64);
impl_Bitline!(Bitline128);

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
fn test_first_and_last() {
    assert_eq!(0b01000000_u8.first_and_last(), 0b01000000_u8);
    assert_eq!(0b00010000_u8.first_and_last(), 0b00010000_u8);
    assert_eq!(0b00010100_u8.first_and_last(), 0b00011100_u8);
    assert_eq!(0b00000100_u8.first_and_last(), 0b00000100_u8);
    assert_eq!(0b00000001_u8.first_and_last(), 0b00000001_u8);
    assert_eq!(0b00000000_u8.first_and_last(), 0b00000000_u8);
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
