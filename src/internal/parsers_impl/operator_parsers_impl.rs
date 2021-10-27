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
        .map_err_is_committed_fallback(n1 != 0)
        .with_add_length(n1)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn and_then_ref<'a, I, A, B, APF, BPF>(parser1: APF, parser2: BPF) -> Self::P<'a, I, (&'a A, &'a B)>
  where
    APF: Fn() -> Self::P<'a, I, &'a A> + 'a,
    BPF: Fn() -> Self::P<'a, I, &'a B> + 'a,
    A: Debug + 'a,
    B: Debug + 'a, {
    Self::flat_map_ref(parser1(), move |e1| Self::map_ref(parser2(), move |e2| (e1, e2)))
  }

  fn attempt<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| parser.run(parse_state).with_un_commit())
  }
}
