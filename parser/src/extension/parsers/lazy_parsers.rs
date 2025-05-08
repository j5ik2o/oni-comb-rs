use crate::core::{Parsers, StaticParsers};
use crate::prelude::ParserRunner;
use std::fmt::Debug;

pub trait LazyParsers: Parsers {
  fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a + Clone,
    A: Clone + Debug + 'a;
}
