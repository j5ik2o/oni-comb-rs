use oni_comb_parser_rs::prelude::*;

use crate::parsers::basic_parsers::{pct_encoded, sub_delims, unreserved};

// "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
pub fn ip_v_future<'a>() -> Parser<'a, char, &'a [char]> {
  (elm('v') + elm_hex_digit().of_many1() + elm('.') + (unreserved() | sub_delims() | elm(':').collect()).of_many1())
    .collect()
}

//  reg-name      = *( unreserved / pct-encoded / sub-delims )
pub fn reg_name<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved().attempt() | pct_encoded().attempt() | sub_delims())
    .of_many0()
    .collect()
    .name("reg-name")
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  use super::*;

  pub fn reg_name_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, {
      Gens::choose_u8(1, 3).flat_map(|n| match n {
        1 => unreserved_char_gen().map(|c| c.into()),
        2 => sub_delims_char_gen().map(|c| c.into()),
        3 => pct_encoded_str_gen(),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn ip_v_future_str_gen() -> Gen<String> {
    let a = rep_char_gen(5, hex_digit_char_gen());
    let b = {
      rep_char_gen(5, {
        Gens::choose_u8(1, 3).flat_map(|n| match n {
          1 => unreserved_char_gen(),
          2 => sub_delims_char_gen(),
          3 => Gen::<char>::unit(|| ':'),
          x => panic!("x = {}", x),
        })
      })
    };
    a.flat_map(move |s1| b.clone().map(move |s2| format!("v{}.{}", s1, s2)))
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

  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_ipv_future() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ip_v_future_str_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ip_v_future = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v_future() - end())
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
  fn test_reg_name() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::reg_name_str_gen(), move |s| {
      counter += 1;
      log::debug!("{}, reg_name = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (reg_name() - end())
        .collect()
        .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
