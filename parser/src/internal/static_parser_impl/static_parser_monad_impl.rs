// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParserMonad, StaticParser};

impl<'a, I, A: 'a> ParserMonad<'a> for StaticParser<'a, I, A> {
  fn flat_map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::P<'a, Self::Input, B> + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a, {
    StaticParser::new(move |state| match self.run(state) {
      crate::core::ParseResult::Success { value, length } => {
        let next_state = state.add_offset(length);
        let next_parser = f(value);
        match next_parser.run(&next_state) {
          crate::core::ParseResult::Success {
            value: next_value,
            length: next_length,
          } => crate::core::ParseResult::successful(next_value, length + next_length),
          crate::core::ParseResult::Failure {
            error,
            committed_status,
          } => crate::core::ParseResult::failed(error, committed_status),
        }
      }
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    })
  }
}
