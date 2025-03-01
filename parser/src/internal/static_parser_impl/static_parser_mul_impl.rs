// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Mul;

use crate::core::StaticParser;

impl<'a, I, A, B> Mul<StaticParser<'a, I, B>> for StaticParser<'a, I, A>
where
  I: Clone + 'a,
  A: Clone + 'a,
  B: Clone + 'a,
{
  type Output = StaticParser<'a, I, B>;

  fn mul(self, rhs: StaticParser<'a, I, B>) -> Self::Output {
    let lhs_method = self.method.clone();
    let rhs_method = rhs.method.clone();

    StaticParser::new(move |state| match lhs_method(state) {
      crate::core::ParseResult::Success {
        value: _,
        length: lhs_length,
      } => {
        let next_state = state.add_offset(lhs_length);
        match rhs_method(&next_state) {
          crate::core::ParseResult::Success {
            value: rhs_value,
            length: rhs_length,
          } => crate::core::ParseResult::successful(rhs_value, lhs_length + rhs_length),
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
