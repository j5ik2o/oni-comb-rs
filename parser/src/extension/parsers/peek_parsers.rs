use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use std::fmt::Debug;

pub trait PeekParsers: Parsers {
  fn peek<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;
}

pub trait StaticPeekParsers: StaticParsers {
  fn peek<'a, I: Clone, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn peek<'a, I: Clone, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
  A: Debug + 'a, {
  use crate::prelude::PeekParser;
  parser.peek()
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn peek<'a, I: Clone, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, A>
  where
    A: Debug + 'a + 'static, {
    use crate::prelude::PeekParser;
    parser.peek()
  }
}
