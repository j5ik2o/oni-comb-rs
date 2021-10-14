use crate::parsers::Parsers;
use crate::range::RangeArgument;
use crate::Tuple;
use std::fmt::Debug;

pub trait BasicCombinators: Parsers {
  fn opt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Option<A>>
  where
    I: Clone + 'a,
    A: Clone + 'a, {
    Self::or(Self::map(parser, Some), Self::successful(None))
  }

  fn or<'a, I, A>(parser: Self::P<'a, I, A>, pb: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a;

  fn and<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, Tuple<A, B>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a,
    Self::P<'a, I, B>: 'a;

  /// `repeat(5)` repeat p exactly 5 times
  /// `repeat(0..)` repeat p zero or more times
  /// `repeat(1..)` repeat p one or more times
  /// `repeat(1..4)` match p at least 1 and at most 3 times
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a, {
    Self::repeat_with_separator::<'a, I, A, (), R>(parser, range, None)
  }

  fn repeat_with_separator<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a,
    B: 'a;

  fn many<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a, {
    Self::repeat_with_separator(parser, 0.., None as Option<Self::P<'a, I, ()>>)
  }

  fn many_with_separator<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    Self::repeat_with_separator(parser, 0.., Some(separator))
  }

  fn many1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a, {
    Self::repeat_with_separator(parser, 1.., None as Option<Self::P<'a, I, ()>>)
  }

  fn many1_with_separator<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    Self::repeat_with_separator(parser, 1.., Some(separator))
  }

  fn many_n_m<'a, I, A>(parser: Self::P<'a, I, A>, n: usize, m: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a, {
    Self::repeat_with_separator(parser, n..m + 1, None as Option<Self::P<'a, I, ()>>)
  }

  fn many_n_m_with_separator<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    n: usize,
    m: usize,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    Self::repeat_with_separator(parser, n..m + 1, Some(separator))
  }

  fn list_of_n<'a, I, A>(parser: Self::P<'a, I, A>, n: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a, {
    Self::repeat_with_separator(parser, n, None as Option<Self::P<'a, I, ()>>)
  }

  fn list_of_n_with_separator<'a, I, A, B>(
    parser: Self::P<'a, I, A>,
    n: usize,
    separator: Self::P<'a, I, B>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    Self::repeat_with_separator(parser, n, Some(separator))
  }

  fn list<'a, I, A, B>(parser: Self::P<'a, I, A>, separator: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    Self::many_with_separator(parser, separator)
  }
}
