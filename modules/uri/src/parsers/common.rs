use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

fn is_unreserved(c: char) -> bool {
  c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_' | '~')
}

fn is_sub_delim(c: char) -> bool {
  matches!(c, '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=')
}

// pct-encoded = "%" HEXDIG HEXDIG
pub fn pct_encoded<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    let mut pct = tag("%");
    pct.parse_next(input)?;
    let mut h1 = satisfy(|c: char| c.is_ascii_hexdigit());
    let mut h2 = satisfy(|c: char| c.is_ascii_hexdigit());
    h1.parse_next(input)?;
    h2.parse_next(input)?;
    Ok(input.slice_since(cp))
  })
}

// unreserved = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub fn unreserved_char<'a>() -> impl Parser<StrInput<'a>, Output = char, Error = ParseError> {
  satisfy(is_unreserved)
}

// sub-delims
pub fn sub_delim_char<'a>() -> impl Parser<StrInput<'a>, Output = char, Error = ParseError> {
  satisfy(is_sub_delim)
}

// pchar = unreserved / pct-encoded / sub-delims / ":" / "@"
pub fn pchar<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    if pct_encoded().attempt().parse_next(input).is_ok() {
      return Ok(input.slice_since(cp));
    }
    if satisfy(|c: char| is_unreserved(c) || is_sub_delim(c) || c == ':' || c == '@')
      .attempt()
      .parse_next(input)
      .is_ok()
    {
      return Ok(input.slice_since(cp));
    }
    Err(oni_comb_parser::fail::Fail::Backtrack(ParseError::from_expected_with_location(
      input.offset(),
      input.line(),
      input.column(),
      Expected::Description("pchar"),
    )))
  })
}
