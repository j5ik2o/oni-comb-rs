use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Not<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Not<P>
where
  I: InputStream,
  P: Parser<I>,
  P::Error: ExpectError,
{
  type Error = P::Error;
  type Output = ();

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<(), P::Error> {
    let cp = input.checkpoint();
    match self.parser.parse_next(input) {
      Ok(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(P::Error::from_position(
          input.position(),
          Expected::Description("not"),
        )))
      }
      Err(Fail::Backtrack(_)) => {
        input.reset(cp);
        Ok(())
      }
      Err(e @ Fail::Cut(_)) => Err(e),
      Err(Fail::Incomplete) => Err(Fail::Incomplete),
      Err(Fail::ZeroProgress) => Ok(()),
    }
  }
}
