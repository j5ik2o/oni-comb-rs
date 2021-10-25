use oni_comb_rs::core::ParserRunner;
use oni_comb_rs::extension::parser::{CollectParser, ConversionParser};
use oni_comb_rs::prelude::*;

fn main() {
  let input = b"'hello world';";

  let parser = surround(
    elm(b'\''),
    (seq(b"hello") + elm_space() + seq(b"world")).collect(),
    elm(b'\'') + elm(b';'),
  )
  .convert(std::str::from_utf8);
  let result = parser.parse(input).unwrap();

  println!("{}", result);
}
