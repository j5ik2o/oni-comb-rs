mod basic_parsers;
mod host_parsers;
mod ip_v4_address_parsers;
mod ip_v6_address_parsers;
mod path_parsers;
mod user_info_parsers;

use crate::parsers::basic_parsers::{pchar, pct_encoded, sub_delims, unreserved};

use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

fn port<'a>() -> Parser<'a, char, u16> {
  elm_digit()
    .of_many0()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u16>())
    .name("port")
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
  use crate::parsers::path_parsers::path;

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
