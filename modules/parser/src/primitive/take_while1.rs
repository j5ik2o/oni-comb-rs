use core::marker::PhantomData;

use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeWhile1<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn take_while1<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> TakeWhile1<F, I> {
  TakeWhile1(f, PhantomData)
}

impl<I: Input, F> Parser<I> for TakeWhile1<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = ParseError;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
    let pos = input.offset();
    let cp = input.checkpoint();
    while let Some(t) = input.peek_token() {
      if (self.0)(t) {
        input.next_token();
      } else {
        break;
      }
    }
    if input.checkpoint() == cp {
      return Err(Fail::Backtrack(ParseError::expected_description(
        pos,
        "at least one matching token",
      )));
    }
    Ok(input.slice_since(cp))
  }
}
