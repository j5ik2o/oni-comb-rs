use std::fmt::Debug;

use crate::extension::BasicCombinators;
use crate::utils::RangeArgument;

pub trait RepeatCombinators: BasicCombinators {
  /// `rep(5)` repeat p exactly 5 times
  /// `rep(0..)` repeat p zero or more times
  /// `rep(1..)` repeat p one or more times
  /// `rep(1..4)` match p at least 1 and at most 3 times
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: Debug + 'a, {
    Self::repeat_sep::<'a, I, A, (), R>(parser, range, None)
  }

  fn many_0<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a, {
    Self::repeat_sep(parser, 0.., None as Option<Self::P<'a, I, ()>>)
  }

  fn many_1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a, {
    Self::repeat_sep(parser, 1.., None as Option<Self::P<'a, I, ()>>)
  }

  fn many_n_m<'a, I, A>(parser: Self::P<'a, I, A>, n: usize, m: usize) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a, {
    Self::repeat_sep(parser, n..=m, None as Option<Self::P<'a, I, ()>>)
  }

  fn count<'a, I, A>(parser: Self::P<'a, I, A>, n: usize) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a, {
    Self::repeat_sep(parser, n, None as Option<Self::P<'a, I, ()>>)
  }

  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: Debug + 'a,
    B: Debug + 'a;

  fn many_0_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, separator: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::repeat_sep(parser, 0.., Some(separator))
  }

  fn many_1_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, separator: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::repeat_sep(parser, 1.., Some(separator))
  }

  fn many_n_m_sep<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    n: usize,
    m: usize,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::repeat_sep(parser, n..=m, Some(separator))
  }

  fn count_sep<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    n: usize,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::repeat_sep(parser, n, Some(separator))
  }
}
