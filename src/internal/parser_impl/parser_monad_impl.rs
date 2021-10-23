use crate::core::{Parser, ParserFilter, ParserMonad, Parsers};
use crate::internal::ParsersImpl;

impl<'a, I, A> ParserFilter<'a> for Parser<'a, I, A> {
  fn with_filter<F>(self, f: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn(&Self::Output) -> bool + 'a,
    Self::Input: 'a,
    Self::Output: 'a, {
    ParsersImpl::filter(self, f)
  }
}

impl<'a, I, A> ParserMonad<'a> for Parser<'a, I, A> {
  fn flat_map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::P<'a, Self::Input, B> + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a, {
    ParsersImpl::flat_map(self, move |e| f(e))
  }
}
