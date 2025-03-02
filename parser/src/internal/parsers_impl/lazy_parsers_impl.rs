use crate::core::{ParserMonad, Parsers};
use crate::extension::parsers::LazyParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl LazyParsers for ParsersImpl {
  fn lazy<'a, I: Clone, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a, {
    Self::successful(()).flat_map(move |_| f())
  }
}
