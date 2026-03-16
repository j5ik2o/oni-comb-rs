use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::models::host::Host;
use crate::parsers::common::{pct_encoded, sub_delim_char, unreserved_char};
use crate::parsers::ipv4::ipv4_address;
use crate::parsers::ipv6::ipv6_address;

// IP-literal = "[" ( IPv6address / IPvFuture ) "]"
fn ip_literal<'a>() -> impl Parser<StrInput<'a>, Output = Host<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    tag("[").parse_next(input)?;

    // Try IPv6 first
    if let Ok(addr) = ipv6_address().attempt().parse_next(input) {
      tag("]").parse_next(input)?;
      return Ok(Host::Ipv6(addr));
    }

    // IPvFuture: "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
    let cp = input.checkpoint();
    let pos = input.offset();
    let v = tag("v");
    if v.attempt().parse_next(input).is_ok() {
      let mut hexdigs = take_while1(|c: char| c.is_ascii_hexdigit());
      hexdigs.parse_next(input)?;
      tag(".").parse_next(input)?;
      let mut body = take_while1(|c: char| {
        c.is_ascii_alphanumeric()
          || matches!(
            c,
            '-' | '.' | '_' | '~' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | ':'
          )
      });
      body.parse_next(input)?;
      let future_str = input.slice_since(cp);
      tag("]").parse_next(input)?;
      return Ok(Host::IpvFuture(future_str));
    }

    Err(Fail::Backtrack(ParseError::expected_description(pos, "IP-literal")))
  })
}

// reg-name = *( unreserved / pct-encoded / sub-delims )
fn reg_name<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    loop {
      if pct_encoded().attempt().parse_next(input).is_ok() {
        continue;
      }
      if unreserved_char().attempt().parse_next(input).is_ok() {
        continue;
      }
      if sub_delim_char().attempt().parse_next(input).is_ok() {
        continue;
      }
      break;
    }
    Ok(input.slice_since(cp))
  })
}

// host = IP-literal / IPv4address / reg-name
pub fn host<'a>() -> impl Parser<StrInput<'a>, Output = Host<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    // IP-literal first (starts with '[')
    if let Ok(h) = ip_literal().attempt().parse_next(input) {
      return Ok(h);
    }
    // IPv4 — only commit if followed by a delimiter (not more host chars)
    let cp_ipv4 = input.checkpoint();
    if let Ok(addr) = ipv4_address().attempt().parse_next(input) {
      match input.peek_token() {
        None | Some(':') | Some('/') | Some('?') | Some('#') => return Ok(Host::Ipv4(addr)),
        _ => input.reset(cp_ipv4), // not a pure IPv4, fall through to reg-name
      }
    }
    // reg-name (can be empty)
    let name = reg_name().parse_next(input)?;
    Ok(Host::RegName(name))
  })
}
