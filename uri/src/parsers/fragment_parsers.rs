use crate::parsers::basic_parsers::pchar;
use oni_comb_parser_rs::prelude::*;

// fragment      = *( pchar / "/" / "?" )
pub fn fragment<'a>() -> Parser<'a, char, &'a [char]> {
  (pchar() | elm_ref_of("/?").collect()).of_many0().collect().name("fragment")
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::basic_parsers::gens::{pchar_str_gen, rep_str_gen};
  use prop_check_rs::gen::{Gen, Gens};

  pub fn fragment_str_gen() -> Gen<String> {
    rep_str_gen(1, u8::MAX - 1, {
      Gens::choose_u8(1, 2).flat_map(|n| match n {
        1 => pchar_str_gen(1, 1),
        2 => Gens::one_of_vec(vec!['/', '?']).map(|c| c.into()),
        x => panic!("x = {}", x),
      })
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
  fn test_fragment() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(fragment_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, fragment = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (fragment() - end())
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
