use crate::core::{ParseError, ParseResult, Parser};
use crate::extension::parser::SkipParser;
use crate::extension::parsers::SkipParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl SkipParsers for ParsersImpl {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful((), n)
      } else {
        ParseResult::failed_with_uncommitted(ParseError::of_in_complete())
      }
    })
  }

  fn skip_left<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method1 = pa.method.clone();
    let method2 = pb.method.clone();
    Parser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { length: n1, .. } => {
        let ps = parse_state.add_offset(n1);
        match (method2)(&ps) {
          ParseResult::Success { value: b, length: n2 } => ParseResult::successful(b, n1 + n2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status).advance_success(n1),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn skip_right<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method1 = pa.method.clone();
    let method2 = pb.method.clone();

    Parser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { value: a, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        match (method2)(&ps) {
          ParseResult::Success { length: n2, .. } => ParseResult::successful(a, n1 + n2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status).advance_success(n1),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn surround<'a, I, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    left_parser.skip_left(parser.skip_right(right_parser))
  }
}
