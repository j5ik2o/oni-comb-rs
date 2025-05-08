use std::fmt::Debug;

use crate::core::{ParseResult, Parser, StaticParser};
use crate::extension::parsers::{OperatorParsers, StaticOperatorParsers};

pub trait SkipParsers: OperatorParsers {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()>;

  fn skip_left<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn skip_right<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn surround<'a, I, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a;
   
}

pub fn skip<'a, I>(n: usize) -> Parser<'a, I, ()> {
  Parser::new(move |state| {
    let input = state.input();
    if input.len() >= n {
      crate::core::ParseResult::successful((), n)
    } else {
      crate::core::ParseResult::failed(
        crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
        crate::core::CommittedStatus::Uncommitted,
      )
    }
  })
}

pub fn skip_left<'a, I, A, B>(pa: Parser<'a, I, A>, pb: Parser<'a, I, B>) -> Parser<'a, I, B>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::extension::parsers::operator_parsers::and_then;
  use crate::prelude::ParserRunner;

  Parser::new(move |state| {
    let result = and_then(pa.clone(), pb.clone()).run(state);
    match result {
      ParseResult::Success { value: (_, b), length } => ParseResult::successful(b, length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  })
}

pub fn skip_right<'a, I, A, B>(pa: Parser<'a, I, A>, pb: Parser<'a, I, B>) -> Parser<'a, I, A>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::extension::parsers::operator_parsers::and_then;
  use crate::prelude::ParserRunner;

  Parser::new(move |state| {
    let result = and_then(pa.clone(), pb.clone()).run(state);
    match result {
      ParseResult::Success { value: (a, _), length } => ParseResult::successful(a, length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  })
}

pub fn surround<'a, I, A, B, C>(
  left_parser: Parser<'a, I, A>,
  parser: Parser<'a, I, B>,
  right_parser: Parser<'a, I, C>,
) -> Parser<'a, I, B>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a,
  C: Clone + Debug + 'a, {
  use crate::extension::parsers::operator_parsers::and_then;
  use crate::prelude::ParserRunner;

  Parser::new(move |state| {
    let left_and_parser = and_then(left_parser.clone(), parser.clone());
    let left_and_parser_and_right = and_then(left_and_parser, right_parser.clone());
    let result = left_and_parser_and_right.run(state);
    match result {
      crate::core::ParseResult::Success {
        value: ((_, p), _),
        length,
      } => crate::core::ParseResult::successful(p, length),
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}
