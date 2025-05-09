use crate::core::ParserFunctor;
use crate::core::ParserMonad;
use crate::core::{CommittedStatus, ParseError, Parser};
use crate::prelude::Element;

/// パーサー関数を提供するトレイト
pub trait Parsers {
  type P<'p, I, A>: ParserMonad<'p, Input = I, Output = A>
  where
    I: 'p,
    A: 'p;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a,
    'b: 'a;

  fn unit<'a, I>() -> Self::P<'a, I, ()> {
    Self::successful(())
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a;

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a;

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a;

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a;

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a,
    I: Element,
    A: Clone + 'a;

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a;

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a;
}

/// 既存のParserを使用するParsers実装
pub struct ParserParsers;

impl Parsers for ParserParsers {
  type P<'p, I, A>
    = Parser<'p, I, A>
  where
    I: 'p,
    A: 'p;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a,
    'b: 'a, {
    use crate::core::parser_runner::ParserRunner;
    parser
      .parse(input)
      .success()
      .ok_or_else(|| ParseError::of_custom(0, None, "Parse failed".to_string()))
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a, {
    Parser::new(move |_| crate::core::ParseResult::successful(value.clone(), 0))
  }

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a, {
    Parser::new(move |_| crate::core::ParseResult::successful(value(), 0))
  }

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    Parser::new(move |_| crate::core::ParseResult::failed(value.clone(), committed))
  }

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    Parser::new(move |_| {
      let (error, committed) = f();
      crate::core::ParseResult::failed(error, committed)
    })
  }

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a,
    I: Element,
    A: Clone + 'a, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| {
      if f(&a) {
        Self::successful(a)
      } else {
        Self::failed(
          ParseError::of_custom(0, None, "Filter predicate failed".to_string()),
          CommittedStatus::Uncommitted,
        )
      }
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    parser.flat_map(f)
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a, {
    parser.map(f)
  }
}
