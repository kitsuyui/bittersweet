# Changelog

## 0.2.0 - Unreleased

### Added

- Add access, rank, rank range, and select operations for bitline values.
- Add left and right rotation helpers.
- Add Gray code conversion helpers.
- Add bit-reversal permutation conversion helpers.
- Add two-bit Gray code rotation.
- Add support for building without the default `std` feature.

### Changed

- Split the bitline implementation into smaller modules.
- Update `radius` to use union semantics when shifted neighborhoods overlap.
- Document that `select_0`, `select_1`, and `select` use a zero-based `nth` argument.

### Fixed

- Guard bit access against out-of-range indexes.
- Guard `radius`, `around`, and `with_around` against out-of-range shift amounts.
- Validate rank and rank range bounds instead of silently clamping invalid inputs.
