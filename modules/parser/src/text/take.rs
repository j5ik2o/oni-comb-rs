use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Take {
  n: usize,
}

/// 先頭から `n` 文字を消費して `&str` として返す。
///
/// 入力に `n` 文字未満しか残っていない場合は `Backtrack` エラーを返す。
///
/// ```
/// use oni_comb_parser::prelude::*;
/// use oni_comb_parser::input::Input;
///
/// let mut p = take(3);
/// let mut input = StrInput::new("abcdef");
/// assert_eq!(p.parse_next(&mut input).unwrap(), "abc");
/// assert_eq!(input.remaining(), "def");
///
/// let mut p = take(5);
/// let mut input = StrInput::new("ab");
/// assert!(p.parse_next(&mut input).is_err());
/// ```
pub fn take(n: usize) -> Take {
  Take { n }
}

impl<'a> Parser<StrInput<'a>> for Take {
  type Error = ParseError;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
    let pos = input.offset();
    let remaining = input.as_str();
    let mut consumed = 0;
    let mut count = 0;
    for c in remaining.chars() {
      if count >= self.n {
        break;
      }
      consumed += c.len_utf8();
      count += 1;
    }
    if count < self.n {
      return Err(Fail::Backtrack(ParseError::expected_description(
        pos,
        "enough characters",
      )));
    }
    input.advance(consumed);
    Ok(&remaining[..consumed])
  }
}
