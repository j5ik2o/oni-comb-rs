use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct TakeWhileNM<F, I: InputStream> {
  min: usize,
  max: usize,
  f: F,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn take_while_n_m<I: InputStream, F: FnMut(I::Token) -> bool>(min: usize, max: usize, f: F) -> TakeWhileNM<F, I> {
  TakeWhileNM {
    min,
    max,
    f,
    _marker: PhantomData,
  }
}

impl<I: InputStream, F> Parser<I> for TakeWhileNM<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let cp = input.checkpoint();
    let mut count = 0;
    while count < self.max {
      match input.peek_token() {
        Some(t) if (self.f)(t) => {
          let _ = input.next_token();
          count += 1;
        }
        _ => break,
      }
    }
    if count < self.min {
      input.reset(cp);
      return Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Description("not enough matching tokens"),
      )));
    }
    Ok(input.slice_since(cp))
  }
}
