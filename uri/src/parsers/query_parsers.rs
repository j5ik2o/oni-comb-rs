use crate::models::query::Query;
use crate::parsers::basic_parsers::pchar_without_eq_amp;
use oni_comb_parser_rs::prelude::*;

//  query         = *( pchar / "/" / "?" )
pub fn query<'a>() -> Parser<'a, u8, Query> {
  let code_point = || {
    (pchar_without_eq_amp() | elm_of(b"/?").collect())
      .of_many0()
      .collect()
      .map(|e| e.to_vec())
      .map_res(String::from_utf8)
  };
  let key_values = || (code_point() + (elm(b'=') * code_point()).opt());
  (key_values() + (elm(b'&') * key_values()).of_many0()).map(|(a, b)| {
    let mut m = vec![a];
    m.extend(b);
    Query::new(m)
  })
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  fn sub_delims_without_gen_of_char() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '\'', '(', ')', '*', '+', ',', ';'])
  }

  fn sub_delims_without_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, sub_delims_without_gen_of_char())
  }

  pub fn pchar_without_eq_amp_gen(min: u8, max: u8) -> Gen<String> {
    repeat_gen_of_string(min, max, {
      Gens::choose_u8(1, 4).flat_map(|n| match n {
        1 => unreserved_gen_of_char().map(|c| c.into()),
        2 => pct_encoded_gen(),
        3 => sub_delims_without_gen_of_char().map(|c| c.into()),
        4 => Gens::one_of_vec(vec![':', '@']).map(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn query_gen() -> Gen<String> {
    Gens::list_of_n(3, {
      pchar_without_eq_amp_gen(1, 10).flat_map(|key| {
        Gens::list_of_n(2, pchar_without_eq_amp_gen(1, 10)).map(move |vl| {
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
  fn test_pchar_without_eq_amp() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(pchar_without_eq_amp_gen(1, u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, query:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (query() - end()).parse(input).to_result();
      let query = result.unwrap();
      log::debug!("{:>03}, query:object = {:?}", counter, query);
      assert_eq!(query.to_string(), s);
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
      log::debug!("{:>03}, query:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (query() - end()).parse(input).to_result();
      let query = result.unwrap();
      log::debug!("{:>03}, query:object = {:?}", counter, query);
      assert_eq!(query.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
