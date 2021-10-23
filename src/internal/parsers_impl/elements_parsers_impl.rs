use crate::core::{ParseError, ParseResult, ParseState, Parser};
use crate::extension::parsers::ElementsParsers;
use crate::internal::ParsersImpl;
use regex::Regex;
use std::fmt::Debug;
use std::iter::FromIterator;

impl ElementsParsers for ParsersImpl {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut index = 0;
      loop {
        if index == tag.len() {
          return ParseResult::successful(tag, index);
        }
        if let Some(str) = input.get(index) {
          if tag[index] != *str {
            let msg = format!("seq {:?} expect: {:?}, found: {:?}", tag, tag[index], str);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
    })
  }

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a, {
    Parser::new(move |parse_state| {
      let input: &[char] = parse_state.input();
      let mut index = 0;
      for c in tag.chars() {
        if let Some(&actual) = input.get(index) {
          if c != actual {
            let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
      ParseResult::successful(tag, index)
    })
  }

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a, {
    Parser::new(move |parse_state: &ParseState<char>| {
      let input = parse_state.input();
      let mut index = 0;
      for c in tag.chars() {
        if let Some(actual) = input.get(index) {
          if !c.eq_ignore_ascii_case(actual) {
            let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
      ParseResult::successful(tag, index)
    })
  }

  fn regex<'a>(regex: Regex) -> Self::P<'a, char, String> {
    Parser::new(move |parse_state| {
      let input: &[char] = parse_state.input();
      let str = String::from_iter(input);
      if let Some(result) = regex.captures(&str).as_ref() {
        if let Some(sp) = result.at(0) {
          ParseResult::successful(sp.to_string(), sp.len())
        } else {
          let msg = format!("regex {:?} found: {:?}", regex, str);
          let pe = ParseError::of_mismatch(input, parse_state.next_offset(), msg);
          return ParseResult::failed_with_un_commit(pe);
        }
      } else {
        return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
      }
    })
  }
}
