# bittersweet

![Crates.io](https://img.shields.io/crates/v/bittersweet)
[![codecov](https://codecov.io/gh/kitsuyui/bittersweet/branch/main/graph/badge.svg?token=G5SJOXT99J)](https://codecov.io/gh/kitsuyui/bittersweet)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

Bittersweet is a library for bit manipulation.

## Motivation

Bit-wise operations are often used in low-level programming and it sometimes contains magical bugs.
I want to manipulate bits with more
- type-safe
- well-tested
- well-documented
- fast and efficient

## Usage

### Installation

You can install this library via cargo.

```sh
$ cargo add bittersweet
```

Or you can add this to your `Cargo.toml` manually.

### Example

```rust
use bittersweet::bitline::{Bitline, Bitline8};
fn main() {
    let t = 0b00111000_u8 as Bitline8;
    if t.includes(0b00110000_u8) {
        println!("Yes!");
    }
}
```

## Supported bitline length

Following Bitline traits are implemented for `u8`, `u16`, `u32`, `u64`, `u128`.
So these operations keep zero-cost abstraction.

- `Bitline8` ... 8 bits (`u8`)
- `Bitline16` ... 16 bits (`u16`)
- `Bitline32` ... 32 bits (`u32`)
- `Bitline64` ... 64 bits (`u64`)
- `Bitline128` ... 128 bits (`u128`)

## Documentation

See [docs.rs](https://docs.rs/bittersweet/latest/bittersweet/)

## Manipulations

https://docs.rs/bittersweet/latest/bittersweet/bitline/trait.Bitline.html

- `as_empty`
- `as_full`
- `by_range`
- `bytes_length`
- `length`
- `is_empty`
- `is_not_empty`
- `is_full`
- `is_not_full`
- `first_index`
- `last_index`
- `radius`
- `around`
- `with_around`
- `first_bit`
- `last_bit`
- `first_bits`
- `last_bits`
- `filled_first_bit_to_last_bit`
- `num_bits`
- `bit_repr`
- `range`
- `includes`
- `overlaps`
- `remove`
- `left_rotate`
- `right_rotate`
- `bin_to_gray_code`
- `gray_code_to_bin`

## License

BSD-3-Clause
