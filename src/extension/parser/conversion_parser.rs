use crate::extension::parser::BasicParser;
use std::fmt::Debug;

pub trait ConversionParser<'a>: BasicParser<'a> {
  fn convert<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
