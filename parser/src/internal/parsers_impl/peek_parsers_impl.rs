use crate::core::{ParseResult, Parser, ParserRunner};
use crate::internal::ParsersImpl;
use crate::prelude::PeekParsers;
use std::fmt::Debug;

impl PeekParsers for ParsersImpl {
  fn peek<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { get, .. } => ParseResult::successful(get, 0),
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}
