use crate::core::{ParseError, ParseResult, Parser};
use crate::extension::parsers::SkipParsers;
use crate::internal::ParsersImpl;

impl SkipParsers for ParsersImpl {
  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful((), n)
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }
}
