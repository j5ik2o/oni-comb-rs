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
        Gens::unit(None)
      }
    })
  }

  // Generators
  fn low_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<Gen<char>> = ('a'..='z').into_iter().map(Gens::unit).collect::<Vec<_>>();
    Gens::one_of(low_alpha_gen)
  }

  fn high_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<Gen<char>> = ('A'..='Z').into_iter().map(Gens::unit).collect::<Vec<_>>();
    Gens::one_of(low_alpha_gen)
  }

  pub fn alpha_char_gen() -> Gen<char> {
    Gens::one_bool().flat_map(|b| if b { low_alpha_gen() } else { high_alpha_gen() })
  }

  pub fn digit_gen(min: char, max: char) -> Gen<char> {
    let low_alpha_gen: Vec<Gen<char>> = (min..=max).into_iter().map(Gens::unit).collect::<Vec<_>>();
    Gens::one_of(low_alpha_gen)
  }

  pub enum HexDigitMode {
    All,
    Lower,
    Upper,
  }

  pub fn hex_digit_gen(mode: HexDigitMode) -> Gen<char> {
    Gens::frequency([
      (1, digit_gen('0', '9')),
      (
        1,
        match mode {
          HexDigitMode::All | HexDigitMode::Upper => Gens::choose('A', 'F'),
          _ => digit_gen('0', '9'),
        },
      ),
      (
        1,
        match mode {
          HexDigitMode::All | HexDigitMode::Lower => Gens::choose('a', 'f'),
          _ => digit_gen('0', '9'),
        },
      ),
    ])
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
    Gens::frequency([
      (1, alpha_char_gen()),
      (1, digit_gen('0', '9')),
      (
        1,
        Gens::one_of(
          vec!['-', '.', '_', '~']
            .into_iter()
            .map(Gens::unit)
            .collect::<Vec<Gen<_>>>(),
        ),
      ),
    ])
  }

  pub fn unreserved_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, unreserved_gen_of_char())
  }

  pub fn gen_delims_gen_of_char() -> Gen<char> {
    Gens::one_of(
      vec![':', '/', '?', '#', '[', ']', '@']
        .into_iter()
        .map(Gens::unit)
        .collect::<Vec<Gen<_>>>(),
    )
  }

  pub fn gen_delims_gen(len: u8) -> Gen<String> {
    repeat_gen_of_char(len, gen_delims_gen_of_char())
  }

  pub fn sub_delims_gen_of_char() -> Gen<char> {
    Gens::one_of(
      vec!['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '=']
        .into_iter()
        .map(Gens::unit)
        .collect::<Vec<Gen<_>>>(),
    )
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
      Gens::frequency([
        (1, unreserved_gen_of_char().map(|c| c.into())),
        (1, pct_encoded_gen()),
        (1, sub_delims_gen_of_char().map(|c| c.into())),
        (
          1,
          Gens::one_of(vec![':', '@'].into_iter().map(Gens::unit).collect::<Vec<Gen<_>>>()).map(|c| c.into()),
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

  use super::*;

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_pchar() -> Result<()> {
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
