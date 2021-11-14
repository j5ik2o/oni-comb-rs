use crate::extension::parsers::DiscardParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;
use crate::core::{ParsedResult, Parser, ParserRunner};

impl DiscardParsers for ParsersImpl {
  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { length, .. } => ParsedResult::successful((), length),
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }
}
