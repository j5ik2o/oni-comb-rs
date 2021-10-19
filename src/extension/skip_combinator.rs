use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait SkipCombinator<'a>: ParserRunner<'a> {
  fn skip_left<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn skip_right<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
