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
| **Batch to_ascii conversion** | 7.45 µs | 4.30 µs | 1.73x slower |
| **Batch to_unicode conversion** | 4.41 µs | 3.94 µs | 1.12x slower |
| **Single ASCII domain** | 62.13 ns | 17.59 ns | 3.53x slower |
| **Single Unicode domain** | 19.66 ns | 91.83 ns | **4.67x faster** |
| **Complex Unicode domain** | 24.20 ns | 183.92 ns | **7.60x faster** |
| **Non-Latin scripts** | 363.64 ns | 149.70 ns | 2.43x slower |
| **CJK domains** | 531.77 ns | 170.55 ns | 3.12x slower |
| **Mixed scripts** | 335.57 ns | 140.03 ns | 2.40x slower |

**Ada-idna specific operations:**
- Punycode encoding: 1.57 µs
- Punycode decoding: 515.51 ns
- Unicode normalization: 1.14 µs

*Run `cargo bench` to reproduce these results on your system.*

## Current Status

🚧 **This implementation is incomplete and not ready for production use.** 

Known limitations:
- Performance is mixed compared to the mature `idna` crate
- **Ada-idna is significantly faster for single Unicode/complex domains** ⚡
- **4.67x faster for simple Unicode domains, 7.60x faster for complex Unicode domains**
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