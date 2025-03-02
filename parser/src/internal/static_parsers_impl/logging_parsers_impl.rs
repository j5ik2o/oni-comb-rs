use crate::core::{ParseResult, StaticParser};
use crate::extension::parsers::{LogLevel, StaticLoggingParsers};
use crate::internal::static_parsers_impl::StaticParsersImpl;
use std::fmt::{Debug, Display};

impl StaticLoggingParsers for StaticParsersImpl {
  fn log_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    I: Debug + std::clone::Clone,
    A: Debug + 'a + 'static,
    B: Display + 'a, {
    StaticParser::new(move |state| {
      let result = parser.run(state);
      match log_level {
        LogLevel::Debug => log::debug!("{}: {}", name, f(&result)),
        LogLevel::Info => log::info!("{}: {}", name, f(&result)),
        LogLevel::Warn => log::warn!("{}: {}", name, f(&result)),
        LogLevel::Err => log::error!("{}: {}", name, f(&result)),
      }
      result
    })
  }

  fn name<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + std::clone::Clone,
    A: Debug + 'a + 'static, {
    Self::log_map(parser, name, LogLevel::Debug, move |result| match result {
      ParseResult::Success { value, .. } => format!("Success: {:?}", value),
      ParseResult::Failure { error, .. } => format!("Failure: {:?}", error),
    })
  }

  fn expect<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug + std::clone::Clone,
    A: Debug + 'a + 'static, {
    Self::log_map(parser, name, LogLevel::Err, move |result| match result {
      ParseResult::Success { value, .. } => format!("Success: {:?}", value),
      ParseResult::Failure { error, .. } => format!("Failure: {:?}", error),
    })
  }
}
