const BASE: i32 = 36;
const TMIN: i32 = 1;
const TMAX: i32 = 26;
const SKEW: i32 = 38;
const DAMP: i32 = 700;
const INITIAL_BIAS: i32 = 72;
const INITIAL_N: u32 = 128;

fn char_to_digit_value(value: u8) -> i32 {
    match value {
        b'a'..=b'z' => (value - b'a') as i32,
        b'0'..=b'9' => (value - b'0') as i32 + 26,
        _ => -1,
    }
}

fn digit_to_char(digit: i32) -> u8 {
    if digit < 26 {
        (digit + 97) as u8
    } else {
        (digit + 22) as u8
    }
}

fn adapt(mut d: i32, n: i32, firsttime: bool) -> i32 {
    d = if firsttime { d / DAMP } else { d / 2 };
    d += d / n;
    let mut k = 0;
    while d > ((BASE - TMIN) * TMAX) / 2 {
        d /= BASE - TMIN;
        k += BASE;
    }
    k + (((BASE - TMIN + 1) * d) / (d + SKEW))
}

pub fn punycode_to_utf32(input: &str) -> Option<Vec<u32>> {
    // See https://github.com/whatwg/url/issues/803
    if input.starts_with("xn--") {
        return None;
    }

    let mut written_out = 0i32;
    let mut out = Vec::with_capacity(input.len().max(16)); // Estimate capacity for better performance
    let mut n = INITIAL_N;
    let mut i = 0i32;
    let mut bias = INITIAL_BIAS;

    let mut input_bytes = input.as_bytes();

    // grab ascii content
    if let Some(end_of_ascii) = input_bytes.iter().rposition(|&b| b == b'-') {
        for &c in &input_bytes[..end_of_ascii] {
            if c >= 0x80 {
                return None;
            }
            out.push(c as u32);
            written_out += 1;
        }
        input_bytes = &input_bytes[end_of_ascii + 1..];
    }

    let mut pos = 0;
    while pos < input_bytes.len() {
        let oldi = i;
        let mut w = 1i32;
        let k = BASE;
        loop {
            if pos >= input_bytes.len() {
                return None;
            }
            let code_point = input_bytes[pos];
            pos += 1;
            let digit = char_to_digit_value(code_point);
            if digit < 0 {
                return None;
            }
            if digit > (0x7fffffff - i) / w {
                return None;
            }
            i += digit * w;
            let t = if k <= bias {
                TMIN
            } else if k >= bias + TMAX {
                TMAX
            } else {
                k - bias
            };
            if digit < t {
                break;
            }
            if w > 0x7fffffff / (BASE - t) {
                return None;
            }
            w *= BASE - t;
        }
        bias = adapt(i - oldi, written_out + 1, oldi == 0);
        if i / (written_out + 1) > (0x7fffffff - n as i32) {
            return None;
        }
        n += (i / (written_out + 1)) as u32;
        i %= written_out + 1;
        if n < 0x80 {
            return None;
        }
        // Optimize: avoid O(n) Vec::insert by growing and shifting manually
        let insert_pos = i as usize;
        out.push(0); // Reserve space
        let len = out.len();
        if insert_pos < len - 1 {
            // Shift elements to the right
            out.copy_within(insert_pos..len - 1, insert_pos + 1);
        }
        out[insert_pos] = n;
        written_out += 1;
        i += 1;
    }

    Some(out)
}

pub fn verify_punycode(input: &str) -> bool {
    if input.starts_with("xn--") {
        return false;
    }

    let mut written_out = 0usize;
    let mut n = INITIAL_N;
    let mut i = 0i32;
    let mut bias = INITIAL_BIAS;

    let mut input_bytes = input.as_bytes();

    // grab ascii content
    if let Some(end_of_ascii) = input_bytes.iter().rposition(|&b| b == b'-') {
        for &c in &input_bytes[..end_of_ascii] {
            if c >= 0x80 {
                return false;
            }
            written_out += 1;
        }
        input_bytes = &input_bytes[end_of_ascii + 1..];
    }

    let mut pos = 0;
    while pos < input_bytes.len() {
        let oldi = i;
        let mut w = 1i32;
        let k = BASE;
        loop {
            if pos >= input_bytes.len() {
                return false;
            }
            let code_point = input_bytes[pos];
            pos += 1;
            let digit = char_to_digit_value(code_point);
            if digit < 0 {
                return false;
            }
            if digit > (0x7fffffff - i) / w {
                return false;
            }
            i += digit * w;
            let t = if k <= bias {
                TMIN
            } else if k >= bias + TMAX {
                TMAX
            } else {
                k - bias
            };
            if digit < t {
                break;
            }
            if w > 0x7fffffff / (BASE - t) {
                return false;
            }
            w *= BASE - t;
        }
        bias = adapt(i - oldi, (written_out + 1) as i32, oldi == 0);
        if i / (written_out + 1) as i32 > (0x7fffffff_u32 - n) as i32 {
            return false;
        }
        n += (i / (written_out + 1) as i32) as u32;
        i %= (written_out + 1) as i32;
        if n < 0x80 {
            return false;
        }
        written_out += 1;
        i += 1;
    }

    true
}

pub fn utf32_to_punycode(input: &[u32]) -> Option<String> {
    let mut out = Vec::new();
    let mut n = INITIAL_N;
    let mut d = 0i32;
    let mut bias = INITIAL_BIAS;
    let mut h = 0usize;

    // first push the ascii content
    for &c in input {
        if c < 0x80 {
            h += 1;
            out.push(c as u8);
        }
        if c > 0x10ffff || (0xd800..0xe000).contains(&c) {
            return None;
        }
    }
    let b = h;
    if b > 0 {
        out.push(b'-');
    }

    while h < input.len() {
        let mut m = 0x10FFFF;
        for &code_point in input {
            if code_point >= n && code_point < m {
                m = code_point;
            }
        }

        if (m - n) > ((0x7fffffff_u32 - d as u32) / (h + 1) as u32) {
            return None;
        }
        d += ((m - n) * (h + 1) as u32) as i32;
        n = m;

        for &c in input {
            if c < n {
                if d == 0x7fffffff {
                    return None;
                }
                d += 1;
            }
            if c == n {
                let mut q = d;
                let k = BASE;
                loop {
                    let t = if k <= bias {
                        TMIN
                    } else if k >= bias + TMAX {
                        TMAX
                    } else {
                        k - bias
                    };

                    if q < t {
                        break;
                    }
                    out.push(digit_to_char(t + ((q - t) % (BASE - t))));
                    q = (q - t) / (BASE - t);
                }
                out.push(digit_to_char(q));
                bias = adapt(d, (h + 1) as i32, h == b);
                d = 0;
                h += 1;
            }
        }
        d += 1;
        n += 1;
    }

    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punycode_encoding() {
        let input = vec![0x00E4];
        let result = utf32_to_punycode(&input);
        assert!(result.is_some());
        let encoded = result.unwrap();
        assert_eq!(encoded, "4ca");
    }

    #[test]
    fn test_punycode_decoding() {
        let input = vec![0x00E4];
        let encoded = utf32_to_punycode(&input).unwrap();
        let decoded = punycode_to_utf32(&encoded);
        assert_eq!(decoded, Some(vec![0x00E4]));
    }

    #[test]
    fn test_verify_punycode() {
        assert!(verify_punycode("4ca"));
        // Empty string is valid punycode (empty input), so test something else
        assert!(verify_punycode(""));
    }

    #[test]
    fn test_xn_prefix_rejection() {
        // Should reject input starting with "xn--"
        assert_eq!(punycode_to_utf32("xn--test"), None);
        assert!(!verify_punycode("xn--test"));
    }
}
