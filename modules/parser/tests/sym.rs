use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;
use oni_comb_parser::primitive::sym::sym;

#[test]
fn sym_matches_char_on_str_input() {
  let mut input = StrInputStream::new("abc");
  let result = sym('a').parse_next(&mut input);
  assert_eq!(result.unwrap(), 'a');
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn sym_matches_u8_on_byte_input() {
  let mut input = ByteInputStream::new(b"abc");
  let result = sym(b'a').parse_next(&mut input);
  assert_eq!(result.unwrap(), b'a');
  assert_eq!(input.remaining(), b"bc");
}

#[test]
fn sym_backtrack_on_mismatch() {
  let mut input = StrInputStream::new("xyz");
  let result = sym('a').parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), "xyz");
}

#[test]
fn sym_backtrack_on_eof() {
  let mut input = StrInputStream::new("");
  let result = sym('a').parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

#[test]
fn sym_byte_backtrack_on_mismatch() {
  let mut input = ByteInputStream::new(b"xyz");
  let result = sym(b'a').parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), b"xyz");
}

#[test]
fn sym_byte_backtrack_on_eof() {
  let mut input = ByteInputStream::new(b"");
  let result = sym(b'a').parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}
