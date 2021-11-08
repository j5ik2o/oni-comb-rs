use crate::extension::parser::OperatorParser;
use std::fmt::Debug;

pub trait ConversionParser<'a>: OperatorParser<'a> {
  fn map_res<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a;
}
