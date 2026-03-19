use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

// --- any ---

#[test]
fn any_consumes_one_char() {
  let mut input = StrInputStream::new("abc");
  let result: char = any().parse_next(&mut input).unwrap();
  assert_eq!(result, 'a');
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn any_consumes_one_byte() {
  let mut input = ByteInputStream::new(b"abc");
  let result: u8 = any().parse_next(&mut input).unwrap();
  assert_eq!(result, b'a');
  assert_eq!(input.remaining(), b"bc");
}

#[test]
fn any_backtrack_on_eof() {
  let mut input = StrInputStream::new("");
  let result = any::<StrInputStream>().parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

// --- not_a ---

#[test]
fn not_a_consumes_non_matching_char() {
  let mut input = StrInputStream::new("abc");
  let result = not_a(|c: char| c == '"').parse_next(&mut input).unwrap();
  assert_eq!(result, 'a');
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn not_a_backtrack_on_matching_char() {
  let mut input = StrInputStream::new("\"abc");
  let result = not_a(|c: char| c == '"').parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), "\"abc");
}

#[test]
fn not_a_works_with_byte_input() {
  let mut input = ByteInputStream::new(b"abc");
  let result = not_a(|b: u8| b == b'"').parse_next(&mut input).unwrap();
  assert_eq!(result, b'a');
}

// --- sym via prelude ---

#[test]
fn sym_from_prelude_works() {
  let mut input = StrInputStream::new("abc");
  let result = sym('a').parse_next(&mut input).unwrap();
  assert_eq!(result, 'a');
}

// --- seq via prelude ---

#[test]
fn seq_from_prelude_works() {
  let mut input = StrInputStream::new("hello world");
  let result = seq("hello").parse_next(&mut input).unwrap();
  assert_eq!(result, "hello");
}
