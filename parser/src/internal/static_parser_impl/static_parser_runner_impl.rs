// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseResult, ParseState, ParserRunner, StaticParser};

impl<'a, I, A: 'a> ParserRunner<'a> for StaticParser<'a, I, A> {
  type Input = I;
  type Output = A;
  type P<'m, X, Y: 'm>
    = StaticParser<'m, X, Y>
  where
    X: 'm;

  fn parse(&self, input: &'a [Self::Input]) -> ParseResult<'a, Self::Input, Self::Output> {
    let parse_state = ParseState::new(input, 0);
    self.run(&parse_state)
  }

  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParseResult<'a, Self::Input, Self::Output> {
    (self.method)(param)
  }
}
