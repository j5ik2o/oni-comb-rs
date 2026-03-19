use crate::error::MergeError;
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Or<P1, P2> {
  pub(crate) left: P1,
  pub(crate) right: P2,
}

impl<I, P1, P2> Parser<I> for Or<P1, P2>
where
  I: InputStream,
  P1: Parser<I>,
  P1::Error: MergeError,
  P2: Parser<I, Output = P1::Output, Error = P1::Error>,
{
  type Error = P1::Error;
  type Output = P1::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let cp = input.checkpoint();
    match self.left.parse_next(input) {
      Ok(v) => Ok(v),
      Err(Fail::Backtrack(left_err)) => {
        input.reset(cp);
        match self.right.parse_next(input) {
          ok @ Ok(_) => ok,
          Err(Fail::Backtrack(right_err)) => Err(Fail::Backtrack(left_err.merge(right_err))),
          err => err,
        }
      }
      Err(Fail::Cut(e)) => Err(Fail::Cut(e)),
      Err(Fail::Incomplete) => Err(Fail::Incomplete),
      Err(Fail::ZeroProgress) => Err(Fail::ZeroProgress),
    }
  }
}
