use crate::core::ParserRunner;
use std::fmt::Debug;
use crate::extension::parsers::LogLevel;

pub trait LoggingParser<'a>: ParserRunner<'a> {
  fn log(self, name: &'a str, log_level: LogLevel) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a;

  fn debug(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
    where
        Self::Input: Debug,
        Self::Output: Debug + 'a;

  fn info(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
    where
        Self::Input: Debug,
        Self::Output: Debug + 'a;

  fn warn(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
    where
        Self::Input: Debug,
        Self::Output: Debug + 'a;

  fn error(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
    where
        Self::Input: Debug,
        Self::Output: Debug + 'a;

  fn name(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a;
}
