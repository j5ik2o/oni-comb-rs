use crate::core::{Element, ParseError, ParseResult, Parser, TakenParsers};
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl TakenParsers for ParsersImpl {
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
}
