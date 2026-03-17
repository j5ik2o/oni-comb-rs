#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::fail::PResult;
use crate::input::Input;

pub trait Parser<I: Input> {
  type Output;
  type Error;

  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error>;
}

#[cfg(feature = "alloc")]
impl<I: Input, P: Parser<I> + ?Sized> Parser<I> for Box<P> {
  type Error = P::Error;
  type Output = P::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    (**self).parse_next(input)
  }
}
