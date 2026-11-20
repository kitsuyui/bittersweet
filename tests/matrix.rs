#![allow(deprecated)] // Covers compatibility tests for the deprecated raw-u64 API.

use bittersweet::matrix::{self, BitMatrix8x8};

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

#[test]
fn bit_matrix8x8_preserves_explicit_row_representation() {
    let rows = [
        0b00001000, 0b10001000, 0b10111110, 0b10001000, 0b10001000, 0b11001000, 0b10001000,
        0b00010000,
    ];
    let expected = [
        0b01111110, 0b00000100, 0b00100000, 0b00100001, 0b11111110, 0b00100000, 0b00100000,
        0b00000000,
    ];

    assert_eq!(
        BitMatrix8x8::from_rows(rows).transpose().into_rows(),
        expected
    );
}

#[test]
fn matrix_transpose8x8_u64_matches_be_byte_rows() {
    let cases: [[u8; 8]; 3] = [
        [
            0b10000000, 0b01000000, 0b00100000, 0b00010000, 0b00001000, 0b00000100, 0b00000010,
            0b00000001,
        ],
        [
            0b00001000, 0b10001000, 0b10111110, 0b10001000, 0b10001000, 0b11001000, 0b10001000,
            0b00010000,
        ],
        [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0],
    ];

    for rows in cases {
        assert_eq!(
            matrix::transpose8x8_u64(u64::from_be_bytes(rows)),
            u64::from_be_bytes(matrix::transpose8x8(rows))
        );
    }
}
