use crate::core::Parser;
use crate::extension::parser::DiscardParser;
use crate::extension::parsers::DiscardParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I: Clone, A> DiscardParser<'a> for Parser<'a, I, A> {
  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::discard(self)
  }
}
