use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::str_input::StrInput;
use oni_comb_parser::text::char::Char;
use oni_comb_parser::text::tag::Tag;

#[test]
fn optional_returns_some_on_success() {
    let mut parser = Char('a').optional();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(Some('a')));
    assert_eq!(input.offset(), 1);
}

#[test]
fn optional_returns_none_on_backtrack() {
    let mut parser = Char('a').optional();
    let mut input = StrInput::new("xyz");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(None));
    assert_eq!(input.offset(), 0);
}

#[test]
fn optional_propagates_cut() {
    let inner = Char('a').then(Char('b').cut());
    let mut parser = inner.optional();
    let mut input = StrInput::new("ac");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn optional_returns_none_on_empty_input() {
    let mut parser = Char('a').optional();
    let mut input = StrInput::new("");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(None));
    assert_eq!(input.offset(), 0);
}

#[test]
fn many0_collects_matching_items() {
    let mut parser = Char('a').many0();
    let mut input = StrInput::new("aaab");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec!['a', 'a', 'a']));
    assert_eq!(input.offset(), 3);
    assert_eq!(input.remaining(), "b");
}

#[test]
fn many0_returns_empty_vec_on_immediate_backtrack() {
    let mut parser = Char('a').many0();
    let mut input = StrInput::new("xyz");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec![]));
    assert_eq!(input.offset(), 0);
}

#[test]
fn many0_succeeds_with_empty_vec_on_empty_input() {
    let mut parser = Char('a').many0();
    let mut input = StrInput::new("");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec![]));
}

#[test]
fn many0_consumes_all_matching() {
    let mut parser = Char('a').many0();
    let mut input = StrInput::new("aaaa");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec!['a', 'a', 'a', 'a']));
    assert!(input.is_eof());
}

#[test]
fn many0_propagates_cut() {
    let item = Char('a').then(Char('b').cut());
    let mut parser = item.many0();
    let mut input = StrInput::new("abac");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn many0_with_tags_collects_strings() {
    let mut parser = Tag("ab").many0();
    let mut input = StrInput::new("ababc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec!["ab", "ab"]));
    assert_eq!(input.offset(), 4);
    assert_eq!(input.remaining(), "c");
}

#[test]
fn many0_with_or_collects_alternatives() {
    let mut parser = Char('a').or(Char('b')).many0();
    let mut input = StrInput::new("abba!");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec!['a', 'b', 'b', 'a']));
    assert_eq!(input.offset(), 4);
}

#[test]
fn optional_after_many0() {
    let mut parser = Char('a').many0().then(Char('!').optional());
    let mut input = StrInput::new("aaa");

    let result = parser.parse_next(&mut input);

    let (items, bang) = result.unwrap();
    assert_eq!(items, vec!['a', 'a', 'a']);
    assert_eq!(bang, None);
}

#[test]
fn optional_after_many0_with_trailing() {
    let mut parser = Char('a').many0().then(Char('!').optional());
    let mut input = StrInput::new("aaa!");

    let result = parser.parse_next(&mut input);

    let (items, bang) = result.unwrap();
    assert_eq!(items, vec!['a', 'a', 'a']);
    assert_eq!(bang, Some('!'));
}

#[test]
fn many0_with_map_transforms_collected() {
    let mut parser = Char('a').many0().map(|items: Vec<char>| items.len());
    let mut input = StrInput::new("aaabc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(3));
}

#[test]
fn many0_detects_zero_progress() {
    let mut parser = Tag("").many0();
    let mut input = StrInput::new("anything");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::ZeroProgress)));
}

struct ZeroProgressParser;

impl Parser<StrInput<'_>> for ZeroProgressParser {
    type Output = char;
    type Error = String;

    fn parse_next(&mut self, _input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        Err(Fail::ZeroProgress)
    }
}

#[test]
fn optional_propagates_zero_progress() {
    let mut parser = ZeroProgressParser.optional();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::ZeroProgress)));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomError(u32);

struct AlwaysSucceedNoConsume;

impl Parser<StrInput<'_>> for AlwaysSucceedNoConsume {
    type Output = ();
    type Error = CustomError;

    fn parse_next(&mut self, _input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        Ok(())
    }
}

#[test]
fn many0_works_with_non_string_error_type() {
    let mut parser = AlwaysSucceedNoConsume.many0();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::ZeroProgress)));
}
