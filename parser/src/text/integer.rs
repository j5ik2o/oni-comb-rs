use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Integer;

pub fn integer() -> Integer {
    Integer
}

impl<'a> Parser<StrInput<'a>> for Integer {
    type Output = i64;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<i64, String> {
        let remaining = input.as_str();
        let mut consumed = 0;

        // optional leading '-'
        let mut chars = remaining.chars();
        if let Some('-') = chars.next() {
            consumed += 1;
        }

        // at least one digit required
        let digit_start = consumed;
        for c in remaining[consumed..].chars() {
            if c.is_ascii_digit() {
                consumed += c.len_utf8();
            } else {
                break;
            }
        }

        if consumed == digit_start {
            return Err(Fail::Backtrack(
                "integer: expected digit".to_string(),
            ));
        }

        let s = &remaining[..consumed];
        let value = s.parse::<i64>().map_err(|e| {
            Fail::Backtrack(format!("integer: {}", e))
        })?;
        input.advance(consumed);
        Ok(value)
    }
}
