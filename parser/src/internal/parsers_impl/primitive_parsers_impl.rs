use crate::core::{ParseError, ParseResult, Parser, Parsers};
use crate::extension::parsers::PrimitiveParsers;
use crate::internal::ParsersImpl;
use std::fmt::{Debug, Display};

impl PrimitiveParsers for ParsersImpl {
  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a + std::clone::Clone, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(actual) = input.get(0) {
        let msg = format!("expect end of input, found: {}", actual);
        let ps = parse_state.add_offset(1);
        let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        ParseResult::successful((), 0)
      }
    })
  }

  fn empty<'a, I>() -> Self::P<'a, I, ()>
  where
    I: std::clone::Clone + 'a {
    Self::unit()
  }
}
