use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::OffsetParsers;
use crate::internal::ParsersImpl;

impl OffsetParsers for ParsersImpl {
  #[inline]
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.advance_by(length);
        ParseResult::successful(ps.last_offset().unwrap_or(0), length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }

  #[inline]
  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.advance_by(length);
        ParseResult::successful(ps.current_offset(), length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}
