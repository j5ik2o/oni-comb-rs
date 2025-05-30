use crate::core::{Parser, ParserFunctor, Parsers};
use crate::internal::ParsersImpl;

impl<'a, I, A> ParserFunctor<'a> for Parser<'a, I, A> {
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a, {
    ParsersImpl::map(self, f)
  }
}
