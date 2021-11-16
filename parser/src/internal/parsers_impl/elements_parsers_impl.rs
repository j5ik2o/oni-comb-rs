use crate::core::{ParseError, ParseResult, ParseState, Parser};
use crate::extension::parsers::ElementsParsers;
use crate::internal::ParsersImpl;
use regex::Regex;
use std::fmt::Debug;
use std::iter::FromIterator;

impl ElementsParsers for ParsersImpl {
  fn seq<'a, 'b, I>(seq: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut index = 0;
      loop {
        if index == seq.len() {
          return ParseResult::successful(seq, index);
        }
        if let Some(str) = input.get(index) {
          if seq[index] != *str {
            let msg = format!("seq {:?} expect: {:?}, found: {:?}", seq, seq[index], str);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), index, msg);
            return ParseResult::failed(pe, (index != 0).into());
          }
        } else {
          return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
        }
        index += 1;
      }
    })
  }

  #[inline(always)]
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
            let pe = ParseError::of_mismatch(input, ps.next_offset(), index, msg);
            return ParseResult::failed(pe, (index != 0).into());
          }
        } else {
          return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
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
            let msg = format!("tag_no_case {:?} expect: {:?}, found: {}", tag, c, actual);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), index, msg);
            return ParseResult::failed(pe, (index != 0).into());
          }
        } else {
          return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
        }
        index += 1;
      }
      ParseResult::successful(tag, index)
    })
  }

  fn regex<'a>(pattern: &str) -> Self::P<'a, char, String> {
    let pattern = if !pattern.starts_with("^") {
      format!("^{}", pattern)
    } else {
      pattern.to_string()
    };
    let regex = Regex::new(&pattern).unwrap();
    Parser::new(move |parse_state| {
      let input: &[char] = parse_state.input();
      log::debug!("regex: input = {:?}", input);
      let str = String::from_iter(input);
      if let Some(captures) = regex.captures(&str).as_ref() {
        if let Some(m) = captures.get(0) {
          let str = m.as_str();
          ParseResult::successful(str.to_string(), str.len())
        } else {
          let msg = format!("regex {:?} found: {:?}", regex, str);
          let pe = ParseError::of_mismatch(input, parse_state.next_offset(), str.len(), msg);
          return ParseResult::failed(pe, (captures.len() != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(ParseError::of_in_complete());
      }
    })
  }
}
