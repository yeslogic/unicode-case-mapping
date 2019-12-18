unicode-case-mapping
====================

<div align="center">
  <a href="https://travis-ci.com/yeslogic/unicode-case-mapping">
    <img src="https://travis-ci.com/yeslogic/unicode-case-mapping.svg?branch=master" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-case-mapping">
    <img src="https://docs.rs/unicode-case-mapping/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/unicode-case-mapping">
    <img src="https://img.shields.io/crates/v/unicode-case-mapping.svg" alt="Version">
  </a>
  <a href="https://github.com/yeslogic/unicode-case-mapping/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-case-mapping.svg" alt="License">
  </a>
</div>

<br>

Fast mapping of `char` to lowercase, uppercase, or titlecase in Rust using
Unicode 12.1 data.

Usage
-----

`Cargo.toml`:

```toml
[dependencies]
unicode-case-mapping = "0.1.0"
```

`main.rs`:

```rust
use unicode_case_mapping::{get_case_mapping, CaseMapping};

fn main() {
    assert_eq!(get_case_mapping('A'), CaseMapping::NonJoining);
}
```

Motivation / When to Use
------------------------

The Rust standard library supplies [to_uppercase] and [to_lowercase] methods on
`char` so you might be wondering why this crate was created or when to use it.
You should almost certainly use the standard library, unless:

* You need support for titlecase conversion according to the Unicode character
  database (UCD).
* You need lower level access to the mapping table data, compared to the iterator
  interface supplied by the standard library.
* You _need_ faster performance that the standard library.

An additional motivation for creating this crate was to be able to version the
UCD data used independent of the Rust version. This allows us to ensure all
our Unicode related crates are all using the same UCD version.

Performance & Implementation Notes
----------------------------------

[ucd-generate] is used to generate `tables.rs`. A build script (`build.rs`)
compiles this into a three level look up table. The look up time is constant as
it is just indexing into the arrays.

The multi-level approach maps a code point to a block, then to a position
within a block, which is then the index of a record describing how to map that
codepoint to lower, upper, and title case. This allows the data to be
deduplicated, saving space, whilst also providing fast lookup. The code is
parameterised over the block size, which must be a power of 2. The value in the
build script is optimal for the data set.

This approach trades off some space for faster lookups. The tables take up
about 101KiB. Benchmarks (run with `cargo bench`) show this approach to be
~5–10× faster than the binary search approach used in the Rust standard
library.

It's possible there are further optimisations that could be made to eliminate
some runs of repeated values in the first level array.

[ucd-generate]: https://github.com/BurntSushi/ucd-generate
[to_uppercase]: https://doc.rust-lang.org/std/primitive.char.html#method.to_uppercase
[to_lowercase]: https://doc.rust-lang.org/std/primitive.char.html#method.to_lowercase
