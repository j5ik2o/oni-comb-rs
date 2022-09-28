use oni_comb_hocon_rs::model::ConfigFactory;
use rust_decimal::prelude::ToPrimitive;

// test_generator::test_expand_paths! { snapshot; "tests/data/*.conf" }

#[test]
fn parse() {
  let config = ConfigFactory::new().load_from_file("tests/data/a_1.conf").unwrap();
  let value = config.get_value("a").unwrap().as_number().unwrap().to_u32().unwrap();
  assert_eq!(value, 1);
}
