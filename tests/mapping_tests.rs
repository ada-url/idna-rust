use ada_idna::mapping;

#[test]
fn test_mapping_minimal() {
    // Direct translation of mapping_tests.cpp from ada-url/idna
    assert_eq!(mapping::map("asciitwontchange"), "asciitwontchange");
    assert_eq!(mapping::map("hasomit\u{00ad}ted"), "hasomitted");
    assert_eq!(mapping::map("\u{00a0}lla"), "alla");
}
