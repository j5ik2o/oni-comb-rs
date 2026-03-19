use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeTill1<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn take_till1<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> TakeTill1<F, I> {
  TakeTill1(f, PhantomData)
}

impl<I: Input, F> Parser<I> for TakeTill1<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    while let Some(t) = input.peek_token() {
      if (self.0)(t) {
        break;
      } else {
        input.next_token();
      }
    }
    if input.checkpoint() == cp {
      return Err(Fail::Backtrack(I::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Description("at least one non-matching token"),
      )));
    }
    Ok(input.slice_since(cp))
  }
}
