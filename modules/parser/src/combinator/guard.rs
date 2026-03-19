use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Guard<F, I: Input> {
  pred: F,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn guard<I: Input, F: Fn(&I) -> bool>(pred: F) -> Guard<F, I> {
  Guard {
    pred,
    _marker: PhantomData,
  }
}

impl<I: Input, F> Parser<I> for Guard<F, I>
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
      Err(Fail::Backtrack(I::Error::from_expected_with_location(
        input.offset(),
        input.line(),
        input.column(),
        Expected::Description("guard condition"),
      )))
    }
  }
}
