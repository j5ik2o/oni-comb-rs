use core::marker::PhantomData;

use crate::error::ParseError;
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
  type Error = ParseError;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, ParseError> {
    let pos = input.offset();
    match input.peek_token() {
      Some(t) if (self.0)(t) => {
        input.next_token();
        Ok(t)
      }
      _ => Err(Fail::Backtrack(ParseError::expected_description(pos, "satisfy"))),
    }
  }
}
