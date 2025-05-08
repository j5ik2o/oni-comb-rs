use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::OffsetParsers;
use crate::internal::ParsersImpl;

impl OffsetParsers for ParsersImpl {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match method(parse_state) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParseResult::successful(ps.last_offset().unwrap_or(0), length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match method(parse_state) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParseResult::successful(ps.current_offset(), length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}
