use crate::core::Parser;
use crate::extension::parsers::PeekParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use crate::extension::parser::PeekParser;

impl<'a, I, A> PeekParser<'a> for Parser<'a, I, A> {
  fn peek(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::peek(self)
  }
}
