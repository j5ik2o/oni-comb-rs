use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct MapRes<P, F> {
  pub(crate) parser: P,
  pub(crate) f: F,
  pub(crate) label: &'static str,
}

impl<I, P, F, O2, E2> Parser<I> for MapRes<P, F>
where
  I: Input,
  P: Parser<I>,
  P::Error: ExpectError,
  F: FnMut(P::Output) -> Result<O2, E2>,
{
  type Error = P::Error;
  type Output = O2;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let pos = input.offset();
    let v = self.parser.parse_next(input)?;
    match (self.f)(v) {
      Ok(o) => Ok(o),
      Err(_) => Err(Fail::Backtrack(Self::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Description(self.label),
      ))),
    }
  }
}
