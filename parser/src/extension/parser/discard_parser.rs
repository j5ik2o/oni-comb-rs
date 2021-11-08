use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait DiscardParser<'a>: ParserRunner<'a> {
  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a;
}
