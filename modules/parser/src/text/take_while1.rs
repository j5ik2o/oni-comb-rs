use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
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
  type Error = ParseError;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
    let pos = input.offset();
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
      return Err(Fail::Backtrack(ParseError::expected_description(
        pos,
        "at least one matching character",
      )));
    }
    input.advance(consumed);
    Ok(&remaining[..consumed])
  }
}
