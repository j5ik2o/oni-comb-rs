use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Char(char);

pub fn char(c: char) -> Char {
  Char(c)
}

impl<'a> Parser<StrInput<'a>> for Char {
  type Error = <StrInput<'a> as Input>::Error;
  type Output = char;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<Self::Output, Self::Error> {
    let pos = input.offset();
    let remaining = input.remaining();
    match remaining.chars().next() {
      Some(c) if c == self.0 => {
        input.advance(c.len_utf8());
        Ok(c)
      }
      _ => Err(Fail::Backtrack(Self::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Char(self.0),
      ))),
    }
  }
}
