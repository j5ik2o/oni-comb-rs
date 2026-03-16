use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Identifier;

pub fn identifier() -> Identifier {
  Identifier
}

impl<'a> Parser<StrInput<'a>> for Identifier {
  type Error = ParseError;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
    let pos = input.offset();
    let remaining = input.as_str();
    let mut chars = remaining.chars();

    match chars.next() {
      Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
      _ => {
        return Err(Fail::Backtrack(ParseError::expected_description(pos, "identifier")));
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
