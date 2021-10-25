use std::fmt::Debug;
use crate::core::ParserRunner;

pub trait DiscardParser<'a>: ParserRunner<'a> {
  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a;
}
