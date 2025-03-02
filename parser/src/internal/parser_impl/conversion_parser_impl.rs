use crate::core::Parser;
use crate::extension::parser::ConversionParser;
use crate::extension::parsers::ConversionParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I: Clone, A> ConversionParser<'a> for Parser<'a, I, A> {
  fn map_res<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::map_res(self, f)
  }
}
