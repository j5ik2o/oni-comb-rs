use crate::core::Parsers;
use std::fmt::Debug;

pub trait PeekParsers: Parsers {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;
}
