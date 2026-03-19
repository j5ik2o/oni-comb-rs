use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── zip_left ──────────────────────────────────

#[test]
fn zip_left_keeps_left_value() {
  let mut parser = char('a').zip_left(char('b'));
  let mut input = StrInputStream::new("abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 'a');
  assert_eq!(input.offset(), 2);
}

#[test]
fn zip_left_fails_if_first_fails() {
  let mut parser = char('a').zip_left(char('b'));
  let mut input = StrInputStream::new("xyz");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn zip_left_fails_if_second_fails() {
  let mut parser = char('a').zip_left(char('b'));
  let mut input = StrInputStream::new("acd");

  assert!(parser.parse_next(&mut input).is_err());
}

#[test]
fn zip_left_propagates_cut() {
  let mut parser = char('a').zip_left(char('b').cut());
  let mut input = StrInputStream::new("ac");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

// ── zip_right ─────────────────────────────────

#[test]
fn zip_right_keeps_right_value() {
  let mut parser = char('a').zip_right(char('b'));
  let mut input = StrInputStream::new("abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');
  assert_eq!(input.offset(), 2);
}

#[test]
fn zip_right_fails_if_first_fails() {
  let mut parser = char('a').zip_right(char('b'));
  let mut input = StrInputStream::new("xyz");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
}

#[test]
fn zip_right_fails_if_second_fails() {
  let mut parser = char('a').zip_right(char('b'));
  let mut input = StrInputStream::new("acd");

  assert!(parser.parse_next(&mut input).is_err());
}

#[test]
fn zip_right_propagates_cut() {
  let mut parser = char('a').cut().zip_right(char('b'));
  let mut input = StrInputStream::new("xyz");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

// ── zip_right with tag ────────────────────────

#[test]
fn zip_right_with_tag_skips_prefix() {
  let mut parser = tag("key:").zip_right(tag("value"));
  let mut input = StrInputStream::new("key:value");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "value");
  assert_eq!(input.offset(), 9);
}

// ── between ───────────────────────────────────

#[test]
fn between_extracts_middle() {
  let mut parser = between(tag("("), tag("x"), tag(")"));
  let mut input = StrInputStream::new("(x)");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "x");
  assert_eq!(input.offset(), 3);
}

#[test]
fn between_fails_on_missing_open() {
  let mut parser = between(tag("("), tag("x"), tag(")"));
  let mut input = StrInputStream::new("x)");

  assert!(parser.parse_next(&mut input).is_err());
}

#[test]
fn between_fails_on_missing_close() {
  let mut parser = between(tag("("), tag("x"), tag(")"));
  let mut input = StrInputStream::new("(x]");

  assert!(parser.parse_next(&mut input).is_err());
}

// ── chaining ──────────────────────────────────

#[test]
fn zip_right_zip_left_chain_as_between() {
  // tag("(").zip_right(expr).zip_left(tag(")")) は between と同じ
  let mut parser = tag("(").zip_right(tag("hello")).zip_left(tag(")"));
  let mut input = StrInputStream::new("(hello)");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 7);
}
