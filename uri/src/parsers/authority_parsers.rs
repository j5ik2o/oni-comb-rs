//  authority     = [ userinfo "@" ] host [ ":" port ]

use crate::parsers::host_parsers::host;
use crate::parsers::port_parsers::port;
use crate::parsers::user_info_parsers::user_info;
use oni_comb_parser_rs::prelude::*;

pub fn authority<'a>() -> Parser<'a, char, &'a [char]> {
  ((user_info() + elm_ref('@')).opt() + host() + (elm_ref(':') + port()).opt()).collect()
}

#[cfg(test)]
pub mod gens {
  use super::*;
  use crate::parsers::basic_parsers::gens::to_option;
  use crate::parsers::host_parsers::gens::host_gen;
  use crate::parsers::port_parsers::gens::port_gen;
  use crate::parsers::user_info_parsers::gens::user_info_gen;
  use prop_check_rs::gen::Gen;

  pub fn authority_gen() -> Gen<String> {
    let user_info_opt_gen = to_option(user_info_gen());
    user_info_opt_gen.flat_map(move |ui| {
      let port_opt_gen = to_option(port_gen());
      host_gen()
        .flat_map(move |h| {
          port_opt_gen.clone().map(move |p| {
            let p = p.map(|s| format!(":{}", s)).unwrap_or("".to_string());
            format!("{}{}", h, p)
          })
        })
        .map(move |hp| {
          let ui = ui.as_ref().map(|s| format!("{}@", s)).unwrap_or("".to_string());
          format!("{}{}", ui, hp)
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

  use super::gens::*;
  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_authority() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(authority_gen(), move |s| {
      counter += 1;
      log::debug!("{:>03}, authority = {}", counter, s);
      let input = s.chars().collect::<Vec<_>>();
      let result = (authority() - end())
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
