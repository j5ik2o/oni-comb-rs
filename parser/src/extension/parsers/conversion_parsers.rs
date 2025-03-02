use std::fmt::Debug;
use std::str::FromStr;

use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;

pub trait ConversionParsers: Parsers {
  fn map_res<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: Debug + 'a,
    B: Debug + 'a;

  fn map_opt<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: Debug + 'a,
    B: Debug + 'a;

  fn convert_from_bytes_to_str<'a, I>(parser: Self::P<'a, I, &'a [u8]>) -> Self::P<'a, I, &'a str> {
    Self::map_res(parser, std::str::from_utf8)
  }

  fn convert_from_str_to_f64<'a, I>(parser: Self::P<'a, I, &'a str>) -> Self::P<'a, I, f64> {
    Self::map_res(parser, f64::from_str)
  }
}

pub trait StaticConversionParsers: StaticParsers {
  fn map_res<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: Debug + 'a + 'static,
    B: Debug + 'a + 'static;

  fn map_opt<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: Debug + 'a + 'static,
    B: Debug + 'a + 'static;

  fn convert_from_bytes_to_str<'a, I>(parser: Self::P<'a, I, &'a [u8]>) -> Self::P<'a, I, &'a str> {
    Self::map_res(parser, std::str::from_utf8)
  }

  fn convert_from_str_to_f64<'a, I>(parser: Self::P<'a, I, &'a str>) -> Self::P<'a, I, f64> {
    Self::map_res(parser, f64::from_str)
  }
}

// 既存のParserを使用する関数
pub fn map_res<'a, I, A, B, E, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, B>
where
  F: Fn(A) -> Result<B, E> + 'a,
  E: Debug,
  A: Debug + 'a,
  B: Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value,
        length: consumed,
      } => match f(value) {
        Ok(mapped) => crate::core::ParseResult::successful(mapped, consumed),
        Err(_) => crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        ),
      },
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}

pub fn map_opt<'a, I, A, B, F>(parser: Parser<'a, I, A>, f: F) -> Parser<'a, I, B>
where
  F: Fn(A) -> Option<B> + 'a,
  A: Debug + 'a,
  B: Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let result = parser.run(state);
    match result {
      crate::core::ParseResult::Success {
        value,
        length: consumed,
      } => match f(value) {
        Some(mapped) => crate::core::ParseResult::successful(mapped, consumed),
        None => crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        ),
      },
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    }
  })
}

pub fn convert_from_bytes_to_str<'a, I>(parser: Parser<'a, I, &'a [u8]>) -> Parser<'a, I, &'a str> {
  map_res(parser, std::str::from_utf8)
}

pub fn convert_from_str_to_f64<'a, I>(parser: Parser<'a, I, &'a str>) -> Parser<'a, I, f64> {
  map_res(parser, f64::from_str)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn map_res<'a, I, A, B, E, F>(parser: StaticParser<'a, I, A>, f: F) -> StaticParser<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: Debug + 'a + 'static,
    B: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => match f(value) {
          Ok(mapped) => crate::core::ParseResult::successful(mapped, consumed),
          Err(_) => crate::core::ParseResult::failed(
            crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
            crate::core::CommittedStatus::Uncommitted,
          ),
        },
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn map_opt<'a, I, A, B, F>(parser: StaticParser<'a, I, A>, f: F) -> StaticParser<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: Debug + 'a + 'static,
    B: Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => match f(value) {
          Some(mapped) => crate::core::ParseResult::successful(mapped, consumed),
          None => crate::core::ParseResult::failed(
            crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
            crate::core::CommittedStatus::Uncommitted,
          ),
        },
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn convert_from_bytes_to_str<'a, I>(parser: StaticParser<'a, I, &'a [u8]>) -> StaticParser<'a, I, &'a str> {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => match std::str::from_utf8(value) {
          Ok(mapped) => crate::core::ParseResult::successful(mapped, consumed),
          Err(_) => crate::core::ParseResult::failed(
            crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
            crate::core::CommittedStatus::Uncommitted,
          ),
        },
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }

  pub fn convert_from_str_to_f64<'a, I>(parser: StaticParser<'a, I, &'a str>) -> StaticParser<'a, I, f64> {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match result {
        crate::core::ParseResult::Success {
          value,
          length: consumed,
        } => match f64::from_str(value) {
          Ok(mapped) => crate::core::ParseResult::successful(mapped, consumed),
          Err(_) => crate::core::ParseResult::failed(
            crate::core::ParseError::of_custom(state.next_offset() + consumed, None, "Conversion failed".to_string()),
            crate::core::CommittedStatus::Uncommitted,
          ),
        },
        crate::core::ParseResult::Failure {
          error,
          committed_status,
        } => crate::core::ParseResult::failed(error, committed_status),
      }
    })
  }
}
