use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Collect<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Collect<P>
where
  I: InputStream,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, P::Error> {
    let cp = input.checkpoint();
    self.parser.parse_next(input)?;
    Ok(input.slice_since(cp))
  }
}
