use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct ZipLeft<P1, P2> {
  pub(crate) first: P1,
  pub(crate) second: P2,
}

impl<I, P1, P2> Parser<I> for ZipLeft<P1, P2>
where
  I: InputStream,
  P1: Parser<I>,
  P2: Parser<I, Error = P1::Error>,
{
  type Error = P1::Error;
  type Output = P1::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let v = self.first.parse_next(input)?;
    self.second.parse_next(input)?;
    Ok(v)
  }
}
