use crate::extension::parser::OperatorParser;
use std::fmt::Debug;

pub trait OffsetParser<'a>: OperatorParser<'a> {
  fn last_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;

  fn next_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a;
}
