use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Eof<I: InputStream>(PhantomData<fn(&mut I)>);

pub fn eof<I: InputStream>() -> Eof<I> {
  Eof(PhantomData)
}

impl<I: InputStream> Parser<I> for Eof<I> {
  type Error = I::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), I::Error> {
    if input.is_eof() {
      Ok(())
    } else {
      Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Eof,
      )))
    }
  }
}
