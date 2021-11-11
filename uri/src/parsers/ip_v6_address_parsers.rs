use crate::parsers::ip_v4_address_parsers::ip_v4_address;
use oni_comb_parser_rs::prelude::*;

//  IPv6address   =                            6( h16 ":" ) ls32
fn ip_v6_address1<'a>() -> Parser<'a, char, &'a [char]> {
  ((h16() + elm(':')).of_count(6) + ls32()).collect()
}

//                /                       "::" 5( h16 ":" ) ls32
fn ip_v6_address2<'a>() -> Parser<'a, char, &'a [char]> {
  (tag("::") + (h16() + elm(':')).of_count(5) + ls32()).collect()
}

//                / [               h16 ] "::" 4( h16 ":" ) ls32
fn ip_v6_address3<'a>() -> Parser<'a, char, &'a [char]> {
  (h16().opt() + tag("::") + (h16() + elm(':')).of_count(4) + ls32()).collect()
}

fn ip_v6_address_p1<'a>(n: usize) -> Parser<'a, char, &'a [char]> {
  (h16() + (elm(':') + h16()).of_many_n_m(0, n)).opt().collect()
}

fn ip_v6_address_p2<'a>(n: usize, m: usize) -> Parser<'a, char, &'a [char]> {
  let p2 = ((h16() + elm(':')).of_count(m) + ls32()).collect();
  (ip_v6_address_p1(n) + tag("::") + p2).collect()
}

//                / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
fn ip_v6_address4<'a>() -> Parser<'a, char, &'a [char]> {
  ip_v6_address_p2(1, 3)
}

//                / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
fn ip_v6_address5<'a>() -> Parser<'a, char, &'a [char]> {
  ip_v6_address_p2(2, 2)
}

//                / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
fn ip_v6_address6<'a>() -> Parser<'a, char, &'a [char]> {
  ip_v6_address_p2(3, 1)
}

//                / [ *4( h16 ":" ) h16 ] "::"              ls32
fn ip_v6_address7<'a>() -> Parser<'a, char, &'a [char]> {
  (ip_v6_address_p1(4) + tag("::") + ls32()).collect()
}

//                / [ *5( h16 ":" ) h16 ] "::"              h16
fn ip_v6_address8<'a>() -> Parser<'a, char, &'a [char]> {
  (ip_v6_address_p1(5) + tag("::") + h16()).collect()
}

//                / [ *6( h16 ":" ) h16 ] "::"
fn ip_v6_address9<'a>() -> Parser<'a, char, &'a [char]> {
  (ip_v6_address_p1(5) + tag("::")).collect()
}

pub fn ip_v6_address<'a>() -> Parser<'a, char, &'a [char]> {
  (ip_v6_address1().attempt()
    | ip_v6_address2().attempt()
    | ip_v6_address3().attempt()
    | ip_v6_address4().attempt()
    | ip_v6_address5().attempt()
    | ip_v6_address6().attempt()
    | ip_v6_address7().attempt()
    | ip_v6_address8().attempt()
    | ip_v6_address9())
  .name("ip_v6_address")
}

//  h16           = 1*4HEXDIG
fn h16<'a>() -> Parser<'a, char, &'a [char]> {
  elm_hex_digit().of_many_n_m(1, 4).collect().name("h16")
}

//  ls32          = ( h16 ":" h16 ) / IPv4address
fn ls32<'a>() -> Parser<'a, char, &'a [char]> {
  (h16() + elm(':') + h16()).collect().attempt() | ip_v4_address().collect()
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::basic_parsers::gens::*;
  use crate::parsers::ip_v4_address_parsers::gens::*;
  use prop_check_rs::gen::*;

  pub fn h16_gen() -> Gen<String> {
    Gens::choose_u8(1, 4).flat_map(|n| rep_char_gen(n, hex_digit_char_gen()))
  }

  pub fn ls32_gen() -> Gen<String> {
    Gens::choose_u8(1, 2).flat_map(|n| match n {
      1 => ipv4_address_str_gen(),
      2 => Gens::list_of_n(2, h16_gen()).map(|sl| sl.join(":")),
      x => panic!("x = {}", x),
    })
  }

  pub fn ipv6_address_gen1() -> Gen<String> {
    Gens::list_of_n(6, h16_gen()).flat_map(|sl| ls32_gen().map(move |ls32| format!("{}:{}", sl.join(":"), ls32)))
  }

  pub fn ipv6_address_gen2() -> Gen<String> {
    Gens::list_of_n(5, h16_gen()).flat_map(|sl| ls32_gen().map(move |ls32| format!("::{}:{}", sl.join(":"), ls32)))
  }

  pub fn ipv6_address_gen3() -> Gen<String> {
    Gens::list_of_n(5, h16_gen()).flat_map(|sl| {
      ls32_gen().map(move |ls32| {
        let (h, t) = sl.split_first().unwrap();
        format!("{}::{}:{}", h, t.join(":"), ls32)
      })
    })
  }

  // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
  pub fn ipv6_address_gen4() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(2, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .flat_map(|s0| {
        Gens::list_of_n(3, h16_gen().map(|v| format!("{}:", v)))
          .map(|sl| sl.join(""))
          .map(|s| format!("::{}", s))
          .flat_map(|s2| ls32_gen().map(move |s3| format!("{}{}", s2, s3)))
          .map(move |s| format!("{}{}", s0, s))
      })
  }

  //  [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
  pub fn ipv6_address_gen5() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(3, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .flat_map(|s0| {
        Gens::list_of_n(2, h16_gen().map(|v| format!("{}:", v)))
          .map(|sl| sl.join(""))
          .map(|s| format!("::{}", s))
          .flat_map(|s2| ls32_gen().map(move |s3| format!("{}{}", s2, s3)))
          .map(move |s| format!("{}{}", s0, s))
      })
  }

  //  [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
  pub fn ipv6_address_gen6() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(3, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .flat_map(|s0| {
        Gens::list_of_n(1, h16_gen().map(|v| format!("{}:", v)))
          .map(|sl| sl.join(""))
          .map(|s| format!("::{}", s))
          .flat_map(|s2| ls32_gen().map(move |s3| format!("{}{}", s2, s3)))
          .map(move |s| format!("{}{}", s0, s))
      })
  }

  // [ *4( h16 ":" ) h16 ] "::"              ls32
  pub fn ipv6_address_gen7() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(4, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .flat_map(|s0| ls32_gen().map(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *5( h16 ":" ) h16 ] "::"              h16
  pub fn ipv6_address_gen8() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(5, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .flat_map(|s0| h16_gen().map(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *6( h16 ":" ) h16 ] "::"
  pub fn ipv6_address_gen9() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::choose_u8(1, 2).flat_map(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(6, h16_gen()).map(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .map(|s0| format!("{}::", s0))
  }

  pub fn ipv6_address_str_gen() -> Gen<String> {
    Gens::choose_u8(1, 9).flat_map(|n| match n {
      1 => ipv6_address_gen1(),
      2 => ipv6_address_gen2(),
      3 => ipv6_address_gen3(),
      4 => ipv6_address_gen4(),
      5 => ipv6_address_gen5(),
      6 => ipv6_address_gen6(),
      7 => ipv6_address_gen7(),
      8 => ipv6_address_gen8(),
      9 => ipv6_address_gen9(),
      x => panic!("x = {}", x),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;
  use std::env;
  use std::iter::FromIterator;

  const TEST_COUNT: TestCases = 100;
  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_h16() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::h16_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, h16 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (h16() - end())
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
  fn test_ls32() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ls32_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ls32 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ls32() - end())
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
  fn test_ipv6_address1() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen1(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address1 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address1() - end())
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
  fn test_ipv6_address2() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen2(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address2 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address2() - end())
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
  fn test_ipv6_address3() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen3(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address3 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address3() - end())
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
  fn test_ipv6_address4() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen4(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address4 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address4().of_many1() - end())
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
  fn test_ipv6_address5() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen5(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address5 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address5().of_many1() - end())
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
  fn test_ipv6_address6() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen6(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address6 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address6() - end())
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
  fn test_ipv6_address7() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen7(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address7 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address7() - end())
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
  fn test_ipv6_address8() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen8(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address8 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address8() - end())
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
  fn test_ipv6_address9() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen9(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address9 = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address9() - end())
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
  fn test_ipv6_address() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (ip_v6_address() - end())
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
