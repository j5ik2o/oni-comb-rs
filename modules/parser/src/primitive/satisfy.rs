use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Satisfy<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn satisfy<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> Satisfy<F, I> {
  Satisfy(f, PhantomData)
}

impl<I: Input, F> Parser<I> for Satisfy<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if (self.0)(t) => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_expected(
          pos,
          Expected::Description("satisfy"),
        )))
      }
      None => Err(Fail::Backtrack(I::Error::from_expected(
        pos,
        Expected::Description("satisfy"),
      ))),
    }
  }
}
