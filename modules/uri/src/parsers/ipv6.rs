use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;
use std::net::Ipv6Addr;

/// Parse IPv6 address per RFC 3986.
/// Collects hex digits, ':', and '.' then delegates to std::net::Ipv6Addr.
pub fn ipv6_address<'a>() -> impl Parser<StrInputStream<'a>, Output = Ipv6Addr, Error = ParseError> {
  take_while1(|c: char| c.is_ascii_hexdigit() || c == ':' || c == '.')
    .map_res(|s: &str| s.parse::<Ipv6Addr>(), "IPv6address")
    .attempt()
}
