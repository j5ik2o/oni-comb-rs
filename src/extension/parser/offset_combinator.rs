use std::fmt::Debug;
use crate::extension::parser::BasicCombinator;

pub trait OffsetCombinator<'a>: BasicCombinator<'a> {
  fn last_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;

  fn next_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;
}
