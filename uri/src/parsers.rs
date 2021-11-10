use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

fn port<'a>() -> Parser<'a, char, u16> {
  elm_digit()
    .of_many0()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u16>())
    .name("port")
}

fn h16<'a>() -> Parser<'a, char, &'a [char]> {
  elm_hex_digit().of_many_n_m(1, 4).collect().name("h16")
}

// https://github.com/j5ik2o/uri-rs/blob/main/src/parser/parsers/path_parsers.rs

//  path-empty    = 0<pchar>

//  path-abempty  = *( "/" segment )
fn path_abempty<'a>() -> Parser<'a, char, &'a [char]> {
  (elm('/') + segment())
    .of_many0()
      .collect()
}

//  path-absolute = "/" [ segment-nz *( "/" segment ) ]
fn path_absolute<'a>() -> Parser<'a, char, &'a [char]> {
  let p = (seqment_nz() + (elm('/') + segment()).collect().of_many0()).opt();
  (elm('/') + p).collect()
}

//  path-rootless = segment-nz *( "/" segment )
fn path_rootless<'a>() -> Parser<'a, char, &'a [char]> {
  (seqment_nz() + (elm('/') + segment()).of_many0()).collect().name("path-rootless")
}

//  path-noscheme = segment-nz-nc *( "/" segment )
fn path_nocheme<'a>() -> Parser<'a, char, &'a [char]> {
  (seqment_nz_nc() + (elm('/') + segment()).of_many0()).collect().name("path-noscheme")
}

// segment       = *pchar
fn segment<'a>() -> Parser<'a, char, &'a [char]> {
  pchar().of_many0().collect().name("segment")
}

// segment-nz    = 1*pchar
fn seqment_nz<'a>() -> Parser<'a, char, &'a [char]> {
  pchar().of_many1().collect().name("segment-nz")
}

// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
// ; non-zero-length segment without any colon ":"
fn seqment_nz_nc<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims() | elm('@').collect())
    .of_many1()
    .collect()
    .name("segment-nz-nc")
}

// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
fn pchar<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims() | elm_of(":@").collect()).collect()
}

//  query         = *( pchar / "/" / "?" )
fn query<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("query")
}

// fragment      = *( pchar / "/" / "?" )
fn fragment<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("fragment")
}

//  pct-encoded   = "%" HEXDIG HEXDIG
fn pct_encoded<'a>() -> Parser<'a, char, &'a [char]> {
  (elm('%') + elm_hex_digit() + elm_hex_digit()).collect()
}

//  unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
fn unreserved<'a>() -> Parser<'a, char, &'a [char]> {
  (elm_alpha() | elm_digit() | elm_of("-._~"))
    .collect()
    .name("unreserved")
}

//  reserved      = gen-delims / sub-delims
fn reserved<'a>() -> Parser<'a, char, &'a [char]> {
  (gen_delims() | sub_delims()).name("reserved")
}

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
fn gen_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_of(":/?#[]@").name("gen_delims").collect()
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
fn sub_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_of("!$&'()*+,;=").name("sub_delims").collect()
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
  fn test_pct_encoded() {
    let input = "%1A".chars().collect::<Vec<_>>();
    let result = (pct_encoded().of_many1() - end())
      .collect()
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), "%1A".to_string());
  }

  #[test]
  fn test_unreserved() {
    let input = "1a-._~".chars().collect::<Vec<_>>();
    let result = (unreserved().of_many1() - end())
      .collect()
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), "1a-._~".to_string());
  }

  #[test]
  fn test_reserved() {
    let input = "!$&'()*+,;=:/?#[]@".chars().collect::<Vec<_>>();
    let result = (reserved().of_many1() - end())
      .collect()
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), "!$&'()*+,;=:/?#[]@".to_string());
  }

  #[test]
  fn test_gen_delims() {
    let input = ":/?#[]@".chars().collect::<Vec<_>>();
    let result = (gen_delims().of_many1() - end())
      .collect()
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), ":/?#[]@".to_string());
  }

  #[test]
  fn test_sub_delims() {
    let input = "!$&'()*+,;=".chars().collect::<Vec<_>>();
    let result = (sub_delims().of_many1() - end())
      .collect()
      .map(String::from_iter)
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), "!$&'()*+,;=".to_string());
  }
}
