use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct SepByFold0<P, S, B, F> {
  pub(crate) parser: P,
  pub(crate) sep: S,
  pub(crate) init: B,
  pub(crate) f: F,
}

impl<I, P, S, B, F, Acc> Parser<I> for SepByFold0<P, S, B, F>
where
  I: InputStream,
  P: Parser<I>,
  S: Parser<I, Error = P::Error>,
  B: FnMut() -> Acc,
  F: FnMut(Acc, P::Output) -> Acc,
{
  type Error = P::Error;
  type Output = Acc;

  #[inline(always)]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let mut acc = (self.init)();

    // 最初の要素
    let cp = input.checkpoint();
    match self.parser.parse_next(input) {
      Ok(v) => acc = (self.f)(acc, v),
      Err(Fail::Backtrack(_)) => {
        input.reset(cp);
        return Ok(acc);
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
        Ok(v) => acc = (self.f)(acc, v),
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
    }

    Ok(acc)
  }
}

pub struct SepByFold1<P, S, B, F> {
  pub(crate) parser: P,
  pub(crate) sep: S,
  pub(crate) init: B,
  pub(crate) f: F,
}

impl<I, P, S, B, F, Acc> Parser<I> for SepByFold1<P, S, B, F>
where
  I: InputStream,
  P: Parser<I>,
  S: Parser<I, Error = P::Error>,
  B: FnMut() -> Acc,
  F: FnMut(Acc, P::Output) -> Acc,
{
  type Error = P::Error;
  type Output = Acc;

  #[inline(always)]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    // 最初の要素は必須
    let first = self.parser.parse_next(input)?;
    let mut acc = (self.f)((self.init)(), first);

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
        Ok(v) => acc = (self.f)(acc, v),
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(e) => return Err(e),
      }
    }

    Ok(acc)
  }
}
