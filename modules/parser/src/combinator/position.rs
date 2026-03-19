use core::marker::PhantomData;

use crate::fail::PResult;
use crate::input::Input;
use crate::parser::Parser;

/// パーサーの現在位置を表す構造体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputPosition {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
}

pub struct Position<I: Input>(PhantomData<fn(&mut I)>);

pub fn position<I: Input>() -> Position<I> {
  Position(PhantomData)
}

impl<I: Input> Parser<I> for Position<I> {
  type Error = I::Error;
  type Output = InputPosition;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<InputPosition, I::Error> {
    Ok(InputPosition {
      offset: input.offset(),
      line: input.line(),
      column: input.column(),
    })
  }
}
