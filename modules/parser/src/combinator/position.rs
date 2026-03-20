use core::marker::PhantomData;

use crate::fail::PResult;
use crate::input_position::InputPosition;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct Position<I: InputStream>(PhantomData<fn(&mut I)>);

pub fn position<I: InputStream>() -> Position<I> {
  Position(PhantomData)
}

impl<I: InputStream> Parser<I> for Position<I> {
  type Error = I::Error;
  type Output = InputPosition;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<InputPosition, I::Error> {
    Ok(input.position())
  }
}
