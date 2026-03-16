use core::marker::PhantomData;

use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct NoneOf<'s, I: Input> {
  set: &'s [I::Token],
  _marker: PhantomData<fn(&mut I)>,
}

pub fn none_of<'s, I: Input>(set: &'s [I::Token]) -> NoneOf<'s, I> {
  NoneOf {
    set,
    _marker: PhantomData,
  }
}

impl<'s, I: Input> Parser<I> for NoneOf<'s, I> {
  type Error = ParseError;
  type Output = I::Token;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Token, ParseError> {
    let pos = input.offset();
    match input.peek_token() {
      Some(t) if !self.set.contains(&t) => {
        input.next_token();
        Ok(t)
      }
      _ => Err(Fail::Backtrack(ParseError::expected_description(pos, "none of set"))),
    }
  }
}
