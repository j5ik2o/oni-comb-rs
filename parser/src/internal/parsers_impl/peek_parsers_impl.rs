use crate::internal::ParsersImpl;
use crate::prelude::PeekParsers;
use std::fmt::Debug;
use crate::core::{ParsedResult, Parser, ParserRunner};

impl PeekParsers for ParsersImpl {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { value, .. } => ParsedResult::successful(value, 0),
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }
}
