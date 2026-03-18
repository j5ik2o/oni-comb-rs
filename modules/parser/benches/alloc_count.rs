#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[path = "shared/oni_comb_json.rs"]
mod oni_comb_json;

/// コンビネータ合成のみを測定するため、String 構築を行わない identifier パーサー。
/// satisfy + take_while0 の .zip() で (char, &str) を返す。
fn parse_identifier_no_alloc(s: &str) -> Option<(char, &str)> {
  let head = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
  let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');
  let mut parser = head.zip(tail);
  let mut input = StrInput::new(s);
  parser.parse_next(&mut input).ok()
}

/// take_while1 は &str を返すのでアロケーション不要。
fn parse_integer_no_alloc(s: &str) -> Option<&str> {
  let mut parser = take_while1(|c: char| c.is_ascii_digit());
  let mut input = StrInput::new(s);
  parser.parse_next(&mut input).ok()
}

/// flat_map 同一型: satisfy + tag のチェーン。Box 不要なのでアロケーション 0 のはず。
fn parse_flat_map_same_type_no_alloc(s: &str) -> Option<&str> {
  let mut parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|c| match c {
    '1' => tag("one"),
    '2' => tag("two"),
    '3' => tag("three"),
    _ => tag(""),
  });
  let mut input = StrInput::new(s);
  parser.parse_next(&mut input).ok()
}

static JSON_STR: &str = include_str!("data/sample.json");

fn main() {
  let compact = r#"{"a":[1,2],"b":{"c":true}}"#;
  let spaced = r#" { "a" : [ 1 , 2 ] , "b" : { "c" : true } } "#;
  assert_eq!(
    oni_comb_json::parse_complete(compact).unwrap(),
    oni_comb_json::parse_complete(spaced).unwrap()
  );

  let _profiler = dhat::Profiler::new_heap();

  // ── Token ワークロード（ゼロアロケーション確認） ──
  let id_inputs = ["x", "foo", "foo_bar_123", "_private"];
  let int_inputs = ["0", "42", "9999999"];
  let flat_map_inputs = ["1one", "2two", "3three"];

  for input in id_inputs {
    let _ = parse_identifier_no_alloc(input);
  }
  for input in int_inputs {
    let _ = parse_integer_no_alloc(input);
  }
  for input in flat_map_inputs {
    let _ = parse_flat_map_same_type_no_alloc(input);
  }

  // ── JSON フルパース（Vec のみアロケーション） ──
  let mut input = StrInput::new(JSON_STR);
  let _ = oni_comb_json::json_value(&mut input).unwrap();
}
