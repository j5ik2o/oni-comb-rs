use crate::core::{Parser, ParserPure, Parsers};
use crate::internal::ParsersImpl;

impl<'a, I, A> ParserPure<'a> for Parser<'a, I, A> {
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a, {
    ParsersImpl::successful_in_closure(value)
  }
}
