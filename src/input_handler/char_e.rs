pub fn handle_char_e(buffer: &mut String, second_last_char: Option<char>, c: char) -> Option<&'static str> {
    fn pop_last_n(buffer: &mut String, n: usize) {
        for _ in 0..n {
            if let Some(ch) = buffer.pop() {
                // Handle multi-byte characters properly
                if ch.len_utf8() > 1 {
                    for _ in 1..ch.len_utf8() {
                        buffer.pop();
                    }
                }
            }
        }
    }

    match (second_last_char, buffer.chars().last(), c) {
        // Single-character patterns for 'e'
        (_, Some('e'), 's') => {
            pop_last_n(buffer, 1);
            Some("é")
        }
        (_, Some('e'), 'f') => {
            pop_last_n(buffer, 1);
            Some("è")
        }
        (_, Some('e'), 'r') => {
            pop_last_n(buffer, 1);
            Some("ẻ")
        }
        (_, Some('e'), 'x') => {
            pop_last_n(buffer, 1);
            Some("ẽ")
        }
        (_, Some('e'), 'j') => {
            pop_last_n(buffer, 1);
            Some("ẹ")
        }
        (_, Some('e'), 'e') => {
            pop_last_n(buffer, 1);
            Some("ê")
        }
        // Default case: no transformation
        _ => None,
    }
}
