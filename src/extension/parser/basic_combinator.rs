use std::fmt::Debug;
use crate::core::ParserRunner;

pub trait BasicCombinator<'a>: ParserRunner<'a> {
  fn and_then<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn or(self, other: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a;

  fn not(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a;

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Debug + 'a;

  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a;

  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a;
}
