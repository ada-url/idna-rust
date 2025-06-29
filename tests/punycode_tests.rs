use ada_idna::{punycode_to_utf32, utf8_to_utf32, utf32_to_punycode, utf32_to_utf8};
use std::fs;

fn check_punycode_roundtrip(utf8_string: &str, puny_string: &str) {
    // UTF-8 <=> UTF-32 roundtrip
    let utf32 = utf8_to_utf32(utf8_string.as_bytes());
    let tmp = String::from_utf8(utf32_to_utf8(&utf32)).unwrap();
    assert_eq!(tmp, utf8_string, "bad utf-8 <==> utf-32 transcoding");

    // UTF-32 => Punycode
    let puny = utf32_to_punycode(&utf32);
    assert!(puny.is_some(), "bad utf-32 => punycode transcoding");
    let puny = puny.unwrap();
    assert_eq!(puny, puny_string, "punycode mismatch");

    // Punycode => UTF-32
    let utf32back = punycode_to_utf32(&puny);
    assert!(utf32back.is_some(), "bad punycode => utf-32 transcoding");
    let utf32back = utf32back.unwrap();

    // Roundtrip Punycode
    let punyback = utf32_to_punycode(&utf32back);
    assert!(
        punyback.is_some(),
        "bad utf-32 => punycode transcoding (second time)"
    );
    let punyback = punyback.unwrap();
    assert_eq!(
        punyback, puny,
        "bad punycode => utf-32 => punycode transcoding"
    );

    // Full roundtrip back to UTF-8
    let finalutf8 = String::from_utf8(utf32_to_utf8(&utf32back)).unwrap();
    assert_eq!(
        finalutf8, utf8_string,
        "bad roundtrip utf8 => utf8 transcoding"
    );
}

#[test]
fn test_punycode_fixture_alternating() {
    let fixture_path = "tests/fixtures/utf8_punycode_alternating.txt";
    let data = fs::read_to_string(fixture_path).expect("Failed to read utf8_punycode_alternating.txt fixture");

    for (i, line) in data.lines().enumerate() {
        // Skip comments and empty lines
        if line.trim().is_empty() || line.starts_with('#') || line.starts_with("idna-rust/") {
            continue;
        }
        let mut parts = line.splitn(2, '\t');
        let utf8 = parts.next().unwrap_or("").trim();
        let puny = parts.next().unwrap_or("").trim();
        if utf8.is_empty() || puny.is_empty() {
            continue;
        }
        check_punycode_roundtrip(utf8, puny);
    }
}
