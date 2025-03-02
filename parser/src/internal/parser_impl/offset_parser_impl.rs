use crate::core::Parser;
use crate::extension::parser::OffsetParser;
use crate::extension::parsers::OffsetParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I: Clone, A> OffsetParser<'a> for Parser<'a, I, A> {
  fn last_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::last_offset(self)
  }

  fn next_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::next_offset(self)
  }
}
