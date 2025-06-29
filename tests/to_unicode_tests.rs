use ada_idna::domain::to_unicode;
use std::fs;

#[test]
fn test_to_unicode_alternating_fixture() {
    let fixture_path = "tests/fixtures/to_unicode_alternating.txt";
    let data = fs::read_to_string(fixture_path).expect("Failed to read fixture");
    let lines: Vec<_> = data.lines().collect();
    assert!(
        lines.len() % 2 == 0,
        "Fixture should have even number of lines"
    );
    for i in (0..lines.len()).step_by(2) {
        let input = lines[i];
        let expected = lines[i + 1];
        let result = to_unicode(input).unwrap_or_else(|_| String::new());
        assert_eq!(result, expected, "Mismatch for input: '{}'", input);
    }
}
