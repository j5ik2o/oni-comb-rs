use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::CollectParsers;
use crate::internal::ParsersImpl;

impl CollectParsers for ParsersImpl {
  #[inline]
  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: 'a, {
    let method = parser.method.clone();
    Parser::new(move |parse_state| match method(parse_state) {
      ParseResult::Success { length, .. } => {
        let slice = parse_state.slice_with_len(length);
        ParseResult::successful(slice, length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}
