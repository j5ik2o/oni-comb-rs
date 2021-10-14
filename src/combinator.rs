use crate::parser::Parser;
use crate::range::RangeArgument;
use crate::Tuple;
use std::fmt::Debug;

pub trait BasicCombineParser<'a>: Parser<'a> {
  fn and<B>(self, pbf: Self::M<'a, Self::Input, B>) -> Self::M<'a, Self::Input, Tuple<Self::Output, B>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    Self::M<'a, Self::Input, B>: 'a;

  fn or(self, pb: Self::M<'a, Self::Input, Self::Output>) -> Self::M<'a, Self::Input, Self::Output>
  where
    Self::Output: 'a;

  fn opt(self) -> Self::M<'a, Self::Input, Option<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + 'a;

  fn repeat<R>(self, range: R) -> Self::M<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: 'a,
    Self: Sized, {
    self.repeat_with_separator::<(), R>(range, None)
  }

  fn repeat_with_separator<B, R>(
    self,
    range: R,
    separator: Option<Self::M<'a, Self::Input, B>>,
  ) -> Self::M<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: 'a,
    B: 'a;
}
