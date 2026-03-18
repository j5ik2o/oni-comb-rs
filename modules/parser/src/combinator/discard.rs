use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct Discard<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Discard<P>
where
  I: Input,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), P::Error> {
    self.parser.parse_next(input).map(|_| ())
  }
}
