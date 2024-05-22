pub fn viet_char_composer(buffer: &mut String, c: char) {
    // Function to pop the last n characters from buffer
    fn pop_last_n(buffer: &mut String, n: usize) {
        let len = buffer.len();
        if n <= len {
            buffer.truncate(len - n);
        } else {
            buffer.clear();
        }
    }

    // Check last character and optional second-to-last character
    let last_char = buffer.chars().last();
    let second_last_char = if buffer.len() >= 2 {
        buffer.chars().nth(buffer.len() - 2)
    } else {
        None
    };

    let composed_char = match (second_last_char, last_char, c) {
        // Special vowel combinations
        (Some('u'), Some('o'), 'w') => {
            pop_last_n(buffer, 2);
            "ươ"
        }

        // Two-character patterns for 'a'
        (Some('a'), Some('a'), 's') => {
            pop_last_n(buffer, 2);
            "ấ"
        }
        (Some('a'), Some('w'), 's') => {
            pop_last_n(buffer, 2);
            "ắ"
        }

        // Single-character patterns for 'a'
        (_, Some('a'), 's') => {
            pop_last_n(buffer, 1);
            "á"
        }
        (_, Some('a'), 'f') => {
            pop_last_n(buffer, 1);
            "à"
        }
        (_, Some('a'), 'r') => {
            pop_last_n(buffer, 1);
            "ả"
        }
        (_, Some('a'), 'x') => {
            pop_last_n(buffer, 1);
            "ã"
        }
        (_, Some('a'), 'j') => {
            pop_last_n(buffer, 1);
            "ạ"
        }

        // Vowel accents for 'e'
        (_, Some('e'), 's') => {
            pop_last_n(buffer, 1);
            "é"
        }
        (_, Some('e'), 'f') => {
            pop_last_n(buffer, 1);
            "è"
        }
        (_, Some('e'), 'r') => {
            pop_last_n(buffer, 1);
            "ẻ"
        }
        (_, Some('e'), 'x') => {
            pop_last_n(buffer, 1);
            "ẽ"
        }
        (_, Some('e'), 'j') => {
            pop_last_n(buffer, 1);
            "ẹ"
        }

        // Vowel accents for 'i'
        (_, Some('i'), 's') => {
            pop_last_n(buffer, 1);
            "í"
        }
        (_, Some('i'), 'f') => {
            pop_last_n(buffer, 1);
            "ì"
        }
        (_, Some('i'), 'r') => {
            pop_last_n(buffer, 1);
            "ỉ"
        }
        (_, Some('i'), 'x') => {
            pop_last_n(buffer, 1);
            "ĩ"
        }
        (_, Some('i'), 'j') => {
            pop_last_n(buffer, 1);
            "ị"
        }

        // Vowel accents for 'o'
        (_, Some('o'), 's') => {
            pop_last_n(buffer, 1);
            "ó"
        }
        (_, Some('o'), 'f') => {
            pop_last_n(buffer, 1);
            "ò"
        }
        (_, Some('o'), 'r') => {
            pop_last_n(buffer, 1);
            "ỏ"
        }
        (_, Some('o'), 'x') => {
            pop_last_n(buffer, 1);
            "õ"
        }
        (_, Some('o'), 'j') => {
            pop_last_n(buffer, 1);
            "ọ"
        }

        // Vowel accents for 'u'
        (_, Some('u'), 's') => {
            pop_last_n(buffer, 1);
            "ú"
        }
        (_, Some('u'), 'f') => {
            pop_last_n(buffer, 1);
            "ù"
        }
        (_, Some('u'), 'r') => {
            pop_last_n(buffer, 1);
            "ủ"
        }
        (_, Some('u'), 'x') => {
            pop_last_n(buffer, 1);
            "ũ"
        }
        (_, Some('u'), 'j') => {
            pop_last_n(buffer, 1);
            "ụ"
        }

        // Special consonant combinations
        (_, Some('d'), 's') => {
            pop_last_n(buffer, 1);
            "đ"
        }
        (_, Some('a'), 'w') => {
            pop_last_n(buffer, 1);
            "ă"
        }
        (_, Some('o'), 'w') => {
            pop_last_n(buffer, 1);
            "ơ"
        }
        (_, Some('u'), 'w') => {
            pop_last_n(buffer, 1);
            "ư"
        }

        // Consonant digraphs
        (_, Some('n'), 'g') => {
            pop_last_n(buffer, 1);
            "ng"
        }
        (_, Some('n'), 'h') => {
            pop_last_n(buffer, 1);
            "nh"
        }

        // Default case: no match found, push the character
        _ => {
            buffer.push(c);
            return;
        }
    };

    buffer.push_str(composed_char);
}
