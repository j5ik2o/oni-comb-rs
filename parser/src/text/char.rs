use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Char(pub char);

impl Parser<StrInput<'_>> for Char {
    type Output = char;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        let remaining = input.remaining();
        match remaining.chars().next() {
            Some(c) if c == self.0 => {
                input.advance(c.len_utf8());
                Ok(c)
            }
            Some(c) => Err(Fail::Backtrack(format!(
                "expected '{}', found '{}'",
                self.0, c
            ))),
            None => Err(Fail::Backtrack(format!(
                "expected '{}', found EOF",
                self.0
            ))),
        }
    }
}
