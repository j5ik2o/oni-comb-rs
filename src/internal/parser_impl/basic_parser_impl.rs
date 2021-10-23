use crate::core::Parser;
use crate::extension::parser::OperatorParser;
use crate::extension::parsers::OperatorParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I, A> OperatorParser<'a> for Parser<'a, I, A> {
  fn and_then<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::and_then(self, pb)
  }

  fn or(self, pb: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::or(self, pb)
  }

  fn not(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::not(self)
  }

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::opt(self)
  }

  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::collect(self)
  }

  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::discard(self)
  }
}
