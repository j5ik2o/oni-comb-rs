use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;
use std::net::Ipv4Addr;

// RFC 3986 dec-octet: no leading zeros allowed
//   "0" / %x31-39 DIGIT / "1" 2DIGIT / "2" %x30-34 DIGIT / "25" %x30-35
fn dec_octet<'a>() -> impl Parser<StrInputStream<'a>, Output = u8, Error = ParseError> {
  fn_parser(|input: &mut StrInputStream<'a>| {
    let pos = input.offset();
    let cp = input.checkpoint();
    let mut digits = take_while1(|c: char| c.is_ascii_digit());
    let s = digits.parse_next(input)?;
    // Reject leading zeros: "0" is ok, "00"/"01"/"001" etc. are not
    if s.len() > 1 && s.starts_with('0') {
      input.reset(cp);
      return Err(Fail::Backtrack(ParseError::from_expected(
        pos,
        Expected::Description("dec-octet (no leading zeros)"),
      )));
    }
    match s.parse::<u16>() {
      Ok(n) if n <= 255 => Ok(n as u8),
      _ => {
        input.reset(cp);
        Err(Fail::Backtrack(ParseError::from_expected(
          pos,
          Expected::Description("dec-octet"),
        )))
      }
    }
  })
}

pub fn ipv4_address<'a>() -> impl Parser<StrInputStream<'a>, Output = Ipv4Addr, Error = ParseError> {
  fn_parser(|input: &mut StrInputStream<'a>| {
    let a = dec_octet().parse_next(input)?;
    tag(".").parse_next(input)?;
    let b = dec_octet().parse_next(input)?;
    tag(".").parse_next(input)?;
    let c = dec_octet().parse_next(input)?;
    tag(".").parse_next(input)?;
    let d = dec_octet().parse_next(input)?;
    Ok(Ipv4Addr::new(a, b, c, d))
  })
}
