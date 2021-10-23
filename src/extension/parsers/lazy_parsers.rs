use crate::extension::parsers::BasicParsers;
use std::fmt::Debug;

pub trait LazyParsers: BasicParsers {
  fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a,
    A: Debug + 'a;
}
