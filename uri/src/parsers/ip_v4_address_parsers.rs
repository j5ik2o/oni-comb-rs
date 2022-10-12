use std::net::Ipv4Addr;

use oni_comb_parser_rs::prelude::*;

// IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
pub fn ip_v4_address<'a>() -> Parser<'a, u8, Ipv4Addr> {
  (dec_octet() - elm(b'.') + dec_octet() - elm(b'.') + dec_octet() - elm(b'.') + dec_octet())
    .map(|(((a, b), c), d)| Ipv4Addr::new(a, b, c, d))
    .name("ip_v4_address")
}

//  dec-octet     = DIGIT                 ; 0-9
//                / %x31-39 DIGIT         ; 10-99
//                / "1" 2DIGIT            ; 100-199
//                / "2" %x30-34 DIGIT     ; 200-249
//                / "25" %x30-35          ; 250-255
pub fn dec_octet<'a>() -> Parser<'a, u8, u8> {
  let p1 = elm_digit().collect();
  let p2 = (elm_digit_1_9() + elm_digit()).collect();
  let p3 = (elm(b'1') + elm_digit() + elm_digit()).collect();
  let p4 = (elm(b'2') + elm_in(b'0', b'4') + elm_digit()).collect();
  let p5 = (elm(b'2') + elm(b'5') + elm_in(b'0', b'5')).collect();

  (p5.attempt() | p4.attempt() | p3.attempt() | p2.attempt() | p1)
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<u8>())
    .name("dec-octet")
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::*;

  pub fn dec_octet_gen() -> Gen<String> {
    Gens::choose_u32(1, 255).map(|n| n.to_string())
  }

  pub fn ipv4_address_gen() -> Gen<String> {
    Gens::list_of_n(4, dec_octet_gen()).map(|sl| sl.join("."))
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use crate::parsers::ip_v4_address_parsers::gens::dec_octet_gen;

  use super::*;

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_dec_octet() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(dec_octet_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, dec_octet = {}", counter, s);
      let input = s.as_bytes();
      let result = (dec_octet() - end()).parse(input).to_result();
      let dec_octet = result.unwrap();
      assert_eq!(dec_octet.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv4_address() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv4_address_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ipv4_address = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v4_address() - end()).parse(input).to_result();
      let ipv4addr = result.unwrap();
      assert_eq!(ipv4addr.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
