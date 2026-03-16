use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Satisfy<F>(pub F);

impl<'a, F> Parser<StrInput<'a>> for Satisfy<F>
where
    F: FnMut(char) -> bool,
{
    type Output = char;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<char, String> {
        let remaining = input.as_str();
        match remaining.chars().next() {
            Some(c) if (self.0)(c) => {
                input.advance(c.len_utf8());
                Ok(c)
            }
            Some(c) => Err(Fail::Backtrack(format!(
                "satisfy: unexpected '{}'",
                c
            ))),
            None => Err(Fail::Backtrack("satisfy: unexpected EOF".to_string())),
        }
    }
}
