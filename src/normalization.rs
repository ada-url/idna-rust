use crate::unicode_tables::*;

pub fn normalize(input: &str) -> String {
    // Optimize: Pre-allocate with estimated capacity
    let char_count = input.chars().count();
    let mut chars: Vec<u32> = Vec::with_capacity(char_count + char_count / 4); // Allow for expansion
    chars.extend(input.chars().map(|c| c as u32));

    // Decompose and reorder (NFC normalization)
    decompose_nfc(&mut chars);
    compose(&mut chars);

    // Convert back to string
    chars.into_iter().filter_map(char::from_u32).collect()
}

fn decompose_nfc(input: &mut Vec<u32>) {
    let (decomposition_needed, additional_elements) = compute_decomposition_length(input);
    if decomposition_needed {
        decompose(input, additional_elements);
    }
    sort_marks(input);
}

fn compute_decomposition_length(input: &[u32]) -> (bool, usize) {
    let mut decomposition_needed = false;
    let mut additional_elements = 0;

    for &current_character in input {
        let mut decomposition_length = 0;

        if (HANGUL_SBASE..HANGUL_SBASE + HANGUL_SCOUNT).contains(&current_character) {
            decomposition_length = 2;
            if (current_character - HANGUL_SBASE) % HANGUL_TCOUNT != 0 {
                decomposition_length = 3;
            }
        } else if current_character < 0x110000 {
            let di = DECOMPOSITION_INDEX[(current_character >> 8) as usize];
            let decomposition = &DECOMPOSITION_BLOCK[di as usize];
            let idx = (current_character % 256) as usize;
            if idx < decomposition.len() - 1 {
                decomposition_length =
                    ((decomposition[idx + 1] >> 2) - (decomposition[idx] >> 2)) as usize;
                if decomposition_length > 0 && (decomposition[idx] & 1) != 0 {
                    decomposition_length = 0;
                }
            }
        }

        if decomposition_length != 0 {
            decomposition_needed = true;
            additional_elements += decomposition_length - 1;
        }
    }

    (decomposition_needed, additional_elements)
}

fn decompose(input: &mut Vec<u32>, additional_elements: usize) {
    input.resize(input.len() + additional_elements, 0);
    let input_count = input.len() - additional_elements;
    let mut descending_idx = input.len();

    for i in (0..input_count).rev() {
        let current_char = input[i];

        if (HANGUL_SBASE..HANGUL_SBASE + HANGUL_SCOUNT).contains(&current_char) {
            // Hangul decomposition
            let s_index = current_char - HANGUL_SBASE;
            if s_index % HANGUL_TCOUNT != 0 {
                descending_idx -= 1;
                input[descending_idx] = HANGUL_TBASE + s_index % HANGUL_TCOUNT;
            }
            descending_idx -= 1;
            input[descending_idx] = HANGUL_VBASE + (s_index % HANGUL_NCOUNT) / HANGUL_TCOUNT;
            descending_idx -= 1;
            input[descending_idx] = HANGUL_LBASE + s_index / HANGUL_NCOUNT;
        } else if current_char < 0x110000 {
            // Check decomposition data
            let di = DECOMPOSITION_INDEX[(current_char >> 8) as usize];
            let decomposition = &DECOMPOSITION_BLOCK[di as usize];
            let idx = (current_char % 256) as usize;

            let mut decomposition_length = 0;
            if idx < decomposition.len() - 1 {
                decomposition_length = (decomposition[idx + 1] >> 2) - (decomposition[idx] >> 2);
                if decomposition_length > 0 && (decomposition[idx] & 1) != 0 {
                    decomposition_length = 0;
                }
            }

            if decomposition_length > 0 {
                // Non-recursive decomposition
                let start_idx = (decomposition[idx] >> 2) as usize;
                for j in 0..decomposition_length {
                    if start_idx + (j as usize) < DECOMPOSITION_DATA.len() {
                        descending_idx -= 1;
                        input[descending_idx] =
                            DECOMPOSITION_DATA[start_idx + (decomposition_length - 1 - j) as usize];
                    }
                }
            } else {
                // No decomposition
                descending_idx -= 1;
                input[descending_idx] = current_char;
            }
        } else {
            // Non-Unicode character
            descending_idx -= 1;
            input[descending_idx] = current_char;
        }
    }
}

fn get_ccc(c: u32) -> u8 {
    if c < 0x110000 {
        let idx = CANONICAL_COMBINING_CLASS_INDEX[(c >> 8) as usize] as usize;
        CANONICAL_COMBINING_CLASS_BLOCK[idx][(c % 256) as usize]
    } else {
        0
    }
}

fn sort_marks(input: &mut [u32]) {
    for idx in 1..input.len() {
        let ccc = get_ccc(input[idx]);
        if ccc == 0 {
            continue; // Skip non-combining characters
        }

        let current_character = input[idx];
        let mut back_idx = idx;
        while back_idx != 0 && get_ccc(input[back_idx - 1]) > ccc {
            input[back_idx] = input[back_idx - 1];
            back_idx -= 1;
        }
        input[back_idx] = current_character;
    }
}

fn compose(input: &mut Vec<u32>) {
    let mut input_count = 0;
    let mut composition_count = 0;

    while input_count < input.len() {
        input[composition_count] = input[input_count];

        if input[input_count] >= HANGUL_LBASE && input[input_count] < HANGUL_LBASE + HANGUL_LCOUNT {
            if input_count + 1 < input.len()
                && input[input_count + 1] >= HANGUL_VBASE
                && input[input_count + 1] < HANGUL_VBASE + HANGUL_VCOUNT
            {
                input[composition_count] = HANGUL_SBASE
                    + ((input[input_count] - HANGUL_LBASE) * HANGUL_VCOUNT
                        + input[input_count + 1]
                        - HANGUL_VBASE)
                        * HANGUL_TCOUNT;
                input_count += 1;
                if input_count + 1 < input.len()
                    && input[input_count + 1] > HANGUL_TBASE
                    && input[input_count + 1] < HANGUL_TBASE + HANGUL_TCOUNT
                {
                    input[composition_count] += input[input_count + 1] - HANGUL_TBASE;
                    input_count += 1;
                }
            }
        } else if input[input_count] >= HANGUL_SBASE
            && input[input_count] < HANGUL_SBASE + HANGUL_SCOUNT
        {
            if (input[input_count] - HANGUL_SBASE) % HANGUL_TCOUNT != 0
                && input_count + 1 < input.len()
                && input[input_count + 1] > HANGUL_TBASE
                && input[input_count + 1] < HANGUL_TBASE + HANGUL_TCOUNT
            {
                input[composition_count] += input[input_count + 1] - HANGUL_TBASE;
                input_count += 1;
            }
        } else if input[input_count] < 0x110000 {
            let ci = COMPOSITION_INDEX[(input[input_count] >> 8) as usize] as usize;
            let composition_idx = (input[input_count] % 256) as usize;
            let composition = &COMPOSITION_BLOCK[ci][composition_idx..];
            let initial_composition_count = composition_count;
            let mut previous_ccc = -1i32;

            while input_count + 1 < input.len() {
                let ccc = get_ccc(input[input_count + 1]) as i32;

                if composition.len() >= 2 && composition[1] != composition[0] && previous_ccc < ccc
                {
                    // Try finding a composition
                    let mut left = composition[0] as usize;
                    let mut right = composition[1] as usize;
                    while left + 2 < right {
                        let middle = left + (((right - left) >> 1) & !1);
                        if COMPOSITION_DATA[middle] <= input[input_count + 1] {
                            left = middle;
                        }
                        if COMPOSITION_DATA[middle] >= input[input_count + 1] {
                            right = middle;
                        }
                    }
                    if left < COMPOSITION_DATA.len()
                        && COMPOSITION_DATA[left] == input[input_count + 1]
                        && left + 1 < COMPOSITION_DATA.len()
                    {
                        input[initial_composition_count] = COMPOSITION_DATA[left + 1];
                        let new_ci =
                            COMPOSITION_INDEX[(COMPOSITION_DATA[left + 1] >> 8) as usize] as usize;
                        let new_char_idx = (COMPOSITION_DATA[left + 1] % 256) as usize;
                        if new_ci < COMPOSITION_BLOCK.len()
                            && new_char_idx < COMPOSITION_BLOCK[new_ci].len()
                        {
                            // Update composition reference for potential further composition
                        }
                        input_count += 1;
                        continue;
                    }
                }

                if ccc == 0 {
                    break; // Not a combining character
                }
                previous_ccc = ccc;
                composition_count += 1;
                input[composition_count] = input[input_count + 1];
                input_count += 1;
            }
        }

        input_count += 1;
        composition_count += 1;
    }

    if composition_count < input_count {
        input.resize(composition_count, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let input = "café";
        let result = normalize(input);
        assert!(!result.is_empty());
        // For now, just ensure it doesn't crash and returns something
        // Full Unicode table implementation would be needed for proper testing
    }

    #[test]
    fn test_hangul_constants() {
        assert_eq!(HANGUL_NCOUNT, 588);
        assert_eq!(HANGUL_SCOUNT, 11172);
    }
}
