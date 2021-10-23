use crate::extension::parser::BasicParser;
use std::fmt::Debug;

pub trait OffsetParser<'a>: BasicParser<'a> {
  fn last_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;

  fn next_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;
}
