use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Satisfy<F>(F);

pub fn satisfy<F: FnMut(char) -> bool>(f: F) -> Satisfy<F> {
    Satisfy(f)
}

impl<'a, F> Parser<StrInput<'a>> for Satisfy<F>
where
    F: FnMut(char) -> bool,
{
    type Output = char;
    type Error = ParseError;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<char, ParseError> {
        let pos = input.offset();
        let remaining = input.as_str();
        match remaining.chars().next() {
            Some(c) if (self.0)(c) => {
                input.advance(c.len_utf8());
                Ok(c)
            }
            _ => Err(Fail::Backtrack(ParseError::expected_description(
                pos, "satisfy",
            ))),
        }
    }
}
