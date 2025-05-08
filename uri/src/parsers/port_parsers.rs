use oni_comb_parser_rs::prelude::*;

pub fn port<'a>() -> Parser<'a, u8, u16> {
  elm_digit()
    .of_many0()
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<u16>())
    .name("port")
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  pub fn port_gen() -> Gen<String> {
    Gens::choose_u16(1, u16::MAX - 1).map(move |n| n.to_string())
  }
}

#[cfg(test)]
mod tests {
  use std::env;

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
  fn test_port() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::port_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, port:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (port() - end()).parse(input).to_result();
      let port = result.unwrap();
      log::debug!("{:>03}, port:object = {:?}", counter, port);
      assert_eq!(port.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
