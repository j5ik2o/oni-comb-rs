use std::fmt::Debug;
use crate::extension::parser::BasicCombinator;

pub trait ConversionCombinator<'a>: BasicCombinator<'a> {
  fn convert<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
