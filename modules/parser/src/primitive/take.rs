use core::marker::PhantomData;

use crate::error::ParseError;
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
  type Error = ParseError;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, ParseError> {
    let pos = input.offset();
    let cp = input.checkpoint();
    for _ in 0..self.n {
      if input.next_token().is_none() {
        input.reset(cp);
        return Err(Fail::Backtrack(ParseError::expected_description(pos, "enough input")));
      }
    }
    Ok(input.slice_since(cp))
  }
}
