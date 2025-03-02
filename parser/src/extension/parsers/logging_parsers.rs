use crate::core::{ParseResult, Parser, Parsers, StaticParser, StaticParsers};
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
    I: Debug + Clone,
    A: Debug + 'a, {
    Self::log_map(parser, name, log_level, move |a| format!("{:?}", a))
  }

  fn log_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug + Clone,
    A: Debug + 'a,
    B: Display + 'a;

  fn name<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a;

  fn expect<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a;
}

pub trait StaticLoggingParsers: StaticParsers {
  fn log<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel) -> Self::P<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static, {
    Self::log_map(parser, name, log_level, move |a| format!("{:?}", a))
  }

  fn log_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug + Clone,
    A: Debug + 'a + 'static,
    B: Display + 'a;

  fn name<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static;

  fn expect<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn log<'a, I, A>(parser: Parser<'a, I, A>, name: &'a str, log_level: LogLevel) -> Parser<'a, I, A>
where
  I: Debug + Clone,
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::log(parser, name, log_level)
}

pub fn log_map<'a, I, A, B, F>(parser: Parser<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Parser<'a, I, A>
where
  F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
  I: Debug + Clone,
  A: Debug + 'a,
  B: Display + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::log_map(parser, name, log_level, f)
}

pub fn name<'a, I, A>(parser: Parser<'a, I, A>, name: &'a str) -> Parser<'a, I, A>
where
  I: Debug + Clone,
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::name(parser, name)
}

pub fn expect<'a, I, A>(parser: Parser<'a, I, A>, name: &'a str) -> Parser<'a, I, A>
where
  I: Debug + Clone,
  A: Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::expect(parser, name)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn log<'a, I, A>(parser: StaticParser<'a, I, A>, name: &'a str, log_level: LogLevel) -> StaticParser<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::log(parser, name, log_level)
  }

  pub fn log_map<'a, I, A, B, F>(
    parser: StaticParser<'a, I, A>,
    name: &'a str,
    log_level: LogLevel,
    f: F,
  ) -> StaticParser<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug + Clone,
    A: Debug + 'a + 'static,
    B: Display + 'a, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::log_map(parser, name, log_level, f)
  }

  pub fn name<'a, I, A>(parser: StaticParser<'a, I, A>, name: &'a str) -> StaticParser<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::name(parser, name)
  }

  pub fn expect<'a, I, A>(parser: StaticParser<'a, I, A>, name: &'a str) -> StaticParser<'a, I, A>
  where
    I: Debug + Clone,
    A: Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::expect(parser, name)
  }
}
