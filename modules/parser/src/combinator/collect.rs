use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

pub struct Collect<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Collect<P>
where
  I: Input,
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
