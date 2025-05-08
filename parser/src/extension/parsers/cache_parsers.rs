use crate::core::{Parsers, StaticParsers};
use std::fmt::Debug;

pub trait CacheParsers: Parsers {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;
}
