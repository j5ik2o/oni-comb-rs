use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct NotA<F, I: InputStream>(F, PhantomData<fn(&mut I)>);

pub fn not_a<I: InputStream, F: FnMut(I::Token) -> bool>(f: F) -> NotA<F, I> {
  NotA(f, PhantomData)
}

impl<I: InputStream, F> Parser<I> for NotA<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if !(self.0)(t) => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_position(
          input.position(),
          Expected::Description("not_a"),
        )))
      }
      None => Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Description("not_a"),
      ))),
    }
  }
}
