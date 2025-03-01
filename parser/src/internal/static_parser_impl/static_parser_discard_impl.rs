// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseResult, StaticParser};
use crate::extension::parser::DiscardParser;
use std::fmt::Debug;

impl<'a, I, A: 'a> DiscardParser<'a> for StaticParser<'a, I, A> {
  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { length, .. } => ParseResult::successful((), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}
