use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// --- not ---

#[test]
fn not_succeeds_when_inner_fails() {
  let mut input = StrInput::new("bc");
  let result = sym('a').not().parse_next(&mut input);
  assert_eq!(result.unwrap(), ());
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn not_fails_when_inner_succeeds() {
  let mut input = StrInput::new("abc");
  let result = sym('a').not().parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
  assert_eq!(input.remaining(), "abc");
}

// --- peek ---

#[test]
fn peek_returns_output_without_consuming() {
  let mut input = StrInput::new("abc");
  let result = sym('a').peek().parse_next(&mut input);
  assert_eq!(result.unwrap(), 'a');
  assert_eq!(input.remaining(), "abc");
}

#[test]
fn peek_propagates_error() {
  let mut input = StrInput::new("xyz");
  let result = sym('a').peek().parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

// --- repeat ---

#[test]
fn repeat_0_or_more() {
  let mut input = StrInput::new("aaab");
  let result = sym('a').repeat(0..).parse_next(&mut input).unwrap();
  assert_eq!(result, vec!['a', 'a', 'a']);
  assert_eq!(input.remaining(), "b");
}

#[test]
fn repeat_1_or_more_fails_on_zero() {
  let mut input = StrInput::new("bbb");
  let result = sym('a').repeat(1..).parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

#[test]
fn repeat_range_inclusive() {
  let mut input = StrInput::new("aaa");
  let result = sym('a').repeat(2..=4).parse_next(&mut input).unwrap();
  assert_eq!(result, vec!['a', 'a', 'a']);
}

#[test]
fn repeat_range_inclusive_too_few() {
  let mut input = StrInput::new("ab");
  let result = sym('a').repeat(2..=4).parse_next(&mut input);
  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

#[test]
fn repeat_range_to() {
  let mut input = StrInput::new("aaaa");
  let result = sym('a').repeat(..3).parse_next(&mut input).unwrap();
  assert_eq!(result, vec!['a', 'a']);
  assert_eq!(input.remaining(), "aa");
}

#[test]
fn repeat_exact() {
  let mut input = StrInput::new("aaaa");
  let result = sym('a').repeat(3).parse_next(&mut input).unwrap();
  assert_eq!(result, vec!['a', 'a', 'a']);
  assert_eq!(input.remaining(), "a");
}

// --- collect ---

#[test]
fn collect_returns_slice() {
  let mut input = StrInput::new("abcdef");
  let result = sym('a')
    .zip(sym('b'))
    .zip(sym('c'))
    .collect()
    .parse_next(&mut input)
    .unwrap();
  assert_eq!(result, "abc");
  assert_eq!(input.remaining(), "def");
}

// --- discard ---

#[test]
fn discard_returns_unit() {
  let mut input = StrInput::new("aaa");
  let result = sym('a').repeat(0..).discard().parse_next(&mut input);
  assert_eq!(result.unwrap(), ());
}

// --- position ---

#[test]
fn position_returns_current_offset() {
  let mut input = StrInput::new("abcdef");
  sym('a').parse_next(&mut input).unwrap();
  sym('b').parse_next(&mut input).unwrap();
  let pos = position().parse_next(&mut input).unwrap();
  assert_eq!(pos.offset, 2);
  assert_eq!(pos.line, 1);
  assert_eq!(pos.column, 3);
  assert_eq!(input.remaining(), "cdef");
}
