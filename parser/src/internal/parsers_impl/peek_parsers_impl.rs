use crate::core::{ParseResult, Parser, ParserRunner};
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use crate::extension::parsers::PeekParsers;

impl PeekParsers for ParsersImpl {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { value, .. } => ParseResult::successful(value, 0),
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}
