use crate::core::{CommittedStatus, ParseError, ParseResult, StaticParser};
use crate::extension::parsers::StaticPrimitiveParsers;
use crate::internal::static_parsers_impl::StaticParsersImpl;
use std::fmt::{Debug, Display};

impl StaticPrimitiveParsers for StaticParsersImpl {
  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a + Clone, {
    StaticParser::new(move |state| {
      let input = state.input();
      if input.is_empty() {
        ParseResult::successful((), 0)
      } else {
        ParseResult::failed(
          ParseError::of_custom(state.next_offset(), None, format!("Unexpected input: {:?}", input[0])),
          CommittedStatus::Uncommitted,
        )
      }
    })
  }

  fn empty<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + 'a, {
    StaticParser::new(move |_| ParseResult::successful((), 0))
  }
}
