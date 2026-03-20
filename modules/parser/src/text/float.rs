use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

pub struct Float;

pub fn float() -> Float {
  Float
}

/// RFC 8259 準拠の数値パーサー。
/// number = [ "-" ] int [ frac ] [ exp ]
/// int    = "0" | ( digit1-9 *digit )
/// frac   = "." 1*digit
/// exp    = ("e" | "E") ["+" | "-"] 1*digit
impl<'a> Parser<StrInputStream<'a>> for Float {
  type Error = <StrInputStream<'a> as InputStream>::Error;
  type Output = f64;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInputStream<'a>) -> PResult<f64, Self::Error> {
    let remaining = input.as_str();
    let bytes = remaining.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    // optional leading '-'
    if i < len && bytes[i] == b'-' {
      i += 1;
    }

    // int part (required)
    if i >= len || !bytes[i].is_ascii_digit() {
      return Err(Fail::Backtrack(Self::Error::from_position(
        input.position(),
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
    let value = s.parse::<f64>().map_err(|_| {
      Fail::Backtrack(Self::Error::from_position(
        input.position(),
        Expected::Description("number"),
      ))
    })?;
    input.advance(i);
    Ok(value)
  }
}
