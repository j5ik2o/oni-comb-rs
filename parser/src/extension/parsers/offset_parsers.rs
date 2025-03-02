use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;
use std::fmt::Debug;

pub trait OffsetParsers: Parsers {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a;

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a;
}

pub trait StaticOffsetParsers: StaticParsers {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a + 'static;

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn last_offset<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, usize>
where
  A: Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value: _,
        length: consumed,
      } => crate::core::ParseResult::successful(state.next_offset(), consumed),
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}

pub fn next_offset<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, usize>
where
  A: Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value: _,
        length: consumed,
      } => crate::core::ParseResult::successful(state.next_offset() + consumed, consumed),
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
  pub fn last_offset<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, usize>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: _,
          length: consumed,
        } => crate::core::ParseResult::successful(state.next_offset(), consumed),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn next_offset<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, usize>
  where
    A: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value: _,
          length: consumed,
        } => crate::core::ParseResult::successful(state.next_offset() + consumed, consumed),
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }
}
