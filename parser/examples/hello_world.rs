use oni_comb_parser_rs::prelude::*;

fn main() {
  let input: &[u8; 14] = b"'hello world';";

  let parser: Parser<u8, &str> = surround(
    elm_ref(b'\''),
    (seq(b"hello") + elm_space() + seq(b"world")).collect(),
    elm_ref(b'\'') + elm_ref(b';'),
  )
  .map_res(std::str::from_utf8);
  let result: &str = parser.parse(input).to_result().unwrap();

  println!("{}", result);
}
