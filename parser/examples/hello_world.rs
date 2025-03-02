use oni_comb_parser_rs::prelude::*;

fn main() {
  let input: &[u8; 14] = b"'hello world';";

  // 従来のParserを使用した例
  let parser1: Parser<u8, &str> = surround(
    elm_ref(b'\''),
    (seq(b"hello") + elm_space() + seq(b"world")).collect(),
    elm_ref(b'\'') + elm_ref(b';'),
  )
  .map_res(std::str::from_utf8);

  let result1 = parser1.parse(input).to_result().unwrap();
  println!("Parser result: {}", result1);

  // 直接StaticParserを使用する例
  {
    use oni_comb_parser_rs::prelude::static_parsers::*;

    // 単純なStaticParserの例
    let hello_parser: StaticParser<u8, String> =
      seq_static(b"hello").map_res(|bytes: Vec<u8>| std::str::from_utf8(&bytes).map(|s| s.to_string()));
    let hello_result = hello_parser.parse(&input[1..6]).to_result().unwrap();
    println!("StaticParser simple result: {}", hello_result);

    // 単純なパーサーを組み合わせる例
    let hello_world_parser: StaticParser<u8, String> =
      seq_static(b"hello world").map_res(|bytes: Vec<u8>| std::str::from_utf8(&bytes).map(|s| s.to_string()));

    let hello_world_result = hello_world_parser.parse(&input[1..12]).to_result().unwrap();
    println!("StaticParser complex result: {}", hello_world_result);
  }
}
