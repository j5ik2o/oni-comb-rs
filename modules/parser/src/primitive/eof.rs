use core::marker::PhantomData;

use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Eof<I: Input>(PhantomData<fn(&mut I)>);

pub fn eof<I: Input>() -> Eof<I> {
  Eof(PhantomData)
}

impl<I: Input> Parser<I> for Eof<I> {
  type Error = ParseError;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), ParseError> {
    if input.is_eof() {
      Ok(())
    } else {
      Err(Fail::Backtrack(ParseError::expected_eof(input.offset())))
    }
  }
}
