use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Take<I: InputStream> {
  n: usize,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn take<I: InputStream>(n: usize) -> Take<I> {
  Take {
    n,
    _marker: PhantomData,
  }
}

impl<I: InputStream> Parser<I> for Take<I> {
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let cp = input.checkpoint();
    for _ in 0..self.n {
      if input.next_token().is_none() {
        input.reset(cp);
        return Err(Fail::Backtrack(I::Error::from_position(
          input.position(),
          Expected::Description("enough input"),
        )));
      }
    }
    Ok(input.slice_since(cp))
  }
}
