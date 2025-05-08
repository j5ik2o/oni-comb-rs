use crate::core::Parsers;
use std::fmt::Debug;

pub trait CollectParsers: Parsers {
  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: Debug + 'a;
}
