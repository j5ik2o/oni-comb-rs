use std::fmt::Debug;
use std::rc::Rc;

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

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn restl1<'a, I, A, PF, PBF, BF>(parser: PF, op: PBF, x: Rc<A>) -> Self::P<'a, I, Rc<A>>
  where
    PF: Fn() -> Self::P<'a, I, A> + Copy + 'a,
    PBF: Fn() -> Self::P<'a, I, BF> + Copy + 'a,
    BF: Fn(&A, &A) -> A + 'a,
    A: Debug + 'a;
}
