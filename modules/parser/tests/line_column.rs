use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn initial_state() {
  let input = StrInputStream::new("hello");
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 1);
}

#[test]
fn initial_state_byte() {
  let input = ByteInputStream::new(b"hello");
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 1);
}

#[test]
fn column_increments_per_char() {
  let mut input = StrInputStream::new("abc");
  input.next_token(); // a
  input.next_token(); // b
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3); // next position
}

#[test]
fn newline_increments_line() {
  let mut input = StrInputStream::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  input.next_token(); // \n
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 1);
  assert_eq!(input.line_start(), 3);
}

#[test]
fn column_after_newline() {
  let mut input = StrInputStream::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  input.next_token(); // \n
  input.next_token(); // c
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 2);
}

#[test]
fn multiple_lines() {
  let mut input = StrInputStream::new("a\nb\nc");
  for _ in 0..5 {
    input.next_token();
  }
  assert_eq!(input.line(), 3);
  assert_eq!(input.column(), 2);
}

#[test]
fn multibyte_char_counts_as_one_column() {
  let mut input = StrInputStream::new("café");
  input.next_token(); // c
  input.next_token(); // a
  input.next_token(); // f
  assert_eq!(input.column(), 4); // é is next
  input.next_token(); // é (1 char, multi-byte)
  assert_eq!(input.column(), 5);
}

#[test]
fn byte_input_column_is_byte_unit() {
  let mut input = ByteInputStream::new(b"abcd");
  input.next_token(); // a
  input.next_token(); // b
  assert_eq!(input.column(), 3);
}

#[test]
fn byte_input_newline() {
  let mut input = ByteInputStream::new(b"ab\ncd");
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
  let mut input = StrInputStream::new("ab\ncd");
  tag("ab").parse_next(&mut input).unwrap();
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);
}

#[test]
fn checkpoint_reset_restores_line_column() {
  let mut input = StrInputStream::new("ab\ncd");
  input.next_token(); // a
  input.next_token(); // b
  let cp = input.checkpoint();
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);

  input.next_token(); // \n
  input.next_token(); // c
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 2);
  assert_eq!(input.line_start(), 3);

  input.reset(cp);
  assert_eq!(input.line(), 1);
  assert_eq!(input.column(), 3);
  assert_eq!(input.offset(), 2);
  assert_eq!(input.line_start(), 0);
}

#[test]
fn checkpoint_ord_compares_by_offset() {
  let mut input = StrInputStream::new("abcdef");
  input.next_token();
  input.next_token();
  input.next_token();
  let cp1 = input.checkpoint(); // offset=3
  input.next_token();
  input.next_token();
  let cp2 = input.checkpoint(); // offset=5
  assert!(cp2 > cp1);
}

#[test]
fn line_start_is_a_line_anchor_not_a_column() {
  let mut input = StrInputStream::new("é\nz");
  input.next_token(); // é
  assert_eq!(input.column(), 2);
  assert_eq!(input.line_start(), 0);

  input.next_token(); // \n
  assert_eq!(input.line(), 2);
  assert_eq!(input.column(), 1);
  assert_eq!(input.line_start(), 3);
}
