// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseError, ParseResult, StaticParser};
use crate::extension::parser::LoggingParser;
use crate::extension::parsers::LogLevel;
use std::fmt::Debug;

impl<'a, I, A: 'a> LoggingParser<'a> for StaticParser<'a, I, A> {
  fn log(self, name: &'a str, log_level: LogLevel) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| {
      let ps = (method)(parse_state);
      let s = format!("{} = {:?}", name, ps);
      match log_level {
        LogLevel::Debug => log::debug!("{}", s),
        LogLevel::Info => log::info!("{}", s),
        LogLevel::Warn => log::warn!("{}", s),
        LogLevel::Err => log::error!("{}", s),
      }
      ps
    })
  }

  fn debug(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    self.log(name, LogLevel::Debug)
  }

  fn info(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    self.log(name, LogLevel::Info)
  }

  fn warn(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    self.log(name, LogLevel::Warn)
  }

  fn error(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    self.log(name, LogLevel::Err)
  }

  fn name(self, name: &'a str) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Input: Debug,
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        error,
        committed_status,
      } => match error {
        ParseError::Custom { .. } => ParseResult::failed(error, committed_status),
        _ => ParseResult::failed(
          ParseError::of_custom(
            parse_state.last_offset().unwrap_or(0),
            Some(Box::new(error)),
            format!("failed to parse {}", name),
          ),
          committed_status,
        ),
      },
    })
  }
}
