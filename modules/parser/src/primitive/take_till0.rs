use core::marker::PhantomData;

use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeTill0<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn take_till0<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> TakeTill0<F, I> {
  TakeTill0(f, PhantomData)
}

impl<I: Input, F> Parser<I> for TakeTill0<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let cp = input.checkpoint();
    while let Some(t) = input.peek_token() {
      if (self.0)(t) {
        break;
      } else {
        input.next_token();
      }
    }
    Ok(input.slice_since(cp))
  }
}
