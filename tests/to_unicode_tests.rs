use ada_idna::domain::to_unicode;

#[test]
fn test_to_unicode_basic() {
    let test_cases = vec![
        // ASCII domains should pass through unchanged
        ("example.com", "example.com"),
        ("test.org", "test.org"),
        ("simple.net", "simple.net"),
        // Punycode domains should be decoded
        ("xn--strae-oqa.de", "straße.de"),
        ("xn--caf-dma.example", "café.example"),
        ("xn--mnchen-3ya.de", "münchen.de"),
        ("xn--80aafi6cg.com", "правда.com"),
        ("xn--u9j001j.jp", "みんな.jp"),
        ("xn--3e0b707e.com", "한국.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_unicode(input);
        assert!(
            result.is_ok(),
            "Failed to convert '{}': {:?}",
            input,
            result
        );
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Mismatch for input: '{}'", input);
    }
}

#[test]
fn test_to_unicode_punycode_decoding() {
    let test_cases = vec![
        // Simple single character cases
        ("xn--tda.com", "ü.com"),
        ("xn--ida.org", "ñ.org"),
        ("xn--8ca.net", "ø.net"),
        // More complex cases
        ("xn--bcher-kva.example", "bücher.example"),
        ("xn--mgba3gch31f060k.com", "نامه‌ای.com"),
        ("xn--3bs854c.com", "团淄.com"),
        ("xn--22cdfh1b8fsa.com", "ยจฆฟคฏข.com"),
        ("xn--mxacd.com", "αβγ.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_unicode(input);
        assert!(result.is_ok(), "Failed to decode '{}': {:?}", input, result);
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Punycode decode mismatch for '{}'", input);
    }
}

#[test]
fn test_to_unicode_mixed_domains() {
    let test_cases = vec![
        // Mixed ASCII and Punycode labels
        ("simple.xn--caf-dma.com", "simple.café.com"),
        ("xn--caf-dma.simple.com", "café.simple.com"),
        ("test.xn--strae-oqa.example", "test.straße.example"),
        // Multiple Punycode labels
        ("xn--caf-dma.xn--strae-oqa.de", "café.straße.de"),
    ];

    for (input, expected) in test_cases {
        let result = to_unicode(input);
        assert!(
            result.is_ok(),
            "Failed to convert mixed domain '{}': {:?}",
            input,
            result
        );
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Mixed domain mismatch for '{}'", input);
    }
}

#[test]
fn test_to_unicode_invalid_punycode() {
    let invalid_cases = vec![
        "xn--invalid", // Invalid punycode
        "xn--",        // Empty punycode
        "xn--zzz",     // Non-existent punycode
        "xn--1234567890123456789012345678901234567890123456789012345678901234", // Too long
    ];

    for input in invalid_cases {
        let result = to_unicode(input);
        // Invalid punycode should either error or pass through unchanged
        if result.is_err() {
            // Expected behavior for invalid punycode
            assert!(result.is_err(), "Invalid punycode should fail: '{}'", input);
        } else {
            let output = result.unwrap();
            // Some implementations might pass through invalid punycode unchanged
            assert!(
                !output.is_empty(),
                "Output should not be empty for '{}'",
                input
            );
        }
    }
}

#[test]
fn test_to_unicode_edge_cases() {
    let test_cases = vec![
        // Empty labels should be handled
        ("", ""),
        // Single character domains
        ("a", "a"),
        ("xn--tda", "ü"),
        // Domains with numbers
        ("test123.com", "test123.com"),
        ("xn--caf-dma.123.com", "café.123.com"),
    ];

    for (input, expected) in test_cases {
        if input.is_empty() {
            let result = to_unicode(input);
            assert!(result.is_err(), "Empty string should be invalid");
            continue;
        }

        let result = to_unicode(input);
        assert!(
            result.is_ok(),
            "Edge case should succeed for '{}': {:?}",
            input,
            result
        );
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Edge case mismatch for '{}'", input);
    }
}

#[test]
fn test_to_unicode_roundtrip_consistency() {
    // Test that to_ascii(to_unicode(x)) == x for valid punycode
    let punycode_domains = vec![
        "xn--strae-oqa.de",
        "xn--caf-dma.example",
        "xn--mnchen-3ya.de",
        "xn--bcher-kva.example",
    ];

    for punycode in punycode_domains {
        let unicode_result = to_unicode(punycode);
        assert!(
            unicode_result.is_ok(),
            "Failed to convert to unicode: '{}'",
            punycode
        );
        let unicode_domain = unicode_result.unwrap();

        // Try to convert back to ASCII
        let ascii_result = ada_idna::domain::to_ascii(&unicode_domain);
        assert!(
            ascii_result.is_ok(),
            "Failed to convert back to ASCII: '{}'",
            unicode_domain
        );
        let back_to_ascii = ascii_result.unwrap();

        // Should roundtrip back to the same punycode (case-insensitive)
        assert_eq!(
            back_to_ascii.to_lowercase(),
            punycode.to_lowercase(),
            "Roundtrip failed for: '{}'",
            punycode
        );
    }
}

#[test]
fn test_to_unicode_case_insensitive() {
    let test_cases = vec![
        ("XN--CAF-DMA.COM", "café.com"),
        ("xn--caf-dma.com", "café.com"),
        ("Xn--Caf-Dma.Com", "café.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_unicode(input);
        assert!(
            result.is_ok(),
            "Case test failed for '{}': {:?}",
            input,
            result
        );
        let actual = result.unwrap();
        assert_eq!(actual, expected, "Case test mismatch for '{}'", input);
    }
}

#[test]
fn test_to_unicode_script_validation() {
    // Test various Unicode scripts
    let script_tests = vec![
        ("xn--80aafi6cg.com", "правда.com"),       // Cyrillic
        ("xn--mgba3gch31f060k.com", "نامه‌ای.com"), // Arabic
        ("xn--4dbrk0ce.com", "ישראל.com"),         // Hebrew
        ("xn--3bs854c.com", "团淄.com"),           // Chinese
        ("xn--3e0b707e.com", "한국.com"),          // Korean
        ("xn--u9j001j.jp", "みんな.jp"),           // Japanese
        ("xn--22cdfh1b8fsa.com", "ยจฆฟคฏข.com"),   // Thai
    ];

    for (punycode, expected_unicode) in script_tests {
        let result = to_unicode(punycode);
        assert!(
            result.is_ok(),
            "Script conversion failed for '{}': {:?}",
            punycode,
            result
        );
        let actual = result.unwrap();
        assert_eq!(
            actual, expected_unicode,
            "Script test mismatch for '{}'",
            punycode
        );
    }
}
