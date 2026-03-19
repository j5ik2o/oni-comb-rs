use alloc::vec::Vec;

use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Many<P> {
  pub(crate) parser: P,
}

impl<I, P> Parser<I> for Many<P>
where
  I: InputStream,
  P: Parser<I>,
{
  type Error = P::Error;
  type Output = Vec<P::Output>;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let mut items = Vec::new();
    loop {
      let cp = input.checkpoint();
      match self.parser.parse_next(input) {
        Ok(v) => {
          if input.checkpoint() == cp {
            return Err(Fail::ZeroProgress);
          }
          items.push(v);
        }
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          return Ok(items);
        }
        Err(Fail::Cut(e)) => return Err(Fail::Cut(e)),
        Err(Fail::Incomplete) => return Err(Fail::Incomplete),
        Err(Fail::ZeroProgress) => return Err(Fail::ZeroProgress),
      }
    }
  }
}
