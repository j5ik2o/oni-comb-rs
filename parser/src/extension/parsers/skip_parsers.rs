use std::fmt::Debug;

use crate::extension::parsers::OperatorParsers;

pub trait SkipParsers: OperatorParsers {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()>;

  fn skip_left<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn skip_right<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn surround<'a, I, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a;
}
