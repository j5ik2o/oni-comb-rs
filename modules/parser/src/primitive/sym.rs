use core::marker::PhantomData;

use crate::error::ExpectError;
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Sym<I: InputStream> {
  token: I::Token,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn sym<I: InputStream>(token: I::Token) -> Sym<I> {
  Sym {
    token,
    _marker: PhantomData,
  }
}

impl<I: InputStream> Parser<I> for Sym<I>
where
  I::Token: SymExpected,
{
  type Error = I::Error;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    match input.next_token() {
      Some(t) if t == self.token => Ok(t),
      Some(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(I::Error::from_expected(pos, self.token.to_expected())))
      }
      None => Err(Fail::Backtrack(I::Error::from_expected(pos, self.token.to_expected()))),
    }
  }
}

/// Token 型から Expected への変換トレイト。
pub trait SymExpected: Copy {
  fn to_expected(self) -> crate::error::Expected;
}

impl SymExpected for char {
  #[inline]
  fn to_expected(self) -> crate::error::Expected {
    crate::error::Expected::Char(self)
  }
}

impl SymExpected for u8 {
  #[inline]
  fn to_expected(self) -> crate::error::Expected {
    crate::error::Expected::Byte(self)
  }
}
