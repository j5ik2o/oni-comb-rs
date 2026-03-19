use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Take<I: Input> {
  n: usize,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn take<I: Input>(n: usize) -> Take<I> {
  Take {
    n,
    _marker: PhantomData,
  }
}

impl<I: Input> Parser<I> for Take<I> {
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    for _ in 0..self.n {
      if input.next_token().is_none() {
        input.reset(cp);
        return Err(Fail::Backtrack(I::Error::from_expected_with_location(
          pos,
          input.line(),
          input.column(),
          Expected::Description("enough input"),
        )));
      }
    }
    Ok(input.slice_since(cp))
  }
}
