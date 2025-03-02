use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use std::fmt::Debug;

pub trait CacheParsers: Parsers {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;
}

pub trait StaticCacheParsers: StaticParsers {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn cache<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a, {
  use crate::prelude::CacheParser;
  parser.cache()
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn cache<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static, {
    use crate::prelude::CacheParser;
    parser.cache()
  }
}
