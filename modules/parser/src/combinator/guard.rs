use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Guard<F, I: InputStream> {
  pred: F,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn guard<I: InputStream, F: Fn(&I) -> bool>(pred: F) -> Guard<F, I> {
  Guard {
    pred,
    _marker: PhantomData,
  }
}

impl<I: InputStream, F> Parser<I> for Guard<F, I>
where
  F: Fn(&I) -> bool,
{
  type Error = I::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), I::Error> {
    if (self.pred)(input) {
      Ok(())
    } else {
      Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Description("guard condition"),
      )))
    }
  }
}
