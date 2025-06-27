# Claude Development Notes

## Project Overview

This is a Rust implementation of IDNA (Internationalized Domain Names in Applications) based on the C++ implementation from [ada-url/ada](https://github.com/ada-url/ada). The project aims to provide a zero-dependency, high-performance IDNA library for Rust.

## Implementation Details

### Core Components

- **`src/domain.rs`** - Main IDNA conversion functions (`to_ascii`, `to_unicode`)
- **`src/punycode.rs`** - RFC 3492 Punycode encoding/decoding (1:1 match with C++ implementation)
- **`src/mapping.rs`** - Character mapping and case folding
- **`src/normalization.rs`** - Unicode NFC normalization with composition tables
- **`src/validation.rs`** - Character and domain validation
- **`src/unicode.rs`** - UTF-8 ↔ UTF-32 conversion utilities
- **`src/unicode_tables.rs`** - Unicode lookup tables extracted from C++ implementation

### Key Implementation Notes

1. **Zero Dependencies**: No external crates are used. All Unicode processing is implemented manually.

2. **Punycode Implementation**: Exact 1:1 match with C++ [ada_idna.cpp](https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/src/ada_idna.cpp) implementation:
   - Uses same constants (BASE=36, TMIN=1, TMAX=26, SKEW=38, DAMP=700, INITIAL_BIAS=72, INITIAL_N=128)
   - Identical algorithm flow and bias adaptation

3. **Unicode Tables**: Extracted from C++ implementation ([ada_idna.cpp](https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/src/ada_idna.cpp)) with proper dimensions:
   - `DECOMPOSITION_BLOCK`: 67×257 elements
   - `CANONICAL_COMBINING_CLASS_BLOCK`: 67×257 elements  
   - `COMPOSITION_BLOCK`: 67×257 elements (17,219 total)

4. **Unicode Normalization**: Complete NFC implementation matching C++ behavior:
   - Canonical decomposition
   - Canonical combining class ordering
   - Canonical composition using two-level lookup tables

## Test Coverage

Comprehensive test suite covering:
- Basic IDNA conversion (`to_ascii_tests.rs`, `to_unicode_tests.rs`)
- Unicode identifier validation (`identifier_tests.rs`)
- Punycode encoding/decoding (`punycode_tests.rs`)
- Mapping and normalization (`mapping_tests.rs`, `normalization_tests.rs`)
- Web Platform Tests compatibility (`wpt_tests.rs`)

## Development Commands

**⚠️ IMPORTANT: Always run tests, formatter, and clippy before committing changes ⚠️**

```bash
# Build
cargo build

# Run tests (ALWAYS run before committing)
cargo test

# Lint (ALWAYS run before committing)
cargo clippy

# Format (ALWAYS run before committing)
cargo fmt

# Run benchmarks (compares with idna crate)
cargo bench
```

### Pre-commit Checklist
1. `cargo test` - All tests must pass
2. `cargo clippy` - No clippy warnings allowed
3. `cargo fmt` - Code must be properly formatted

## Benchmarks

Comprehensive benchmarks comparing ada-idna performance with the popular "idna" crate:

- **to_ascii conversion**: Batch Unicode → ASCII conversion performance
- **to_unicode conversion**: Batch ASCII → Unicode conversion performance  
- **Punycode encoding/decoding**: Individual punycode operations
- **Unicode normalization**: NFC normalization performance
- **Single domain performance**: Per-domain conversion overhead


Run `cargo bench` to execute all benchmarks and compare performance.

## Current Status

**⚠️ INCOMPLETE IMPLEMENTATION ⚠️**

Known limitations:
- Some test cases may fail due to expected value discrepancies
- Unicode table data may need refinement
- Error handling needs improvement
- API subject to change

## Source References

- Original C++ header: https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/include/ada/ada_idna.h
- Original C++ implementation: https://raw.githubusercontent.com/ada-url/ada/refs/heads/main/src/ada_idna.cpp
- Test cases adapted from: https://github.com/ada-url/idna/tree/main/tests

## Architecture Decisions

1. **Static vs Const Arrays**: Large Unicode tables use `static` instead of `const` to avoid stack overflow during compilation.

2. **UTF-32 Processing**: All Unicode processing is done in UTF-32 code points for simplicity and correctness.

3. **Error Handling**: Custom `IdnaError` enum for specific IDNA-related errors.

4. **Performance**: Optimized for common ASCII cases while maintaining full Unicode support.

## Build Configuration

- **Target**: Rust 2024 edition
- **Dependencies**: None (zero-dependency implementation)
- **Features**: No optional features
- **Minimum Rust Version**: 1.85+ (for Rust 2024 edition)