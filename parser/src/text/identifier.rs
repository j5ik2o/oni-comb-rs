use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Identifier;

pub fn identifier() -> Identifier {
    Identifier
}

impl<'a> Parser<StrInput<'a>> for Identifier {
    type Output = &'a str;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, String> {
        let remaining = input.as_str();
        let mut chars = remaining.chars();

        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
            Some(c) => {
                return Err(Fail::Backtrack(format!(
                    "identifier: expected alphabetic or '_', found '{}'",
                    c
                )));
            }
            None => {
                return Err(Fail::Backtrack(
                    "identifier: unexpected EOF".to_string(),
                ));
            }
        }

        let mut consumed = remaining.chars().next().unwrap().len_utf8();
        for c in chars {
            if c.is_ascii_alphanumeric() || c == '_' {
                consumed += c.len_utf8();
            } else {
                break;
            }
        }

        input.advance(consumed);
        Ok(&remaining[..consumed])
    }
}
