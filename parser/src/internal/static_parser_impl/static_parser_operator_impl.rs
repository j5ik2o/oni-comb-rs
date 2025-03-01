// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::{ParseError, ParseResult, StaticParser};
use crate::extension::parser::OperatorParser;
use std::fmt::Debug;

impl<'a, I, A: 'a> OperatorParser<'a> for StaticParser<'a, I, A> {
  fn and_then<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    let method1 = self.method.clone();
    let method2 = other.method.clone();

    StaticParser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { value: a, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        match (method2)(&ps) {
          ParseResult::Success { value: b, length: n2 } => ParseResult::successful((a, b), n1 + n2),
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

  fn or(self, other: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    let method1 = self.method.clone();
    let _method2 = other.method.clone();

    StaticParser::new(move |parse_state| {
      let result = (method1)(parse_state);
      if let Some(committed_status) = result.committed_status() {
        if committed_status.is_uncommitted() {
          return (_method2)(parse_state);
        }
      }
      result
    })
  }

  fn exists(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { .. } => ParseResult::successful(true, 0),
      ParseResult::Failure { .. } => ParseResult::successful(false, 0),
    })
  }

  fn not(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { .. } => {
        let ps = parse_state.add_offset(0);
        let parser_error = ParseError::of_mismatch(
          ps.input(),
          ps.last_offset().unwrap_or(0),
          0,
          "not predicate failed".to_string(),
        );
        ParseResult::failed_with_uncommitted(parser_error)
      }
      ParseResult::Failure { .. } => ParseResult::successful((), 0),
    })
  }

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Clone + Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { value, length } => ParseResult::successful(Some(value), length),
      ParseResult::Failure { .. } => ParseResult::successful(None, 0),
    })
  }

  fn attempt(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    let method = self.method.clone();

    StaticParser::new(move |parse_state| (method)(parse_state).with_uncommitted())
  }

  fn scan_right1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let method1 = self.method.clone();
    let _method2 = op.method.clone();

    StaticParser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { value: x, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        self
          .clone()
          .rest_left1(op.clone(), x.clone())
          .run(&ps)
          .with_add_length(n1)
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn chain_right0<BOP>(
    self,
    _op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let default_value = x.clone();
    let value = default_value.clone();
    self
      .clone()
      .or(StaticParser::new(move |_| ParseResult::successful(value.clone(), 0)))
  }

  fn chain_left0<BOP>(
    self,
    _op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let default_value = x.clone();
    let value = default_value.clone();
    self
      .clone()
      .or(StaticParser::new(move |_| ParseResult::successful(value.clone(), 0)))
  }

  fn chain_right1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    self.clone().scan_right1(op)
  }

  fn chain_left1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let method1 = self.method.clone();
    let _method2 = op.method.clone();

    StaticParser::new(move |parse_state| match (method1)(parse_state) {
      ParseResult::Success { value: x, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        self.clone().rest_left1(op.clone(), x).run(&ps).with_add_length(n1)
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn rest_right1<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let default_value = x.clone();
    let method1 = op.method.clone();
    let method2 = self.method.clone();

    StaticParser::new(move |parse_state| {
      let op_result = (method1)(parse_state);
      match op_result {
        ParseResult::Success { value: f, length: n1 } => {
          let ps = parse_state.add_offset(n1);
          match (method2)(&ps) {
            ParseResult::Success { value: y, length: n2 } => {
              ParseResult::successful(f(default_value.clone(), y), n1 + n2)
            }
            ParseResult::Failure {
              error,
              committed_status,
            } => ParseResult::failed(error, committed_status).with_add_length(n1),
          }
        }
        ParseResult::Failure { .. } => ParseResult::successful(default_value.clone(), 0),
      }
    })
  }

  fn rest_left1<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a, {
    let default_value = x.clone();
    let method1 = op.method.clone();
    let method2 = self.method.clone();

    StaticParser::new(move |parse_state| {
      let mut ps = parse_state.add_offset(0);
      match (method1)(&ps) {
        ParseResult::Success { value: f, length: n1 } => {
          ps = ps.add_offset(n1);
          (match (method2)(&ps) {
            ParseResult::Success { value: y, length: n2 } => {
              ps = ps.add_offset(n2);
              self
                .clone()
                .rest_left1(op.clone(), f(default_value.clone(), y))
                .run(&ps)
                .with_add_length(n2)
            }
            ParseResult::Failure {
              error,
              committed_status,
            } => ParseResult::failed(error, committed_status),
          })
          .with_committed_fallback(n1 != 0)
          .with_add_length(n1)
        }
        ParseResult::Failure { .. } => ParseResult::successful(default_value.clone(), 0),
      }
    })
  }
}
