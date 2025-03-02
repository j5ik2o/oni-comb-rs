use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;
use std::fmt::Debug;

pub trait DiscardParsers: Parsers {
  fn discard<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a;
}

pub trait StaticDiscardParsers: StaticParsers {
  fn discard<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn discard<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, ()>
where
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  <ParsersImpl as DiscardParsers>::discard(parser)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn discard<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, ()>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: _,
          length: consumed,
        } => crate::core::ParseResult::successful((), consumed),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }
}
