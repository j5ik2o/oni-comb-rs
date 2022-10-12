use oni_comb_hocon_rs::model::ConfigFactory;
use rust_decimal::prelude::ToPrimitive;

// test_generator::test_expand_paths! { snapshot; "tests/data/*.conf" }
#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn parse_a_1() {
    let config = ConfigFactory::new().load_from_file("tests/data/a_1.conf").unwrap();
    let value = config.get_value("a").unwrap().as_number().unwrap().to_u32().unwrap();
    assert_eq!(value, 1);
  }

  #[test]
  fn parse_b_2() {
    let config = ConfigFactory::new().load_from_file("tests/data/b_1.conf").unwrap();
    let value = config.get_value("b").unwrap().as_number().unwrap().to_u32().unwrap();
    assert_eq!(value, 2);
  }

  #[test]
  fn parse_bom() {
    let config = ConfigFactory::new().load_from_file("tests/data/bom.conf").unwrap();
    let value = config.get_value("foo").unwrap().as_string().unwrap();
    assert_eq!(value, "bar");
  }

  // #[test]
  // fn parse_test_01() {
  //   let result = ConfigFactory::new().load_from_file("tests/data/test01.conf");
  //   println!("{:?}", result.unwrap());
  // }
}
