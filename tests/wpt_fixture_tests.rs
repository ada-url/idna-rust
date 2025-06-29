use ada_idna::domain::{contains_forbidden_domain_code_point, to_ascii};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct IdnaTestCase {
    input: String,
    output: Option<String>,
    // comment and other fields are ignored
}

#[test]
fn test_idna_test_v2_fixture() {
    // Path relative to the project root
    let fixture_path = "tests/fixtures/IdnaTestV2.json";
    let data = fs::read_to_string(fixture_path).expect("Failed to read IdnaTestV2.json fixture");

    // The fixture is a JSON array of objects and comments (strings)
    let raw: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON");
    let arr = raw.as_array().expect("Fixture is not a JSON array");

    for (i, item) in arr.iter().enumerate() {
        // Skip comments (strings)
        if item.is_string() {
            continue;
        }
        let case: IdnaTestCase = serde_json::from_value(item.clone())
            .unwrap_or_else(|_| panic!("Failed to parse test case at index {}", i));

        let output_cow = to_ascii(&case.input).unwrap_or_else(|_| String::new().into());
        let output = if contains_forbidden_domain_code_point(&output_cow) {
            String::new()
        } else {
            output_cow.into_owned()
        };

        match &case.output {
            Some(expected) => assert_eq!(
                &output, expected,
                "Mismatch for input: '{}', expected: '{}', got: '{}'",
                case.input, expected, output
            ),
            None => assert!(
                output.is_empty(),
                "Expected empty output for '{}', got '{}'",
                case.input,
                output
            ),
        }
    }
}
