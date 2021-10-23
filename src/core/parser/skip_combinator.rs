use crate::core::Parser;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use crate::extension::parser::SkipCombinator;
use crate::extension::parsers::SkipCombinators;

impl<'a, I, A> SkipCombinator<'a> for Parser<'a, I, A> {
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
