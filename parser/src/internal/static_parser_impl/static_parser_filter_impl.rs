// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseError, ParseResult, ParserFilter, StaticParser};

impl<'a, I, A: 'a> ParserFilter<'a> for StaticParser<'a, I, A> {
  fn with_filter<F>(self, f: F) -> Self
  where
    F: Fn(&Self::Output) -> bool + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    Self: Sized, {
    StaticParser::new(move |state| match self.run(state) {
      ParseResult::Success { value, length } => {
        if f(&value) {
          ParseResult::successful(value, length)
        } else {
          let offset = state.next_offset() + length;
          let msg = "filter: predicate returned false".to_string();
          let pe = ParseError::of_custom(offset, None, msg);
          ParseResult::failed(pe, crate::core::CommittedStatus::Uncommitted)
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  // Use the default implementation from the trait
}
