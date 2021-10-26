use crate::core::{ParseResult};
use crate::core::ParserRunner;
use crate::core::{ParseError, Parsers};
use std::fmt::Debug;
use std::rc::Rc;

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
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n1 + n2),
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
        .map_err_is_committed_fallback(n1 != 0)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| parser.run(parse_state).with_un_commit())
  }

  fn restl1<'a, I, A, PF, PBF, BF>(parser: PF, op: PBF, x: Rc<A>) -> Self::P<'a, I, Rc<A>>
  where
    PF: Fn() -> Self::P<'a, I, A> + Copy + 'a,
    PBF: Fn() -> Self::P<'a, I, BF> + Copy + 'a,
    BF: Fn(&A, &A) -> A + 'a,
    A: Debug + 'a, {
    let vx = x.clone();
    Parser::new(move |parse_state| match op().run(parse_state) {
      ParseResult::Success { get: f, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        (match parser().run(&ps) {
          ParseResult::Success { get: y, length: n2 } => {
            let ps = ps.add_offset(n2);
            let r = f(&*vx, &y);
            Self::restl1(parser, op, Rc::new(r))
              .run(&ps)
              .map_err_is_committed_fallback(n2 != 0)
              .with_add_length(n2)
          }
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        })
        .map_err_is_committed_fallback(n1 != 0)
        .with_add_length(n1)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    }) | Self::successful(move || x.clone())
  }
}
