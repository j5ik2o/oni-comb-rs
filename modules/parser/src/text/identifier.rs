use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

pub struct Identifier;

pub fn identifier() -> Identifier {
  Identifier
}

impl<'a> Parser<StrInputStream<'a>> for Identifier {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<&'a str, Self::Error> {
    let remaining = input.as_str();
    let mut chars = remaining.chars();

    match chars.next() {
      Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
      _ => {
        return Err(Fail::Backtrack(Self::Error::from_position(
          input.position(),
          Expected::Description("identifier"),
        )));
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
