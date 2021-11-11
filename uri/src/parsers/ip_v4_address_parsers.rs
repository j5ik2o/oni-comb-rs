use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

// IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
pub fn ip_v4_address<'a>() -> Parser<'a, char, (u8, u8, u8, u8)> {
  (dec_octet() - elm('.') + dec_octet() - elm('.') + dec_octet() - elm('.') + dec_octet())
    .map(|(((a, b), c), d)| (a, b, c, d))
    .name("ip_v4_address")
}

//  dec-octet     = DIGIT                 ; 0-9
//                / %x31-39 DIGIT         ; 10-99
//                / "1" 2DIGIT            ; 100-199
//                / "2" %x30-34 DIGIT     ; 200-249
//                / "25" %x30-35          ; 250-255
pub fn dec_octet<'a>() -> Parser<'a, char, u8> {
  let p1 = elm_digit().collect();
  let p2 = (elm_digit_1_9() + elm_digit()).collect();
  let p3 = (elm('1') + elm_digit() + elm_digit()).collect();
  let p4 = (elm('2') + elm_of("01234") + elm_digit()).collect();
  let p5 = (elm('2') + elm('5') + elm_of("012345")).collect();

  (p5.attempt() | p4.attempt() | p3.attempt() | p2.attempt() | p1)
    .collect()
    .map(String::from_iter)
    .map_res(|s| s.parse::<u8>())
    .name("dec-octet")
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::*;

  pub fn dec_octet_str_gen() -> Gen<String> {
    Gens::choose_u32(1, 255).map(|n| n.to_string())
  }

  pub fn ipv4_address_str_gen() -> Gen<String> {
    Gens::list_of_n(4, dec_octet_str_gen()).map(|sl| sl.join("."))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use prop_check_rs::prop::TestCases;
  use std::env;
  const TEST_COUNT: TestCases = 100;
  use crate::parsers::ip_v4_address_parsers::gens::dec_octet_str_gen;
  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::rng::RNG;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_dec_octet() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(dec_octet_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, dec_octet = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (dec_octet().of_many1() - end())
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
  fn test_ipv4_address() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv4_address_str_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ipv4_address = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v4_address() - end())
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
