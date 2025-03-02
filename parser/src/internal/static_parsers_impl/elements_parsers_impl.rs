use crate::core::{Element, ParseError, ParseResult, StaticParser};
use crate::extension::parsers::StaticElementsParsers;
use crate::internal::static_parsers_impl::StaticParsersImpl;
use regex::Regex;
use std::fmt::Debug;
use std::str;

impl StaticElementsParsers for StaticParsersImpl {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, Vec<I>>
  where
    I: crate::core::Element + PartialEq + Debug + Clone + 'a + 'static,
    'b: 'a, {
    let tag_len = tag.len();
    let tag = tag.to_vec();
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.len() < tag_len {
        return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
          input,
          state.next_offset(),
          0,
          format!("expect {:?}, but input is too short", tag),
        ));
      }
      for i in 0..tag_len {
        if input[i] != tag[i] {
          return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
            input,
            state.next_offset(),
            i,
            format!("expect {:?}, but found {:?}", tag, &input[0..tag_len]),
          ));
        }
      }
      ParseResult::successful(input[0..tag_len].to_vec(), tag_len)
    })
  }

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a, {
    let tag_chars: Vec<char> = tag.chars().collect();
    let tag_len = tag_chars.len();
    let tag_string = tag.to_string();
    StaticParser::new(move |state| {
      let input: &[char] = state.input();
      if input.len() < tag_len {
        return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
          input,
          state.next_offset(),
          0,
          format!("expect {:?}, but input is too short", tag_string),
        ));
      }
      for i in 0..tag_len {
        if input[i] != tag_chars[i] {
          return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
            input,
            state.next_offset(),
            i,
            format!("expect {:?}, but found {:?}", tag_string, &input[0..tag_len]),
          ));
        }
      }
      ParseResult::successful(tag_string.clone(), tag_len)
    })
  }

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a, {
    let tag_chars: Vec<char> = tag.chars().collect();
    let tag_len = tag_chars.len();
    let tag_string = tag.to_string();
    StaticParser::new(move |state| {
      let input: &[char] = state.input();
      if input.len() < tag_len {
        return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
          input,
          state.next_offset(),
          0,
          format!("expect {:?}, but input is too short", tag_string),
        ));
      }
      for i in 0..tag_len {
        if input[i].to_lowercase().next() != tag_chars[i].to_lowercase().next() {
          return ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
            input,
            state.next_offset(),
            i,
            format!("expect {:?}, but found {:?}", tag_string, &input[0..tag_len]),
          ));
        }
      }
      ParseResult::successful(tag_string.clone(), tag_len)
    })
  }

  fn regex<'a>(pattern: &'a str) -> Self::P<'a, char, String> {
    let re = Regex::new(pattern).unwrap();
    StaticParser::new(move |state| {
      let input: &[char] = state.input();
      let input_str: String = input.iter().collect();
      if let Some(m) = re.find(&input_str) {
        if m.start() == 0 {
          let matched_str = m.as_str().to_string();
          let matched_len = matched_str.chars().count();
          ParseResult::successful(matched_str, matched_len)
        } else {
          ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
            input,
            state.next_offset(),
            0,
            format!("regex pattern {} not matched at start", pattern),
          ))
        }
      } else {
        ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
          input,
          state.next_offset(),
          0,
          format!("regex pattern {} not matched", pattern),
        ))
      }
    })
  }
}
