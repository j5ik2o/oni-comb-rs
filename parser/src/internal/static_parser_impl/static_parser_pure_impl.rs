// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParserPure, StaticParser};

impl<'a, I, A: 'a> ParserPure<'a> for StaticParser<'a, I, A> {
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a, {
    StaticParser::new(move |_| crate::core::ParseResult::successful(value(), 0))
  }
}
