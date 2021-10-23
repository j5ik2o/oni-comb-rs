use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parser::CollectParser;
use crate::extension::parsers::CollectParsers;
use crate::internal::ParsersImpl;

impl<'a, I, A> CollectParser<'a> for Parser<'a, I, A> {
  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::collect(self)
  }
}
