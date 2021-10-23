use crate::core::ElementsParsers;
use crate::core::{Element, ParseError, ParseResult, ParseState, Parser};
use crate::internal::ParsersImpl;
use crate::utils::Set;
use regex::Regex;
use std::fmt::{Debug, Display};
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

  fn take<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful(parse_state.slice_with_len(n), n)
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn take_while0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut start: Option<usize> = None;
      let mut len = 0;
      let mut index = 0;
      while let Some(c) = input.get(index) {
        if f(c) {
          if start.is_none() {
            start = Some(index);
          }
          len += 1;
        }
        index += 1;
      }
      match start {
        Some(s) => ParseResult::successful(&input[s..s + len], 0),
        None => ParseResult::successful(parse_state.slice_with_len(0), 0),
      }
    })
  }

  fn take_while1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut start: Option<usize> = None;
      let mut len = 0;
      let mut index = 0;
      while let Some(c) = input.get(index) {
        if f(c) {
          if start.is_none() {
            start = Some(index);
          }
          len += 1;
        }
        index += 1;
      }
      match start {
        Some(s) => ParseResult::successful(&input[s..s + len], 0),
        None => ParseResult::failed_with_un_commit(ParseError::of_in_complete()),
      }
    })
  }

  fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut start: Option<usize> = None;
      let mut len = 0;
      let mut index = 0;
      while let Some(c) = input.get(index) {
        if f(c) {
          if start.is_none() {
            start = Some(index);
          }
          len += 1;
        }
        index += 1;
      }
      match start {
        Some(s) => {
          let str = &input[s..s + len];
          if n <= str.len() && str.len() <= m {
            ParseResult::successful(str, 0)
          } else {
            ParseResult::failed_with_un_commit(ParseError::of_in_complete())
          }
        }
        None => ParseResult::failed_with_un_commit(ParseError::of_in_complete()),
      }
    })
  }

  fn take_till0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut index = 0;
      let mut b = false;
      while let Some(c) = input.get(index) {
        if f(c) {
          b = true;
          break;
        }
        index += 1;
      }
      if !b {
        ParseResult::successful(parse_state.slice_with_len(0), 0)
      } else {
        ParseResult::successful(parse_state.slice_with_len(index + 1), index + 1)
      }
    })
  }

  fn take_till1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut index = 0;
      let mut b = false;
      while let Some(c) = input.get(index) {
        if f(c) {
          b = true;
          break;
        }
        index += 1;
      }
      if !b {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      } else {
        ParseResult::successful(parse_state.slice_with_len(index + 1), index + 1)
      }
    })
  }

  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful((), n)
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn elm_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect one of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    Parser::new(move |parse_state| {
      let set = start..=end;
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    Parser::new(move |parse_state| {
      let set = start..end;
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn none_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if !set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect none of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
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
