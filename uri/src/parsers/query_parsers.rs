use crate::models::query::Query;
use crate::parsers::basic_parsers::{pchar, pchar_without_eq_ampersand};
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

//  query         = *( pchar / "/" / "?" )
fn query<'a>() -> Parser<'a, char, Query> {
  let code_point = || {
    (pchar_without_eq_ampersand() | elm_of("/?").collect())
      .of_many0()
      .collect()
      .map(String::from_iter)
  };
  let key_values = || (code_point() + (elm('=') * code_point()).opt());
  (key_values() + (elm('&') * key_values()).of_many0()).map(|(a, b)| {
    let mut m = vec![a];
    m.extend(b);
    Query::new(m)
  })
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  fn sub_delims_without_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '\'', '(', ')', '*', '+', ',', ';'])
  }

  fn sub_delims_without_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, sub_delims_without_char_gen())
  }

  pub fn pchar_without_eq_and_str_gen(min: u8, max: u8) -> Gen<String> {
    rep_str_gen(min, max, {
      Gens::choose_u8(1, 4).flat_map(|n| match n {
        1 => unreserved_char_gen().map(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_without_char_gen().map(|c| c.into()),
        4 => Gens::one_of_vec(vec![':', '@']).map(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn query_gen() -> Gen<String> {
    Gens::list_of_n(3, {
      pchar_without_eq_and_str_gen(1, 10).flat_map(|key| {
        Gens::list_of_n(2, pchar_without_eq_and_str_gen(1, 10)).map(move |vl| {
          let kvl = vl.into_iter().map(|v| format!("{}={}", key, v)).collect::<Vec<_>>();
          kvl.join("&")
        })
      })
    })
    .map(|v| v.join("&"))
  }
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::iter::FromIterator;

  use anyhow::Result;

  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use super::gens::*;
  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_pchar_without_eq_and() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(pchar_without_eq_and_str_gen(1, u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (query() - end())
        .collect()
        .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_query() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(query_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, query = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (query() - end()).parse(&input).to_result();
      assert_eq!(result.unwrap().to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
