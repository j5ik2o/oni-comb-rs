use oni_comb_parser_rs::prelude::*;

// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
pub(crate) fn pchar<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims() | elm_ref_of(":@").collect())
    .collect()
    .name("pchar")
}

pub(crate) fn pchar_without_eq_ampersand<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims_without_eq_ampersand() | elm_ref_of(":@").collect())
    .collect()
    .name("pchar")
}

//  pct-encoded   = "%" HEXDIG HEXDIG
pub(crate) fn pct_encoded<'a>() -> Parser<'a, char, &'a [char]> {
  (elm_ref('%') + elm_hex_digit_ref() + elm_hex_digit_ref())
    .collect()
    .name("pct-encoded")
}

//  unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub(crate) fn unreserved<'a>() -> Parser<'a, char, &'a [char]> {
  (elm_alpha_ref() | elm_digit_ref() | elm_ref_of("-._~"))
    .collect()
    .name("unreserved")
}

//  reserved      = gen-delims / sub-delims
pub(crate) fn reserved<'a>() -> Parser<'a, char, &'a [char]> {
  (gen_delims() | sub_delims()).name("reserved")
}

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub(crate) fn gen_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_ref_of(":/?#[]@").name("gen-delims").collect()
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
pub(crate) fn sub_delims<'a>() -> Parser<'a, char, &'a [char]> {
  elm_ref_of("!$&'()*+,;=").name("sub-delims").collect()
}

pub(crate) fn sub_delims_without_eq_ampersand<'a>() -> Parser<'a, char, &'a [char]> {
  elm_ref_of("!$'()*+,;").name("sub-delims").collect()
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  pub fn to_option(gen: Gen<String>) -> Gen<Option<String>> {
    Gens::one_bool().flat_map(move |b| {
      if b {
        let g = gen.clone();
        g.map(|v| Some(v))
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

  pub fn hex_digit_char_gen() -> Gen<char> {
    Gens::choose_u8(1, 3).flat_map(|n| match n {
      1 => digit_gen('0', '9'),
      2 => Gens::choose('A', 'F'),
      3 => Gens::choose('a', 'f'),
      x => panic!("x = {}", x),
    })
  }

  pub fn rep_char_gen(len: u8, g: Gen<char>) -> Gen<String> {
    Gens::choose_u8(1, len)
      .flat_map(move |len| Gens::list_of_n(len as usize, g.clone()).map(|sl| sl.into_iter().collect()))
  }

  pub fn rep_str_gen(min: u8, max: u8, g: Gen<String>) -> Gen<String> {
    Gens::choose_u8(min, max)
      .flat_map(move |len| Gens::list_of_n(len as usize, g.clone()).map(|sl| sl.into_iter().collect()))
  }

  pub fn unreserved_char_gen() -> Gen<char> {
    Gens::choose(1u8, 3).flat_map(|n| match n {
      1 => alpha_char_gen(),
      2 => digit_gen('0', '9'),
      3 => Gens::one_of_vec(vec!['-', '.', '_', '~']),
      x => panic!("x = {}", x),
    })
  }

  pub fn unreserved_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, unreserved_char_gen())
  }

  pub fn gen_delims_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec![':', '/', '?', '#', '[', ']', '@'])
  }

  pub fn gen_delims_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, gen_delims_char_gen())
  }

  pub fn sub_delims_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='])
  }

  pub fn sub_delims_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, sub_delims_char_gen())
  }

  fn reserved_char_gen() -> Gen<char> {
    Gens::one_bool().flat_map(|b| {
      if b {
        gen_delims_char_gen()
      } else {
        sub_delims_char_gen()
      }
    })
  }

  pub fn reserved_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, reserved_char_gen())
  }

  pub fn pct_encoded_str_gen() -> Gen<String> {
    Gens::list_of_n(2, hex_digit_char_gen()).map(|cl| {
      let s = cl.into_iter().collect::<String>();
      format!("%{}", s)
    })
  }

  pub fn pchar_str_gen(min: u8, max: u8) -> Gen<String> {
    rep_str_gen(min, max, {
      Gens::choose_u8(1, 4).flat_map(|n| match n {
        1 => unreserved_char_gen().map(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_char_gen().map(|c| c.into()),
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
    let prop = prop::for_all(gens::pchar_str_gen(1, u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (pchar().of_many1() - end())
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
  fn test_pct_encoded() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::pct_encoded_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (pct_encoded().of_many1() - end())
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
  fn test_unreserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::unreserved_str_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (unreserved().of_many1() - end())
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
  fn test_reserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::reserved_str_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (reserved().of_many1() - end())
        .collect()
        .map(String::from_iter)
        .parse(&input)
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
    let prop = prop::for_all(gens::gen_delims_str_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (gen_delims().of_many1() - end())
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
  fn test_sub_delims() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::sub_delims_str_gen(u8::MAX - 1), move |s| {
      counter += 1;
      log::debug!("{:>03}, value = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (sub_delims().of_many1() - end())
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
