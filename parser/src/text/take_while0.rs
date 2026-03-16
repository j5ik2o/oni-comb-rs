use crate::error::ParseError;
use crate::fail::PResult;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct TakeWhile0<F>(F);

pub fn take_while0<F: FnMut(char) -> bool>(f: F) -> TakeWhile0<F> {
    TakeWhile0(f)
}

impl<'a, F> Parser<StrInput<'a>> for TakeWhile0<F>
where
    F: FnMut(char) -> bool,
{
    type Output = &'a str;
    type Error = ParseError;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
        let remaining = input.as_str();
        let mut consumed = 0;
        for c in remaining.chars() {
            if (self.0)(c) {
                consumed += c.len_utf8();
            } else {
                break;
            }
        }
        input.advance(consumed);
        Ok(&remaining[..consumed])
    }
}
