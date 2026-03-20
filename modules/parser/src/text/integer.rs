use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

pub struct Integer;

pub fn integer() -> Integer {
  Integer
}

impl<'a> Parser<StrInputStream<'a>> for Integer {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = i64;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<i64, Self::Error> {
    let remaining = input.as_str();
    let mut consumed = 0;

    // optional leading '-'
    if remaining.starts_with('-') {
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
      return Err(Fail::Backtrack(Self::Error::from_position(
        input.position(),
        Expected::Description("integer"),
      )));
    }

    let s = &remaining[..consumed];
    let value = s.parse::<i64>().map_err(|_| {
      Fail::Backtrack(Self::Error::from_position(
        input.position(),
        Expected::Description("integer"),
      ))
    })?;
    input.advance(consumed);
    Ok(value)
  }
}
