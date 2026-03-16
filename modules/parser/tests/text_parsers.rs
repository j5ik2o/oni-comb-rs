use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

// ── whitespace ────────────────────────────────

#[test]
fn whitespace0_matches_spaces() {
  let mut parser = whitespace0();
  let mut input = StrInput::new("   abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "   ");
  assert_eq!(input.offset(), 3);
}

#[test]
fn whitespace0_matches_empty() {
  let mut parser = whitespace0();
  let mut input = StrInput::new("abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "");
  assert_eq!(input.offset(), 0);
}

#[test]
fn whitespace0_matches_mixed() {
  let mut parser = whitespace0();
  let mut input = StrInput::new(" \t\n\rabc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), " \t\n\r");
  assert_eq!(input.offset(), 4);
}

#[test]
fn whitespace1_matches_spaces() {
  let mut parser = whitespace1();
  let mut input = StrInput::new("  abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "  ");
}

#[test]
fn whitespace1_fails_on_no_whitespace() {
  let mut parser = whitespace1();
  let mut input = StrInput::new("abc");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
}

// ── identifier ────────────────────────────────

#[test]
fn identifier_simple() {
  let mut parser = identifier();
  let mut input = StrInput::new("foo bar");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "foo");
  assert_eq!(input.offset(), 3);
}

#[test]
fn identifier_with_underscore_prefix() {
  let mut parser = identifier();
  let mut input = StrInput::new("_private");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "_private");
}

#[test]
fn identifier_with_digits() {
  let mut parser = identifier();
  let mut input = StrInput::new("foo_bar_123!");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "foo_bar_123");
  assert_eq!(input.offset(), 11);
}

#[test]
fn identifier_single_char() {
  let mut parser = identifier();
  let mut input = StrInput::new("x");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "x");
}

#[test]
fn identifier_fails_on_digit_start() {
  let mut parser = identifier();
  let mut input = StrInput::new("123abc");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn identifier_fails_on_empty() {
  let mut parser = identifier();
  let mut input = StrInput::new("");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
}

// ── integer ───────────────────────────────────

#[test]
fn integer_positive() {
  let mut parser = integer();
  let mut input = StrInput::new("42abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 42);
  assert_eq!(input.offset(), 2);
}

#[test]
fn integer_negative() {
  let mut parser = integer();
  let mut input = StrInput::new("-7 ");

  assert_eq!(parser.parse_next(&mut input).unwrap(), -7);
  assert_eq!(input.offset(), 2);
}

#[test]
fn integer_zero() {
  let mut parser = integer();
  let mut input = StrInput::new("0");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 0);
}

#[test]
fn integer_large() {
  let mut parser = integer();
  let mut input = StrInput::new("9999999");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 9999999);
}

#[test]
fn integer_fails_on_non_digit() {
  let mut parser = integer();
  let mut input = StrInput::new("abc");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
}

#[test]
fn integer_fails_on_lone_minus() {
  let mut parser = integer();
  let mut input = StrInput::new("-abc");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}
