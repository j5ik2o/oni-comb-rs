use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn lexeme_consumes_trailing_whitespace() {
  let mut parser = lexeme(tag("hello"));
  let mut input = StrInput::new("hello   world");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 8); // "hello" + "   "
  assert_eq!(input.remaining(), "world");
}

#[test]
fn lexeme_works_without_trailing_whitespace() {
  let mut parser = lexeme(tag("hello"));
  let mut input = StrInput::new("helloworld");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 5);
}

#[test]
fn lexeme_with_integer() {
  let mut parser = lexeme(integer());
  let mut input = StrInput::new("42  +");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 42);
  assert_eq!(input.remaining(), "+");
}

#[test]
fn lexeme_chain_for_tokens() {
  // "1 + 2" をトークンとしてパース
  let mut parser = lexeme(integer()).zip_left(lexeme(char('+'))).zip(lexeme(integer()));
  let mut input = StrInput::new("1 + 2");

  let (a, b) = parser.parse_next(&mut input).unwrap();
  assert_eq!(a, 1);
  assert_eq!(b, 2);
}

#[test]
fn lexeme_with_tabs_and_newlines() {
  let mut parser = lexeme(tag("key"));
  let mut input = StrInput::new("key\t\n  value");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "key");
  assert_eq!(input.remaining(), "value");
}
