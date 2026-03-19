use alloc::format;
use alloc::string::String;
use core::fmt;

use regex_automata::meta::Regex;
use regex_automata::{Anchored, Input as ReInput};

use crate::error::{ExpectError, Expected, ParseError};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

/// Error returned when a regex pattern fails to compile.
#[derive(Debug)]
pub struct RegexBuildError {
  pattern: String,
  message: String,
}

impl RegexBuildError {
  pub fn pattern(&self) -> &str {
    &self.pattern
  }

  pub fn message(&self) -> &str {
    &self.message
  }
}

impl fmt::Display for RegexBuildError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "invalid regex pattern '{}': {}", self.pattern, self.message)
  }
}

/// A parser that matches a regular expression at the current input position.
///
/// Created via [`regex()`]. Returns `&'a str` (zero-copy) on success.
#[derive(Debug)]
pub struct RegexParser {
  re: Regex,
}

/// Create a parser that matches the given regex pattern at the current position.
///
/// The match is anchored at the start of remaining input — the regex does not
/// search forward. Returns `&'a str` (zero-copy slice of input).
///
/// # Errors
///
/// Returns `Err(RegexBuildError)` if the regex pattern is invalid.
///
/// # Examples
///
/// ```ignore
/// use oni_comb_parser::prelude::*;
///
/// let mut p = regex(r"[0-9]+").unwrap();
/// let mut input = StrInput::new("123abc");
/// assert_eq!(p.parse_next(&mut input).unwrap(), "123");
/// assert_eq!(input.remaining(), "abc");
/// ```
pub fn regex(pattern: &str) -> Result<RegexParser, RegexBuildError> {
  let re = Regex::new(pattern).map_err(|e| RegexBuildError {
    pattern: String::from(pattern),
    message: format!("{}", e),
  })?;
  Ok(RegexParser { re })
}

impl<'a> Parser<StrInput<'a>> for RegexParser {
  type Error = ParseError;
  type Output = &'a str;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<&'a str, ParseError> {
    let pos = input.offset();
    let remaining = input.remaining();
    let re_input = ReInput::new(remaining).anchored(Anchored::Yes);
    match self.re.find(re_input) {
      Some(m) => {
        let matched = &remaining[..m.len()];
        input.advance(m.len());
        Ok(matched)
      }
      None => Err(Fail::Backtrack(ParseError::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        Expected::Description("regex match"),
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser_ext::ParserExt;
  use crate::prelude::{eof, tag};

  #[test]
  fn regex_digits() {
    let mut p = regex(r"[0-9]+").unwrap();
    let mut input = StrInput::new("123abc");
    assert_eq!(p.parse_next(&mut input).unwrap(), "123");
    assert_eq!(input.remaining(), "abc");
  }

  #[test]
  fn regex_no_match() {
    let mut p = regex(r"[0-9]+").unwrap();
    let mut input = StrInput::new("abc");
    assert!(p.parse_next(&mut input).is_err());
  }

  #[test]
  fn regex_anchored_at_start() {
    let mut p = regex(r"[0-9]+").unwrap();
    let mut input = StrInput::new("abc123");
    assert!(p.parse_next(&mut input).is_err());
  }

  #[test]
  fn regex_empty_match() {
    let mut p = regex(r"[0-9]*").unwrap();
    let mut input = StrInput::new("abc");
    assert_eq!(p.parse_next(&mut input).unwrap(), "");
  }

  #[test]
  fn regex_full_input() {
    let mut p = regex(r"[a-z]+").unwrap().zip_left(eof());
    let mut input = StrInput::new("hello");
    assert_eq!(p.parse_next(&mut input).unwrap(), "hello");
  }

  #[test]
  fn regex_combined_with_tag() {
    let mut p = tag("key=").zip_right(regex(r"[a-zA-Z0-9]+").unwrap());
    let mut input = StrInput::new("key=value123");
    assert_eq!(p.parse_next(&mut input).unwrap(), "value123");
  }

  #[test]
  fn regex_with_or() {
    let mut p = regex(r"[0-9]+").unwrap().or(regex(r"[a-z]+").unwrap());
    let mut input = StrInput::new("hello");
    assert_eq!(p.parse_next(&mut input).unwrap(), "hello");
  }

  #[test]
  fn regex_invalid_pattern() {
    let err = regex(r"[invalid").unwrap_err();
    assert_eq!(err.pattern(), "[invalid");
    assert!(!err.message().is_empty());
  }
}
