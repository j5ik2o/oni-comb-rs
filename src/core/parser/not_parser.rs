use crate::core::Parser;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use std::ops::Not;
use crate::extension::parsers::BasicCombinators;

impl<'a, I, A> Not for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Parser<'a, I, bool>;

  fn not(self) -> Self::Output {
    ParsersImpl::not(self)
  }
}
