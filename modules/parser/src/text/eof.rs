use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Eof;

pub fn eof() -> Eof {
    Eof
}

impl Parser<StrInput<'_>> for Eof {
    type Output = ();
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<(), ParseError> {
        if input.is_eof() {
            Ok(())
        } else {
            Err(Fail::Backtrack(ParseError::expected_eof(input.offset())))
        }
    }
}
