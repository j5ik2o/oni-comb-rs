use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::{char, eof, tag};
use oni_comb_parser::str_input_stream::StrInputStream;

#[test]
fn char_matches_expected_character() {
  let mut parser = char('a');
  let mut input = StrInputStream::new("abc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('a'));
  assert_eq!(input.offset(), 1);
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn char_fails_on_mismatch() {
  let mut parser = char('a');
  let mut input = StrInputStream::new("bcd");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn char_fails_on_empty_input() {
  let mut parser = char('a');
  let mut input = StrInputStream::new("");

  let result = parser.parse_next(&mut input);

  assert!(result.is_err());
  assert_eq!(input.offset(), 0);
}

#[test]
fn char_handles_multibyte_character() {
  let mut parser = char('日');
  let mut input = StrInputStream::new("日本語");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('日'));
  assert_eq!(input.offset(), '日'.len_utf8());
}

#[test]
fn char_does_not_consume_on_multibyte_mismatch() {
  let mut parser = char('本');
  let mut input = StrInputStream::new("日本語");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn tag_matches_expected_string() {
  let mut parser = tag("hello");
  let mut input = StrInputStream::new("hello world");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok("hello"));
  assert_eq!(input.offset(), 5);
  assert_eq!(input.remaining(), " world");
}

#[test]
fn tag_fails_on_mismatch() {
  let mut parser = tag("hello");
  let mut input = StrInputStream::new("world");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn tag_fails_on_partial_match() {
  let mut parser = tag("hello");
  let mut input = StrInputStream::new("hel");

  let result = parser.parse_next(&mut input);

  assert!(result.is_err());
  assert_eq!(input.offset(), 0);
}

#[test]
fn tag_empty_string_triggers_many0_zero_progress() {
  let mut parser = tag("").many0();
  let mut input = StrInputStream::new("anything");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}

#[test]
fn eof_succeeds_at_end_of_input() {
  let mut parser = eof();
  let mut input = StrInputStream::new("");

  let result = parser.parse_next(&mut input);

  assert!(result.is_ok());
}

#[test]
fn eof_fails_when_input_remains() {
  let mut parser = eof();
  let mut input = StrInputStream::new("remaining");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
}
