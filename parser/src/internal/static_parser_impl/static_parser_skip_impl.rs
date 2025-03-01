// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseResult, StaticParser};
use crate::extension::parser::SkipParser;
use std::fmt::Debug;

impl<'a, I, A: 'a> SkipParser<'a> for StaticParser<'a, I, A> {
  fn skip_left<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method1 = self.method.clone();
    let method2 = other.method.clone();

    StaticParser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { length: n1, .. } => {
        let ps = parse_state.add_offset(n1);
        match (method2)(&ps) {
          ParseResult::Success { value: b, length: n2 } => ParseResult::successful(b, n1 + n2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status).with_add_length(n1),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn skip_right<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method1 = self.method.clone();
    let method2 = other.method.clone();

    StaticParser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { value: a, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        match (method2)(&ps) {
          ParseResult::Success { length: n2, .. } => ParseResult::successful(a, n1 + n2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status).with_add_length(n1),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn surround<B, C>(
    self,
    left_parser: Self::P<'a, Self::Input, B>,
    right_parser: Self::P<'a, Self::Input, C>,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    left_parser.skip_left(self.skip_right(right_parser))
  }
}
