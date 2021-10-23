use std::fmt::Debug;
use crate::extension::parsers::BasicCombinators;

pub trait LazyCombinators: BasicCombinators {
  fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a,
    A: Debug + 'a;
}
