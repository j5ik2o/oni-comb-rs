use crate::core::parser_filter::ParserFilter;
use crate::core::parser_functor::ParserFunctor;

pub trait ParserMonad<'a>: ParserFunctor<'a> + ParserFilter<'a> {
  fn flat_map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::P<'a, Self::Input, B> + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a;
}
