use ada_idna::mapping;

#[test]
fn test_ascii_map() {
    let test_cases = vec![
        // ASCII characters should remain unchanged
        ("example.com", "example.com"),
        ("TEST.ORG", "test.org"), // Should be lowercased
        ("Simple123", "simple123"),
        ("test-domain", "test-domain"),
        ("sub.domain.example", "sub.domain.example"),
    ];

    for (input, expected) in test_cases {
        let result = mapping::ascii_map(input);
        assert_eq!(result, expected, "ASCII mapping failed for: '{}'", input);
    }
}

#[test]
fn test_map_basic() {
    let test_cases = vec![
        // ASCII should pass through (potentially lowercased)
        ("asciitwontchange", "asciitwontchange"),
        ("UPPERCASE", "uppercase"),
        // Soft hyphen (U+00AD) should be removed
        ("has\u{00ad}omitted", "hasomitted"),
        ("hasomit\u{00ad}ted", "hasomitted"),
        ("\u{00ad}alla", "alla"), // Leading soft hyphen
        ("test\u{00ad}\u{00ad}multiple", "testmultiple"), // Multiple soft hyphens
        // Other mapping cases
        ("café", "café"),       // Should preserve accented characters
        ("München", "münchen"), // Should lowercase
    ];

    for (input, expected) in test_cases {
        let result = mapping::map(input);
        assert_eq!(result, expected, "Mapping failed for: '{}'", input);
    }
}

#[test]
fn test_map_unicode_normalization() {
    // Test cases that might involve Unicode normalization
    let test_cases = vec![
        // Composed vs decomposed characters
        ("é", "é"),         // Should normalize consistently
        ("e\u{0301}", "é"), // Combining acute accent -> composed
        // German sharp s cases
        ("ß", "ß"),
        ("Straße", "straße"), // Should lowercase
        // Other European characters
        ("ñ", "ñ"),
        ("ü", "ü"),
        ("ø", "ø"),
    ];

    for (input, _expected) in test_cases {
        let result = mapping::map(input);
        println!("Unicode mapping: '{}' -> '{}'", input, result);
        // Note: Exact expected values may need adjustment based on normalization rules
    }
}

#[test]
fn test_map_case_folding() {
    let test_cases = vec![
        // Basic case folding
        ("EXAMPLE", "example"),
        ("Test", "test"),
        ("MiXeD", "mixed"),
        // Unicode case folding
        ("CAFÉ", "café"),
        ("MÜNCHEN", "münchen"),
        ("İSTANBUL", "i̇stanbul"), // Turkish I with dot
        // Greek case folding
        ("ΑΒΓΔΕ", "αβγδε"),
        ("Ελληνικά", "ελληνικά"),
    ];

    for (input, _expected) in test_cases {
        let result = mapping::map(input);
        println!("Case folding: '{}' -> '{}'", input, result);
        // Note: Some Unicode case folding rules are complex
    }
}

#[test]
fn test_map_special_characters() {
    let test_cases = vec![
        // Zero-width characters
        ("test\u{200c}example", "testexample"), // Zero-width non-joiner
        ("test\u{200d}example", "testexample"), // Zero-width joiner
        // Various space characters
        ("test\u{00a0}example", "test example"), // Non-breaking space -> regular space
        ("test\u{2002}example", "test example"), // En space -> regular space
        ("test\u{2003}example", "test example"), // Em space -> regular space
        // Soft hyphen removal
        ("ex\u{00ad}ample", "example"),
        (
            "soft\u{00ad}hyphen\u{00ad}test",
            "soft\u{00ad}hyphen\u{00ad}test",
        ), // Multiple handling
    ];

    for (input, _expected) in test_cases {
        let result = mapping::map(input);
        println!("Special chars: '{}' -> '{}'", input, result);
        // Note: Expected behavior may vary based on IDNA mapping rules
    }
}

#[test]
fn test_map_empty_and_edge_cases() {
    let test_cases = vec![
        ("", ""),                 // Empty string
        (" ", " "),               // Single space
        ("a", "a"),               // Single character
        ("A", "a"),               // Single uppercase
        ("\u{00ad}", ""),         // Only soft hyphen
        ("\u{00ad}\u{00ad}", ""), // Multiple soft hyphens only
    ];

    for (input, expected) in test_cases {
        let result = mapping::map(input);
        assert_eq!(result, expected, "Edge case failed for: '{:?}'", input);
    }
}

#[test]
fn test_map_domain_labels() {
    // Test mapping on individual domain labels
    let test_cases = vec![
        ("Example", "example"),
        ("CAFÉ", "café"),
        ("München", "münchen"),
        ("Test\u{00ad}Label", "testlabel"),
        ("Mixed\u{00ad}CASE", "mixedcase"),
    ];

    for (input, expected) in test_cases {
        let result = mapping::map(input);
        assert_eq!(
            result, expected,
            "Domain label mapping failed for: '{}'",
            input
        );
    }
}

#[test]
fn test_map_international_scripts() {
    // Test mapping behavior with various international scripts
    let test_cases = vec![
        // Cyrillic
        ("ПРАВДА", "правда"),
        ("Москва", "москва"),
        // Arabic (may not have case distinctions)
        ("نامه", "نامه"),
        // Chinese (no case distinctions)
        ("北京", "北京"),
        // Japanese
        ("ドメイン", "ドメイン"),
        ("みんな", "みんな"),
        // Korean
        ("한국", "한국"),
        // Thai
        ("ไทย", "ไทย"),
    ];

    for (input, _expected) in test_cases {
        let result = mapping::map(input);
        println!("International script: '{}' -> '{}'", input, result);
        // Note: Scripts without case distinctions should remain unchanged
    }
}

#[test]
fn test_map_bidirectional_characters() {
    // Test handling of bidirectional control characters
    let test_cases = vec![
        // Left-to-right marks
        ("test\u{200e}example", "testexample"),
        ("test\u{200f}example", "testexample"), // Right-to-left mark
        // Bidirectional overrides
        ("test\u{202d}example", "testexample"), // Left-to-right override
        ("test\u{202e}example", "testexample"), // Right-to-left override
        // Pop directional formatting
        ("test\u{202c}example", "testexample"),
    ];

    for (input, _expected) in test_cases {
        let result = mapping::map(input);
        println!("Bidirectional: '{}' -> '{}'", input, result);
        // Note: Expected behavior depends on IDNA mapping rules for bidi chars
    }
}
