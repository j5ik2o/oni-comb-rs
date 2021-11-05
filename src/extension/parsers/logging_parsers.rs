use crate::core::{ParseResult, Parsers};
use std::fmt::{Debug, Display};

pub trait LoggingParsers: Parsers {
  fn logging<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a, {
    Self::logging_map(parser, name, move |a| format!("{:?}", a))
  }

  fn logging_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug,
    A: Debug + 'a,
    B: Display + 'a;
}
