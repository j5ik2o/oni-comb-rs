use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Eof<I: Input>(PhantomData<fn(&mut I)>);

pub fn eof<I: Input>() -> Eof<I> {
  Eof(PhantomData)
}

impl<I: Input> Parser<I> for Eof<I> {
  type Error = I::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), I::Error> {
    if input.is_eof() {
      Ok(())
    } else {
      Err(Fail::Backtrack(I::Error::from_expected_with_location(input.offset(), input.line(), input.column(), Expected::Eof)))
    }
  }
}
