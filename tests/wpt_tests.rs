use ada_idna::domain::{contains_forbidden_domain_code_point, to_ascii, to_unicode};

#[test]
fn test_wpt_basic_ascii_domains() {
    // Basic Web Platform Test cases for ASCII domains
    let test_cases = vec![
        ("example.com", "example.com"),
        ("test.org", "test.org"),
        ("simple.net", "simple.net"),
        ("sub.domain.example", "sub.domain.example"),
        ("123.456.789", "123.456.789"),
        ("a.b.c.d.e", "a.b.c.d.e"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "WPT: Failed to convert ASCII domain '{}'",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "WPT: ASCII domain mismatch for '{}'",
            input
        );
    }
}

#[test]
fn test_wpt_unicode_to_ascii() {
    // Web Platform Test cases for Unicode to ASCII conversion
    let test_cases = vec![
        // German
        ("straße.de", "xn--strae-oqa.de"),
        ("faß.de", "xn--fa-hia.de"),
        ("Faß.de", "xn--fa-hia.de"), // Should be case-folded
        // French
        ("café.fr", "xn--caf-dma.fr"),
        ("naïve.example", "xn--nave-6pa.example"),
        // Spanish
        ("niño.es", "xn--nio-5qa.es"),
        ("español.com", "xn--espaol-zwa.com"),
        // Mixed scripts
        ("test-café.com", "test-xn--caf-dma.com"),
        ("münchen-test.de", "xn--mnchen-test-jjb.de"),
    ];

    for (input, expected) in test_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "WPT: Failed to convert Unicode->ASCII for '{}'",
            input
        );
        let actual = result.unwrap();
        assert_eq!(
            actual, expected,
            "WPT: Unicode->ASCII mismatch for '{}'",
            input
        );
    }
}

#[test]
fn test_wpt_ascii_to_unicode() {
    // Web Platform Test cases for ASCII to Unicode conversion
    let test_cases = vec![
        // Simple ASCII (should pass through)
        ("example.com", "example.com"),
        ("test.org", "test.org"),
        // Punycode to Unicode
        ("xn--strae-oqa.de", "straße.de"),
        ("xn--fa-hia.de", "faß.de"),
        ("xn--caf-dma.fr", "café.fr"),
        ("xn--nio-5qa.es", "niño.es"),
        ("xn--espaol-zwa.com", "español.com"),
        // Mixed domains
        ("test.xn--caf-dma.example", "test.café.example"),
        ("xn--caf-dma.test.com", "café.test.com"),
    ];

    for (input, expected) in test_cases {
        let result = to_unicode(input);
        assert!(
            result.is_ok(),
            "WPT: Failed to convert ASCII->Unicode for '{}'",
            input
        );
        let actual = result.unwrap();
        assert_eq!(
            actual, expected,
            "WPT: ASCII->Unicode mismatch for '{}'",
            input
        );
    }
}

#[test]
fn test_wpt_forbidden_characters() {
    // Test detection of forbidden domain code points
    let forbidden_cases = vec![
        // Control characters
        "test\u{0000}example.com", // NULL
        "test\u{0001}example.com", // SOH
        "test\u{007F}example.com", // DEL
        // Format characters
        "test\u{200C}example.com", // Zero-width non-joiner
        "test\u{200D}example.com", // Zero-width joiner
        // Bidirectional characters
        "test\u{202A}example.com", // Left-to-right embedding
        "test\u{202B}example.com", // Right-to-left embedding
        "test\u{202C}example.com", // Pop directional formatting
        "test\u{202D}example.com", // Left-to-right override
        "test\u{202E}example.com", // Right-to-left override
        // Other forbidden characters
        "test\u{FEFF}example.com", // Byte order mark
        "test\u{FFF9}example.com", // Interlinear annotation anchor
        "test\u{FFFA}example.com", // Interlinear annotation separator
        "test\u{FFFB}example.com", // Interlinear annotation terminator
    ];

    for input in forbidden_cases {
        let has_forbidden = contains_forbidden_domain_code_point(input);
        assert!(
            has_forbidden,
            "WPT: Should detect forbidden character in '{:?}'",
            input
        );

        // These should also fail to_ascii conversion
        let ascii_result = to_ascii(input);
        assert!(
            ascii_result.is_err(),
            "WPT: Should reject forbidden character in to_ascii: '{:?}'",
            input
        );
    }
}

#[test]
fn test_wpt_edge_cases() {
    // Web Platform Test edge cases
    let max_label = "a".repeat(63);
    let too_long_label = "a".repeat(64);
    let edge_cases = vec![
        // Empty labels
        ("", false),
        (".", false),
        ("...", false),
        ("example..com", false),
        // Label length limits
        (&max_label, true),       // Maximum valid label length
        (&too_long_label, false), // Too long
        // Special characters
        ("example-.com", false), // Hyphen at end
        ("-example.com", false), // Hyphen at start
        ("ex--ample.com", true), // Double hyphen in middle (may be valid)
        // Case sensitivity
        ("EXAMPLE.COM", true),
        ("Example.Com", true),
        ("eXaMpLe.CoM", true),
    ];

    for (input, should_succeed) in edge_cases {
        if input.is_empty() {
            continue; // Skip empty string test
        }

        let result = to_ascii(input);
        if should_succeed {
            assert!(result.is_ok(), "WPT: Expected success for '{}'", input);
        } else {
            assert!(result.is_err(), "WPT: Expected failure for '{}'", input);
        }
    }
}

#[test]
fn test_wpt_international_domains() {
    // International domain name test cases
    let international_cases = vec![
        // Cyrillic
        ("пример.рф", "xn--e1afmkfd.xn--p1ai"),
        ("тест.com", "xn--e1aybc.com"),
        // Arabic
        ("مثال.شبكة", "xn--mgbh0fb.xn--ngbc5azd"),
        ("اختبار.com", "xn--kgbechtv.com"),
        // Chinese
        ("例子.中国", "xn--fsq.xn--fiqs8s"),
        ("测试.com", "xn--0zwm56d.com"),
        // Japanese
        ("例え.日本", "xn--r8jz45g.xn--wgv71a119e"),
        ("テスト.com", "xn--zckzah.com"),
        // Korean
        ("예시.한국", "xn--9n2bp8q.xn--3e0b707e"),
        ("테스트.com", "xn--o39a.com"),
        // Thai
        ("ตัวอย่าง.ไทย", "xn--12c1fe0br.xn--o3cw4h"),
        ("ทดสอบ.com", "xn--12co0c3b4eva.com"),
    ];

    for (unicode_input, expected_ascii) in international_cases {
        let ascii_result = to_ascii(unicode_input);
        assert!(
            ascii_result.is_ok(),
            "WPT: Failed to convert international domain '{}'",
            unicode_input
        );
        let actual_ascii = ascii_result.unwrap();
        assert_eq!(
            actual_ascii, expected_ascii,
            "WPT: International domain mismatch for '{}'",
            unicode_input
        );

        // Test roundtrip
        let unicode_result = to_unicode(&actual_ascii);
        assert!(
            unicode_result.is_ok(),
            "WPT: Failed to roundtrip convert '{}'",
            actual_ascii
        );
        let roundtrip_unicode = unicode_result.unwrap();
        assert_eq!(
            roundtrip_unicode, unicode_input,
            "WPT: Roundtrip mismatch for '{}'",
            unicode_input
        );
    }
}

#[test]
fn test_wpt_normalization_cases() {
    // Test cases involving Unicode normalization
    let normalization_cases = vec![
        // Composed vs decomposed forms
        ("café.com", "xn--caf-dma.com"),         // Composed é
        ("cafe\u{0301}.com", "xn--caf-dma.com"), // Decomposed e + combining acute
        // German sharp s
        ("weiß.de", "xn--wei-xka.de"),
        ("WEISS.de", "weiss.de"), // Capital sharp s handling
        // Ligatures
        ("ﬁle.com", "file.com"), // fi ligature
        ("ﬀ.com", "ff.com"),     // ff ligature
        // Compatibility characters
        ("²test.com", "2test.com"), // Superscript 2
        ("test³.com", "test3.com"), // Superscript 3
    ];

    for (input, expected) in normalization_cases {
        let result = to_ascii(input);
        assert!(
            result.is_ok(),
            "WPT: Failed to convert normalization case '{}'",
            input
        );
        let actual = result.unwrap();
        assert_eq!(
            actual, expected,
            "WPT: Normalization mismatch for '{}'",
            input
        );
    }
}

#[test]
fn test_wpt_mixed_script_validation() {
    // Test mixed script validation (should generally be allowed)
    let mixed_script_cases = vec![
        ("test-例子.com", true),    // Latin + Chinese
        ("café-тест.org", true),    // Latin + Cyrillic
        ("example-مثال.net", true), // Latin + Arabic
        ("test-テスト.jp", true),   // Latin + Japanese
        ("demo-한국.kr", true),     // Latin + Korean
        ("sample-ไทย.th", true),    // Latin + Thai
    ];

    for (input, should_succeed) in mixed_script_cases {
        let result = to_ascii(input);
        if should_succeed {
            assert!(
                result.is_ok(),
                "WPT: Mixed script should succeed for '{}'",
                input
            );
        } else {
            assert!(
                result.is_err(),
                "WPT: Mixed script should fail for '{}'",
                input
            );
        }
    }
}
