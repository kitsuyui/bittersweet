use bittersweet::bitline::Bitline;
use bittersweet::matrix;

#[test]
fn bitline_empty_and_full() {
    assert!(u8::as_empty().is_empty());
    assert!(!u8::as_empty().is_not_empty());
    assert!(u8::as_full().is_full());
    assert!(!u8::as_full().is_not_full());
}

#[test]
fn bitline_by_range() {
    let b: u8 = Bitline::by_range(2, 5);
    assert_eq!(b, 0b00111000_u8);
}

#[test]
fn bitline_first_and_last_index() {
    let b = 0b00111000_u8;
    assert_eq!(b.first_index(), Some(2));
    assert_eq!(b.last_index(), Some(4));
    assert_eq!(u8::as_empty().first_index(), None);
    assert_eq!(u8::as_empty().last_index(), None);
}

#[test]
fn bitline_num_bits() {
    assert_eq!(0b01101100_u8.num_bits(), 4);
    assert_eq!(u8::as_empty().num_bits(), 0);
    assert_eq!(u8::as_full().num_bits(), 8);
}

#[test]
fn bitline_length_and_bytes_length() {
    assert_eq!(u8::length(), 8);
    assert_eq!(u8::bytes_length(), 1);
    assert_eq!(u16::length(), 16);
    assert_eq!(u16::bytes_length(), 2);
}

#[test]
fn bitline_rotate() {
    let b = 0b01101100_u8;
    assert_eq!(b.left_rotate(1), 0b11011000_u8);
    assert_eq!(b.right_rotate(1), 0b00110110_u8);
    assert_eq!(b.left_rotate(8), b);
}

#[test]
fn bitline_gray_code_roundtrip() {
    let b = 0b00000010_u8;
    assert_eq!(b.bin_to_gray_code().gray_code_to_bin(), b);
}

#[test]
fn matrix_transpose8x8_identity() {
    let identity: [u8; 8] = [
        0b10000000, 0b01000000, 0b00100000, 0b00010000, 0b00001000, 0b00000100, 0b00000010,
        0b00000001,
    ];
    assert_eq!(matrix::transpose8x8(identity), identity);
}

#[test]
fn matrix_transpose8x8_u64_involution() {
    let n: u64 = 0x0102030405060708;
    assert_eq!(matrix::transpose8x8_u64(matrix::transpose8x8_u64(n)), n);
}
