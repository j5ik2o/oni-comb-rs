use std::fmt::Debug;

use crate::extension::parsers::BasicCombinators;

pub trait OffsetCombinators: BasicCombinators {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a;

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: Debug + 'a;
}
