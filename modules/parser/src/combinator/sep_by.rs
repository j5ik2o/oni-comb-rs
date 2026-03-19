use alloc::vec;
use alloc::vec::Vec;

use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct SepBy0<P, S> {
  pub(crate) parser: P,
  pub(crate) sep: S,
}

impl<I, P, S> Parser<I> for SepBy0<P, S>
where
  I: InputStream,
  P: Parser<I>,
  S: Parser<I, Error = P::Error>,
{
  type Error = P::Error;
  type Output = Vec<P::Output>;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let mut items = Vec::new();

    // 最初の要素
    let cp = input.checkpoint();
    match self.parser.parse_next(input) {
      Ok(v) => items.push(v),
      Err(Fail::Backtrack(_)) => {
        input.reset(cp);
        return Ok(items);
      }
      Err(e) => return Err(e),
    }

    // sep + 要素 の繰り返し
    loop {
      let cp = input.checkpoint();
      match self.sep.parse_next(input) {
        Ok(_) => {}
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
      match self.parser.parse_next(input) {
        Ok(v) => items.push(v),
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
    }

    Ok(items)
  }
}

pub struct SepBy1<P, S> {
  pub(crate) parser: P,
  pub(crate) sep: S,
}

impl<I, P, S> Parser<I> for SepBy1<P, S>
where
  I: InputStream,
  P: Parser<I>,
  S: Parser<I, Error = P::Error>,
{
  type Error = P::Error;
  type Output = Vec<P::Output>;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    // 最初の要素は必須
    let first = self.parser.parse_next(input)?;
    let mut items = vec![first];

    // sep + 要素 の繰り返し
    loop {
      let cp = input.checkpoint();
      match self.sep.parse_next(input) {
        Ok(_) => {}
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
      match self.parser.parse_next(input) {
        Ok(v) => items.push(v),
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
    }

    Ok(items)
  }
}
