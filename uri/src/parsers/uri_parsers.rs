use crate::models::uri::Uri;
use crate::parsers::fragment_parsers::fragment;
use crate::parsers::hier_part_parsers::hier_part;
use crate::parsers::query_parsers::query;
use crate::parsers::scheme_parsers::scheme;
use oni_comb_parser_rs::prelude::*;

//  absolute-URI  = scheme ":" hier-part [ "?" query ]
pub fn absolute_uri<'a>() -> Parser<'a, u8, Uri> {
  ((scheme().opt() - elm(b':')) + hier_part() + (elm(b'?') * query()).opt()).map(|((a, b), c)| Uri::new(a, b, c, None))
}

// URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
pub fn uri<'a>() -> Parser<'a, u8, Uri> {
  ((scheme().opt() - elm(b':')) + hier_part() + (elm(b'?') * query()).opt() + (elm(b'#') * fragment()).opt())
    .map(|(((schema, hier_path), query), fragment)| Uri::new(schema, hier_path, query, fragment))
}

#[cfg(test)]
pub mod gens {
  use crate::parsers::fragment_parsers::gens::fragment_gen;
  use crate::parsers::hier_part_parsers::gens::hier_part_gen;
  use crate::parsers::path_parsers::gens::Pair;
  use crate::parsers::query_parsers::gens::query_gen;
  use crate::parsers::scheme_parsers::gens::scheme_gen;
  use prop_check_rs::gen::{Gen, Gens};

  // URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
  pub fn uri_gen() -> Gen<String> {
    scheme_gen().flat_map(|scheme| {
      let base_gen =
        hier_part_gen().map(move |Pair(hier_part, is_empty)| (format!("{}:{}", scheme, hier_part), is_empty));
      let query_gen = base_gen.flat_map(|(s, is_empty_opt)| {
        if is_empty_opt.unwrap_or(false) {
          Gens::unit((s.clone(), is_empty_opt))
        } else {
          query_gen().map(move |q| (format!("{}?{}", s, q), is_empty_opt))
        }
      });
      let fragment_gen = query_gen.flat_map(|(s, is_empty_opt)| {
        if is_empty_opt.unwrap_or(false) {
          Gens::unit(s.clone())
        } else {
          fragment_gen().map(move |f| format!("{}#{}", s, f))
        }
      });
      fragment_gen
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

  use super::gens::*;
  use super::*;

  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_uri() -> Result<()> {
    let mut counter = 0;
    let uri_gen = uri_gen();
    let prop = prop::for_all(uri_gen, move |s| {
      counter += 1;
      log::debug!("{:>03}, uri:string = {}", counter, s);
      let input = s.as_bytes();
      let uri = (uri() - end()).parse(input).success().unwrap();
      log::debug!("{:>03}, uri:object = {:?}", counter, uri);
      assert_eq!(uri.to_string(), s);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
