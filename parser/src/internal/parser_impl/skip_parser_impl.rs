use crate::core::Parser;
use crate::extension::parser::SkipParser;
use crate::extension::parsers::SkipParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I: Clone, A> SkipParser<'a> for Parser<'a, I, A> {
  fn skip_left<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::skip_left(self, pb)
  }

  fn skip_right<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::skip_right(self, pb)
  }

  fn surround<B, C>(
    self,
    left_parser: Self::P<'a, Self::Input, B>,
    right_parser: Self::P<'a, Self::Input, C>,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    ParsersImpl::surround(left_parser, self, right_parser)
  }
}
