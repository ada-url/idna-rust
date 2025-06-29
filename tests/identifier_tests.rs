use ada_idna::{unicode, validation};

fn verify_code_point(input: &str, first: bool, expected: bool) {
    let code_points = unicode::utf8_to_utf32(input.as_bytes());
    assert!(!code_points.is_empty(), "Failed to convert: {}", input);
    let actual = if first {
        validation::valid_name_code_point_first_position(code_points[0])
    } else {
        validation::valid_name_code_point_other_position(code_points[0])
    };
    assert_eq!(
        actual, expected,
        "Test failed for input: '{}', first: {}",
        input, first
    );
}

#[test]
fn test_first_position_code_points() {
    verify_code_point("a", true, true);
    verify_code_point("é", true, true);
    verify_code_point("A", true, true);
    verify_code_point("0", true, false);
}

#[test]
fn test_other_position_code_points() {
    verify_code_point("a", false, true);
    verify_code_point("A", false, true);
    verify_code_point("À", false, true);
    verify_code_point("0", false, true);
    verify_code_point(" ", false, false);
}
