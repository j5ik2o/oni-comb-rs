use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct OneOf<'s, I: InputStream> {
  set: &'s [I::Token],
  _marker: PhantomData<fn(&mut I)>,
}

pub fn one_of<'s, I: InputStream>(set: &'s [I::Token]) -> OneOf<'s, I> {
  OneOf {
    set,
    _marker: PhantomData,
  }
}

impl<'s, I: InputStream> Parser<I> for OneOf<'s, I> {
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if self.set.contains(&t) => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_position(
          input.position(),
          Expected::Description("one of set"),
        )))
      }
      None => Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Description("one of set"),
      ))),
    }
  }
}
