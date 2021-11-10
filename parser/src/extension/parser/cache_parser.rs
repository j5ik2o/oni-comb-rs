use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait CacheParser<'a>: ParserRunner<'a> {
  fn cache(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a;
}
