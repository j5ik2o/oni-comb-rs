use crate::core::{ParsedError, ParsedResult, Parser};
use crate::extension::parsers::SkipParsers;
use crate::internal::ParsersImpl;

impl SkipParsers for ParsersImpl {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParsedResult::successful((), n)
      } else {
        ParsedResult::failed_with_un_commit(ParsedError::of_in_complete())
      }
    })
  }
}
