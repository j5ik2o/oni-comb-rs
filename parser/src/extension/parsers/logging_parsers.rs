use crate::core::{ParseResult, Parsers};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Err,
}

pub trait LoggingParsers: Parsers {
  fn log<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a, {
    Self::log_map(parser, name, log_level, move |a| format!("{:?}", a))
  }

  fn log_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug,
    A: Debug + 'a,
    B: Display + 'a;

  fn name<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a;

  fn expect<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a;
}
