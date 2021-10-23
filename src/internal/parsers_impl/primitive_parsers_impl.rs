use std::fmt::{Debug, Display};
use crate::core::{ParseError, Parser, ParseResult, Parsers};
use crate::internal::ParsersImpl;
use crate::core::PrimitiveParsers;

impl PrimitiveParsers for ParsersImpl {
    fn end<'a, I>() -> Self::P<'a, I, ()>
        where
            I: Debug + Display + 'a, {
        Parser::new(move |parse_state| {
            let input = parse_state.input();
            if let Some(actual) = input.get(0) {
                let msg = format!("expect end of input, found: {}", actual);
                let ps = parse_state.add_offset(1);
                let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
                ParseResult::failed_with_un_commit(pe)
            } else {
                ParseResult::successful((), 0)
            }
        })
    }

    fn empty<'a, I>() -> Self::P<'a, I, ()> {
        Self::unit()
    }
}
