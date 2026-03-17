use alloc::string::String;

use crate::error::{ExpectError, Expected, ParseError};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Escaped<F> {
  open: char,
  close: char,
  escape: char,
  handler: F,
}

pub fn escaped<F>(open: char, close: char, escape: char, handler: F) -> Escaped<F>
where
  F: FnMut(char) -> Option<char>, {
  Escaped {
    open,
    close,
    escape,
    handler,
  }
}

impl<'a, F> Parser<StrInput<'a>> for Escaped<F>
where
  F: FnMut(char) -> Option<char>,
{
  type Error = ParseError;
  type Output = String;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<String, ParseError> {
    let pos = input.offset();
    let remaining = input.as_str();
    let mut chars = remaining.chars();

    // opening delimiter
    match chars.next() {
      Some(c) if c == self.open => {}
      _ => {
        return Err(Fail::Backtrack(ParseError::from_expected(
          pos,
          Expected::Char(self.open),
        )));
      }
    }

    let mut result = String::new();
    let mut consumed = self.open.len_utf8();

    loop {
      match chars.next() {
        Some(c) if c == self.close => {
          consumed += c.len_utf8();
          input.advance(consumed);
          return Ok(result);
        }
        Some(c) if c == self.escape => {
          consumed += c.len_utf8();
          match chars.next() {
            Some(next) => {
              consumed += next.len_utf8();
              match (self.handler)(next) {
                Some(replacement) => result.push(replacement),
                None => {
                  return Err(Fail::Cut(ParseError::from_expected(
                    pos + consumed - next.len_utf8(),
                    Expected::Description("valid escape sequence"),
                  )));
                }
              }
            }
            None => {
              return Err(Fail::Cut(ParseError::from_expected(
                pos + consumed,
                Expected::Description("escape character"),
              )));
            }
          }
        }
        Some(c) => {
          consumed += c.len_utf8();
          result.push(c);
        }
        None => {
          return Err(Fail::Cut(ParseError::from_expected(
            pos + consumed,
            Expected::Char(self.close),
          )));
        }
      }
    }
  }
}
