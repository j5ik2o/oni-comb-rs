use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::ParserRunner;
use std::fmt::Debug;

pub trait LazyParsers: Parsers {
  fn lazy<'a, I: Clone, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a;
}

pub trait StaticLazyParsers: StaticParsers {
  fn lazy<'a, I: Clone, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn lazy<'a, I: Clone, A, F>(f: F) -> Parser<'a, I, A>
where
  F: Fn() -> Parser<'a, I, A> + 'a + Clone,
  A: Clone + Debug + 'a, {
  // 直接実装を使用
  Parser::new(move |state| {
    let parser = f();
    parser.run(state)
  })
}

// failed_lazy_static関数の実装 (Cloneを要求)
pub fn failed_lazy_static<'a, I: Clone, E, F>(f: F) -> StaticParser<'a, I, E>
where
  F: Fn() -> (crate::core::ParseError<'a, I>, crate::core::CommittedStatus) + 'a + Clone, {
  StaticParser::new(move |state| {
    let current_f = f.clone();
    let (error, committed_status) = current_f();
    crate::core::ParseResult::failed(error, committed_status)
  })
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn lazy<'a, I: Clone, A, F>(f: F) -> StaticParser<'a, I, A>
  where
    F: Fn() -> StaticParser<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a + 'static, {
    // 直接実装を使用
    StaticParser::new(move |state| {
      let parser = f();
      parser.run(state)
    })
  }
}
