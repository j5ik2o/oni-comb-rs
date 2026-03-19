use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Tag(&'static str);

pub fn tag(s: &'static str) -> Tag {
  Tag(s)
}

impl<'a> Parser<StrInput<'a>> for Tag {
  type Error = <StrInput<'a> as Input>::Error;
  type Output = &'static str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<Self::Output, Self::Error> {
    let pos = input.offset();
    let remaining = input.remaining();
    if remaining.starts_with(self.0) {
      input.advance(self.0.len());
      Ok(self.0)
    } else {
      Err(Fail::Backtrack(Self::Error::from_expected_with_location(pos, input.line(), input.column(), Expected::Tag(self.0))))
    }
  }
}
