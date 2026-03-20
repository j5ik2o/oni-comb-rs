use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

pub struct Char(char);

pub fn char(c: char) -> Char {
  Char(c)
}

impl<'a> Parser<StrInputStream<'a>> for Char {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = char;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<Self::Output, Self::Error> {
    let remaining = input.remaining();
    match remaining.chars().next() {
      Some(c) if c == self.0 => {
        input.advance(c.len_utf8());
        Ok(c)
      }
      _ => Err(Fail::Backtrack(Self::Error::from_position(
        input.position(),
        Expected::Char(self.0),
      ))),
    }
  }
}
