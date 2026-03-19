use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeWhileNM<F, I: Input> {
  min: usize,
  max: usize,
  f: F,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn take_while_n_m<I: Input, F: FnMut(I::Token) -> bool>(min: usize, max: usize, f: F) -> TakeWhileNM<F, I> {
  TakeWhileNM {
    min,
    max,
    f,
    _marker: PhantomData,
  }
}

impl<I: Input, F> Parser<I> for TakeWhileNM<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    let mut count = 0;
    while count < self.max {
      let item_cp = input.checkpoint();
      match input.next_token() {
        Some(t) if (self.f)(t) => {
          count += 1;
        }
        Some(_) => {
          input.reset(item_cp);
          break;
        }
        None => break,
      }
    }
    if count < self.min {
      input.reset(cp);
      return Err(Fail::Backtrack(I::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Description("not enough matching tokens"),
      )));
    }
    Ok(input.slice_since(cp))
  }
}
