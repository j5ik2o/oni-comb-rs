use std::fmt::{Debug, Display};

use crate::core::Parsers;

pub trait OperatorParsers: Parsers {
  fn logging<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Self::logging_map(parser, name, move |a| format!("{:?}", a))
  }

  fn logging_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> B + 'a,
    A: Debug + 'a,
    B: Display + 'a;

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
    A: Clone + Debug + 'a,
    B: Debug + 'a;

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn chain_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + Copy + 'a,
    A: Clone + Debug + 'a;

  fn rest_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + Copy + 'a,
    A: Clone + Debug + 'a;
}
