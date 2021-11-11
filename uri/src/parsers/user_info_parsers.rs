use crate::parsers::basic_parsers::*;
use oni_comb_parser_rs::prelude::*;

//  userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
pub fn user_info<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved().attempt() | pct_encoded().attempt() | sub_delims().attempt() | elm(':').collect())
    .of_many0()
    .collect()
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::{pct_encoded_str_gen, rep_str_gen, sub_delims_str_gen, unreserved_str_gen};

  pub fn user_info_gen() -> Gen<String> {
    let gen = {
      rep_str_gen(1, 5, {
        Gens::choose_u8(1, 3).flat_map(|n| match n {
          1 => unreserved_str_gen(1),
          2 => pct_encoded_str_gen(),
          3 => sub_delims_str_gen(1),
          x => panic!("x = {}", x),
        })
      })
    };
    Gens::one_bool().flat_map(move |b| {
      let g = gen.clone();
      if b {
        gen
          .clone()
          .flat_map(move |s1| g.clone().map(move |s2| format!("{}:{}", s1, s2)))
      } else {
        gen.clone().map(|s| format!("{}", s))
      }
    })
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
  fn test_user_info() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(user_info_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, user_info = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (user_info() - end())
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
