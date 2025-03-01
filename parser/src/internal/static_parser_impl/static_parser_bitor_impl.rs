// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::BitOr;

use crate::core::{ParseError, StaticParser};

impl<'a, I, A> BitOr<StaticParser<'a, I, A>> for StaticParser<'a, I, A>
where
  I: Clone + 'a,
  A: Clone + 'a,
{
  type Output = StaticParser<'a, I, A>;

  fn bitor(self, rhs: StaticParser<'a, I, A>) -> Self::Output {
    let lhs_method = self.method.clone();
    let rhs_method = rhs.method.clone();

    StaticParser::new(move |state| match lhs_method(state) {
      crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(value, length),
      crate::core::ParseResult::Failure {
        error: lhs_error,
        committed_status: lhs_committed_status,
      } => match lhs_committed_status {
        crate::core::CommittedStatus::Committed => crate::core::ParseResult::failed(lhs_error, lhs_committed_status),
        crate::core::CommittedStatus::Uncommitted => match rhs_method(state) {
          crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(value, length),
          crate::core::ParseResult::Failure {
            error: rhs_error,
            committed_status: rhs_committed_status,
          } => {
            let offset = state.next_offset();
            let msg = format!("{} or {}", lhs_error, rhs_error);
            let pe = ParseError::of_custom(offset, None, msg);
            crate::core::ParseResult::failed(pe, rhs_committed_status)
          }
        },
      },
    })
  }
}
