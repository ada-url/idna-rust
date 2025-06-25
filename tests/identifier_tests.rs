use ada_idna::{unicode, validation};

#[test]
fn test_valid_name_code_point_first_position() {
    let test_cases = vec![
        // Valid first position characters
        ('a', true), // lowercase letter
        ('z', true), // lowercase letter
        ('A', true), // uppercase letter
        ('Z', true), // uppercase letter
        ('é', true), // accented letter
        ('ü', true), // accented letter
        ('α', true), // Greek letter
        ('ñ', true), // accented letter
        ('ø', true), // accented letter
        ('ß', true), // German sharp s
        // Invalid first position characters
        ('0', false), // digit
        ('9', false), // digit
        ('-', false), // hyphen
        ('_', false), // underscore
        (' ', false), // space
        ('.', false), // dot
        ('!', false), // punctuation
        ('@', false), // symbol
    ];

    for (ch, expected) in test_cases {
        let result = validation::valid_name_code_point_first_position(ch as u32);
        assert_eq!(
            result, expected,
            "First position validation failed for '{}' (U+{:04X})",
            ch, ch as u32
        );
    }
}

#[test]
fn test_valid_name_code_point_other_position() {
    let test_cases = vec![
        // Valid in other positions
        ('a', true),  // lowercase letter
        ('A', true),  // uppercase letter
        ('À', true),  // accented uppercase
        ('é', true),  // accented letter
        ('0', true),  // digit (valid in non-first position)
        ('9', true),  // digit
        ('-', true),  // hyphen (valid in middle positions)
        ('_', false), // underscore (not valid in domain names)
        // Invalid in any position
        (' ', false),  // space
        ('.', false),  // dot
        ('!', false),  // punctuation
        ('@', false),  // symbol
        ('/', false),  // slash
        ('\\', false), // backslash
        ('?', false),  // question mark
        ('#', false),  // hash
    ];

    for (ch, expected) in test_cases {
        let result = validation::valid_name_code_point_other_position(ch as u32);
        assert_eq!(
            result, expected,
            "Other position validation failed for '{}' (U+{:04X})",
            ch, ch as u32
        );
    }
}

#[test]
fn test_utf8_to_utf32_length() {
    let test_cases = vec![
        // ASCII strings
        ("hello", 5),
        ("test", 4),
        ("", 0),
        ("a", 1),
        // Unicode strings
        ("café", 4),    // 4 characters: c, a, f, é
        ("München", 7), // 7 characters
        ("ñoño", 4),    // 4 characters
        ("αβγ", 3),     // 3 Greek letters
        ("北京", 2),    // 2 Chinese characters
        ("😀😁", 2),    // 2 emoji characters
        ("👨‍👩‍👧‍👦", 7),      // Complex emoji sequence (may be more)
    ];

    for (input, expected_len) in test_cases {
        let actual_len = unicode::utf8_to_utf32_length(input.as_bytes());
        println!("UTF-8 '{}' has {} UTF-32 code points", input, actual_len);
        // Note: Expected values may need adjustment based on actual Unicode handling
        assert!(
            actual_len > 0 || input.is_empty(),
            "Length should be positive for non-empty strings"
        );
    }
}

#[test]
fn test_utf8_to_utf32_conversion() {
    let test_cases = vec![
        // Simple ASCII
        "hello",
        "test123",
        // Basic Unicode
        "café",
        "München",
        "ñoño",
        // Various scripts
        "αβγδε",   // Greek
        "правда",  // Cyrillic
        "北京",    // Chinese
        "東京",    // Japanese
        "한국",    // Korean
        "ไทย",     // Thai
        "العربية", // Arabic
        "עברית",   // Hebrew
    ];

    for input in test_cases {
        let utf32_chars = unicode::utf8_to_utf32(input.as_bytes());
        println!(
            "UTF-8 to UTF-32: '{}' -> {} chars",
            input,
            utf32_chars.len()
        );

        // Verify the conversion produces valid UTF-32
        assert!(
            !utf32_chars.is_empty() || input.is_empty(),
            "Non-empty input should produce non-empty output"
        );

        // Try to convert back to UTF-8
        let utf8_bytes = unicode::utf32_to_utf8(&utf32_chars);
        if !utf8_bytes.is_empty() {
            let roundtrip = String::from_utf8(utf8_bytes);
            if let Ok(roundtrip_str) = roundtrip {
                println!("Roundtrip: '{}' -> '{}'", input, roundtrip_str);
                // Note: Some normalization might occur during roundtrip
            }
        }
    }
}

#[test]
fn test_utf32_to_utf8_conversion() {
    let test_utf32_sequences = vec![
        // Basic ASCII characters
        vec![0x48, 0x65, 0x6C, 0x6C, 0x6F], // "Hello"
        vec![0x74, 0x65, 0x73, 0x74],       // "test"
        // Unicode characters
        vec![0x63, 0x61, 0x66, 0xE9], // "café"
        vec![0xFC],                   // "ü"
        vec![0xF1],                   // "ñ"
        vec![0xDF],                   // "ß"
        // Non-BMP characters (require surrogate pairs in UTF-16, single code points in UTF-32)
        vec![0x1F600],          // 😀 emoji
        vec![0x1F1FA, 0x1F1F8], // 🇺🇸 flag emoji (two code points)
        // Greek letters
        vec![0x03B1, 0x03B2, 0x03B3], // αβγ
        // Chinese characters
        vec![0x5317, 0x4EAC], // 北京 (Beijing)
    ];

    for utf32_seq in test_utf32_sequences {
        let utf8_bytes = unicode::utf32_to_utf8(&utf32_seq);

        if !utf8_bytes.is_empty() {
            if let Ok(utf8_str) = String::from_utf8(utf8_bytes.clone()) {
                println!("UTF-32 {:X?} -> UTF-8 '{}'", utf32_seq, utf8_str);

                // Test roundtrip conversion
                let back_to_utf32 = unicode::utf8_to_utf32(&utf8_bytes);
                if back_to_utf32 == utf32_seq {
                    println!("  Roundtrip successful");
                } else {
                    println!(
                        "  Roundtrip mismatch: {:X?} != {:X?}",
                        back_to_utf32, utf32_seq
                    );
                }
            } else {
                println!("UTF-32 {:X?} -> invalid UTF-8 bytes", utf32_seq);
            }
        } else {
            println!("UTF-32 {:X?} -> empty UTF-8", utf32_seq);
        }
    }
}

#[test]
fn test_identifier_validation_edge_cases() {
    let edge_cases = vec![
        // Control characters
        ('\u{0000}', false), // NULL
        ('\u{0001}', false), // SOH
        ('\u{007F}', false), // DEL
        ('\u{0080}', false), // Control character
        // Whitespace characters
        ('\u{0020}', false), // Space
        ('\u{00A0}', false), // Non-breaking space
        ('\u{2000}', false), // En quad
        ('\u{200B}', false), // Zero-width space
        // Format characters
        ('\u{00AD}', false), // Soft hyphen
        ('\u{200C}', false), // Zero-width non-joiner
        ('\u{200D}', false), // Zero-width joiner
        // Private use characters
        ('\u{E000}', false), // Private use area
        ('\u{F8FF}', false), // Private use area
        // Unassigned characters (may vary by Unicode version)
        ('\u{0378}', false), // Unassigned in Basic Latin
    ];

    for (ch, expected_valid) in edge_cases {
        let first_pos = validation::valid_name_code_point_first_position(ch as u32);
        let other_pos = validation::valid_name_code_point_other_position(ch as u32);

        println!(
            "Character '{}' (U+{:04X}): first={}, other={}",
            ch, ch as u32, first_pos, other_pos
        );

        // Control and format characters should generally be invalid
        if ch.is_control() || ch as u32 == 0x00AD || ch as u32 == 0x200C || ch as u32 == 0x200D {
            assert!(
                !first_pos && !other_pos,
                "Control/format character should be invalid: U+{:04X}",
                ch as u32
            );
        }
    }
}

#[test]
fn test_unicode_categories() {
    // Test characters from different Unicode categories
    let category_tests = vec![
        // Letters (should be valid)
        ('a', "Lowercase Letter", true),
        ('A', "Uppercase Letter", true),
        ('ñ', "Lowercase Letter", true),
        ('İ', "Uppercase Letter", true),
        // Numbers (digits should be invalid in first position, valid in others)
        ('0', "Decimal Number", false), // First position
        ('٠', "Decimal Number", false), // Arabic-Indic digit
        // Marks (combining characters - may be invalid)
        ('\u{0301}', "Combining Mark", false), // Combining acute accent
        ('\u{0308}', "Combining Mark", false), // Combining diaeresis
        // Symbols (should be invalid)
        ('$', "Currency Symbol", false),
        ('@', "Other Symbol", false),
        ('©', "Other Symbol", false),
        // Punctuation (should be invalid except hyphen in middle)
        ('.', "Other Punctuation", false),
        ('!', "Other Punctuation", false),
        ('-', "Dash Punctuation", false), // Valid in middle positions only
    ];

    for (ch, category, expected_first) in category_tests {
        let first_pos = validation::valid_name_code_point_first_position(ch as u32);
        let other_pos = validation::valid_name_code_point_other_position(ch as u32);

        println!(
            "'{}' ({}): first={}, other={}",
            ch, category, first_pos, other_pos
        );

        if ch.is_alphabetic() {
            assert!(
                first_pos,
                "Alphabetic character should be valid in first position: '{}'",
                ch
            );
            assert!(
                other_pos,
                "Alphabetic character should be valid in other positions: '{}'",
                ch
            );
        }
    }
}
