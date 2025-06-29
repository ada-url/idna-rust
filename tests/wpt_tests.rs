use ada_idna::domain::{contains_forbidden_domain_code_point, to_ascii};

#[test]
fn test_wpt_minimal_cases() {
    // Minimal direct translation of ada-url/idna wpt_tests.cpp logic
    let test_cases = vec![
        // (input, expected_output)
        ("example.com", Some("example.com")),
        ("straße.de", Some("xn--strae-oqa.de")),
        ("xn--strae-oqa.de", Some("xn--strae-oqa.de")),
        ("xn--caf-dma.com", Some("xn--caf-dma.com")),
        ("xn--invalid", None),             // Should fail (invalid punycode)
        ("test\u{0000}example.com", None), // Forbidden character
    ];

    for (input, expected) in test_cases {
        let mut output = to_ascii(input).unwrap_or_else(|_| String::new().into());
        if contains_forbidden_domain_code_point(&output) {
            output = "".into();
        }
        let output: String = output.into_owned();
        match expected {
            Some(expected_str) => {
                assert_eq!(output, expected_str, "Mismatch for input: '{}'", input)
            }
            None => assert!(
                output.is_empty(),
                "Expected empty output for '{}', got '{}'",
                input,
                output
            ),
        }
    }
}
