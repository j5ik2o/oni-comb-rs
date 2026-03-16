use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;
use std::net::Ipv4Addr;

fn dec_octet<'a>() -> impl Parser<StrInput<'a>, Output = u8, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let pos = input.offset();
    let cp = input.checkpoint();
    let mut digits = take_while1(|c: char| c.is_ascii_digit());
    let s = digits.parse_next(input)?;
    if s.len() > 3 {
      input.reset(cp);
      return Err(Fail::Backtrack(ParseError::expected_description(pos, "dec-octet")));
    }
    match s.parse::<u16>() {
      Ok(n) if n <= 255 => Ok(n as u8),
      _ => {
        input.reset(cp);
        Err(Fail::Backtrack(ParseError::expected_description(pos, "dec-octet")))
      }
    }
  })
}

pub fn ipv4_address<'a>() -> impl Parser<StrInput<'a>, Output = Ipv4Addr, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
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
