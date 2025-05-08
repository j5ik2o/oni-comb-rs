// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Not;

use crate::core::{CommittedStatus, ParseError, ParseResult, StaticParser};

impl<'a, I, A> Not for StaticParser<'a, I, A>
where
  I: Clone + 'a,
  A: Clone + 'a,
{
  type Output = StaticParser<'a, I, ()>;

  fn not(self) -> Self::Output {
    let method = self.method.clone();

    StaticParser::new(move |state| match method(state) {
      ParseResult::Success { value: _, length: _ } => {
        let offset = state.next_offset();
        let msg = "not: parser succeeded".to_string();
        let pe = ParseError::of_custom(offset, None, msg);
        ParseResult::failed(pe, crate::core::CommittedStatus::Uncommitted)
      }
      ParseResult::Failure {
        error: _,
        committed_status,
      } => match committed_status {
        CommittedStatus::Committed => {
          let offset = state.next_offset();
          let msg = "not: parser failed with committed status".to_string();
          let pe = ParseError::of_custom(offset, None, msg);
          ParseResult::failed(pe, committed_status)
        }
        CommittedStatus::Uncommitted => ParseResult::successful((), 0),
      },
    })
  }
}
