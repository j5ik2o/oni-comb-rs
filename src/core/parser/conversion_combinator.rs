use crate::core::Parser;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use crate::extension::parser::ConversionCombinator;
use crate::extension::parsers::ConversionCombinators;

impl<'a, I, A> ConversionCombinator<'a> for Parser<'a, I, A> {
  fn convert<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::convert(self, f)
  }
}
