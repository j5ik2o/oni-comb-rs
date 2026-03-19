use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct NoneOf<'s, I: InputStream> {
  set: &'s [I::Token],
  _marker: PhantomData<fn(&mut I)>,
}

pub fn none_of<'s, I: InputStream>(set: &'s [I::Token]) -> NoneOf<'s, I> {
  NoneOf {
    set,
    _marker: PhantomData,
  }
}

impl<'s, I: InputStream> Parser<I> for NoneOf<'s, I> {
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if !self.set.contains(&t) => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_expected(
          pos,
          Expected::Description("none of set"),
        )))
      }
      None => Err(Fail::Backtrack(I::Error::from_expected(
        pos,
        Expected::Description("none of set"),
      ))),
    }
  }
}
