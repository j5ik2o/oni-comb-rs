use std::fmt::Debug;

use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::extension::parsers::{OperatorParsers, StaticOperatorParsers};

pub trait SkipParsers: OperatorParsers {
  fn skip<'a, I: Clone>(n: usize) -> Self::P<'a, I, ()>;

  fn skip_left<'a, I: Clone, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    Self::map(Self::and_then(pa, pb), |(_, b)| b)
  }

  fn skip_right<'a, I: Clone, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    Self::map(Self::and_then(pa, pb), |(a, _)| a)
  }

  fn surround<'a, I: Clone, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    Self::skip_left(left_parser, Self::skip_right(parser, right_parser))
  }
}

pub trait StaticSkipParsers: StaticOperatorParsers {
  fn skip<'a, I: Clone>(n: usize) -> Self::P<'a, I, ()>;

  fn skip_left<'a, I: Clone, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    Self::map(Self::and_then(pa, pb), |(_, b)| b)
  }

  fn skip_right<'a, I: Clone, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    Self::map(Self::and_then(pa, pb), |(a, _)| a)
  }

  fn surround<'a, I: Clone, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static,
    C: Clone + Debug + 'a + 'static, {
    Self::skip_left(left_parser, Self::skip_right(parser, right_parser))
  }
}

// 既存のParserを使用する関数
pub fn skip<'a, I: Clone>(n: usize) -> Parser<'a, I, ()> {
  use crate::prelude::OperatorParser;
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

pub fn skip_left<'a, I: Clone, A, B>(pa: Parser<'a, I, A>, pb: Parser<'a, I, B>) -> Parser<'a, I, B>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::extension::parsers::operator_parsers::and_then;
  use crate::prelude::ParserRunner;

  Parser::new(move |state| {
    let result = and_then(pa.clone(), pb.clone()).run(state);
    match result {
      crate::core::ParseResult::Success { value: (_, b), length } => crate::core::ParseResult::successful(b, length),
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}

pub fn skip_right<'a, I: Clone, A, B>(pa: Parser<'a, I, A>, pb: Parser<'a, I, B>) -> Parser<'a, I, A>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::extension::parsers::operator_parsers::and_then;
  use crate::prelude::ParserRunner;

  Parser::new(move |state| {
    let result = and_then(pa.clone(), pb.clone()).run(state);
    match result {
      crate::core::ParseResult::Success { value: (a, _), length } => crate::core::ParseResult::successful(a, length),
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}

pub fn surround<'a, I: Clone, A, B, C>(
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

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn skip<'a, I: Clone>(n: usize) -> StaticParser<'a, I, ()> {
    StaticParser::new(move |state| {
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

  pub fn skip_left<'a, I: Clone, A, B>(pa: StaticParser<'a, I, A>, pb: StaticParser<'a, I, B>) -> StaticParser<'a, I, B>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    use crate::extension::parsers::operator_parsers::static_parsers::and_then;
    use crate::prelude::ParserRunner;

    StaticParser::new(move |state| {
      let result = and_then(pa.clone(), pb.clone()).run(state);
      match result {
        crate::core::ParseResult::Success { value: (_, b), length } => crate::core::ParseResult::successful(b, length),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn skip_right<'a, I: Clone, A, B>(pa: StaticParser<'a, I, A>, pb: StaticParser<'a, I, B>) -> StaticParser<'a, I, A>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    use crate::extension::parsers::operator_parsers::static_parsers::and_then;
    use crate::prelude::ParserRunner;

    StaticParser::new(move |state| {
      let result = and_then(pa.clone(), pb.clone()).run(state);
      match result {
        crate::core::ParseResult::Success { value: (a, _), length } => crate::core::ParseResult::successful(a, length),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn surround<'a, I: Clone, A, B, C>(
    left_parser: StaticParser<'a, I, A>,
    parser: StaticParser<'a, I, B>,
    right_parser: StaticParser<'a, I, C>,
  ) -> StaticParser<'a, I, B>
  where
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static,
    C: Clone + Debug + 'a + 'static, {
    use crate::extension::parsers::operator_parsers::static_parsers::and_then;
    use crate::prelude::ParserRunner;

    StaticParser::new(move |state| {
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
}
