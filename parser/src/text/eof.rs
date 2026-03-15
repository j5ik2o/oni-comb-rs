use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Eof;

impl Parser<StrInput<'_>> for Eof {
    type Output = ();
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        if input.is_eof() {
            Ok(())
        } else {
            Err(Fail::Backtrack(format!(
                "expected EOF, found \"{}\"",
                input.remaining()
            )))
        }
    }
}
