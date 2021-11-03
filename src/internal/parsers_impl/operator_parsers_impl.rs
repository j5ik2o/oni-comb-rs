use crate::core::{ParseResult, Parsers};
use crate::core::ParserRunner;
use crate::core::{ParseError, };
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
    A: Clone + 'a,
    B: Clone + 'a, {
    Self::flat_map(parser1, move |a| Self::map(parser2.clone(), move |b| (a.clone(), b)))
    // Parser::new(move |parse_state| match parser1.run(parse_state) {
    //   ParseResult::Success { get: r1, length: n1 } => {
    //     let ps = parse_state.add_offset(n1);
    //     (match parser2.run(&ps) {
    //       ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n2),
    //       ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    //     })
    //     .with_committed_fallback(n1 != 0)
    //     .with_add_length(n1)
    //   }
    //   ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    // })
  }

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| parser.run(parse_state).with_un_commit())
  }

  fn chain_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + Copy + 'a,
    A: Clone + Debug + 'a, {
    Parser::new(move |parse_state| match p.run(parse_state) {
      ParseResult::Success { get: x, length: n } => {
        let ps = parse_state.add_offset(n);
        Self::rest_left1(p.clone(), op.clone(), x)
          .run(&ps)
          .with_committed_fallback(n != 0)
          .with_add_length(n)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn rest_left1<'a, I, A, BOP>(p: Self::P<'a, I, A>, op: Self::P<'a, I, BOP>, x: A) -> Self::P<'a, I, A>
  where
    BOP: Fn(A, A) -> A + Copy + 'a,
    A: Clone + Debug + 'a, {
    Parser::new(move |parse_state| {
      let mut ps = parse_state.add_offset(0);
      let mut cur_x = x.clone();
      let mut len = 0;
      loop {
        match op.run(&ps) {
          ParseResult::Success { get: f, length: n1 } => {
            ps = parse_state.add_offset(n1);
            match p.run(&ps) {
              ParseResult::Success { get: y, length: n2 } => {
                ps = ps.add_offset(n2);
                cur_x = f(x.clone(), y);
                len = n1 + n2;
                continue;
              }
              ParseResult::Failure { .. } => return ParseResult::successful(cur_x.clone(), len),
            }
          }
          ParseResult::Failure { .. } => return ParseResult::successful(cur_x.clone(), len),
        }
      }
    })
  }

}
