use crate::models::path::Path;
use crate::parsers::basic_parsers::*;
use oni_comb_parser_rs::prelude::*;

//  path          = path-abempty    ; begins with "/" or is empty
//                / path-absolute   ; begins with "/" but not "//"
//                / path-noscheme   ; begins with a non-colon segment
//                / path-rootless   ; begins with a segment
//                / path-empty      ; zero characters
pub fn path<'a>() -> Parser<'a, u8, Option<Path>> {
  (path_rootless().attempt() | path_abempty(true).attempt() | path_absolute().attempt() | path_noscheme())
    .opt()
    .name("path")
}

//  path-abempty  = *( "/" segment )
pub fn path_abempty<'a>(required: bool) -> Parser<'a, u8, Path> {
  let n = if required { 1 } else { 0 };
  ((elm(b'/') + segment()).collect())
    .map(|e| e.to_vec())
    .map_res(String::from_utf8)
    .repeat(n..)
    .map(|e| Path::of_abempty_from_strings(&e))
    .name("path-abempty")
}

//  path-absolute = "/" [ segment-nz *( "/" segment ) ]
pub fn path_absolute<'a>() -> Parser<'a, u8, Path> {
  let p = (seqment_nz() + ((elm(b'/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .opt();
  (elm(b'/').collect() + p)
    .map(|(a, b_opt)| match b_opt {
      None => vec![a],
      Some(b) => {
        let mut l = vec![a];
        l.extend_from_slice(&b);
        l
      }
    })
    .map(|e| {
      e.into_iter()
        .map(|e| e.to_vec())
        .map(|v| String::from_utf8(v).unwrap())
        .collect::<Vec<_>>()
    })
    .map(|e| Path::of_absolute_from_strings(&e))
    .name("path-absolute")
}

//  path-rootless = segment-nz *( "/" segment )
pub fn path_rootless<'a>() -> Parser<'a, u8, Path> {
  (seqment_nz() + ((elm(b'/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .map(|e| {
      e.into_iter()
        .map(|e| e.to_vec())
        .map(|v| String::from_utf8(v).unwrap())
        .collect::<Vec<_>>()
    })
    .map(|e| Path::of_rootless_from_strings(&e))
    .name("path-rootless")
}

//  path-noscheme = segment-nz-nc *( "/" segment )
pub fn path_noscheme<'a>() -> Parser<'a, u8, Path> {
  (seqment_nz_nc() + ((elm(b'/') + segment()).collect()).of_many0())
    .map(|(a, b)| {
      let mut l = vec![a];
      l.extend_from_slice(&b);
      l
    })
    .map(|e| {
      e.into_iter()
        .map(|e| e.to_vec())
        .map(|v| String::from_utf8(v).unwrap())
        .collect::<Vec<_>>()
    })
    .map(|e| Path::of_rootless_from_strings(&e))
    .name("path-noscheme")
}

// segment       = *pchar
fn segment<'a>() -> Parser<'a, u8, &'a [u8]> {
  pchar().of_many0().collect().name("segment")
}

// segment-nz    = 1*pchar
fn seqment_nz<'a>() -> Parser<'a, u8, &'a [u8]> {
  pchar().of_many1().collect().name("segment-nz")
}

// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
// ; non-zero-length segment without any colon ":"
fn seqment_nz_nc<'a>() -> Parser<'a, u8, &'a [u8]> {
  (unreserved() | pct_encoded() | sub_delims() | elm(b'@').collect())
    .of_many1()
    .collect()
    .name("segment-nz-nc")
}

#[cfg(test)]
pub mod gens {
  use std::fmt::Formatter;

  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::basic_parsers::gens::*;

  pub fn segment_gen() -> Gen<String> {
    pchar_gen(0, u8::MAX - 1)
  }

  pub fn segment_nz_gen() -> Gen<String> {
    pchar_gen(1, u8::MAX - 1)
  }

  pub fn segment_nz_nc_gen() -> Gen<String> {
    repeat_gen_of_string(1, u8::MAX - 1, {
      Gens::frequency([
        (1, unreserved_gen_of_char().map(|c| c.into())),
        (1, pct_encoded_gen()),
        (1, sub_delims_gen_of_char().map(|c| c.into())),
        (1, Gens::pure('@').map(|c| c.into())),
      ])
    })
  }

  pub fn path_abempty_gen() -> Gen<String> {
    repeat_gen_of_string(1, 10, segment_gen().map(|s| format!("/{}", s)))
  }

  pub fn path_absolute_gen() -> Gen<String> {
    repeat_gen_of_string(1, 10, segment_nz_gen().map(|s| format!("/{}", s))).flat_map(|s1| {
      path_abempty_gen().map(move |s2| {
        let prefix = if !s1.starts_with("/") { "/" } else { "" };
        format!("{}{}{}", prefix, s1, s2)
      })
    })
  }

  pub fn path_no_scheme_gen() -> Gen<String> {
    segment_nz_nc_gen().flat_map(|s1| {
      repeat_gen_of_string(1, 10, segment_gen().map(|s2| format!("/{}", s2))).map(move |s2| format!("{}{}", s1, s2))
    })
  }

  pub fn path_rootless_gen() -> Gen<String> {
    segment_nz_gen().flat_map(|s1| {
      repeat_gen_of_string(1, 10, segment_gen().map(|s2| format!("/{}", s2))).map(move |s2| format!("{}{}", s1, s2))
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

  pub fn path_with_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::frequency([
      (1, path_abempty_gen().map(|s| Pair("abempty_path".to_string(), s))),
      (1, path_absolute_gen().map(|s| Pair("absolute_path".to_string(), s))),
      (1, path_no_scheme_gen().map(|s| Pair("no_scheme_path".to_string(), s))),
      (1, path_rootless_gen().map(|s| Pair("rootless_path".to_string(), s))),
      (1, Gens::pure(Pair("empty_path".to_string(), "".to_string()))),
    ])
  }

  pub fn path_str_without_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::frequency([
      (1, path_absolute_gen().map(|s| Pair("absolute_path".to_string(), s))),
      (1, path_rootless_gen().map(|s| Pair("rootless_path".to_string(), s))),
      (1, Gens::pure(Pair("empty_path".to_string(), "".to_string()))),
    ])
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

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_path() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::path_with_abempty_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_str_with_abempty:string = {}", counter, s);
      let input = s.1.as_bytes();
      let result = (path() - end()).parse(input).to_result();
      let path = result.unwrap();
      log::debug!("{:>03}, path_str_with_abempty:object = {:?}", counter, path);
      assert_eq!(path.map(|e| e.to_string()).unwrap_or("".to_string()), s.1);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_abempty() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::path_abempty_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_abempty:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (path_abempty(true) - end()).parse(input).to_result();
      let path = result.unwrap();
      log::debug!("{:>03}, path_abempty:object = {:?}", counter, path);
      assert_eq!(path.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_absolute() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::path_absolute_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_absolute:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (path_absolute() - end()).parse(input).to_result();
      let path = result.unwrap();
      log::debug!("{:>03}, path_absolute:object = {:?}", counter, path);
      assert_eq!(path.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_noscheme() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::path_no_scheme_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_noscheme:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (path_noscheme() - end()).parse(input).to_result();
      let path = result.unwrap();
      log::debug!("{:>03}, path_noscheme:object = {:?}", counter, path);
      assert_eq!(path.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_rootless() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(gens::path_rootless_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, path_rootless:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (path_rootless() - end()).parse(input).to_result();
      let path = result.unwrap();
      log::debug!("{:>03}, path_rootless:object = {:?}", counter, path);
      assert_eq!(path.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
