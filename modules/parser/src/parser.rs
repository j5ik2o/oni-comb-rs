#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::fail::PResult;
use crate::input_stream::InputStream;

pub trait Parser<I: InputStream> {
  type Output;
  type Error;

  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error>;
}

#[cfg(feature = "alloc")]
impl<I: InputStream, P: Parser<I> + ?Sized> Parser<I> for Box<P> {
  type Error = P::Error;
  type Output = P::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    (**self).parse_next(input)
  }
}
