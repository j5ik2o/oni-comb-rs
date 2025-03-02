// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParserFunctor, StaticParser};

impl<'a, I: std::clone::Clone, A: 'a> ParserFunctor<'a> for StaticParser<'a, I, A> {
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: 'a,
    Self::Output: Clone + 'a,
    B: Clone + 'a, {
    StaticParser::new(move |state| match self.run(state) {
      crate::core::ParseResult::Success { value, length } => crate::core::ParseResult::successful(f(value), length),
      crate::core::ParseResult::Failure {
        error,
        committed_status,
      } => crate::core::ParseResult::failed(error, committed_status),
    })
  }
}
