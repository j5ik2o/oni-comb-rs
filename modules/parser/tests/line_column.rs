use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn initial_state() {
  let input = StrInput::new("hello");
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 1);
}

#[test]
fn initial_state_byte() {
  let input = ByteInput::new(b"hello");
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 1);
}

#[test]
fn column_increments_per_char() {
  let mut input = StrInput::new("abc");
  input.next_token(); // a
  input.next_token(); // b
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3); // next position
}

#[test]
fn newline_increments_line() {
  let mut input = StrInput::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  input.next_token(); // \n
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 1);
}

#[test]
fn column_after_newline() {
  let mut input = StrInput::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  input.next_token(); // \n
  input.next_token(); // c
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 2);
}

#[test]
fn multiple_lines() {
  let mut input = StrInput::new("a\nb\nc");
  for _ in 0..5 {
    input.next_token();
  }
  assert_eq!(input.line(), 3);
  assert_eq!(input.column(), 2);
}

#[test]
fn multibyte_char_counts_as_one_column() {
  let mut input = StrInput::new("café");
  input.next_token(); // c
  input.next_token(); // a
  input.next_token(); // f
  assert_eq!(input.column(), 4); // é is next
  input.next_token(); // é (1 char, multi-byte)
  assert_eq!(input.column(), 5);
}

#[test]
fn byte_input_column_is_byte_unit() {
  let mut input = ByteInput::new(b"abcd");
  input.next_token(); // a
  input.next_token(); // b
  assert_eq!(input.column(), 3);
}

#[test]
fn byte_input_newline() {
  let mut input = ByteInput::new(b"ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  input.next_token(); // \n
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 1);
  input.next_token(); // c
  assert_eq!(input.column(), 2);
}

#[test]
fn parser_updates_line_column() {
  let mut input = StrInput::new("ab\ncd");
  tag("ab").parse_next(&mut input).unwrap();
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);
}

#[test]
fn checkpoint_reset_restores_line_column() {
  let mut input = StrInput::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  let cp = input.checkpoint();
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);

  input.next_token(); // \n
  input.next_token(); // c
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 2);

  input.reset(cp);
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);
  assert_eq!(input.offset(), 2);
}

#[test]
fn checkpoint_ord_compares_by_offset() {
  let mut input = StrInput::new("abcdef");
  input.next_token();
  input.next_token();
  input.next_token();
  let cp1 = input.checkpoint(); // offset=3
  input.next_token();
  input.next_token();
  let cp2 = input.checkpoint(); // offset=5
  assert!(cp2 > cp1);
}
