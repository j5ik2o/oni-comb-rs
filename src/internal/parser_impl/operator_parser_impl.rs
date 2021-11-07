use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parser::OperatorParser;
use crate::extension::parsers::OperatorParsers;
use crate::internal::ParsersImpl;

impl<'a, I, A> OperatorParser<'a> for Parser<'a, I, A> {
  fn and_then<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    ParsersImpl::and_then(self, pb)
  }

  fn or(self, pb: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::or(self, pb)
  }

  fn exists(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::exists(self)
  }

  fn not(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::not(self)
  }

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Clone + Debug + 'a, {
    ParsersImpl::opt(self)
  }

  fn attempt(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::attempt(self)
  }
}
