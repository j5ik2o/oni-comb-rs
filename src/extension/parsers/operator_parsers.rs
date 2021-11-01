use std::fmt::Debug;

use crate::core::Parsers;

pub trait OperatorParsers: Parsers {
  fn not<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: Debug + 'a;

  fn opt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Option<A>>
  where
    A: Debug + 'a, {
    Self::or(Self::map(parser, Some), Self::successful(|| None))
  }

  fn or<'a, I, A>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn and_then<'a, I, A, B>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: Debug + 'a,
    B: Debug + 'a;

  fn and_then_ref<'a, I, A, B>(
    parser1: Self::P<'a, I, &'a A>,
    parser2: Self::P<'a, I, &'a B>,
  ) -> Self::P<'a, I, (&'a A, &'a B)>
  where
    A: Debug + 'a,
    B: Debug + 'a;

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn chain_left1<'a, I, P1, P2, A, BOP, XF1>(p: P1, op: P2) -> Self::P<'a, I, A>
  where
    P1: Fn() -> Self::P<'a, I, XF1> + Copy + 'a,
    P2: Fn() -> Self::P<'a, I, BOP> + Copy + 'a,
    BOP: Fn(A, A) -> A + Copy + 'a,
    XF1: Fn() -> A + Copy + 'a,
    A: Debug + 'a;

  fn rest_left1<'a, I, P1, P2, A, BOP, XF1, XF2>(p: P1, op: P2, x: XF2) -> Self::P<'a, I, A>
  where
    P1: Fn() -> Self::P<'a, I, XF1> + Copy + 'a,
    P2: Fn() -> Self::P<'a, I, BOP> + Copy + 'a,
    BOP: Fn(A, A) -> A + Copy + 'a,
    XF1: Fn() -> A + Copy + 'a,
    XF2: Fn() -> A + Copy + 'a,
    A: Debug + 'a;
}
