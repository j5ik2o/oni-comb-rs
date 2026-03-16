use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct TakeWhile1<F>(F);

pub fn take_while1<F: FnMut(char) -> bool>(f: F) -> TakeWhile1<F> {
    TakeWhile1(f)
}

impl<'a, F> Parser<StrInput<'a>> for TakeWhile1<F>
where
    F: FnMut(char) -> bool,
{
    type Output = &'a str;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, String> {
        let remaining = input.as_str();
        let mut consumed = 0;
        for c in remaining.chars() {
            if (self.0)(c) {
                consumed += c.len_utf8();
            } else {
                break;
            }
        }
        if consumed == 0 {
            return Err(Fail::Backtrack(
                "take_while1: expected at least one matching character".to_string(),
            ));
        }
        input.advance(consumed);
        Ok(&remaining[..consumed])
    }
}
