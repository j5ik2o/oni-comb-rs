use crate::core::{ParseError, ParseResult, Parser, Parsers};
use crate::extension::parsers::PrimitiveParsers;
use crate::internal::ParsersImpl;
use std::fmt::{Debug, Display};

impl PrimitiveParsers for ParsersImpl {
  #[inline]
  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(actual) = input.get(0) {
        let msg = format!("expect end of input, found: {}", actual);
        let ps = parse_state.advance_by(1);
        let pe = ParseError::of_mismatch(input, ps.current_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      } else {
        ParseResult::successful((), 0)
      }
    })
  }

  #[inline]
  fn empty<'a, I>() -> Self::P<'a, I, ()> {
    Self::unit()
  }
}
