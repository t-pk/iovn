use iovn::input_handler::viet_char_composer;

#[test]
fn test_viet_char_composer() {
  let mut buffer = String::new();

  // Test chữ 'a' và các dấu
  buffer.push('a');
  viet_char_composer(&mut buffer, 's');
  assert_eq!(buffer, "á");

  buffer.clear();
  buffer.push('a');
  viet_char_composer(&mut buffer, 'f');
  assert_eq!(buffer, "à");

  buffer.clear();
  buffer.push('a');
  viet_char_composer(&mut buffer, 'r');
  assert_eq!(buffer, "ả");

  buffer.clear();
  buffer.push('a');
  viet_char_composer(&mut buffer, 'x');
  assert_eq!(buffer, "ã");

  buffer.clear();
  buffer.push('a');
  viet_char_composer(&mut buffer, 'j');
  assert_eq!(buffer, "ạ");

  // Test trường hợp không kết hợp được
  buffer.clear();
  buffer.push('b');
  viet_char_composer(&mut buffer, 's');
  assert_eq!(buffer, "bs");
}
