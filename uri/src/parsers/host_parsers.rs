use crate::models::host_name::{HostName, IpLiteral};
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

use crate::parsers::basic_parsers::{pct_encoded, sub_delims, unreserved};
use crate::parsers::ip_v4_address_parsers::ip_v4_address;
use crate::parsers::ip_v6_address_parsers::ip_v6_address;

// host          = IP-literal / IPv4address / reg-name
pub fn host<'a>() -> Parser<'a, char, HostName> {
  (ip_literal().map(HostName::IpLiteral).attempt() | ip_v4_address().map(HostName::Ipv4Address).attempt() | reg_name())
    .name("host")
}

// IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"
pub fn ip_literal<'a>() -> Parser<'a, char, IpLiteral> {
  (elm_ref('[') * (ip_v6_address().map(IpLiteral::Ipv6Address).attempt() | ip_v_future().map(IpLiteral::IpvFuture))
    - elm_ref(']'))
  .name("ip-literal")
}

// "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
pub fn ip_v_future<'a>() -> Parser<'a, char, String> {
  (elm_ref('v')
    + elm_hex_digit().of_many1()
    + elm('.')
    + (unreserved() | sub_delims() | elm_ref(':').collect()).of_many1())
  .collect()
  .map(String::from_iter)
  .name("ipv-future")
}

//  reg-name      = *( unreserved / pct-encoded / sub-delims )
pub fn reg_name<'a>() -> Parser<'a, char, HostName> {
  (unreserved().attempt() | pct_encoded().attempt() | sub_delims())
    .of_many0()
    .collect()
    .map(String::from_iter)
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
      Gens::choose_u8(1, 3).flat_map(|n| match n {
        1 => unreserved_gen_of_char().map(|c| c.into()),
        2 => sub_delims_gen_of_char().map(|c| c.into()),
        3 => pct_encoded_gen(),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn ip_v_future_gen() -> Gen<String> {
    let a = repeat_gen_of_char(5, hex_digit_gen(HexDigitMode::Lower));
    let b = {
      repeat_gen_of_char(5, {
        Gens::choose_u8(1, 3).flat_map(|n| match n {
          1 => unreserved_gen_of_char(),
          2 => sub_delims_gen_of_char(),
          3 => Gen::<char>::unit(|| ':'),
          x => panic!("x = {}", x),
        })
      })
    };
    a.flat_map(move |s1| b.clone().map(move |s2| format!("v{}.{}", s1, s2)))
  }

  pub fn ip_literal_gen() -> Gen<String> {
    Gens::choose_u8(1, 2)
      .flat_map(|n| match n {
        1 => ipv6_address_gen(),
        2 => ip_v_future_gen(),
        x => panic!("x = {}", x),
      })
      .map(|s| format!("[{}]", s))
  }

  pub fn host_gen() -> Gen<String> {
    Gens::choose_u8(1, 3).flat_map(|n| match n {
      1 => ip_literal_gen(),
      2 => ipv4_address_gen(),
      3 => reg_name_gen(),
      x => panic!("x = {}", x),
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

  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_ip_v_future() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ip_v_future_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ip_v_future = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v_future() - end()).parse(&input).to_result();
      let ip_v_future = result.unwrap();
      log::debug!("{}, ip_v_future = {}", counter, ip_v_future);
      assert_eq!(ip_v_future.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_reg_name() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::reg_name_gen(), move |s| {
      counter += 1;
      log::debug!("{}, reg_name = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (reg_name() - end()).parse(&input).to_result();
      let reg_name = result.unwrap();
      log::debug!("{}, reg_name = {}", counter, reg_name);
      assert_eq!(reg_name.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ip_literal() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ip_literal_gen(), move |s| {
      counter += 1;
      log::debug!("{}, ip_literal = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_literal() - end()).parse(&input).to_result();
      let ip_literal = result.unwrap();
      log::debug!("{}, ip_literal = {}", counter, ip_literal);
      assert_eq!(ip_literal.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_host() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::host_gen(), move |s| {
      counter += 1;
      log::debug!("{}, host = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (host() - end()).parse(&input).to_result();
      let host_name = result.unwrap();
      log::debug!("{}, host = {}", counter, host_name);
      assert_eq!(host_name.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
