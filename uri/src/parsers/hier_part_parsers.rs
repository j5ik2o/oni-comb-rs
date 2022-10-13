use crate::models::hier_part::HierPart;
use crate::parsers::authority_parsers::authority;
use crate::parsers::path_parsers::{path_abempty, path_rootless};
use oni_comb_parser_rs::prelude::*;

//  hier-part     = "//" authority path-abempty
//                / path-absolute
//                / path-rootless
//                / path-empty
pub fn hier_part<'a>() -> Parser<'a, u8, Option<HierPart>> {
  let p1 = (seq(b"//") * authority() + path_abempty(false)).map(|(a, b)| HierPart::new(Some(a), b));
  let p2 = (path_abempty(true).attempt() | path_rootless()).map(|e| HierPart::of_path(e));
  (p1.attempt() | p2).opt()
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::authority_parsers::gens::authority_gen;
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parsers::path_parsers::gens::{path_abempty_gen, path_str_without_abempty_gen, Pair};

  pub fn hier_part_gen() -> Gen<Pair<String, Option<bool>>> {
    let gen1 = {
      authority_gen().flat_map(move |authority| {
        path_abempty_gen().map(move |path_abempty| format!("//{}{}", authority, path_abempty))
      })
    };
    let gen2 = { path_str_without_abempty_gen().map(|Pair(p1, p2)| Pair(p2, Some(p1 == "empty_path".to_string()))) };
    Gens::one_bool().flat_map(move |b| {
      if b {
        gen1.clone().map(|s| Pair(s, None))
      } else {
        gen2.clone()
      }
    })
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use crate::parsers::path_parsers::gens::Pair;
  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use super::gens::*;
  use super::*;

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_hier_part() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(hier_part_gen(), move |Pair(s, _b)| {
      counter += 1;
      log::debug!("{:>03}, hier_part:string = {}", counter, s);
      let input = s.as_bytes();
      let result = (hier_part() - end()).parse(input).to_result();
      let hier_port = result.unwrap();
      log::debug!("{:>03}, hier_part:object = {:?}", counter, hier_port);
      assert_eq!(hier_port.map(|e| e.to_string()).unwrap_or("".to_string()), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
