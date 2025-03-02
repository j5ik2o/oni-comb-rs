use crate::core::Parser;
use crate::extension::parsers::OperatorParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use std::ops::Not;

impl<'a, I: Clone, A> Not for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Parser<'a, I, ()>;

  fn not(self) -> Self::Output {
    ParsersImpl::not(self)
  }
}
