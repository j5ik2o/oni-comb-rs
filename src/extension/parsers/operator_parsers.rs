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

  fn and_then_ref<'a, I, A, B, APF, BPF>(parser1: APF, parser2: BPF) -> Self::P<'a, I, (&'a A, &'a B)>
  where
    APF: Fn() -> Self::PR<'a, I, A> + 'a,
    BPF: Fn() -> Self::PR<'a, I, B> + 'a,
    A: Debug + 'a,
    B: Debug + 'a;

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn restl1<'a, I, A, PF, PBF, BF>(parser: PF, op: PBF, x: A) -> Self::P<'a, I, A>
  where
    PF: Fn() -> Self::PR<'a, I, A> + Copy + 'a,
    PBF: Fn() -> Self::PR<'a, I, BF> + Copy + 'a,
    BF: Fn(&A, &'a A) -> A + 'a,
    A: Debug + Clone + 'a;
}
