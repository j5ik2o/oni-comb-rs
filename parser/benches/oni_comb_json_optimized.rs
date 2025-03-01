use oni_comb_parser_rs::prelude::*;
use oni_comb_parser_rs::StaticParser;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
  Null,
  Bool(bool),
  Str(String),
  Num(f64),
  Array(Vec<JsonValue>),
  Object(HashMap<String, JsonValue>),
}

// 静的ディスパッチを使用した最適化されたパーサー実装

// 通常のParserを返す関数（内部実装用）
fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

// StaticParserを返す関数（外部公開用）
fn space_optimized<'a>(
) -> StaticParser<'a, char, (), impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, ()> + Clone + 'a> {
  space().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn number_parser<'a>() -> Parser<'a, char, f64> {
  let integer = elm_digit_1_9_ref() - elm_digit_ref().of_many0() | elm_ref('0');
  let frac = elm_ref('.') + elm_digit_ref().of_many1();
  let exp = elm_of("eE") + elm_of("+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref('-').opt() + integer + frac.opt() + exp.opt();
  number.collect().map(String::from_iter).map_res(|s| f64::from_str(&s))
}

// StaticParserを返す関数（外部公開用）
fn number_optimized<'a>(
) -> StaticParser<'a, char, f64, impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, f64> + Clone + 'a> {
  number_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn string_parser<'a>() -> Parser<'a, char, String> {
  // エスケープ文字の処理
  let special_char = elm_ref('\\')
    | elm_ref('/')
    | elm_ref('"')
    | elm_ref('b').map(|_| &'\x08')
    | elm_ref('f').map(|_| &'\x0C')
    | elm_ref('n').map(|_| &'\n')
    | elm_ref('r').map(|_| &'\r')
    | elm_ref('t').map(|_| &'\t');

  // エスケープシーケンス
  let escape_sequence = elm_ref('\\') * special_char;

  // 通常の文字列（エスケープシーケンスを含む）
  let char_string = (none_ref_of("\\\"") | escape_sequence)
    .map(|c| *c) // Clone::clone の代わりに参照外し
    .of_many1()
    .map(String::from_iter);

  // UTF-16文字の処理（ベンチマークでは使用されないが、完全性のために残す）
  let utf16_char = tag("\\u")
    * elm_pred(|c: &char| c.is_digit(16))
      .of_count(4)
      .map(String::from_iter)
      .map_res(|digits| u16::from_str_radix(&digits, 16));

  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });

  // 文字列全体
  let string = surround(
    elm_ref('"'),
    (char_string | utf16_string).of_many0().cache(), // キャッシュを追加
    elm_ref('"'),
  );

  string.map(|strings| strings.concat())
}

// StaticParserを返す関数（外部公開用）
fn string_optimized<'a>(
) -> StaticParser<'a, char, String, impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, String> + Clone + 'a> {
  string_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn boolean_parser<'a>() -> Parser<'a, char, bool> {
  tag("true").map(|_| true) | tag("false").map(|_| false)
}

// StaticParserを返す関数（外部公開用）
fn boolean_optimized<'a>(
) -> StaticParser<'a, char, bool, impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, bool> + Clone + 'a> {
  boolean_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn array_parser<'a>() -> Parser<'a, char, Vec<JsonValue>> {
  // 空白を含むカンマ区切りのパターン
  let comma_sep = space() * elm_ref(',') - space();

  // 配列要素のパーサー（遅延評価）
  // 循環参照を避けるために、value_parserの代わりにvalue_parser_lazyを使用
  let value_parser_lazy = || -> Parser<'a, char, JsonValue> {
    // 各種パーサーを組み合わせて値パーサーを作成
    (string_parser().map(|text| JsonValue::Str(text)).cache()
      | number_parser().map(|num| JsonValue::Num(num)).cache()
      | boolean_parser().map(|b| JsonValue::Bool(b)).cache()
      | tag("null").map(|_| JsonValue::Null).cache()
      | lazy(array_parser).map(|arr| JsonValue::Array(arr)).cache()
      | lazy(object_parser).map(|obj| JsonValue::Object(obj)).cache())
      - space()
  };

  let elems = lazy(value_parser_lazy).cache().of_many0_sep(comma_sep);

  // 配列全体のパーサー（角括弧で囲まれた要素）
  surround(elm_ref('[') - space(), elems, space() * elm_ref(']'))
}

// StaticParserを返す関数（外部公開用）
fn array_optimized<'a>() -> StaticParser<
  'a,
  char,
  Vec<JsonValue>,
  impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, Vec<JsonValue>> + Clone + 'a,
> {
  array_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn object_parser<'a>() -> Parser<'a, char, HashMap<String, JsonValue>> {
  // 循環参照を避けるために、value_parserの代わりにvalue_parser_lazyを使用
  let value_parser_lazy = || -> Parser<'a, char, JsonValue> {
    // 各種パーサーを組み合わせて値パーサーを作成
    (string_parser().map(|text| JsonValue::Str(text)).cache()
      | number_parser().map(|num| JsonValue::Num(num)).cache()
      | boolean_parser().map(|b| JsonValue::Bool(b)).cache()
      | tag("null").map(|_| JsonValue::Null).cache()
      | lazy(array_parser).map(|arr| JsonValue::Array(arr)).cache()
      | lazy(object_parser).map(|obj| JsonValue::Object(obj)).cache())
      - space()
  };

  // キーと値のペアのパーサー
  let member = string_parser() - space() - elm_ref(':') - space() + lazy(value_parser_lazy).cache();

  // 空白を含むカンマ区切りのパターン
  let comma_sep = space() * elm_ref(',') - space();

  // オブジェクトメンバーのパーサー
  let members = member.of_many0_sep(comma_sep);

  // オブジェクト全体のパーサー（波括弧で囲まれたメンバー）
  let obj = surround(elm_ref('{') - space(), members, space() * elm_ref('}'));

  // メンバーをHashMapに変換
  obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
}

// StaticParserを返す関数（外部公開用）
fn object_optimized<'a>() -> StaticParser<
  'a,
  char,
  HashMap<String, JsonValue>,
  impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, HashMap<String, JsonValue>> + Clone + 'a,
> {
  object_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn value_parser<'a>() -> Parser<'a, char, JsonValue> {
  // 各種パーサーを組み合わせて値パーサーを作成
  (string_parser().map(|text| JsonValue::Str(text)).cache()
    | number_parser().map(|num| JsonValue::Num(num)).cache()
    | boolean_parser().map(|b| JsonValue::Bool(b)).cache()
    | tag("null").map(|_| JsonValue::Null).cache()
    | lazy(array_parser).map(|arr| JsonValue::Array(arr)).cache()
    | lazy(object_parser).map(|obj| JsonValue::Object(obj)).cache())
    - space()
}

// StaticParserを返す関数（外部公開用）
fn value_optimized<'a>(
) -> StaticParser<'a, char, JsonValue, impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a>
{
  value_parser().to_static_parser()
}

// 通常のParserを返す関数（内部実装用）
fn json_parser<'a>() -> Parser<'a, char, JsonValue> {
  // 先頭の空白をスキップし、値をパースし、終端を確認
  space() * value_parser() - end()
}

// StaticParserを返す関数（外部公開用）
fn json_optimized<'a>(
) -> StaticParser<'a, char, JsonValue, impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a>
{
  json_parser().to_static_parser()
}

pub fn oni_comb_parse_json_optimized(s: &str) {
  // 文字列を文字のベクターに変換
  let input: Vec<char> = s.chars().collect();

  // 最適化されたパーサーを使用
  let parser = json_optimized();

  // パース実行
  let _ = parser.parse(&input).success().unwrap();
}
