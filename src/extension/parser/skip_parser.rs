use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait SkipParser<'a>: ParserRunner<'a> {
  fn skip_left<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn skip_right<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn surround<B, C>(
    self,
    left_parser: Self::P<'a, Self::Input, B>,
    right_parser: Self::P<'a, Self::Input, C>,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a,
    C: Debug + 'a;
}
