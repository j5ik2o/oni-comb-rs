use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Many1Fold<P, B, F> {
  pub(crate) parser: P,
  pub(crate) init: B,
  pub(crate) f: F,
}

impl<I, P, B, F, Acc> Parser<I> for Many1Fold<P, B, F>
where
  I: Input,
  P: Parser<I>,
  B: FnMut() -> Acc,
  F: FnMut(Acc, P::Output) -> Acc,
{
  type Error = P::Error;
  type Output = Acc;

  #[inline(always)]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let first = self.parser.parse_next(input)?;
    let mut acc = (self.f)((self.init)(), first);
    loop {
      let cp = input.checkpoint();
      match self.parser.parse_next(input) {
        Ok(v) => {
          if input.checkpoint() == cp {
            return Err(Fail::ZeroProgress);
          }
          acc = (self.f)(acc, v);
        }
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          return Ok(acc);
        }
        Err(Fail::Cut(e)) => return Err(Fail::Cut(e)),
        Err(Fail::Incomplete) => return Err(Fail::Incomplete),
        Err(Fail::ZeroProgress) => return Err(Fail::ZeroProgress),
      }
    }
  }
}
