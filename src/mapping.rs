pub fn ascii_map(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

pub fn map(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            // Soft hyphen - remove completely
            '\u{00AD}' => continue,

            // Special case mappings
            '\u{00DF}' => result.push_str("ss"),
            '\u{0130}' => result.push_str("i\u{0307}"),
            '\u{FB00}' => result.push_str("ff"),
            '\u{FB01}' => result.push_str("fi"),
            '\u{FB02}' => result.push_str("fl"),
            '\u{FB03}' => result.push_str("ffi"),
            '\u{FB04}' => result.push_str("ffl"),
            '\u{FB05}' => result.push_str("st"),
            '\u{FB06}' => result.push_str("st"),
            '\u{0587}' => result.push_str("\u{0565}\u{0582}"),
            '\u{FB13}' => result.push_str("\u{0574}\u{0576}"),
            '\u{FB14}' => result.push_str("\u{0574}\u{0565}"),
            '\u{FB15}' => result.push_str("\u{0574}\u{056B}"),
            '\u{FB16}' => result.push_str("\u{057E}\u{0576}"),
            '\u{FB17}' => result.push_str("\u{0574}\u{056D}"),

            // Zero-width characters - remove
            '\u{200C}' | '\u{200D}' => continue,

            // Format characters - remove
            '\u{200E}' | '\u{200F}' => continue,
            '\u{202A}' | '\u{202B}' | '\u{202C}' | '\u{202D}' | '\u{202E}' => continue,

            // Case folding - optimize for common ASCII case
            _ => {
                // Fast path for ASCII characters
                if c.is_ascii() && c.is_ascii_uppercase() {
                    result.push(c.to_ascii_lowercase());
                } else if c.is_ascii() {
                    result.push(c);
                } else {
                    // Full Unicode case folding for non-ASCII
                    for lowercase_char in c.to_lowercase() {
                        result.push(lowercase_char);
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_map() {
        assert_eq!(ascii_map("HELLO"), "hello");
        assert_eq!(ascii_map("Hello"), "hello");
        assert_eq!(ascii_map("hello"), "hello");
    }

    #[test]
    fn test_map_with_special_chars() {
        assert_eq!(map("ß"), "ss");
        assert_eq!(map("İ"), "i̇");
    }
}
