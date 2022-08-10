# bittersweet

![Crates.io](https://img.shields.io/crates/v/bittersweet)
[![codecov](https://codecov.io/gh/kitsuyui/bittersweet/branch/main/graph/badge.svg?token=G5SJOXT99J)](https://codecov.io/gh/kitsuyui/bittersweet)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)


Currently under development.

## Documentation

Check out [doc.rs](https://docs.rs/bittersweet/latest/bittersweet/)

## Usage

```rust
mod bitline;
use crate::bitline::Bitline;
fn main() {
    let t = 0b00111000_u8 as bitline::Bitline8;
    if t.includes(0b00110000_u8) {
        println!("Yes!");
    }
}
```

## License

BSD-3-Clause
