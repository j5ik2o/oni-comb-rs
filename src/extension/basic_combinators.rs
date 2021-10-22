use std::fmt::Debug;

use crate::core::CoreParsers;

pub trait BasicCombinators: CoreParsers {
  fn not<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: Debug + 'a;

  fn opt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Option<A>>
  where
    A: Debug + 'a, {
    Self::or(Self::map(parser, Some), Self::successful(|| None))
  }

  fn or<'a, I, A>(parser: Self::P<'a, I, A>, pb: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a;

  fn and_then<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: Debug + 'a,
    B: Debug + 'a;

  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: Debug + 'a;

  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a;
}
