use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Tag(&'static str);

pub fn tag(s: &'static str) -> Tag {
    Tag(s)
}

impl Parser<StrInput<'_>> for Tag {
    type Output = &'static str;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        let pos = input.offset();
        let remaining = input.remaining();
        if remaining.starts_with(self.0) {
            input.advance(self.0.len());
            Ok(self.0)
        } else {
            Err(Fail::Backtrack(ParseError::expected_tag(pos, self.0)))
        }
    }
}
