use crate::core::Parser;
use crate::extension::parser::SkipParser;
use crate::extension::parsers::SkipParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I, A> SkipParser<'a> for Parser<'a, I, A> {
  fn skip_left<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::skip_left(self, pb)
  }

  fn skip_right<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::skip_right(self, pb)
  }
}
