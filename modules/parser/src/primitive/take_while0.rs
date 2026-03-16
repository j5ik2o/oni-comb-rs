use core::marker::PhantomData;

use crate::error::ParseError;
use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeWhile0<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn take_while0<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> TakeWhile0<F, I> {
  TakeWhile0(f, PhantomData)
}

impl<I: Input, F> Parser<I> for TakeWhile0<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = ParseError;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
    let cp = input.checkpoint();
    while let Some(t) = input.peek_token() {
      if (self.0)(t) {
        input.next_token();
      } else {
        break;
      }
    }
    Ok(input.slice_since(cp))
  }
}
