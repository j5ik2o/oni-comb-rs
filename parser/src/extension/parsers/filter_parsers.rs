use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;

pub trait FilterParsers: Parsers {
  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a;
}

pub trait StaticFilterParsers: StaticParsers {
  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + 'static;
}

// 既存のParserを使用する関数
pub fn filter<'a, I, A, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, A>
where
  F: Fn(&A) -> bool + 'a + Clone,
  I: 'a + Clone,
  A: 'a + Clone, {
  Parser::new(move |state| {
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value,
        length: consumed,
      } => {
        if f(&value) {
          crate::core::ParseResult::successful(value, consumed)
        } else {
          crate::core::ParseResult::failed(
            crate::core::ParseError::of_custom(
              state.next_offset() + consumed,
              None,
              "Filter predicate failed".to_string(),
            ),
            crate::core::CommittedStatus::Uncommitted,
          )
        }
      }
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
  pub fn filter<'a, I, A, F>(parser: StaticParser<'a, I, A>, f: F) -> StaticParser<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + 'static, {
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => {
          if f(&value) {
            crate::core::ParseResult::successful(value, consumed)
          } else {
            crate::core::ParseResult::failed(
              crate::core::ParseError::of_custom(
                state.next_offset() + consumed,
                None,
                "Filter predicate failed".to_string(),
              ),
              crate::core::CommittedStatus::Uncommitted,
            )
          }
        }
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }
}
