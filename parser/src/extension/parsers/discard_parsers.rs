use crate::core::Parsers;
use std::fmt::Debug;

pub trait DiscardParsers: Parsers {
  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a;
}
