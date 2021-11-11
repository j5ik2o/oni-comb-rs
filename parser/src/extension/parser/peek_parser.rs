use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait PeekParser<'a>: ParserRunner<'a> {
  fn peek(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a;
}
