use crate::core::{ParsedResult, Parser, ParserRunner};
use crate::extension::parsers::OffsetParsers;
use crate::internal::ParsersImpl;

impl OffsetParsers for ParsersImpl {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParsedResult::successful(ps.last_offset().unwrap_or(0), length)
      }
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParsedResult::successful(ps.next_offset(), length)
      }
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }
}
