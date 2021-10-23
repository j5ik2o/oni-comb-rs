use std::fmt::{Debug, Display};

use crate::core::ParseError;
use crate::core::ParseResult;
use crate::core::{ParserRunner, PrimitiveParsers};

use crate::core::Parser;
use crate::core::Parsers;
use crate::extension::parsers::{OperatorParsers, LazyParsers, OffsetParsers, SkipParsers};
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
          "not predicate failed".to_string(),
        );
        ParseResult::failed_with_un_commit(parser_error)
      }
      ParseResult::Failure { .. } => ParseResult::successful(true, 0),
    })
  }

  fn or<'a, I, A>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a, {
    Parser::new(move |parse_state| {
      let result = pa.run(parse_state);
      if let Some(is_committed) = result.is_committed() {
        if !is_committed {
          return pb.run(parse_state);
        }
      }
      result
    })
  }

  fn and_then<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match pa.run(parse_state) {
      ParseResult::Success { get: r1, length: n1 } => {
        let ps = parse_state.add_offset(n1);
        match pb.run(&ps) {
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n1 + n2),
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        }
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { length, .. } => {
        let slice = parse_state.slice_with_len(length);
        ParseResult::successful(slice, length)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { length, .. } => ParseResult::successful((), length),
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}




