use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::{char, tag};
use oni_comb_parser::str_input_stream::StrInputStream;

#[test]
fn optional_returns_some_on_success() {
  let mut parser = char('a').optional();
  let mut input = StrInputStream::new("abc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(Some('a')));
  assert_eq!(input.offset(), 1);
}

#[test]
fn optional_returns_none_on_backtrack() {
  let mut parser = char('a').optional();
  let mut input = StrInputStream::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(None));
  assert_eq!(input.offset(), 0);
}

#[test]
fn optional_propagates_cut() {
  let inner = char('a').zip(char('b').cut());
  let mut parser = inner.optional();
  let mut input = StrInputStream::new("ac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn optional_returns_none_on_empty_input() {
  let mut parser = char('a').optional();
  let mut input = StrInputStream::new("");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(None));
  assert_eq!(input.offset(), 0);
}

#[test]
fn many0_collects_matching_items() {
  let mut parser = char('a').many0();
  let mut input = StrInputStream::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a']));
  assert_eq!(input.offset(), 3);
  assert_eq!(input.remaining(), "b");
}

#[test]
fn many0_returns_empty_vec_on_immediate_backtrack() {
  let mut parser = char('a').many0();
  let mut input = StrInputStream::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec![]));
  assert_eq!(input.offset(), 0);
}

#[test]
fn many0_succeeds_with_empty_vec_on_empty_input() {
  let mut parser = char('a').many0();
  let mut input = StrInputStream::new("");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec![]));
}

#[test]
fn many0_consumes_all_matching() {
  let mut parser = char('a').many0();
  let mut input = StrInputStream::new("aaaa");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a', 'a']));
  assert!(input.is_eof());
}

#[test]
fn many0_propagates_cut() {
  let item = char('a').zip(char('b').cut());
  let mut parser = item.many0();
  let mut input = StrInputStream::new("abac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn many0_with_tags_collects_strings() {
  let mut parser = tag("ab").many0();
  let mut input = StrInputStream::new("ababc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!["ab", "ab"]));
  assert_eq!(input.offset(), 4);
  assert_eq!(input.remaining(), "c");
}

#[test]
fn many0_with_or_collects_alternatives() {
  let mut parser = char('a').or(char('b')).many0();
  let mut input = StrInputStream::new("abba!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'b', 'b', 'a']));
  assert_eq!(input.offset(), 4);
}

#[test]
fn optional_after_many0() {
  let mut parser = char('a').many0().zip(char('!').optional());
  let mut input = StrInputStream::new("aaa");

  let result = parser.parse_next(&mut input);

  let (items, bang) = result.unwrap();
  assert_eq!(items, vec!['a', 'a', 'a']);
  assert_eq!(bang, None);
}

#[test]
fn optional_after_many0_with_trailing() {
  let mut parser = char('a').many0().zip(char('!').optional());
  let mut input = StrInputStream::new("aaa!");

  let result = parser.parse_next(&mut input);

  let (items, bang) = result.unwrap();
  assert_eq!(items, vec!['a', 'a', 'a']);
  assert_eq!(bang, Some('!'));
}

#[test]
fn many0_with_map_transforms_collected() {
  let mut parser = char('a').many0().map(|items: Vec<char>| items.len());
  let mut input = StrInputStream::new("aaabc");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(3));
}

#[test]
fn many0_detects_zero_progress() {
  let mut parser = tag("").many0();
  let mut input = StrInputStream::new("anything");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}

struct ZeroProgressParser;

impl Parser<StrInputStream<'_>> for ZeroProgressParser {
  type Error = ParseError;
  type Output = char;

  fn parse_next(&mut self, _input: &mut StrInputStream<'_>) -> PResult<Self::Output, Self::Error> {
    Err(Fail::ZeroProgress)
  }
}

#[test]
fn optional_propagates_zero_progress() {
  let mut parser = ZeroProgressParser.optional();
  let mut input = StrInputStream::new("abc");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomError(u32);

struct AlwaysSucceedNoConsume;

impl Parser<StrInputStream<'_>> for AlwaysSucceedNoConsume {
  type Error = CustomError;
  type Output = ();

  fn parse_next(&mut self, _input: &mut StrInputStream<'_>) -> PResult<Self::Output, Self::Error> {
    Ok(())
  }
}

#[test]
fn many0_works_with_non_string_error_type() {
  let mut parser = AlwaysSucceedNoConsume.many0();
  let mut input = StrInputStream::new("abc");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}
