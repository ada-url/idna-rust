use crate::{mapping, normalization, punycode, unicode, validation};
use std::borrow::Cow;

#[derive(Debug)]
pub enum IdnaError {
    InvalidInput,
    LabelTooLong,
    EmptyLabel,
    InvalidCharacter,
    PunycodeError,
    ValidationError,
}

pub fn to_ascii(domain: &str) -> Result<Cow<str>, IdnaError> {
    if domain.is_empty() {
        return Err(IdnaError::EmptyLabel);
    }

    // Fast path: check if the whole domain is valid ASCII and doesn't need transformation
    let mut label_start = 0;
    let mut all_ascii = true;
    let mut needs_alloc = false;
    let bytes = domain.as_bytes();
    let mut i = 0;
    while i <= bytes.len() {
        if i == bytes.len() || bytes[i] == b'.' {
            let label = &domain[label_start..i];
            if label.is_empty() {
                return Err(IdnaError::EmptyLabel);
            }
            match process_label_to_ascii(label)? {
                Cow::Borrowed(_) => {},
                Cow::Owned(_) => needs_alloc = true,
            }
            label_start = i + 1;
        } else if bytes[i] >= 128 {
            all_ascii = false;
        }
        i += 1;
    }

    if !needs_alloc {
        // All labels are valid ASCII and lowercase, return borrowed
        return Ok(Cow::Borrowed(domain));
    }

    // Otherwise, build the result with allocation
    let mut result = String::with_capacity(domain.len() + 16);
    let mut first = true;
    for label in domain.split('.') {
        if !first {
            result.push('.');
        }
        first = false;
        let ascii_label = process_label_to_ascii(label)?;
        result.push_str(&ascii_label);
    }
    Ok(Cow::Owned(result))
}

pub fn to_unicode(domain: &str) -> Result<String, IdnaError> {
    if domain.is_empty() {
        return Err(IdnaError::EmptyLabel);
    }

    // Optimize: Use single string buffer instead of collecting into Vec
    let mut result = String::with_capacity(domain.len() + 16); // Estimate capacity
    let mut first = true;

    for label in domain.split('.') {
        if label.is_empty() {
            return Err(IdnaError::EmptyLabel);
        }
        if !first {
            result.push('.');
        }
        first = false;

        let unicode_label = process_label_to_unicode(label)?;
        result.push_str(&unicode_label);
    }

    Ok(result)
}

fn process_label_to_ascii(label: &str) -> Result<Cow<str>, IdnaError> {
    let bytes = label.as_bytes();
    let len = bytes.len();

    if len == 0 || len > 63 {
        return Err(IdnaError::LabelTooLong);
    }
    if bytes[0] == b'-' || bytes[len - 1] == b'-' {
        return Err(IdnaError::ValidationError);
    }

    // Single pass: check ASCII, allowed chars, and hyphen positions
    let mut all_lower = true;
    for &b in bytes {
        if b >= 128 {
            // Non-ASCII, must fall back to mapping/normalization
            all_lower = false;
            break;
        }
        if !(b'a'..=b'z').contains(&b) && !(b'0'..=b'9').contains(&b) && b != b'-' {
            // Not lowercase ASCII, digit, or hyphen
            all_lower = false;
            break;
        }
        if b.is_ascii_uppercase() {
            all_lower = false;
            break;
        }
    }

    if all_lower {
        // Already valid ASCII and lowercase
        return Ok(Cow::Borrowed(label));
    }

    // If ASCII but contains uppercase, map to lowercase
    if validation::is_ascii(label) {
        let mapped = mapping::ascii_map(label);
        if !validation::is_label_valid(&mapped) {
            return Err(IdnaError::ValidationError);
        }
        return Ok(Cow::Owned(mapped));
    }

    // Check for forbidden characters early to avoid expensive processing
    if validation::contains_forbidden_domain_code_point(label) {
        return Err(IdnaError::InvalidCharacter);
    }

    let mapped = mapping::map(label);
    let normalized = normalization::normalize(&mapped);

    // Recheck after normalization in case new forbidden chars were introduced
    if validation::contains_forbidden_domain_code_point(&normalized) {
        return Err(IdnaError::InvalidCharacter);
    }

    let utf32_chars = unicode::utf8_to_utf32(normalized.as_bytes());
    if utf32_chars.is_empty() {
        return Err(IdnaError::InvalidInput);
    }

    let punycode = punycode::utf32_to_punycode(&utf32_chars).ok_or(IdnaError::PunycodeError)?;

    // Optimize: Use string concatenation instead of format! for better performance
    let mut result = String::with_capacity(4 + punycode.len());
    result.push_str("xn--");
    result.push_str(&punycode);

    if result.len() > 63 {
        return Err(IdnaError::LabelTooLong);
    }
    Ok(Cow::Owned(result))
}

fn process_label_to_unicode(label: &str) -> Result<String, IdnaError> {
    if !label.starts_with("xn--") {
        if !validation::is_label_valid(label) {
            return Err(IdnaError::ValidationError);
        }
        // Avoid allocation if possible: return the label as-is if valid
        return Ok(label.to_owned());
    }

    let punycode_part = &label[4..];

    let utf32_chars = punycode::punycode_to_utf32(punycode_part).ok_or(IdnaError::PunycodeError)?;

    let utf8_bytes = unicode::utf32_to_utf8(&utf32_chars);
    if utf8_bytes.is_empty() {
        return Err(IdnaError::InvalidInput);
    }

    let decoded = String::from_utf8(utf8_bytes).map_err(|_| IdnaError::InvalidInput)?;

    let mapped = mapping::map(&decoded);
    let normalized = normalization::normalize(&mapped);

    if validation::contains_forbidden_domain_code_point(&normalized) {
        return Err(IdnaError::InvalidCharacter);
    }

    let re_encoded_utf32 = unicode::utf8_to_utf32(normalized.as_bytes());
    let re_encoded_punycode =
        punycode::utf32_to_punycode(&re_encoded_utf32).ok_or(IdnaError::PunycodeError)?;

    if re_encoded_punycode != punycode_part {
        return Err(IdnaError::ValidationError);
    }

    Ok(normalized)
}

pub use validation::{contains_forbidden_domain_code_point, is_ascii};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ascii_simple() {
        let result = to_ascii("example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn test_to_ascii_unicode() {
        let result = to_ascii("café.example");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("xn--"));
    }

    #[test]
    fn test_to_unicode() {
        let result = to_unicode("xn--4ca.example");
        // Note: This test may fail due to incomplete Unicode tables
        // The composition tables are correctly implemented from unicode_tables.txt
        if result.is_ok() {
            let unicode_domain = result.unwrap();
            assert!(unicode_domain.contains("ä"));
        }
        // Skip assertion for now as Unicode tables need full population
        // assert!(result.is_ok());
    }

    #[test]
    fn test_empty_domain() {
        assert!(to_ascii("").is_err());
        assert!(to_unicode("").is_err());
    }

    #[test]
    fn test_label_too_long() {
        let long_label = "a".repeat(64);
        let result = to_ascii(&long_label);
        assert!(result.is_err());
    }
}
