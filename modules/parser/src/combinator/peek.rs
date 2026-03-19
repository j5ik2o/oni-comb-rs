use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Peek<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Peek<P>
where
  I: InputStream,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = P::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<P::Output, P::Error> {
    let cp = input.checkpoint();
    match self.parser.parse_next(input) {
      Ok(v) => {
        input.reset(cp);
        Ok(v)
      }
      Err(e) => Err(e),
    }
  }
}
