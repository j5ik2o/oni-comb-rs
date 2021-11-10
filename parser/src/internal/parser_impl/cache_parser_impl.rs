use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parser::{CacheParser, DiscardParser};
use crate::extension::parsers::{CacheParsers, DiscardParsers};
use crate::internal::ParsersImpl;

impl<'a, I, A> CacheParser<'a> for Parser<'a, I, A> {
  fn cache(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
      ParsersImpl::cache(self)
  }
}
