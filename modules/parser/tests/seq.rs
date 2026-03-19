use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;
use oni_comb_parser::primitive::seq::seq;

#[test]
fn seq_matches_str_on_str_input() {
  let mut input = StrInputStream::new("hello world");
  let result = seq("hello").parse_next(&mut input);
  assert_eq!(result.unwrap(), "hello");
  assert_eq!(input.remaining(), " world");
}

#[test]
fn seq_matches_bytes_on_byte_input() {
  let mut input = ByteInputStream::new(b"hello world");
  let result = seq(b"hello" as &[u8]).parse_next(&mut input);
  assert_eq!(result.unwrap(), b"hello");
  assert_eq!(input.remaining(), b" world");
}

#[test]
fn seq_backtrack_on_partial_match() {
  let mut input = StrInputStream::new("help");
  let result = seq("hello").parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), "help");
}

#[test]
fn seq_byte_backtrack_on_partial_match() {
  let mut input = ByteInputStream::new(b"help");
  let result = seq(b"hello" as &[u8]).parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), b"help");
}

#[test]
fn seq_empty_tag_succeeds() {
  let mut input = StrInputStream::new("anything");
  let result = seq("").parse_next(&mut input);
  assert_eq!(result.unwrap(), "");
  assert_eq!(input.remaining(), "anything");
}
