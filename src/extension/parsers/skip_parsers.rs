use std::fmt::Debug;

use crate::extension::parsers::OperatorParsers;

pub trait SkipParsers: OperatorParsers {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()>;

  fn skip_left<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, B>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::map(Self::and_then(pa, pb), |(_a, b)| b)
  }

  fn skip_right<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::map(Self::and_then(pa, pb), |(a, _b)| a)
  }

  fn surround<'a, I, A, B, C>(
    left_parser: Self::P<'a, I, A>,
    parser: Self::P<'a, I, B>,
    right_parser: Self::P<'a, I, C>,
  ) -> Self::P<'a, I, B>
  where
    A: Debug + 'a,
    B: Debug + 'a,
    C: Debug + 'a, {
    Self::skip_left(left_parser, Self::skip_right(parser, right_parser))
  }
}
