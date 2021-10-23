use std::fmt::Debug;
use crate::core::ParserRunner;

pub trait SkipParser<'a>: ParserRunner<'a> {
  fn skip_left<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn skip_right<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
