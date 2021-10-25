use std::fmt::Debug;
use crate::core::ParserRunner;

pub trait CollectParser<'a>: ParserRunner<'a> {
  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a;
}
