use bittersweet::matrix;

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
