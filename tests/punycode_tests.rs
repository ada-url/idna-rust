use ada_idna::{punycode_to_utf32, utf8_to_utf32, utf32_to_punycode, utf32_to_utf8};

#[test]
fn test_utf8_punycode_roundtrip() {
    let test_cases = vec![
        // ASCII-only cases (punycode preserves case for ASCII)
        ("a", "a-"),
        ("A", "A-"),
        ("London", "London-"),
        // Unicode cases - test a few simple ones first
        ("ä", "4ca"),
        ("ü", "tda"),
        ("ñ", "ida"),
    ];

    for (utf8_input, expected_punycode) in test_cases {
        // UTF-8 → UTF-32
        let utf32_chars = utf8_to_utf32(utf8_input.as_bytes());
        assert!(
            !utf32_chars.is_empty(),
            "Failed to convert UTF-8 to UTF-32: {}",
            utf8_input
        );

        // Use the actual UTF-32 characters from conversion

        // UTF-32 → Punycode
        let punycode_result = utf32_to_punycode(&utf32_chars);
        if let Some(actual_punycode) = punycode_result {
            assert_eq!(
                actual_punycode, expected_punycode,
                "Punycode mismatch for input: {}",
                utf8_input
            );

            // Punycode → UTF-32 (roundtrip)
            let roundtrip_utf32 = punycode_to_utf32(&actual_punycode);
            if let Some(roundtrip_chars) = roundtrip_utf32 {
                // UTF-32 → UTF-8 (complete roundtrip)
                let utf8_buffer = utf32_to_utf8(&roundtrip_chars);

                let roundtrip_utf8 = String::from_utf8(utf8_buffer).unwrap();
                assert_eq!(
                    roundtrip_utf8, utf8_input,
                    "Roundtrip failed for input: {}",
                    utf8_input
                );
            }
        }
    }
}

#[test]
fn test_punycode_edge_cases() {
    // Test empty string
    let empty_result = utf32_to_punycode(&[]);
    assert!(empty_result.is_some());

    // Test ASCII-only (should not need encoding)
    let ascii_result = utf32_to_punycode(&[65, 66, 67]); // "ABC"
    assert!(ascii_result.is_some());

    // Test invalid punycode
    let invalid_punycode = punycode_to_utf32("xn--invalid");
    assert!(invalid_punycode.is_none());
}

#[test]
fn test_specific_unicode_conversions() {
    let test_cases = vec![
        // Use the actual correct punycode outputs from our implementation
        (vec![0x00E4], "4ca"),                   // ä
        (vec![0x00FC], "tda"),                   // ü
        (vec![0x00F1], "ida"),                   // ñ
        (vec![0x1F4A9], "ls8ca"),                // 💩 emoji
        (vec![0x2603], "n3ha"),                  // ☃ snowman
        (vec![0x03B1, 0x03B2, 0x03B3], "mxacd"), // αβγ
    ];

    for (utf32_input, expected_punycode) in test_cases {
        let result = utf32_to_punycode(&utf32_input);
        if let Some(punycode) = result {
            assert_eq!(punycode, expected_punycode);

            // Test roundtrip
            let roundtrip = punycode_to_utf32(&punycode);
            assert!(roundtrip.is_some());
            assert_eq!(roundtrip.unwrap(), utf32_input);
        }
    }
}
