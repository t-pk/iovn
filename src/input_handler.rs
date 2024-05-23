mod char_a;
mod char_e;
mod char_ee;
// Add more mod declarations as needed

pub fn viet_char_composer(buffer: &mut String, c: char) {
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

    let last_char = buffer.chars().last();
    let second_last_char = if buffer.len() >= 2 {
        buffer.chars().nth(buffer.len() - 2)
    } else {
        None
    };

    let composed_char = match last_char {
        Some('a') => char_a::handle_char_a(buffer, second_last_char, c),
        Some('e') => char_e::handle_char_e(buffer, second_last_char, c),
        Some('ê') => char_ee::handle_char_ee(buffer, second_last_char, c),
        // Add cases for other characters
        Some('d') if c == 's' => {
            pop_last_n(buffer, 1);
            Some("đ")
        }
        Some('o') if c == 'w' => {
            pop_last_n(buffer, 1);
            Some("ơ")
        }
        Some('u') if c == 'w' => {
            pop_last_n(buffer, 1);
            Some("ư")
        }
        Some('n') if c == 'g' => {
            pop_last_n(buffer, 1);
            Some("ng")
        }
        Some('n') if c == 'h' => {
            pop_last_n(buffer, 1);
            Some("nh")
        }
        _ => None,
    };

    if let Some(composed) = composed_char {
        buffer.push_str(composed);
    } else {
        buffer.push(c);
    }
}
