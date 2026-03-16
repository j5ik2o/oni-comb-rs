use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::models::authority::Authority;
use crate::models::user_info::UserInfo;
use crate::parsers::common::pct_encoded;
use crate::parsers::host::host;

// userinfo = *( unreserved / pct-encoded / sub-delims / ":" )
fn user_info<'a>() -> impl Parser<StrInput<'a>, Output = UserInfo<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    loop {
      if pct_encoded().attempt().parse_next(input).is_ok() {
        continue;
      }
      if satisfy(|c: char| {
        c.is_ascii_alphanumeric()
          || matches!(
            c,
            '-' | '.' | '_' | '~' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | ':'
          )
      })
      .attempt()
      .parse_next(input)
      .is_ok()
      {
        continue;
      }
      break;
    }
    let info_str = input.slice_since(cp);
    // Must be followed by '@'
    tag("@").parse_next(input)?;

    let (user_name, password) = match info_str.split_once(':') {
      Some((u, p)) => (u, Some(p)),
      None => (info_str, None),
    };
    Ok(UserInfo::new(user_name, password))
  })
}

// port = *DIGIT (RFC 3986: zero or more digits after ':')
// Returns Some(u16) if digits present, None if empty port (e.g., "host:")
fn port<'a>() -> impl Parser<StrInput<'a>, Output = Option<u16>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let pos = input.offset();
    let cp = input.checkpoint();
    tag(":").parse_next(input)?;
    let mut digits = take_while0(|c: char| c.is_ascii_digit());
    let s = digits.parse_next(input)?;
    if s.is_empty() {
      return Ok(None); // empty port is valid per RFC 3986
    }
    match s.parse::<u16>() {
      Ok(n) => Ok(Some(n)),
      Err(_) => {
        input.reset(cp);
        Err(Fail::Backtrack(ParseError::expected_description(pos, "port")))
      }
    }
  })
}

// authority = [ userinfo "@" ] host [ ":" port ]
pub fn authority<'a>() -> impl Parser<StrInput<'a>, Output = Authority<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let ui = user_info().attempt().parse_next(input).ok();
    let h = host().parse_next(input)?;
    let p = port().attempt().parse_next(input).ok().flatten();
    Ok(Authority::new(ui, h, p))
  })
}
