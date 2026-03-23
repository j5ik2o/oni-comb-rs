use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

pub struct Whitespace0;

pub fn whitespace0() -> Whitespace0 {
  Whitespace0
}

impl<'a> Parser<StrInputStream<'a>> for Whitespace0 {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<&'a str, Self::Error> {
    Ok(input.consume_ascii_whitespace_prefix())
  }
}

pub struct Whitespace1;

pub fn whitespace1() -> Whitespace1 {
  Whitespace1
}

impl<'a> Parser<StrInputStream<'a>> for Whitespace1 {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<&'a str, Self::Error> {
    let matched = input.consume_ascii_whitespace_prefix();

    if matched.is_empty() {
      return Err(Fail::Backtrack(Self::Error::from_position(
        input.position(),
        Expected::Description("at least one matching token"),
      )));
    }

    Ok(matched)
  }
}
