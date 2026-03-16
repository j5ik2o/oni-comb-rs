use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct TakeWhileNM<F> {
  min: usize,
  max: usize,
  f: F,
}

/// 述語を満たす文字を最小 `min` 個、最大 `max` 個消費する。
///
/// `min` 個未満しかマッチしない場合は `Backtrack` エラーを返す。
/// `max` 個に達するかマッチしなくなった時点で停止する。
///
/// ```
/// use oni_comb_parser::prelude::*;
///
/// let mut p = take_while_n_m(2, 4, |c: char| c.is_ascii_digit());
/// let mut input = StrInput::new("123abc");
/// assert_eq!(p.parse_next(&mut input).unwrap(), "123");
///
/// let mut p = take_while_n_m(2, 4, |c: char| c.is_ascii_digit());
/// let mut input = StrInput::new("12345");
/// assert_eq!(p.parse_next(&mut input).unwrap(), "1234");
///
/// let mut p = take_while_n_m(2, 4, |c: char| c.is_ascii_digit());
/// let mut input = StrInput::new("1abc");
/// assert!(p.parse_next(&mut input).is_err());
/// ```
pub fn take_while_n_m<F: FnMut(char) -> bool>(min: usize, max: usize, f: F) -> TakeWhileNM<F> {
  TakeWhileNM { min, max, f }
}

impl<'a, F> Parser<StrInput<'a>> for TakeWhileNM<F>
where
  F: FnMut(char) -> bool,
{
  type Error = ParseError;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
    let pos = input.offset();
    let remaining = input.as_str();
    let mut consumed = 0;
    let mut count = 0;
    for c in remaining.chars() {
      if count >= self.max {
        break;
      }
      if (self.f)(c) {
        consumed += c.len_utf8();
        count += 1;
      } else {
        break;
      }
    }
    if count < self.min {
      return Err(Fail::Backtrack(ParseError::expected_description(
        pos,
        "not enough matching characters",
      )));
    }
    input.advance(consumed);
    Ok(&remaining[..consumed])
  }
}
