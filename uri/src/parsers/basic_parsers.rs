use oni_comb_parser_rs::prelude::*;

// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
pub(crate) fn pchar<'a>() -> Parser<'a, u8, &'a [u8]> {
  (unreserved() | pct_encoded() | sub_delims() | elm_ref_of(b":@").collect())
    .collect()
    .name("pchar")
}

pub(crate) fn pchar_without_eq_amp<'a>() -> Parser<'a, u8, &'a [u8]> {
  (unreserved() | pct_encoded() | sub_delims_without_eq_ampersand() | elm_ref_of(b":@").collect())
    .collect()
    .name("pchar")
}

//  pct-encoded   = "%" HEXDIG HEXDIG
pub(crate) fn pct_encoded<'a>() -> Parser<'a, u8, &'a [u8]> {
  (elm_ref(b'%') + elm_hex_digit_ref() + elm_hex_digit_ref())
    .collect()
    .name("pct-encoded")
}

//  unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub(crate) fn unreserved<'a>() -> Parser<'a, u8, &'a [u8]> {
  (elm_alpha_ref() | elm_digit_ref() | elm_ref_of(b"-._~"))
    .collect()
    .name("unreserved")
}

//  reserved      = gen-delims / sub-delims
pub(crate) fn reserved<'a>() -> Parser<'a, u8, &'a [u8]> {
  (gen_delims() | sub_delims()).name("reserved")
}

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub(crate) fn gen_delims<'a>() -> Parser<'a, u8, &'a [u8]> {
  elm_ref_of(b":/?#[]@").name("gen-delims").collect()
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
pub(crate) fn sub_delims<'a>() -> Parser<'a, u8, &'a [u8]> {
  elm_ref_of(b"!$&'()*+,;=").name("sub-delims").collect()
}

pub(crate) fn sub_delims_without_eq_ampersand<'a>() -> Parser<'a, u8, &'a [u8]> {
  elm_ref_of(b"!$'()*+,;").name("sub-delims").collect()
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  pub fn to_option(gen: Gen<String>) -> Gen<Option<String>> {
    Gens::one_bool().flat_map(move |b| {
      if b {
        let g = gen.clone();
        g.map(Some)
      } else {
        Gen::<String>::unit(|| None)
      }
    })
  }

  // Generators
  fn low_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<char> = ('a'..='z').into_iter().collect::<Vec<_>>();
    Gens::one_of_vec(low_alpha_gen)
  }

  fn high_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<_>>();
    Gens::one_of_vec(low_alpha_gen)
  }

  pub fn alpha_char_gen() -> Gen<char> {
    Gens::one_bool().flat_map(|b| if b { low_alpha_gen() } else { high_alpha_gen() })
  }

  pub fn digit_gen(min: char, max: char) -> Gen<char> {
    let low_alpha_gen: Vec<char> = (min..=max).into_iter().collect::<Vec<_>>();
    Gens::one_of_vec(low_alpha_gen)
  }

  pub enum HexDigitMode {
    All,
    Lower,
    Upper,
  }

  pub fn hex_digit_gen(mode: HexDigitMode) -> Gen<char> {
    Gens::choose_u8(1, 3).flat_map(move |n| match n {
      1 => digit_gen('0', '9'),
      2 => match mode {
        HexDigitMode::All | HexDigitMode::Upper => Gens::choose('A', 'F'),
        _ => digit_gen('0', '9'),
      },
      3 => match mode {
        HexDigitMode::All | HexDigitMode::Lower => Gens::choose('a', 'f'),
        _ => digit_gen('0', '9'),
      },
      x => panic!("x = {}", x),
    })
  }

  pub fn repeat_gen_of_char(len: u8, g: Gen<char>) -> Gen<String> {
    Gens::choose_u8(1, len)
      .flat_map(move |len| Gens::list_of_n(len as usize, g.clone()).map(|sl| sl.into_iter().collect()))
  }

  pub fn repeat_gen_of_string(min: u8, max: u8, g: Gen<String>) -> Gen<String> {
    Gens::choose_u8(min, max)
      .flat_map(move |len| Gens::list_of_n(len as usize, g.clone()).map(|sl| sl.into_iter().collect()))
  }

  pub fn unreserved_gen_of_char() -> Gen<char> {
    Gens::choose(1u8, 3).flat_map(|n| match n {
      1 => alpha_char_gen(),
      2 => digit_gen('0', '9'),
      3 => Gens::one_of_vec(vec!['-', '.', '_', '~']),
      x => panic!("x = {}", x),
    })
  }

  pub fn unreserved_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, unreserved_gen_of_char())
  }

  pub fn gen_delims_gen_of_char() -> Gen<char> {
    Gens::one_of_vec(vec![':', '/', '?', '#', '[', ']', '@'])
  }

  pub fn gen_delims_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, gen_delims_gen_of_char())
  }

  pub fn sub_delims_gen_of_char() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='])
  }

  pub fn sub_delims_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, sub_delims_gen_of_char())
  }

  fn reserved_gen_of_char() -> Gen<char> {
    Gens::one_bool().flat_map(|b| {
      if b {
        gen_delims_gen_of_char()
      } else {
        sub_delims_gen_of_char()
      }
    })
  }

  pub fn reserved_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, reserved_gen_of_char())
  }

  pub fn pct_encoded_gen() -> Gen<String> {
    Gens::list_of_n(2, hex_digit_gen(HexDigitMode::Lower)).map(|cl| {
      let s = cl.into_iter().collect::<String>();
      format!("%{}", s)
    })
  }

  pub fn pchar_gen(min: u8, max: u8) -> Gen<String> {
    repeat_gen_of_string(min, max, {
      Gens::choose_u8(1, 4).flat_map(|n| match n {
        1 => unreserved_gen_of_char().map(|c| c.into()),
        2 => pct_encoded_gen(),
        3 => sub_delims_gen_of_char().map(|c| c.into()),
        4 => Gens::one_of_vec(vec![':', '@']).map(|c| c.into()),
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

  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_pchar() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::pchar_gen(1, u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (pchar().of_many1() - end())
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

  #[test]
  fn test_pct_encoded() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::pct_encoded_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (pct_encoded().of_many1() - end())
        .collect()
        .map(|e| e.to_vec())
        .map_res(String::from_utf8)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_unreserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::unreserved_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (unreserved().of_many1() - end())
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

  #[test]
  fn test_reserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::reserved_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (reserved().of_many1() - end())
        .collect()
        .map(|e| e.to_vec())
        .map_res(String::from_utf8)
        .parse(input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, 1000, RNG::new())
  }

  #[test]
  fn test_gen_delims() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::gen_delims_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (gen_delims().of_many1() - end())
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

  #[test]
  fn test_sub_delims() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::sub_delims_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.as_bytes();
      let result = (sub_delims().of_many1() - end())
        .collect()
        .map_res(std::str::from_utf8)
        .parse(input)
        .to_result();
      assert_eq!(result.unwrap(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
