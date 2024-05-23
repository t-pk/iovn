pub fn handle_char_ee(
  buffer: &mut String,
  second_last_char: Option<char>,
  c: char,
) -> Option<&'static str> {
  match (second_last_char, buffer.chars().last(), c) {
    (_, Some('ê'), 's') => {
      if let Some(start) = buffer.char_indices().rev().find_map(|(i, _)| Some(i)) {
        buffer.replace_range(start.., "ế");
      }
      Some("")
    }
    
    (_, Some('ê'), 'f') => {
      if let Some(start) = buffer.char_indices().rev().find_map(|(i, _)| Some(i)) {
        buffer.replace_range(start.., "ề");
      }
      Some("")
    }
    (_, Some('ê'), 'r') => {
      if let Some(start) = buffer.char_indices().rev().find_map(|(i, _)| Some(i)) {
        buffer.replace_range(start.., "ể");
      }
      Some("")
    }
    (_, Some('ê'), 'x') => {
      if let Some(start) = buffer.char_indices().rev().find_map(|(i, _)| Some(i)) {
        buffer.replace_range(start.., "ễ");
      }
      Some("")
    }
    (_, Some('ê'), 'j') => {
      if let Some(start) = buffer.char_indices().rev().find_map(|(i, _)| Some(i)) {
        buffer.replace_range(start.., "ệ");
      }
      Some("")
    }
    // Default case: no transformation
    _ => None,
  }
}
