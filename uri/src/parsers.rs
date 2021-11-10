use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

fn port<'a>() -> Parser<'a, char, u16> {
  elm_digit()
    .of_many0()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u16>())
    .name("port")
}

fn h16<'a>() -> Parser<'a, char, String> {
  elm_hex_digit().of_many_n_m(1, 4).map(String::from_iter).name("h16")
}

//  path-noscheme = segment-nz-nc *( "/" segment )
//  path-rootless = segment-nz *( "/" segment )
//  path-empty    = 0<pchar>

//  path-abempty  = *( "/" segment )
fn path_abempty<'a>() -> Parser<'a, char, String> {
  (elm('/') + segment())
    .map(|(a, b)| format!("{}{}", a, b))
    .of_many0()
    .map(String::from_iter)
}

//  path-absolute = "/" [ segment-nz *( "/" segment ) ]
fn path_absolute<'a>() -> Parser<'a, char, String> {
  let p = (seqment_nz()
    + (elm('/') + segment())
      .map(|(a, b)| format!("{}{}", a, b))
      .of_many0()
      .map(String::from_iter))
  .map(|(a, b)| format!("{}{}", a, b));
  (elm('/') + p).map(|(a, b)| format!("{}{}", a, b))
}

// segment       = *pchar
fn segment<'a>() -> Parser<'a, char, String> {
  pchar().of_many0().map(String::from_iter).name("segment")
}

// segment-nz    = 1*pchar
fn seqment_nz<'a>() -> Parser<'a, char, String> {
  pchar().of_many1().map(String::from_iter).name("segment-nz")
}

// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
// ; non-zero-length segment without any colon ":"
fn seqment_nz_nc<'a>() -> Parser<'a, char, String> {
  (unreserved() | pct_encoded() | sub_delims() | elm('@').map(|c| c.to_string()))
    .of_many1()
    .map(String::from_iter)
    .name("segment-nz-nc")
}

// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
fn pchar<'a>() -> Parser<'a, char, String> {
  unreserved() | pct_encoded() | sub_delims() | elm_of(":@").map(|c| c.to_string())
}

//  query         = *( pchar / "/" / "?" )
fn query<'a>() -> Parser<'a, char, String> {
  (pchar() | elm_of("/?").map(|c| c.to_string()))
    .of_many0()
    .map(String::from_iter)
    .name("query")
}

// fragment      = *( pchar / "/" / "?" )
fn fragment<'a>() -> Parser<'a, char, String> {
  (pchar() | elm_of("/?").map(|c| c.to_string()))
    .of_many0()
    .map(String::from_iter)
    .name("fragment")
}

//  pct-encoded   = "%" HEXDIG HEXDIG
fn pct_encoded<'a>() -> Parser<'a, char, String> {
  (elm('%') + elm_hex_digit() + elm_hex_digit()).map(|((a, b), c)| format!("{}{}{}", a, b, c))
}

//  unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
fn unreserved<'a>() -> Parser<'a, char, String> {
  (elm_alpha() | elm_digit() | elm_of("-._~"))
    .map(|c| c.to_string())
    .name("unreserved")
}

//  reserved      = gen-delims / sub-delims
fn reserved<'a>() -> Parser<'a, char, String> {
  (gen_delims() | sub_delims()).name("reserved")
}

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
fn gen_delims<'a>() -> Parser<'a, char, String> {
  elm_of(":/?#[]@").map(|c| c.to_string()).name("gen_delims")
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
fn sub_delims<'a>() -> Parser<'a, char, String> {
  elm_of("!$&'()*+,;=").map(|c| c.to_string()).name("sub_delims")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_port() {
    let input = "123".chars().collect::<Vec<_>>();
    let result = port().parse(&input).to_result();
    assert_eq!(result.unwrap(), 123);
  }

  #[test]
  fn test_sub_delims() {
    let input = "!$&'()*+,;=".chars().collect::<Vec<_>>();
    let result = (sub_delims().of_many1() - end())
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), "!$&'()*+,;=".to_string());
  }
}
