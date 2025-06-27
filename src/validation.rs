const FORBIDDEN_SPECIFIC_CHARS: &[u32] = &[
    0x0020, 0x0022, 0x0023, 0x0025, 0x002F, 0x003A, 0x003C, 0x003E, 0x003F, 0x0040, 0x005B, 0x005C,
    0x005D, 0x005E, 0x007C,
];

#[inline]
fn is_forbidden_domain_char(cp: u32) -> bool {
    // Control characters (fast range checks)
    if cp <= 0x001F || (0x007F..=0x009F).contains(&cp) {
        return true;
    }

    // Specific forbidden characters (small array linear search is fast)
    FORBIDDEN_SPECIFIC_CHARS.contains(&cp)
}

pub fn valid_name_code_point(cp: u32) -> bool {
    // Fast path for common ASCII
    if cp <= 0x007F {
        return matches!(cp, 0x002D | 0x0030..=0x0039 | 0x0061..=0x007A);
    }

    // Range-based checks for better performance
    matches!(
        cp,
        0x00B7
            | 0x00C0..=0x00D6
            | 0x00D8..=0x00F6
            | 0x00F8..=0x037D
            | 0x037F..=0x1FFF
            | 0x200C
            | 0x200D
            | 0x203F
            | 0x2040
            | 0x2070..=0x218F
            | 0x2C00..=0x2FEF
            | 0x3001..=0xD7FF
            | 0xF900..=0xFDCF
            | 0xFDF0..=0xFFFD
            | 0x10000..=0xEFFFF
    )
}

pub fn valid_name_code_point_first_position(cp: u32) -> bool {
    // First position cannot be a digit or hyphen
    match cp {
        0x0030..=0x0039 => false, // digits
        0x002D => false,          // hyphen
        _ => is_letter(cp),
    }
}

pub fn valid_name_code_point_other_position(cp: u32) -> bool {
    // Other positions can be letters, digits, or hyphen (but not special chars)
    match cp {
        0x0030..=0x0039 => true, // digits
        0x002D => true,          // hyphen
        _ => is_letter(cp),
    }
}

const GREEK_UNASSIGNED: &[u32] = &[0x0378, 0x0379, 0x0380, 0x0381, 0x0382, 0x0383];

#[inline]
fn is_arabic_indic_digit(cp: u32) -> bool {
    matches!(cp, 0x0660..=0x0669)
}

#[inline]
fn is_greek_unassigned(cp: u32) -> bool {
    GREEK_UNASSIGNED.contains(&cp)
}

fn is_letter(cp: u32) -> bool {
    // Fast path for ASCII letters
    if cp <= 0x007A {
        return matches!(cp, 0x0041..=0x005A | 0x0061..=0x007A);
    }

    match cp {
        // Latin-1 Supplement letters
        0x00C0..=0x00D6 | 0x00D8..=0x00F6 | 0x00F8..=0x00FF => true,

        // Latin Extended-A and Extended-B
        0x0100..=0x024F => true,

        // Greek and Coptic - exclude unassigned
        0x0370..=0x0373 | 0x0376..=0x0377 | 0x037B..=0x037D | 0x037F..=0x03FF => {
            !is_greek_unassigned(cp)
        }

        // Cyrillic
        0x0400..=0x04FF => true,

        // Hebrew
        0x0590..=0x05FF => true,

        // Arabic - exclude Arabic-Indic digits
        0x0600..=0x06FF => !is_arabic_indic_digit(cp),

        // CJK, Hiragana, Katakana
        0x4E00..=0x9FFF | 0x3040..=0x309F | 0x30A0..=0x30FF => true,

        // Hangul Syllables
        0xAC00..=0xD7AF => true,

        // Thai
        0x0E00..=0x0E7F => true,

        _ => false,
    }
}

pub fn contains_forbidden_domain_code_point(input: &str) -> bool {
    input.chars().any(|c| is_forbidden_domain_char(c as u32))
}

pub fn is_ascii(input: &str) -> bool {
    input.is_ascii()
}

pub fn is_label_valid(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }

    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }

    if let Some(stripped) = label.strip_prefix("xn--") {
        return crate::punycode::verify_punycode(stripped);
    }

    for c in label.chars() {
        if !valid_name_code_point(c as u32) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name_code_point() {
        assert!(valid_name_code_point(b'a' as u32));
        assert!(valid_name_code_point(b'0' as u32));
        assert!(valid_name_code_point(b'-' as u32));
        assert!(!valid_name_code_point(b' ' as u32));
    }

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii("hello"));
        assert!(!is_ascii("café"));
    }

    #[test]
    fn test_is_label_valid() {
        assert!(is_label_valid("hello"));
        assert!(is_label_valid("test-domain"));
        assert!(!is_label_valid("-invalid"));
        assert!(!is_label_valid("invalid-"));
        assert!(!is_label_valid(""));
    }

    #[test]
    fn test_contains_forbidden_domain_code_point() {
        assert!(!contains_forbidden_domain_code_point("example.com"));
        assert!(contains_forbidden_domain_code_point("exam ple.com"));
        assert!(contains_forbidden_domain_code_point("example.com/path"));
    }
}
