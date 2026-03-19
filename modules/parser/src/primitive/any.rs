use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Any<I: Input>(PhantomData<fn(&mut I)>);

pub fn any<I: Input>() -> Any<I> {
  Any(PhantomData)
}

impl<I: Input> Parser<I> for Any<I> {
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let pos = input.offset();
    match input.next_token() {
      Some(t) => Ok(t),
      None => Err(Fail::Backtrack(I::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Description("any token"),
      ))),
    }
  }
}
