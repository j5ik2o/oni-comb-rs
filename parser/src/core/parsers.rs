use crate::core::parser_monad::ParserMonad;
use crate::core::{CommittedStatus, ParseError, Parser, StaticParser};

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
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + Clone;

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a + Clone,
    A: 'a,
    B: 'a + Clone;

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: 'a,
    B: Clone + 'a;
}

/// 静的ディスパッチを使用したパーサー関数を提供するトレイト
pub trait StaticParsers {
  type P<'p, I, A>: ParserMonad<'p, Input = I, Output = A>
  where
    I: 'p,
    A: 'p + 'static;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a + 'static,
    'b: 'a;

  fn unit<'a, I>() -> Self::P<'a, I, ()> {
    Self::successful(())
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a + 'static;

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a + 'static;

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a + 'static;

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a + 'static;

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + 'static + Clone;

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a + Clone,
    A: 'a + 'static,
    B: 'a + 'static + Clone;

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: 'a + 'static,
    B: Clone + 'a + 'static;
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
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + Clone, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| {
      if f(&a) {
        Self::successful(a)
      } else {
        Self::failed(
          crate::core::ParseError::of_custom(0, None, "Filter predicate failed".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a + Clone,
    A: 'a,
    B: 'a + Clone, {
    parser.flat_map(f)
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: 'a,
    B: Clone + 'a, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| Self::successful(f(a)))
  }
}

/// StaticParserを使用するParsers実装
pub struct StaticParserParsers;

impl StaticParsers for StaticParserParsers {
  type P<'p, I, A>
    = StaticParser<'p, I, A>
  where
    I: 'p,
    A: 'p + 'static;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a + 'static,
    'b: 'a, {
    use crate::core::parser_runner::ParserRunner;
    parser
      .parse(input)
      .success()
      .ok_or_else(|| ParseError::of_custom(0, None, "Parse failed".to_string()))
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a + 'static, {
    StaticParser::new(move |_| crate::core::ParseResult::successful(value.clone(), 0))
  }

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| crate::core::ParseResult::successful(value(), 0))
  }

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| crate::core::ParseResult::failed(value.clone(), committed))
  }

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a + 'static, {
    StaticParser::new(move |_| {
      let (error, committed) = f();
      crate::core::ParseResult::failed(error, committed)
    })
  }

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + 'static + Clone, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| {
      if f(&a) {
        Self::successful(a)
      } else {
        Self::failed(
          crate::core::ParseError::of_custom(0, None, "Filter predicate failed".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a + Clone,
    A: 'a + 'static,
    B: 'a + 'static + Clone, {
    parser.flat_map(f)
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a + Clone,
    A: 'a + 'static,
    B: Clone + 'a + 'static, {
    // 直接実装を使用
    Self::flat_map(parser, move |a| Self::successful(f(a)))
  }
}
