pub fn handle_char_a(buffer: &mut String, second_last_char: Option<char>, c: char) -> Option<&'static str> {
  // Function to pop the last n characters from buffer
  fn pop_last_n(buffer: &mut String, n: usize) {
      let len = buffer.len();
      if n <= len {
          buffer.truncate(len - n);
      } else {
          buffer.clear();
      }
  }

  match (second_last_char, buffer.chars().last(), c) {
      // Two-character patterns for 'a'
      (Some('a'), Some('a'), 's') => {
          pop_last_n(buffer, 2);
          Some("ấ")
      }
      (Some('a'), Some('w'), 's') => {
          pop_last_n(buffer, 2);
          Some("ắ")
      }

      // Single-character patterns for 'a'
      (_, Some('a'), 's') => {
          pop_last_n(buffer, 1);
          Some("á")
      }
      (_, Some('a'), 'f') => {
          pop_last_n(buffer, 1);
          Some("à")
      }
      (_, Some('a'), 'r') => {
          pop_last_n(buffer, 1);
          Some("ả")
      }
      (_, Some('a'), 'x') => {
          pop_last_n(buffer, 1);
          Some("ã")
      }
      (_, Some('a'), 'j') => {
          pop_last_n(buffer, 1);
          Some("ạ")
      }

      (_, Some('a'), 'w') => {
          pop_last_n(buffer, 1);
          Some("ă")
      }
      _ => None,
  }
}
