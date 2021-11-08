use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait LoggingParser<'a>: ParserRunner<'a> {
  fn log(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a;

  fn name(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a;
}
