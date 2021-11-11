use crate::models::path::Path;
use crate::parsers::basic_parsers::*;
use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

//  path          = path-abempty    ; begins with "/" or is empty
//                / path-absolute   ; begins with "/" but not "//"
//                / path-noscheme   ; begins with a non-colon segment
//                / path-rootless   ; begins with a segment
//                / path-empty      ; zero characters
pub fn path<'a>() -> Parser<'a, char, Option<Path>> {
  (path_rootless().attempt() | path_abempty(true).attempt() | path_absolute().attempt() | path_noscheme())
    .opt()
    .name("path")
}

//  path-abempty  = *( "/" segment )
pub fn path_abempty<'a>(required: bool) -> Parser<'a, char, Path> {
  let n = if required { 1 } else { 0 };
  ((elm('/') + segment()).collect())
    .map(String::from_iter)
    .repeat(n..)
    .map(|e| Path::of_abempty_from_strings(&e))
    .name("path-abempty")
}

//  path-absolute = "/" [ segment-nz *( "/" segment ) ]
pub fn path_absolute<'a>() -> Parser<'a, char, Path> {
  let p = (seqment_nz() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .opt();
  (elm('/').collect() + p)
    .map(|(a, b_opt)| match b_opt {
      None => vec![a],
      Some(b) => {
        let mut l = vec![a];
        l.extend_from_slice(&b);
        l
      }
    })
    .map(|e| e.into_iter().map(String::from_iter).collect::<Vec<_>>())
    .map(|e| Path::of_absolute_from_strings(&e))
    .name("path-absolute")
}

//  path-rootless = segment-nz *( "/" segment )
pub fn path_rootless<'a>() -> Parser<'a, char, Path> {
  (seqment_nz() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .map(|e| e.into_iter().map(String::from_iter).collect::<Vec<_>>())
    .map(|e| Path::of_rootless_from_strings(&e))
    .name("path-rootless")
}

//  path-noscheme = segment-nz-nc *( "/" segment )
pub fn path_noscheme<'a>() -> Parser<'a, char, Path> {
  (seqment_nz_nc() + ((elm('/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .map(|e| e.into_iter().map(String::from_iter).collect::<Vec<_>>())
    .map(|e| Path::of_rootless_from_strings(&e))
    .name("path-noscheme")
}

// segment       = *pchar
fn segment<'a>() -> Parser<'a, char, &'a [char]> {
  pchar().of_many0().collect().name("segment")
}

// segment-nz    = 1*pchar
fn seqment_nz<'a>() -> Parser<'a, char, &'a [char]> {
  pchar().of_many1().collect().name("segment-nz")
}

// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
// ; non-zero-length segment without any colon ":"
fn seqment_nz_nc<'a>() -> Parser<'a, char, &'a [char]> {
  (unreserved() | pct_encoded() | sub_delims() | elm('@').collect())
    .of_many1()
    .collect()
    .name("segment-nz-nc")
}

#[cfg(test)]
pub mod gens {
  use std::fmt::Formatter;

  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  pub fn segment_str_gen() -> Gen<String> {
    pchar_str_gen(0, u8::MAX - 1)
  }

  pub fn segment_nz_str_gen() -> Gen<String> {
    pchar_str_gen(1, u8::MAX - 1)
  }

  pub fn segment_nz_nc_str_gen() -> Gen<String> {
    rep_str_gen(1, u8::MAX - 1, {
      Gens::choose_u8(1, 2).flat_map(|n| match n {
        1 => unreserved_char_gen().map(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_char_gen().map(|c| c.into()),
        4 => Gens::one_of_vec(vec!['@']).map(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn path_abempty_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, segment_str_gen().map(|s| format!("/{}", s)))
  }

  pub fn path_absolute_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, segment_nz_str_gen().map(|s| format!("/{}", s))).flat_map(|s1| {
      path_abempty_str_gen().map(move |s2| {
        let prefix = if !s1.starts_with("/") { "/" } else { "" };
        format!("{}{}{}", prefix, s1, s2)
      })
    })
  }

  pub fn path_no_scheme_str_gen() -> Gen<String> {
    segment_nz_nc_str_gen().flat_map(|s1| {
      rep_str_gen(1, 10, segment_str_gen().map(|s2| format!("/{}", s2))).map(move |s2| format!("{}{}", s1, s2))
    })
  }

  pub fn path_rootless_str_gen() -> Gen<String> {
    segment_nz_str_gen().flat_map(|s1| {
      rep_str_gen(1, 10, segment_str_gen().map(|s2| format!("/{}", s2))).map(move |s2| format!("{}{}", s1, s2))
    })
  }

  #[derive(Clone, Debug)]
  pub struct Pair<A, B>(pub(crate) A, pub(crate) B);

  impl<A, B> std::fmt::Display for Pair<A, B>
  where
    A: std::fmt::Display,
    B: std::fmt::Display,
  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "({},{})", self.0, self.1)
    }
  }

  pub fn path_str_with_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::choose_u8(1, 5).flat_map(|n| match n {
      1 => path_abempty_str_gen().map(|s| Pair("abempty_path".to_string(), s)),
      2 => path_absolute_str_gen().map(|s| Pair("absolute_path".to_string(), s)),
      3 => path_no_scheme_str_gen().map(|s| Pair("no_scheme_path".to_string(), s)),
      4 => path_rootless_str_gen().map(|s| Pair("rootless_path".to_string(), s)),
      5 => Gen::<String>::unit(|| Pair("empty_path".to_string(), "".to_string())),
      x => panic!("x = {}", x),
    })
  }

  pub fn path_str_without_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::choose_u8(1, 3).flat_map(|n| match n {
      1 => path_absolute_str_gen().map(|s| Pair("absolute_path".to_string(), s)),
      2 => path_rootless_str_gen().map(|s| Pair("rootless_path".to_string(), s)),
      3 => Gen::<String>::unit(|| Pair("empty_path".to_string(), "".to_string())),
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
  fn test_path() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::path_str_with_abempty_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_str_with_abempty_gen = {}", counter, s);
      let input = s.1.chars().collect::<Vec<_>>();
      let result = (path() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap().map(|e| e.to_string()).unwrap_or("".to_string()), s.1);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_abempty() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::path_abempty_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_abempty = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (path_abempty(true) - end())
        // .collect()
        // .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap().to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_absolute() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::path_absolute_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_abempty = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (path_absolute() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap().to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_noscheme() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::path_no_scheme_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_abempty = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (path_noscheme() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap().to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_rootless() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(gens::path_rootless_str_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_rootless_str_gen = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (path_rootless() - end())
        // .collect()
        // .map(String::from_iter)
        .parse(&input)
        .to_result();
      assert_eq!(result.unwrap().to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
