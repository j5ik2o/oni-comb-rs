use crate::core::{Parsers, StaticParsers};
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
