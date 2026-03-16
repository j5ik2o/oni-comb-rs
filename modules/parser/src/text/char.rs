use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Char(char);

pub fn char(c: char) -> Char {
    Char(c)
}

impl Parser<StrInput<'_>> for Char {
    type Output = char;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        let pos = input.offset();
        let remaining = input.remaining();
        match remaining.chars().next() {
            Some(c) if c == self.0 => {
                input.advance(c.len_utf8());
                Ok(c)
            }
            _ => Err(Fail::Backtrack(ParseError::expected_char(pos, self.0))),
        }
    }
}
