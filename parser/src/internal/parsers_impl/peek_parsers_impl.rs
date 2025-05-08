use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::PeekParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl PeekParsers for ParsersImpl {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { value, .. } => ParseResult::successful(value, 0),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}
