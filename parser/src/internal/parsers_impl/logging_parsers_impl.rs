use crate::core::{ParseError, ParseResult, Parser};
use crate::extension::parsers::{LogLevel, LoggingParsers};
use crate::internal::ParsersImpl;
use std::fmt::{Debug, Display};

impl LoggingParsers for ParsersImpl {
  fn log_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, log_level: LogLevel, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    A: Debug + 'a,
    B: Display + 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| {
      let ps = method(parse_state);
      let s = format!("{} = {}", name, f(&ps));
      match log_level {
        LogLevel::Debug => log::debug!("{}", s),
        LogLevel::Info => log::info!("{}", s),
        LogLevel::Warn => log::warn!("{}", s),
        LogLevel::Err => log::error!("{}", s),
      }
      ps
    })
  }

  fn name<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match method(parse_state) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => match error {
        ParseError::Custom { .. } => ParseResult::failed(error, is_committed),
        _ => ParseResult::failed(
          ParseError::of_custom(
            parse_state.last_offset().unwrap_or(0),
            Some(Box::new(error)),
            format!("failed to parse {}", name),
          ),
          is_committed,
        ),
      },
    })
  }

  fn expect<'a, I, A>(parser: Self::P<'a, I, A>, name: &'a str) -> Self::P<'a, I, A>
  where
    I: Debug,
    A: Debug + 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match method(parse_state) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(
        ParseError::of_expect(
          parse_state.last_offset().unwrap_or(0),
          Box::new(error),
          format!("Expect {}", name),
        ),
        is_committed,
      ),
    })
  }
}
