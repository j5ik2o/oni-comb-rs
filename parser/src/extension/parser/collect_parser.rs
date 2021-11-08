use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait CollectParser<'a>: ParserRunner<'a> {
  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a;
}
