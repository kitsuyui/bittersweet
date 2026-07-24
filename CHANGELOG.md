# Changelog

## Unreleased

### Changed

- Publish the deprecation lifecycle for `is_not_empty` and `is_not_full`: both
  aliases remain available throughout the 0.3.x line and are scheduled for
  removal in 0.4.0.

## 0.3.0 - 2026-06-18

### Added

- Add non-panicking variants for index-based access methods: `try_access`,
  `try_rank_0`, `try_rank_1`, `try_rank_range_0`, `try_rank_range_1`.

### Fixed

- Reject inverted bit ranges (`begin > end`) in `by_range` and `range`
  instead of returning a silently empty result.
- Panic when the `end` index in `by_range` or `range` exceeds the bitline
  length instead of silently clamping to the bitline boundary.

### Changed

- Document that rotation distance is normalised modulo the bitline width.

### Docs

- Add cross-references between the `matrix` and `bitline` modules.
- Complete the list of `Bitline` trait manipulations in the README.

## 0.2.0 - 2026-05-14

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
