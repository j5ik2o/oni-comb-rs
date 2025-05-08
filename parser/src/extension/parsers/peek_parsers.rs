use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use std::fmt::Debug;
use crate::extension::parser::PeekParser;

pub trait PeekParsers: Parsers {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;
}

pub trait StaticPeekParsers: StaticParsers {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn peek<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, A>
where
  A: Debug + 'a, {
  parser.peek()
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use crate::extension::parser::PeekParser;
  use super::*;

  // StaticParserを使用する関数
  pub fn peek<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, A>
  where
    A: Debug + 'a + 'static, {
    parser.peek()
  }
}
