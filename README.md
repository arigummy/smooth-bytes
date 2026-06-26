# smooth-bytes

`smooth-bytes` is a tiny Rust crate that adds convenient bit operations to byte slices.

It is useful when you already have data as `[u8]`, `Vec<u8>`, or another byte buffer and want simple methods for checking, setting, clearing, listing, or masking bits without introducing a larger bit-vector abstraction.

The crate provides one extension trait:

```rust
use smooth_bytes::SmoothBytes;
```

After importing the trait, the methods are available on `[u8]`, arrays, and `Vec<u8>`.

## Installation

Install with Cargo:

```toml
[dependencies]
smooth-bytes = "0.1"
```

Install from Git:

```toml
[dependencies]
smooth-bytes = { git = "https://github.com/arigummy/smooth-bytes.git" }
```

## Bit order

Bit indexes are counted from the least significant bit of the last byte.

For example, in `[0x03, 0xe9]`:

```text
byte index:       0         1
bytes:         00000011  11101001
bit indexes:   15 ... 8  7 6 5 4 3 2 1 0
```

So `get_bit(0)` reads the lowest bit of `0xe9`, and `get_bit(8)` reads the lowest bit of `0x03`.

## Usage

```rust
use smooth_bytes::SmoothBytes;

let mut bytes = [0x03, 0xe9];

assert!(bytes.get_bit(8));
assert!(!bytes.get_bit(2));

bytes.set_bit(2);
assert!(bytes.get_bit(2));

bytes.reset_bit(3);
bytes.or_mask(0x0414u16);

let set_bits = bytes.get_set_bits();
```

## API

- `get_bit(bit)` returns `true` when the bit is set. Out-of-range indexes return `false`.
- `set_bit(bit)` sets a bit. Out-of-range indexes are ignored.
- `reset_bit(bit)` clears a bit. Out-of-range indexes are ignored.
- `get_set_bits()` returns all set bit indexes as `Vec<usize>`.
- `get_signs_bits()` is a compatibility alias that returns `Vec<u8>`.
- `or_mask(mask)` applies a bitwise OR mask.
- `and_mask(mask)` applies a bitwise AND mask.
- `is_zero()` returns `true` when all bytes are zero.

## Scope

This crate intentionally stays small. If you need a full bit-vector type, custom bit ordering, packed collections, or more advanced bit-slice APIs, consider using [`bitvec`](https://crates.io/crates/bitvec).
