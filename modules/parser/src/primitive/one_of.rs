use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct OneOf<'s, I: Input> {
  set: &'s [I::Token],
  _marker: PhantomData<fn(&mut I)>,
}

pub fn one_of<'s, I: Input>(set: &'s [I::Token]) -> OneOf<'s, I> {
  OneOf {
    set,
    _marker: PhantomData,
  }
}

impl<'s, I: Input> Parser<I> for OneOf<'s, I> {
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if self.set.contains(&t) => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_expected(
          pos,
          Expected::Description("one of set"),
        )))
      }
      None => Err(Fail::Backtrack(I::Error::from_expected(
        pos,
        Expected::Description("one of set"),
      ))),
    }
  }
}
