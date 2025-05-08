use oni_comb_parser_rs::extension::parser::*;
use crate::models::host_name::{HostName, IpLiteral};
use oni_comb_parser_rs::prelude::*;

use crate::parsers::basic_parsers::{pct_encoded, sub_delims, unreserved};
use crate::parsers::ip_v4_address_parsers::ip_v4_address;
use crate::parsers::ip_v6_address_parsers::ip_v6_address;

// host          = IP-literal / IPv4address / reg-name
pub fn host<'a>() -> Parser<'a, u8, HostName> {
  (ip_literal().map(HostName::IpLiteral).attempt() | ip_v4_address().map(HostName::Ipv4Address).attempt() | reg_name())
    .name("host")
}

// IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"
pub fn ip_literal<'a>() -> Parser<'a, u8, IpLiteral> {
  (elm_ref(b'[') * (ip_v6_address().map(IpLiteral::Ipv6Address).attempt() | ip_v_future().map(IpLiteral::IpvFuture))
    - elm_ref(b']'))
  .name("ip-literal")
}

// "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
pub fn ip_v_future<'a>() -> Parser<'a, u8, String> {
  (elm_ref(b'v')
    + elm_hex_digit().of_many1()
    + elm(b'.')
    + (unreserved() | sub_delims() | elm_ref(b':').collect()).of_many1())
  .collect()
  .map(|e| e.to_vec())
  .map_res(String::from_utf8)
  .name("ipv-future")
}

//  reg-name      = *( unreserved / pct-encoded / sub-delims )
pub fn reg_name<'a>() -> Parser<'a, u8, HostName> {
  (unreserved().attempt() | pct_encoded().attempt() | sub_delims())
    .of_many1()
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map(HostName::RegName)
    .name("reg-name")
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;
  use crate::parsers::ip_v4_address_parsers::gens::*;
  use crate::parsers::ip_v6_address_parsers::gens::ipv6_address_gen;

  pub fn reg_name_gen() -> Gen<String> {
    repeat_gen_of_string(1, 10, {
      Gens::frequency([
        (1, unreserved_gen_of_char().map(|c| c.into())),
        (1, sub_delims_gen_of_char().map(|c| c.into())),
        (1, pct_encoded_gen()),
      ])
    })
  }

  pub fn ip_v_future_gen() -> Gen<String> {
    let a = repeat_gen_of_char(5, hex_digit_gen(HexDigitMode::Lower));
    let b = {
      repeat_gen_of_char(5, {
        Gens::frequency([
          (1, unreserved_gen_of_char()),
          (1, sub_delims_gen_of_char()),
          (1, Gens::pure(':')),
        ])
      })
    };
    a.flat_map(move |s1| b.clone().map(move |s2| format!("v{}.{}", s1, s2)))
  }

  pub fn ip_literal_gen() -> Gen<String> {
    Gens::frequency([(1, ipv6_address_gen()), (1, ip_v_future_gen())]).map(|s| format!("[{}]", s))
  }

  pub fn host_gen() -> Gen<String> {
    Gens::frequency([(1, ip_literal_gen()), (1, ipv4_address_gen()), (1, reg_name_gen())])
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

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_ip_v_future() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::ip_v_future_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ip_v_future = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v_future() - end()).parse(input).to_result();
      let ip_v_future = result.unwrap();
      log::debug!("{}, ip_v_future = {}", counter, ip_v_future);
      assert_eq!(ip_v_future.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_reg_name() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::reg_name_gen(), move |s| {
      counter += 1;
      log::debug!("{}, reg_name = {}", counter, s);
      let input = s.as_bytes();
      let result = (reg_name() - end()).parse(input).to_result();
      let reg_name = result.unwrap();
      log::debug!("{}, reg_name = {}", counter, reg_name);
      assert_eq!(reg_name.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ip_literal() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::ip_literal_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ip_literal = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_literal() - end()).parse(input).to_result();
      let ip_literal = result.unwrap();
      log::debug!("{}, ip_literal = {}", counter, ip_literal);
      assert_eq!(ip_literal.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_host() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::host_gen(), move |s| {
      counter += 1;
      log::debug!("{}, host = {}", counter, s);
      let input = s.as_bytes();
      let result = (host() - end()).parse(input).to_result();
      let host_name = result.unwrap();
      log::debug!("{}, host = {}", counter, host_name);
      assert_eq!(host_name.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
