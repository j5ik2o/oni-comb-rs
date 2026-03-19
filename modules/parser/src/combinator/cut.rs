use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Cut<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Cut<P>
where
  I: InputStream,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = P::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    match self.parser.parse_next(input) {
      Ok(v) => Ok(v),
      Err(Fail::Backtrack(e)) => Err(Fail::Cut(e)),
      Err(other) => Err(other),
    }
  }
}
