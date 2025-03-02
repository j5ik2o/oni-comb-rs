use crate::core::{Parser, ParserFunctor, Parsers};
use crate::internal::ParsersImpl;

impl<'a, I: Clone, A> ParserFunctor<'a> for Parser<'a, I, A> {
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a + Clone,
    Self::Input: 'a,
    Self::Output: Clone + 'a,
    B: Clone + 'a, {
    ParsersImpl::map(self, f)
  }
}
