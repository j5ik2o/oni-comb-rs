use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parser::CacheParser;
use crate::extension::parsers::CacheParsers;
use crate::internal::ParsersImpl;

impl<'a, I, A> CacheParser<'a> for Parser<'a, I, A> {
  fn cache(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a, {
    ParsersImpl::cache(self)
  }
}
