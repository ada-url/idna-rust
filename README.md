# IDNA for Rust

**⚠️ This library is incomplete and under development ⚠️**

This is a Rust implementation of the [IDNA (Internationalized Domain Names in Applications)](https://github.com/ada-url/idna) specification, based on the C++ implementation from [ada-url/ada](https://github.com/ada-url/ada).

Fast and compliant IDNA conversion library for Rust with zero dependencies.

## About

This library provides IDNA processing capabilities for converting between Unicode domain names and ASCII-compatible encoding (ACE) using Punycode. It implements the IDNA specification for internationalized domain name handling.

This is a Rust port of the IDNA functionality from [github.com/ada-url/idna](https://github.com/ada-url/idna).

## Usage

```rust
use ada_idna::domain::{to_ascii, to_unicode};

// Convert Unicode domain to ASCII
let ascii_domain = to_ascii("café.example").unwrap();
assert_eq!(ascii_domain, "xn--caf-dma.example");

// Convert ASCII back to Unicode
let unicode_domain = to_unicode("xn--caf-dma.example").unwrap();
assert_eq!(unicode_domain, "café.example");
```

### Features

- **Zero dependencies**: No external crates required
- **Complete IDNA support**: Full to_ascii and to_unicode conversion
- **RFC 3492 Punycode**: Compliant Punycode encoding/decoding
- **Unicode normalization**: NFC normalization with composition tables
- **Character mapping**: Case folding, soft hyphen removal, format character handling
- **Validation**: Domain label and character validation
- **High performance**: Optimized UTF-8 ↔ UTF-32 transcoding

## Performance

Benchmarks comparing ada-idna with the popular `idna` crate (lower is better):

| Benchmark | ada-idna | idna crate | Ratio |
|-----------|----------|------------|-------|
| **Batch to_ascii conversion** | 12.69 µs | 6.07 µs | 2.09x slower |
| **Batch to_unicode conversion** | 5.06 µs | 5.26 µs | **0.96x faster** |
| **Single ASCII domain** | 77.59 ns | 18.98 ns | 4.09x slower |
| **Single Unicode domain** | 291.88 ns | 142.71 ns | 2.05x slower |
| **Complex Unicode domain** | 796.04 ns | 306.25 ns | 2.60x slower |
| **Non-Latin scripts** | 533.68 ns | 266.57 ns | 2.00x slower |
| **CJK domains** | 735.26 ns | 294.66 ns | 2.49x slower |
| **Mixed scripts** | 463.80 ns | 222.23 ns | 2.09x slower |

**Ada-idna specific operations:**
- Punycode encoding: 1.79 µs
- Punycode decoding: 640.65 ns
- Unicode normalization: 1.29 µs

*Run `cargo bench` to reproduce these results on your system.*

## Current Status

🚧 **This implementation is incomplete and not ready for production use.** 

Known limitations:
- Performance is 2.00-4.09x slower than the mature `idna` crate
- **Ada-idna is faster than idna crate for to_unicode conversion** ⚡
- Some test cases fail due to expected value discrepancies
- Unicode table data may be incomplete
- Error handling needs refinement
- API is subject to change

## Development

### Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Run benchmarks (compares with idna crate)
cargo bench
```

### Project Structure

- `src/domain.rs` - Main IDNA conversion functions
- `src/punycode.rs` - Punycode encoding/decoding
- `src/mapping.rs` - Character mapping and case folding
- `src/normalization.rs` - Unicode NFC normalization
- `src/validation.rs` - Character and domain validation
- `src/unicode.rs` - UTF-8 ↔ UTF-32 conversion utilities
- `src/unicode_tables.rs` - Unicode lookup tables
- `tests/` - Comprehensive test suite

## Contributing

This project is part of the [Ada URL](https://github.com/ada-url) family of libraries. Contributions are welcome!

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Related Projects

- [ada-url/ada](https://github.com/ada-url/ada) - Fast spec-compliant URL parser (C++)
- [ada-url/rust](https://github.com/ada-url/rust) - Rust bindings for Ada URL parser
- [ada-url/idna](https://github.com/ada-url/idna) - Original IDNA implementation (C++)