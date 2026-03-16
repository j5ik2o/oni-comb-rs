use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::{char, tag};
use oni_comb_parser::str_input::StrInput;

struct ZeroProgressParser;

impl Parser<StrInput<'_>> for ZeroProgressParser {
  type Error = ParseError;
  type Output = char;

  fn parse_next(&mut self, _input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
    Err(Fail::ZeroProgress)
  }
}

#[test]
fn or_returns_left_on_left_success() {
  let mut parser = char('a').or(char('b'));
  let mut input = StrInput::new("abc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('a'));
  assert_eq!(input.offset(), 1);
}

#[test]
fn or_returns_right_when_left_backtracks() {
  let mut parser = char('a').or(char('b'));
  let mut input = StrInput::new("bcd");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('b'));
  assert_eq!(input.offset(), 1);
}

#[test]
fn or_fails_when_both_sides_backtrack() {
  let mut parser = char('a').or(char('b'));
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn or_propagates_cut_from_left_without_trying_right() {
  let left = char('a').zip(char('b').cut());
  let right = char('a').zip(char('c'));
  let mut parser = left.or(right);
  let mut input = StrInput::new("ac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn or_rewinds_input_on_left_backtrack() {
  let mut parser = tag("abc").or(tag("abd"));
  let mut input = StrInput::new("abd");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok("abd"));
  assert_eq!(input.offset(), 3);
}

#[test]
fn attempt_passes_through_success() {
  let mut parser = char('a').attempt();
  let mut input = StrInput::new("abc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('a'));
  assert_eq!(input.offset(), 1);
}

#[test]
fn attempt_downgrades_cut_to_backtrack() {
  let inner = char('a').zip(char('b').cut());
  let mut parser = inner.attempt();
  let mut input = StrInput::new("ac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn attempt_passes_through_backtrack() {
  let mut parser = char('a').attempt();
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn attempt_enables_backtracking_in_or() {
  let left = char('a').zip(char('b').cut()).attempt();
  let right = char('a').zip(char('c'));
  let mut parser = left.or(right);
  let mut input = StrInput::new("ac");

  let result = parser.parse_next(&mut input);

  assert!(result.is_ok());
  let (a, c) = result.unwrap();
  assert_eq!(a, 'a');
  assert_eq!(c, 'c');
}

#[test]
fn cut_passes_through_success() {
  let mut parser = char('a').cut();
  let mut input = StrInput::new("abc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok('a'));
}

#[test]
fn cut_upgrades_backtrack_to_cut() {
  let mut parser = char('a').cut();
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn cut_after_tag_prevents_or_fallthrough() {
  let left = tag(":").zip(tag("value").cut());
  let right = tag(":").zip(tag("other"));
  let mut parser = left.or(right);
  let mut input = StrInput::new(":other");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn or_propagates_zero_progress_from_left() {
  let mut parser = ZeroProgressParser.or(char('b'));
  let mut input = StrInput::new("abc");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}

#[test]
fn attempt_propagates_zero_progress() {
  let mut parser = ZeroProgressParser.attempt();
  let mut input = StrInput::new("abc");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}
