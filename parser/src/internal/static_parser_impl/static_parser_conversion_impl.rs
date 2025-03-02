// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseError, ParseResult, StaticParser};
use crate::extension::parser::ConversionParser;
use std::fmt::Debug;

impl<'a, I: std::clone::Clone, A: 'a> ConversionParser<'a> for StaticParser<'a, I, A> {
  fn map_res<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { value: a, length } => match f(a) {
        Ok(value) => ParseResult::successful(value, length),
        Err(err) => {
          let ps = parse_state.add_offset(0);
          let msg = format!("Conversion error: {:?}", err);
          let parser_error = ParseError::of_conversion(ps.input(), ps.last_offset().unwrap_or(0), 0, msg);
          ParseResult::failed_with_uncommitted(parser_error)
        }
      },
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}
