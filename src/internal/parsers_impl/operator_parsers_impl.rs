use crate::core::ParseResult;
use crate::core::ParserRunner;
use crate::core::{ParseError, Parsers};
use std::fmt::Debug;

use crate::core::Parser;
use crate::extension::parsers::OperatorParsers;
use crate::internal::ParsersImpl;

impl OperatorParsers for ParsersImpl {
  fn not<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { .. } => {
        let ps = parse_state.add_offset(0);
        let parser_error = ParseError::of_mismatch(
          ps.input(),
          ps.last_offset().unwrap_or(0),
          0,
          "not predicate failed".to_string(),
        );
        ParseResult::failed_with_un_commit(parser_error)
      }
      ParseResult::Failure { .. } => ParseResult::successful(true, 0),
    })
  }

  fn or<'a, I, A>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a, {
    Parser::new(move |parse_state| {
      let result = parser1.run(parse_state);
      if let Some(is_committed) = result.is_committed() {
        if is_committed == false {
          return parser2.run(parse_state);
        }
      }
      result
    })
  }

  fn and_then<'a, I, A, B>(parser1: Self::P<'a, I, A>, parser2: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser1.run(parse_state) {
      ParseResult::Success { get: r1, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        (match parser2.run(&ps) {
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n2),
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
        .with_committed_fallback(n1 != 0)
        .with_add_length(n1)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn and_then_ref<'a, I, A, B>(
    parser1: Self::P<'a, I, &'a A>,
    parser2: Self::P<'a, I, &'a B>,
  ) -> Self::P<'a, I, (&'a A, &'a B)>
  where
    A: Debug + 'a,
    B: Debug + 'a, {
    Parser::new(move |parse_state| match parser1.run(parse_state) {
      ParseResult::Success { get: r1, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        (match parser2.run(&ps) {
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n2),
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
        .with_committed_fallback(n1 != 0)
        .with_add_length(n1)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| parser.run(parse_state).with_un_commit())
  }

  fn chanl1<'a, I, P1, P2, A, BOP, XF1>(p: P1, op: P2) -> Self::P<'a, I, A>
  where
    P1: Fn() -> Self::P<'a, I, XF1> + Copy + 'a,
    P2: Fn() -> Self::P<'a, I, BOP> + Copy + 'a,
    BOP: Fn(A, A) -> A + Copy + 'a,
    XF1: Fn() -> A + Copy + 'a,
    A: Debug + 'a, {
    Parser::new(move |parse_state| match p().run(parse_state) {
      ParseResult::Success { get: x, length: n1 } => {
        let ps = parse_state.add_offset(n);
        Self::restl1(p, op, x)
          .run(&ps)
          .with_committed_fallback(n != 0)
          .with_add_length(n)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn restl1<'a, I, P1, P2, A, BOP, XF1, XF2>(p: P1, op: P2, x: XF2) -> Self::P<'a, I, A>
  where
    P1: Fn() -> Self::P<'a, I, XF1> + Copy + 'a,
    P2: Fn() -> Self::P<'a, I, BOP> + Copy + 'a,
    BOP: Fn(A, A) -> A + Copy + 'a,
    XF1: Fn() -> A + Copy + 'a,
    XF2: Fn() -> A + Copy + 'a,
    A: Debug + 'a, {
    let p = Parser::new(move |parse_state| match op().run(parse_state) {
      ParseResult::Success { get: f, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        (match p().run(&ps) {
          ParseResult::Success { get: y, length: n2 } => {
            let ps = parse_state.add_offset(n2);
            Self::restl1(p, op, move || f(x(), y()))
              .run(&ps)
              .with_committed_fallback(n2 != 0)
              .with_add_length(n2)
          }
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
        .with_committed_fallback(n1 != 0)
        .with_add_length(n1)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    });
    Self::or(p, Self::successful(x))
  }
}
