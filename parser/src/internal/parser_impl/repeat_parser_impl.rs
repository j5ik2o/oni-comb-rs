use crate::core::Parser;
use crate::extension::parser::RepeatParser;
use crate::extension::parsers::RepeatParsers;
use crate::internal::ParsersImpl;
use crate::utils::RangeArgument;
use std::fmt::Debug;

impl<'a, I, A> RepeatParser<'a> for Parser<'a, I, A> {
  fn repeat<R>(self, range: R) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    Self: Sized, {
    ParsersImpl::repeat(self, range)
  }

  fn of_many0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    ParsersImpl::many0(self)
  }

  fn of_many1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    ParsersImpl::many1(self)
  }

  fn of_many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    self.repeat(n..=m)
  }

  fn of_count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    ParsersImpl::count(self, n)
  }

  fn of_rep_sep<B, R>(
    self,
    range: R,
    separator: Option<Self::P<'a, Self::Input, B>>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::repeat_sep(self, range, separator)
  }

  fn of_many0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::many0_sep(self, separator)
  }

  fn of_many1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::many1_sep(self, separator)
  }

  fn of_many_n_m_sep<B>(
    self,
    n: usize,
    m: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::repeat_sep(self, n..=m, Some(separator))
  }

  fn of_count_sep<B>(
    self,
    n: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::repeat_sep(self, n..=n, Some(separator))
  }
}
