use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;
use std::net::Ipv6Addr;

/// Parse IPv6 address per RFC 3986.
/// Collects hex digits, ':', and '.' then delegates to std::net::Ipv6Addr.
pub fn ipv6_address<'a>() -> impl Parser<StrInput<'a>, Output = Ipv6Addr, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let pos = input.offset();
    let cp = input.checkpoint();
    let mut text = take_while1(|c: char| c.is_ascii_hexdigit() || c == ':' || c == '.');
    let s = text.parse_next(input)?;
    match s.parse::<Ipv6Addr>() {
      Ok(addr) => Ok(addr),
      Err(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(ParseError::expected_description(pos, "IPv6address")))
      }
    }
  })
}
