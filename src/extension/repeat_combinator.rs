use crate::extension::BasicCombinator;
use crate::utils::RangeArgument;
use std::fmt::Debug;

pub trait RepeatCombinator<'a>: BasicCombinator<'a> {
  fn repeat<R>(self, range: R) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    Self: Sized;

  fn of_many0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a;

  fn of_many1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a;

  fn of_many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a;

  fn of_count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a;

  fn of_rep_sep<B, R>(
    self,
    range: R,
    separator: Option<Self::P<'a, Self::Input, B>>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn of_many0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn of_many1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn of_many_n_m_sep<B>(
    self,
    n: usize,
    m: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;

  fn of_count_sep<B>(
    self,
    n: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
