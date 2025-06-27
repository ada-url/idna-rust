use ada_idna::domain::{IdnaError, to_ascii};

#[test]
fn test_to_ascii_basic() {
    let test_cases = vec![
        ("example.com", "example.com"),
        ("straße.de", "xn--strae-oqa.de"),
        ("café.example", "xn--caf-dma.example"),
        ("münchen.de", "xn--mnchen-3ya.de"),
        ("правда.com", "xn--80aafi6cg.com"),
        ("ドメイン名例.jp", "xn--eckwd4c7c.xn--wgv71a119e.jp"),
        ("test.중국", "test.xn--fiqz9s"),
        ("bücher.example", "xn--bcher-kva.example"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "Failed to convert '{}': {:?}",
            input,
            result
        );
        assert_eq!(result.unwrap(), expected, "Mismatch for input: '{}'", input);
    }
}

#[test]
fn test_to_ascii_special_characters() {
    // Test German capital sharp S (ẞ)
    let result = to_ascii("faẞ.de");
    assert_eq!(result.unwrap(), "xn--fa-hia.de");

    // Test soft hyphen removal
    let result = to_ascii("ex\u{AD}ample.com"); // Contains U+00AD soft hyphen
    assert_eq!(result.unwrap(), "example.com");

    // Test replacement character
    let result = to_ascii("ex�ample.com"); // Contains U+FFFD replacement character
    // This should likely fail or be handled specially
    assert!(
        result.is_err(),
        "Replacement character should cause an error"
    );
}

#[test]
fn test_to_ascii_invalid_inputs() {
    let long_label = "a".repeat(64);
    let invalid_cases = vec![
        "",             // Empty string
        ".",            // Just a dot
        "...",          // Multiple dots
        "example..com", // Double dot
        &long_label,    // Label too long
    ];

    for input in invalid_cases {
        let result = to_ascii(input);
        assert!(result.is_err(), "Expected error for input: '{}'", input);
    }
}

#[test]
fn test_to_ascii_comma_handling() {
    // Test comma in domain name
    let result = to_ascii("test,example.com");
    // Comma should likely cause an error or be handled specially
    if result.is_err() {
        assert!(matches!(result.unwrap_err(), IdnaError::InvalidCharacter));
    }
}

#[test]
fn test_to_ascii_edge_cases() {
    let test_cases = vec![
        // Pure ASCII should pass through unchanged
        ("simple.com", "simple.com"),
        ("test123.org", "test123.org"),
        ("sub.domain.example", "sub.domain.example"),
        // Mixed ASCII and non-ASCII
        ("café.simple.com", "xn--caf-dma.simple.com"),
        ("simple.café.com", "simple.xn--caf-dma.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "Failed to convert '{}': {:?}",
            input,
            result
        );
        assert_eq!(result.unwrap(), expected, "Mismatch for input: '{}'", input);
    }
}

#[test]
fn test_to_ascii_unicode_scripts() {
    let test_cases = vec![
        // Arabic
        ("نامه‌ای.com", "xn--mgba3gch31f060k.com"),
        // Chinese
        ("团淄.com", "xn--3bs854c.com"),
        // Japanese Hiragana
        ("みんな.jp", "xn--u9j001j.jp"),
        // Korean
        ("한국.com", "xn--3e0b707e.com"),
        // Thai
        ("ยจฆฟคฏข.com", "xn--22cdfh1b8fsa.com"),
        // Greek
        ("αβγ.com", "xn--mxacd.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "Failed to convert Unicode script '{}'",
            input
        );
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Unicode script mismatch for '{}'", input);
    }
}

#[test]
fn test_to_ascii_label_length_limits() {
    // Test maximum label length (63 characters)
    let long_ascii = "a".repeat(63);
    let result = to_ascii(&long_ascii);
    assert!(result.is_ok(), "63-character ASCII label should be valid");

    // Test label that's too long
    let too_long = "a".repeat(64);
    let result = to_ascii(&too_long);
    assert!(result.is_err(), "64-character label should be invalid");
    assert!(matches!(result.unwrap_err(), IdnaError::LabelTooLong));
}

#[test]
fn test_to_ascii_punycode_expansion() {
    // Test cases where Unicode characters expand to longer Punycode
    let test_cases = vec![
        ("ü.com", "xn--tda.com"),
        ("ñ.org", "xn--ida.org"),
        ("ø.net", "xn--8ca.net"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "Failed to convert punycode expansion '{}'",
            input
        );
        let actual = result.unwrap();
        assert_eq!(
            actual, expected,
            "Punycode expansion mismatch for '{}'",
            input
        );

        // Verify it starts with xn-- for Unicode labels
        if !input.is_ascii() {
            assert!(
                actual.contains("xn--"),
                "Unicode input should produce punycode output"
            );
        }
    }
}
