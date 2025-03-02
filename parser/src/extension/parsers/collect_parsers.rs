use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;
use std::fmt::Debug;

pub trait CollectParsers: Parsers {
  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: Debug + 'a;
}

pub trait StaticCollectParsers: StaticParsers {
  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: Debug + Clone + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn collect<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, &'a [I]>
where
  A: Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let input = state.input();
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value: _,
        length: consumed,
      } => crate::core::ParseResult::successful(&input[..consumed], consumed),
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
  pub fn collect<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, &'a [I]>
  where
    A: Debug + Clone + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let input = state.input();
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: _,
          length: consumed,
        } => crate::core::ParseResult::successful(&input[..consumed], consumed),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }
}
