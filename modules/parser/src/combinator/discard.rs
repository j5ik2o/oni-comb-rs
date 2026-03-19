use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Discard<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Discard<P>
where
  I: InputStream,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), P::Error> {
    self.parser.parse_next(input).map(|_| ())
  }
}
