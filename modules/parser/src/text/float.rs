use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct Float;

pub fn float() -> Float {
  Float
}

/// RFC 8259 準拠の数値パーサー。
/// number = [ "-" ] int [ frac ] [ exp ]
/// int    = "0" | ( digit1-9 *digit )
/// frac   = "." 1*digit
/// exp    = ("e" | "E") ["+" | "-"] 1*digit
impl<'a> Parser<StrInput<'a>> for Float {
  type Error = <StrInput<'a> as Input>::Error;
  type Output = f64;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<f64, Self::Error> {
    let pos = input.offset();
    let remaining = input.as_str();
    let bytes = remaining.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    // optional leading '-'
    if i < len && bytes[i] == b'-' {
      i += 1;
    }

    // int part (required)
    let int_start = i;
    if i >= len || !bytes[i].is_ascii_digit() {
      return Err(Fail::Backtrack(Self::Error::from_expected(
        pos,
        Expected::Description("number"),
      )));
    }

    if bytes[i] == b'0' {
      i += 1;
      // RFC 8259: leading zeros not allowed (0 must be followed by . or e or nothing)
    } else {
      // digit1-9 followed by *digit
      i += 1;
      while i < len && bytes[i].is_ascii_digit() {
        i += 1;
      }
    }

    if i == int_start {
      return Err(Fail::Backtrack(Self::Error::from_expected(
        pos,
        Expected::Description("number"),
      )));
    }

    // optional frac
    if i < len && bytes[i] == b'.' {
      i += 1;
      let frac_start = i;
      while i < len && bytes[i].is_ascii_digit() {
        i += 1;
      }
      if i == frac_start {
        // "." without digits is not valid; backtrack to before the dot
        i -= 1;
      }
    }

    // optional exp
    if i < len && (bytes[i] == b'e' || bytes[i] == b'E') {
      let exp_mark = i;
      i += 1;
      if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
      }
      let exp_start = i;
      while i < len && bytes[i].is_ascii_digit() {
        i += 1;
      }
      if i == exp_start {
        // "e" without digits is not valid; backtrack to before exp
        i = exp_mark;
      }
    }

    let s = &remaining[..i];
    let value = s
      .parse::<f64>()
      .map_err(|_| Fail::Backtrack(Self::Error::from_expected(pos, Expected::Description("number"))))?;
    input.advance(i);
    Ok(value)
  }
}
