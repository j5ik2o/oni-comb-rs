use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

fn port<'a>() -> Parser<'a, char, u16> {
  elm_digit()
    .of_many0()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u16>())
    .name("port")
}

//  IPv6address   =                            6( h16 ":" ) ls32
//                /                       "::" 5( h16 ":" ) ls32
//                / [               h16 ] "::" 4( h16 ":" ) ls32
//                / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
//                / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
//                / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
//                / [ *4( h16 ":" ) h16 ] "::"              ls32
//                / [ *5( h16 ":" ) h16 ] "::"              h16
//                / [ *6( h16 ":" ) h16 ] "::"
fn ip_v6_address<'a>() -> Parser<'a, char, &'a [char]> {
  let p1 = ((h16() + elm(':')).of_count(6) + ls32()).collect().attempt();
  let p2 = (tag("::") + (h16() + elm(':')).of_count(5) + ls32())
    .collect();
  let p3 = (h16().opt() + tag("::") + (h16() + elm(':')).of_count(4) + ls32())
    .collect();
  let p4 = (((h16() + elm(':')).of_many_n_m(0, 1) + h16()).opt() + tag("::") + (h16() + elm(':')).of_count(3) + ls32())
    .collect();
  let p5 = (((h16() + elm(':')).of_many_n_m(0, 2) + h16()).opt() + tag("::") + (h16() + elm(':')).of_count(2) + ls32())
    .collect();
  let p6 = (((h16() + elm(':')).of_many_n_m(0, 3) + h16()).opt() + tag("::") + h16() + elm(':') + ls32())
    .collect();
  let p7 = (((h16() + elm(':')).of_many_n_m(0, 4) + h16()).opt() + tag("::") + ls32())
    .collect();
  let p8 = (((h16() + elm(':')).of_many_n_m(0, 5) + h16()).opt() + tag("::") + h16())
    .collect();
  let p9 = (((h16() + elm(':')).of_many_n_m(0, 6) + h16()).opt() + tag("::"))
    .collect();
  (p1.attempt() | p2.attempt() | p3.attempt() | p4.attempt() | p5.attempt() | p6.attempt() | p7.attempt() | p8.attempt() | p9).name("ip_v6_address")
}

fn h16<'a>() -> Parser<'a, char, &'a [char]> {
  elm_hex_digit().of_many_n_m(1, 4).collect().name("h16")
}

// https://github.com/j5ik2o/uri-rs/blob/main/src/parser/parsers/path_parsers.rs
//  ls32          = ( h16 ":" h16 ) / IPv4address

fn ls32<'a>() -> Parser<'a, char, &'a [char]> {
  (h16() + elm(':') + h16()).collect() | ip_v4_address().collect()
}

// IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
fn ip_v4_address<'a>() -> Parser<'a, char, (u8, u8, u8, u8)> {
  (dec_octet() - elm('.') + dec_octet() - elm('.') + dec_octet() - elm('.') + dec_octet())
    .map(|(((a, b), c), d)| (a, b, c, d))
    .name("ip_v4_address")
}

//  dec-octet     = DIGIT                 ; 0-9
//                / %x31-39 DIGIT         ; 10-99
//                / "1" 2DIGIT            ; 100-199
//                / "2" %x30-34 DIGIT     ; 200-249
//                / "25" %x30-35          ; 250-255
fn dec_octet<'a>() -> Parser<'a, char, u8> {
  let p1 = elm_digit().collect();
  let p2 = (elm_digit_1_9() + elm_digit()).collect();
  let p3 = (elm('1') + elm_digit() + elm_digit()).collect();
  let p4 = (elm('2') + elm_of("01234") + elm_digit()).collect();
  let p5 = (elm('2') + elm('5') + elm_of("012345")).collect();

  (p5.attempt() | p4.attempt() | p3.attempt() | p2.attempt() | p1)
    .collect()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u8>())
    .name("dec-octet")
}

//  reg-name      = *( unreserved / pct-encoded / sub-delims )
fn reg_name<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims())
    .of_many0()
    .collect()
    .name("reg-name")
}

//  path          = path-abempty    ; begins with "/" or is empty
//                / path-absolute   ; begins with "/" but not "//"
//                / path-noscheme   ; begins with a non-colon segment
//                / path-rootless   ; begins with a segment
//                / path-empty      ; zero characters
fn path<'a>() -> Parser<'a, char, Option<Vec<&'a [char]>>> {
  (path_abempty(true).attempt() | path_absolute().attempt() | path_noscheme().attempt() | path_rootless())
    .opt()
    .name("path")
}

//  path-abempty  = *( "/" segment )
fn path_abempty<'a>(required: bool) -> Parser<'a, char, Vec<&'a [char]>> {
  let n = if required { 1 } else { 0 };
  ((elm('/') + segment()).collect()).repeat(n..).name("path-abempty")
}

//  path-absolute = "/" [ segment-nz *( "/" segment ) ]
fn path_absolute<'a>() -> Parser<'a, char, Vec<&'a [char]>> {
  let p = (seqment_nz() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .opt();
  (elm('/').collect() + p)
    .map(|(a, b_opt)| match b_opt {
      None => vec![a],
      Some(b) => {
        let mut l = vec![a];
        l.extend_from_slice(&b);
        l
      }
    })
    .name("path-absolute")
}

//  path-rootless = segment-nz *( "/" segment )
fn path_rootless<'a>() -> Parser<'a, char, Vec<&'a [char]>> {
  (seqment_nz() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .name("path-rootless")
}

//  path-noscheme = segment-nz-nc *( "/" segment )
fn path_noscheme<'a>() -> Parser<'a, char, Vec<&'a [char]>> {
  (seqment_nz_nc() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .name("path-noscheme")
}

// fn path_without_abempty<'a>() -> Parser<'a, char, &'a [char]> {
//   path_absolute().attempt() | path_rootless()
// }

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
  (unreserved() | pct_encoded() | sub_delims() | elm_of(":@").collect())
    .collect()
    .name("pchar")
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
  fn test_ip_v4_address() {
    let input = "255.1.10.30".chars().collect::<Vec<_>>();
    let result = ip_v4_address().parse(&input).to_result();
    assert_eq!(result.unwrap(), (1, 2, 3, 4));
  }

  #[test]
  fn test_path_empty() {
    let input = "".chars().collect::<Vec<_>>();
    let result = path().parse(&input).to_result();
    assert_eq!(result.unwrap(), None);
  }

  #[test]
  fn test_path_absolute() {
    let input = "/abc".chars().collect::<Vec<_>>();
    let result = path()
      .map(|result_opt| match result_opt {
        None => vec!["".to_string()],
        Some(results) => results.into_iter().map(String::from_iter).collect::<Vec<_>>(),
      })
      .parse(&input)
      .to_result();
    assert_eq!(result.unwrap(), vec!["/abc".to_string()]);
  }

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
