use crate::core::Parsers;
use crate::prelude::RangeArgument;
use std::fmt::Debug;

pub trait RepeatParsers: Parsers {
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many0<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn count<'a, I, A>(parser: Self::P<'a, I, A>, count: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many0_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn many1_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;
}
