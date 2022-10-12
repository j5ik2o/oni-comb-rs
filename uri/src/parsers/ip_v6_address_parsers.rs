use crate::parsers::ip_v4_address_parsers::ip_v4_address;
use oni_comb_parser_rs::prelude::*;
use std::fmt::Formatter;

use std::net::{Ipv4Addr, Ipv6Addr};

//  IPv6address   =                            6( h16 ":" ) ls32
fn ip_v6_address1<'a>() -> Parser<'a, u8, Ipv6Addr> {
  ((h16() - elm(b':')).of_count(6) + ls32()).map(|(vec, b)| {
    let vec = vec.into_iter().map(|e| e.to_u16()).collect::<Vec<_>>();
    match b {
      LS32::Ls32(l, r) => Ipv6Addr::new(vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], l.to_u16(), r.to_u16()),
      LS32::Ipv4Address(ipv4addr) => {
        let o = ipv4addr.octets();
        let l: u16 = ((o[0] as u16) << 8) | o[1] as u16;
        let r: u16 = ((o[2] as u16) << 8) | o[3] as u16;
        Ipv6Addr::new(vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], l, r)
      }
    }
  }) //.collect()
}

//                /                       "::" 5( h16 ":" ) ls32
fn ip_v6_address2<'a>() -> Parser<'a, u8, Ipv6Addr> {
  (seq(b"::") * (h16() - elm(b':')).of_count(5) + ls32()).map(|(vec, b)| {
    let vec = vec.into_iter().map(|e| e.to_u16()).collect::<Vec<_>>();
    match b {
      LS32::Ls32(l, r) => Ipv6Addr::new(0, vec[0], vec[1], vec[2], vec[3], vec[4], l.to_u16(), r.to_u16()),
      LS32::Ipv4Address(ipv4addr) => {
        let o = ipv4addr.octets();
        let l: u16 = ((o[0] as u16) << 8) | o[1] as u16;
        let r: u16 = ((o[2] as u16) << 8) | o[3] as u16;
        Ipv6Addr::new(0, vec[0], vec[1], vec[2], vec[3], vec[4], l, r)
      }
    }
  })
}

//                / [               h16 ] "::" 4( h16 ":" ) ls32
fn ip_v6_address3<'a>() -> Parser<'a, u8, Ipv6Addr> {
  ((h16().opt() - seq(b"::")) + (h16() - elm(b':')).of_count(4) + ls32()).map(|((h, vec), b)| {
    let vec = vec.into_iter().map(|e| e.to_u16()).collect::<Vec<_>>();
    match b {
      LS32::Ls32(l, r) => Ipv6Addr::new(
        h.map(|e| e.to_u16()).unwrap_or(0),
        0,
        vec[0],
        vec[1],
        vec[2],
        vec[3],
        l.to_u16(),
        r.to_u16(),
      ),
      LS32::Ipv4Address(ipv4addr) => {
        let o = ipv4addr.octets();
        let l: u16 = ((o[0] as u16) << 8) | o[1] as u16;
        let r: u16 = ((o[2] as u16) << 8) | o[3] as u16;
        Ipv6Addr::new(
          h.map(|e| e.to_u16()).unwrap_or(0),
          0,
          vec[0],
          vec[1],
          vec[2],
          vec[3],
          l,
          r,
        )
      }
    }
  })
}

fn ip_v6_address_p1<'a>(n: usize) -> Parser<'a, u8, &'a [u8]> {
  ((h16() + (elm(b':') * h16()).of_many_n_m(0, n)).map(|(h, vec)| {
    vec.into_iter().fold(vec![h], |mut acc, e| {
      acc.push(e);
      acc
    });
  }))
  .opt()
  .collect()
}

fn ip_v6_address_p2<'a>(n: usize, m: usize) -> Parser<'a, u8, &'a [u8]> {
  let p2 = ((h16() + elm(b':')).of_count(m) + ls32()).collect();
  (ip_v6_address_p1(n) + seq(b"::") + p2).collect()
}

//                / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
fn ip_v6_address4<'a>() -> Parser<'a, u8, Ipv6Addr> {
  ip_v6_address_p2(1, 3)
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

//                / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
fn ip_v6_address5<'a>() -> Parser<'a, u8, Ipv6Addr> {
  ip_v6_address_p2(2, 2)
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

//                / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
fn ip_v6_address6<'a>() -> Parser<'a, u8, Ipv6Addr> {
  ip_v6_address_p2(3, 1)
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

//                / [ *4( h16 ":" ) h16 ] "::"              ls32
fn ip_v6_address7<'a>() -> Parser<'a, u8, Ipv6Addr> {
  (ip_v6_address_p1(4) + seq(b"::") + ls32())
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

//                / [ *5( h16 ":" ) h16 ] "::"              h16
fn ip_v6_address8<'a>() -> Parser<'a, u8, Ipv6Addr> {
  (ip_v6_address_p1(5) + seq(b"::") + h16())
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

//                / [ *6( h16 ":" ) h16 ] "::"
fn ip_v6_address9<'a>() -> Parser<'a, u8, Ipv6Addr> {
  (ip_v6_address_p1(5) + seq(b"::"))
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| s.parse::<Ipv6Addr>())
}

pub fn ip_v6_address<'a>() -> Parser<'a, u8, Ipv6Addr> {
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

#[derive(Debug, Clone)]
struct H16(u16);

impl H16 {
  pub fn to_u16(self) -> u16 {
    self.0
  }
}

impl std::fmt::Display for H16 {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:x}", self.0)
  }
}

//  h16           = 1*4HEXDIG
fn h16<'a>() -> Parser<'a, u8, H16> {
  elm_hex_digit()
    .of_many_n_m(1, 4)
    .collect()
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .map_res(|s| u16::from_str_radix(&s, 16))
    .map(H16)
    .name("h16")
}

#[derive(Debug, Clone)]
enum LS32 {
  Ls32(H16, H16),
  Ipv4Address(Ipv4Addr),
}

impl std::fmt::Display for LS32 {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LS32::Ls32(l, r) => write!(f, "{}:{}", l, r),
      LS32::Ipv4Address(ip) => write!(f, "{}", ip),
    }
  }
}

//  ls32          = ( h16 ":" h16 ) / IPv4address
fn ls32<'a>() -> Parser<'a, u8, LS32> {
  (h16() - elm(b':') + h16()).map(|(a, b)| LS32::Ls32(a, b)).attempt() | ip_v4_address().map(|a| LS32::Ipv4Address(a))
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::basic_parsers::gens::*;
  use crate::parsers::ip_v4_address_parsers::gens::*;
  use prop_check_rs::gen::*;
  use std::net::Ipv6Addr;

  pub fn h16_gen() -> Gen<String> {
    Gens::choose_u8(1, 4)
      .flat_map(|n| repeat_gen_of_char(n, hex_digit_gen(HexDigitMode::Lower)))
      .map(|s| u16::from_str_radix(&s, 16).unwrap())
      .map(|n| format!("{:x}", n))
  }

  pub fn ls32_gen() -> Gen<String> {
    Gens::frequency([
      (1, ipv4_address_gen()),
      (1, Gens::list_of_n(2, h16_gen()).map(|sl| sl.join(":"))),
    ])
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
          Gens::frequency([
            (1, h16_gen()),
            (1, Gens::list_of_n(2, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
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
          Gens::frequency([
            (1, h16_gen()),
            (2, Gens::list_of_n(3, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
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
          Gens::frequency([
            (1, h16_gen()),
            (1, Gens::list_of_n(3, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
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
          Gens::frequency([
            (1, h16_gen()),
            (1, Gens::list_of_n(4, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
        }
      })
      .flat_map(|s0| ls32_gen().map(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *5( h16 ":" ) h16 ] "::"              h16
  pub fn ipv6_address_gen8() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::frequency([
            (1, h16_gen()),
            (1, Gens::list_of_n(5, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
        }
      })
      .flat_map(|s0| h16_gen().map(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *6( h16 ":" ) h16 ] "::"
  pub fn ipv6_address_gen9() -> Gen<String> {
    Gens::one_bool()
      .flat_map(|b| {
        if b {
          Gens::frequency([
            (1, h16_gen()),
            (1, Gens::list_of_n(6, h16_gen()).map(|sl| sl.join(":"))),
          ])
        } else {
          Gens::unit("".to_string())
        }
      })
      .map(|s0| format!("{}::", s0))
  }

  pub fn ipv6_address_gen() -> Gen<String> {
    Gens::frequency([
      (1, ipv6_address_gen1()),
      (1, ipv6_address_gen2()),
      (1, ipv6_address_gen3()),
      (1, ipv6_address_gen4()),
      (1, ipv6_address_gen5()),
      (1, ipv6_address_gen6()),
      (1, ipv6_address_gen7()),
      (1, ipv6_address_gen8()),
      (1, ipv6_address_gen9()),
    ])
    .map(|s| s.parse::<Ipv6Addr>().unwrap())
    .map(|i| i.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;
  use rand::Rng;
  use std::env;

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  fn new_rng() -> RNG {
    let mut rand = rand::thread_rng();
    let rng = RNG::new_with_seed(rand.gen());
    rng
  }

  #[test]
  fn test_h16() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::h16_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, h16 = {}", counter, s);
      let input = s.as_bytes();
      let result = (h16() - end()).parse(input).to_result();
      let h16 = result.unwrap();
      assert_eq!(h16.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ls32() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ls32_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ls32 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ls32() - end()).parse(input).to_result();
      let ls32 = result.unwrap();
      assert_eq!(ls32.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address1() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen1(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address1 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address1() - end()).parse(input).to_result();
      let ipv6_address1 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address1, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address2() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen2(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address2 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address2() - end()).parse(input).to_result();
      let ipv6_address2 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address2, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address3() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen3(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address3 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address3() - end()).parse(input).to_result();
      let ipv6_address3 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address3, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address4() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen4(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address4 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address4() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(input)
        .to_result();
      let ipv6_address4 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address4, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address5() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen5(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address5 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address5() - end()).parse(input).to_result();
      let ipv6_address5 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address5, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address6() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen6(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address6 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address6() - end()).parse(input).to_result();
      let ipv6_address6 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address6, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address7() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen7(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address7 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address7() - end()).parse(input).to_result();
      let ipv6_address7 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address7, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address8() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen8(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address8 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address8() - end()).parse(input).to_result();
      let ipv6_address8 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address8, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address9() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen9(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address9 = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address9() - end()).parse(input).to_result();
      let ipv6_address9 = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address9, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn test_ipv6_address() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all(gens::ipv6_address_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, ipv6_address = {}", counter, s);
      let input = s.as_bytes();
      let result = (ip_v6_address() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(input)
        .to_result();
      let ipv6_address = result.unwrap();
      let expected = s.parse::<Ipv6Addr>().unwrap();
      assert_eq!(ipv6_address, expected);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }
}
