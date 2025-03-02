use crate::core::Parser;
use crate::extension::parser::LoggingParser;
use crate::extension::parsers::{LogLevel, LoggingParsers};
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl<'a, I: Clone, A> LoggingParser<'a> for Parser<'a, I, A> {
  fn log(self, name: &'a str, log_level: LogLevel) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::log(self, name, log_level)
  }

  fn debug(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::log(self, name, LogLevel::Debug)
  }

  fn info(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::log(self, name, LogLevel::Info)
  }

  fn warn(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::log(self, name, LogLevel::Warn)
  }

  fn error(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::log(self, name, LogLevel::Err)
  }

  fn name(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    ParsersImpl::name(self, name)
  }
}
