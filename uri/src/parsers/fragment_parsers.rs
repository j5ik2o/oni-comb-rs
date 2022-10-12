use crate::parsers::basic_parsers::pchar;
use oni_comb_parser_rs::prelude::*;

// fragment      = *( pchar / "/" / "?" )
pub fn fragment<'a>() -> Parser<'a, u8, String> {
  (pchar() | elm_ref_of(b"/?").collect())
    .of_many0()
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .name("fragment")
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::basic_parsers::gens::{pchar_gen, repeat_gen_of_string};
  use prop_check_rs::gen::{Gen, Gens};

  pub fn fragment_gen() -> Gen<String> {
    repeat_gen_of_string(1, u8::MAX - 1, {
      Gens::frequency([
        (1, pchar_gen(1, 1)),
        (
          1,
          Gens::one_of(vec!['/', '?'].into_iter().map(Gens::unit).collect::<Vec<Gen<_>>>()).map(|c| c.into()),
        ),
      ])
    })
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
  fn test_fragment() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(fragment_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, fragment = {}", counter, s);
      let input = s.as_bytes();
      let result = (fragment() - end())
        .collect()
        .map(|e| e.to_vec())
        .map_res(String::from_utf8)
        .parse(input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
