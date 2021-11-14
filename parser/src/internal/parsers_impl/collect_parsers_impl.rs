use crate::core::{ParsedResult, Parser, ParserRunner};
use crate::extension::parsers::CollectParsers;
use crate::internal::ParsersImpl;

impl CollectParsers for ParsersImpl {
  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { length, .. } => {
        let slice = parse_state.slice_with_len(length);
        ParsedResult::successful(slice, length)
      }
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }
}
