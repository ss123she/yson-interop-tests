# YSON Interoperability Tests (Go <-> Rust)

Verifies binary and text YSON compatibility between Go and Rust implementations.

## Covered cases

- Integers (min/max)
- Floats (NaN, Inf)
- Strings (escape sequences)
- Bytes
- Option / nil
- Nested collections
- Attributes

## How to run
```
just test-all
```
Run from repository root.

## Requirements
[Rust (stable)](https://rust-lang.org/tools/install/)
[Go](https://go.dev/doc/install)
[just](https://crates.io/crates/just)
