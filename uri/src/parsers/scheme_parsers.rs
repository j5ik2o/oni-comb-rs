//  scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

use crate::models::scheme::Scheme;
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

pub fn scheme<'a>() -> Parser<'a, u8, Scheme> {
  ((elm_alpha_ref() + (elm_alpha_ref() | elm_digit_ref() | elm_ref_of(b"+-.")).of_many0()).collect())
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map(Scheme::new)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  pub fn scheme_gen() -> Gen<String> {
    repeat_gen_of_char(5, {
      Gens::choose_u8(1, 5).flat_map(|n| match n {
        1 => alpha_char_gen(),
        2 => digit_gen('0', '9'),
        3 => Gen::<char>::unit(|| '+'),
        4 => Gen::<char>::unit(|| '-'),
        5 => Gen::<char>::unit(|| '.'),
        x => panic!("x = {}", x),
      })
    })
    .flat_map(|s| alpha_char_gen().map(move |c| format!("{}{}", c, s)))
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
  fn test_scheme() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(scheme_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, scheme:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (scheme() - end()).parse(input).to_result();
      let scheme = result.unwrap();
      log::debug!("{:>03}, scheme:object = {:?}", counter, scheme);
      assert_eq!(scheme.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
