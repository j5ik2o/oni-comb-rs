use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::DiscardParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl DiscardParsers for ParsersImpl {
  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match (method)(parse_state) {
      ParseResult::Success { length, .. } => ParseResult::successful((), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}
