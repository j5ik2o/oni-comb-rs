use std::fmt::Debug;
use crate::core::Parsers;

pub trait LazyParsers: Parsers {
  fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a,
    A: Debug + 'a;
}
