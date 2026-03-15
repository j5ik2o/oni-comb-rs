use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Tag(pub &'static str);

impl Parser<StrInput<'_>> for Tag {
    type Output = &'static str;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'_>) -> PResult<Self::Output, Self::Error> {
        let remaining = input.remaining();
        if remaining.starts_with(self.0) {
            input.advance(self.0.len());
            Ok(self.0)
        } else {
            Err(Fail::Backtrack(format!(
                "expected \"{}\", found \"{}\"",
                self.0,
                &remaining[..remaining.len().min(self.0.len())]
            )))
        }
    }
}
