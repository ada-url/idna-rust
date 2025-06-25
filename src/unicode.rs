use std::ptr;

pub fn utf8_to_utf32(buf: &[u8]) -> Vec<u32> {
    let mut pos = 0;
    let len = buf.len();
    let mut output = Vec::new();

    while pos < len {
        if pos + 16 <= len {
            let mut v1: u64 = 0;
            let mut v2: u64 = 0;
            unsafe {
                ptr::copy_nonoverlapping(
                    buf.as_ptr().add(pos),
                    (&mut v1 as *mut u64) as *mut u8,
                    8,
                );
                ptr::copy_nonoverlapping(
                    buf.as_ptr().add(pos + 8),
                    (&mut v2 as *mut u64) as *mut u8,
                    8,
                );
            }
            let v = v1 | v2;
            if (v & 0x8080808080808080) == 0 {
                let final_pos = pos + 16;
                while pos < final_pos {
                    output.push(buf[pos] as u32);
                    pos += 1;
                }
                continue;
            }
        }

        let leading_byte = buf[pos];
        if leading_byte < 0x80 {
            output.push(leading_byte as u32);
            pos += 1;
        } else if (leading_byte & 0xE0) == 0xC0 {
            if pos + 1 >= len {
                return vec![];
            }
            if (buf[pos + 1] & 0xC0) != 0x80 {
                return vec![];
            }
            let code_point = (((leading_byte & 0x1F) as u32) << 6) | ((buf[pos + 1] & 0x3F) as u32);
            if !(0x80..=0x7FF).contains(&code_point) {
                return vec![];
            }
            output.push(code_point);
            pos += 2;
        } else if (leading_byte & 0xF0) == 0xE0 {
            if pos + 2 >= len {
                return vec![];
            }
            if (buf[pos + 1] & 0xC0) != 0x80 {
                return vec![];
            }
            if (buf[pos + 2] & 0xC0) != 0x80 {
                return vec![];
            }
            let code_point = (((leading_byte & 0x0F) as u32) << 12)
                | (((buf[pos + 1] & 0x3F) as u32) << 6)
                | ((buf[pos + 2] & 0x3F) as u32);
            if !(0x800..=0xFFFF).contains(&code_point)
                || (code_point > 0xD7FF && code_point < 0xE000)
            {
                return vec![];
            }
            output.push(code_point);
            pos += 3;
        } else if (leading_byte & 0xF8) == 0xF0 {
            if pos + 3 >= len {
                return vec![];
            }
            if (buf[pos + 1] & 0xC0) != 0x80 {
                return vec![];
            }
            if (buf[pos + 2] & 0xC0) != 0x80 {
                return vec![];
            }
            if (buf[pos + 3] & 0xC0) != 0x80 {
                return vec![];
            }
            let code_point = (((leading_byte & 0x07) as u32) << 18)
                | (((buf[pos + 1] & 0x3F) as u32) << 12)
                | (((buf[pos + 2] & 0x3F) as u32) << 6)
                | ((buf[pos + 3] & 0x3F) as u32);
            if code_point <= 0xFFFF || code_point > 0x10FFFF {
                return vec![];
            }
            output.push(code_point);
            pos += 4;
        } else {
            return vec![];
        }
    }

    output
}

pub fn utf8_length_from_utf32(buf: &[u32]) -> usize {
    let mut counter = 0;
    for &cp in buf {
        counter += 1;
        if cp > 0x7F {
            counter += 1;
        }
        if cp > 0x7FF {
            counter += 1;
        }
        if cp > 0xFFFF {
            counter += 1;
        }
    }
    counter
}

pub fn utf32_length_from_utf8(buf: &[u8]) -> usize {
    buf.iter().filter(|&&b| (b as i8) > -65).count()
}

pub fn utf8_to_utf32_length(buf: &[u8]) -> usize {
    utf32_length_from_utf8(buf)
}

pub fn utf32_to_utf8(buf: &[u32]) -> Vec<u8> {
    let mut pos = 0;
    let len = buf.len();
    let mut output = Vec::new();

    while pos < len {
        if pos + 2 <= len {
            let mut v: u64 = 0;
            unsafe {
                ptr::copy_nonoverlapping(
                    buf.as_ptr().add(pos),
                    (&mut v as *mut u64) as *mut u32,
                    2,
                );
            }
            if (v & 0xFFFFFF80FFFFFF80) == 0 {
                output.push(buf[pos] as u8);
                output.push(buf[pos + 1] as u8);
                pos += 2;
                continue;
            }
        }

        let word = buf[pos];
        if (word & 0xFFFFFF80) == 0 {
            output.push(word as u8);
            pos += 1;
        } else if (word & 0xFFFFF800) == 0 {
            output.push(((word >> 6) | 0xC0) as u8);
            output.push(((word & 0x3F) | 0x80) as u8);
            pos += 1;
        } else if (word & 0xFFFF0000) == 0 {
            if (0xD800..=0xDFFF).contains(&word) {
                return vec![];
            }
            output.push(((word >> 12) | 0xE0) as u8);
            output.push((((word >> 6) & 0x3F) | 0x80) as u8);
            output.push(((word & 0x3F) | 0x80) as u8);
            pos += 1;
        } else {
            if word > 0x10FFFF {
                return vec![];
            }
            output.push(((word >> 18) | 0xF0) as u8);
            output.push((((word >> 12) & 0x3F) | 0x80) as u8);
            output.push((((word >> 6) & 0x3F) | 0x80) as u8);
            output.push(((word & 0x3F) | 0x80) as u8);
            pos += 1;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_to_utf32_ascii() {
        let input = b"hello";
        let result = utf8_to_utf32(input);
        assert_eq!(result, vec![104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_utf8_to_utf32_unicode() {
        let input = "café".as_bytes();
        let result = utf8_to_utf32(input);
        assert_eq!(result, vec![99, 97, 102, 233]);
    }

    #[test]
    fn test_utf32_to_utf8() {
        let input = vec![99, 97, 102, 233];
        let result = utf32_to_utf8(&input);
        let expected = "café".as_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_utf8_length_from_utf32() {
        let input = vec![99, 97, 102, 233]; // "café" - 'é' takes 2 bytes in UTF-8
        let result = utf8_length_from_utf32(&input);
        assert_eq!(result, 5); // c(1) + a(1) + f(1) + é(2) = 5 bytes
    }

    #[test]
    fn test_utf32_length_from_utf8() {
        let input = "café".as_bytes();
        let result = utf32_length_from_utf8(input);
        assert_eq!(result, 4);
    }
}
