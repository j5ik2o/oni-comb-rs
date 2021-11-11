mod basic_parsers;
mod host;
mod ip_v4_address_parsers;
mod ip_v6_address_parsers;

use crate::parsers::basic_parsers::{pchar, pct_encoded, sub_delims, unreserved};
use crate::parsers::ip_v4_address_parsers::ip_v4_address;
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

fn port<'a>() -> Parser<'a, char, u16> {
  elm_digit()
    .of_many0()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u16>())
    .name("port")
}

// https://github.com/j5ik2o/uri-rs/blob/main/src/parser/parsers/path_parsers.rs

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

//  query         = *( pchar / "/" / "?" )
fn query<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("query")
}

// fragment      = *( pchar / "/" / "?" )
fn fragment<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_of("/?").collect()).of_many0().collect().name("fragment")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parsers::basic_parsers::{gen_delims, reserved};

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
}
