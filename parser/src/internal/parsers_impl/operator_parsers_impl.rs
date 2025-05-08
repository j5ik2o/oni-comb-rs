use crate::core::{ParseError, ParseResult, ParserRunner, Parsers};
use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parsers::OperatorParsers;
use crate::internal::ParsersImpl;

impl OperatorParsers for ParsersImpl {
  #[inline]
  fn exists<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { .. } => ParseResult::successful(true, 0),
      ParseResult::Failure { .. } => ParseResult::successful(false, 0),
    })
  }

  #[inline]
  fn not<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { .. } => {
        let ps = parse_state.advance_by(0);
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

  #[inline]
  fn or<'a, I, A>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a, {
    Parser::new(move |parse_state| {
      let result = parser1.run(parse_state);
      if let Some(committed_status) = result.committed_status() {
        if committed_status.is_uncommitted() {
          return parser2.run(parse_state);
        }
      }
      result
    })
  }

  #[inline]
  fn and_then<'a, I, A, B>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: 'a,
    B: 'a, {
    let method1 = parser1.method.clone();
    let method2 = parser2.method.clone();
    Parser::new(move |parse_state| match method1(parse_state) {
      ParseResult::Success { value: a, length: n1 } => {
        let ps = parse_state.advance_by(n1);
        match method2(&ps) {
          ParseResult::Success { value: b, length: n2 } => ParseResult::successful((a, b), n1 + n2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status), //.advance_success(n1),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  #[inline]
  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| parser.run(parse_state).with_uncommitted())
  }

  #[inline]
  fn chain_right1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a,
    A: Clone + Debug + 'a, {
    Self::flat_map(p.clone(), move |x| Self::rest_left1(p.clone(), op.clone(), x.clone()))
  }

  #[inline]
  fn chain_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a,
    A: Clone + Debug + 'a, {
    Self::flat_map(p.clone(), move |x| Self::rest_left1(p.clone(), op.clone(), x))
  }

  #[inline]
  fn rest_right1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a,
    A: Clone + Debug + 'a, {
    let default_value = x.clone();
    Self::or(
      Self::flat_map(op.clone(), move |f| {
        let default_value = x.clone();
        Self::map(p.clone(), move |y| f(default_value.clone(), y.clone()))
      }),
      Self::successful(default_value.clone()),
    )
  }

  #[inline]
  fn rest_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + 'a,
    A: Clone + Debug + 'a, {
    let default_value = x.clone();
    let method = p.method.clone();
    let op_method = op.method.clone();
    Self::or(
      Parser::new(move |parse_state| {
        let mut ps = parse_state.advance_by(0);
        match op_method(&ps) {
          ParseResult::Success { value: f, length: n1 } => {
            ps = ps.advance_by(n1);
            (match method(&ps) {
              ParseResult::Success { value: y, length: n2 } => {
                ps = ps.advance_by(n2);
                Self::rest_left1(p.clone(), op.clone(), f(default_value.clone(), y))
                  .run(&ps)
                  .advance_success(n2)
              }
              ParseResult::Failure {
                error,
                committed_status,
              } => ParseResult::failed(error, committed_status),
            })
            .add_commit(n1 != 0)
            .advance_success(n1)
          }
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status),
        }
      }),
      Self::successful(x.clone()),
    )
  }
}
