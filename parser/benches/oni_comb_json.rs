use oni_comb_parser_rs::prelude::*;
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

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn number<'a>() -> Parser<'a, char, f64> {
  let integer = elm_digit_1_9_ref() - elm_digit_ref().of_many0() | elm_ref('0');
  let frac = elm_ref('.') + elm_digit_ref().of_many1();
  let exp = elm_of("eE") + elm_of("+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref('-').opt() + integer + frac.opt() + exp.opt();
  number.collect().map(String::from_iter).map_res(|s| f64::from_str(&s))
}

fn string<'a>() -> Parser<'a, char, String> {
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
  let utf16_char: Parser<char, u16> = tag("\\u")
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

fn array<'a>() -> Parser<'a, char, Vec<JsonValue>> {
  // 空白を含むカンマ区切りのパターン
  let comma_sep = space() * elm_ref(',') - space();

  // 配列要素のパーサー（遅延評価）
  let elems = lazy(value).cache().of_many0_sep(comma_sep);

  // 配列全体のパーサー（角括弧で囲まれた要素）
  surround(elm_ref('[') - space(), elems, space() * elm_ref(']'))
}

fn object<'a>() -> Parser<'a, char, HashMap<String, JsonValue>> {
  // キーと値のペアのパーサー
  let member = string().cache() - space() - elm_ref(':') - space() + lazy(value).cache();

  // 空白を含むカンマ区切りのパターン
  let comma_sep = space() * elm_ref(',') - space();

  // オブジェクトメンバーのパーサー
  let members = member.of_many0_sep(comma_sep);

  // オブジェクト全体のパーサー（波括弧で囲まれたメンバー）
  let obj = surround(elm_ref('{') - space(), members, space() * elm_ref('}'));

  // メンバーをHashMapに変換
  obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
}

fn boolean<'a>() -> Parser<'a, char, bool> {
  tag("true").map(|_| true) | tag("false").map(|_| false)
}

fn value<'a>() -> Parser<'a, char, JsonValue> {
  // 各種JSONの値をパースするパーサーを組み合わせる
  // 最も頻度の高いものから順に試す（パフォーマンス向上のため）
  (string().map(|text| JsonValue::Str(text)).cache()
    | number().map(|num| JsonValue::Num(num)).cache()
    | boolean().map(|b| JsonValue::Bool(b)).cache()
    | tag("null").map(|_| JsonValue::Null).cache()
    | array().map(|arr| JsonValue::Array(arr)).cache()
    | object().map(|obj| JsonValue::Object(obj)).cache())
    - space()
}

pub fn json<'a>() -> Parser<'a, char, JsonValue> {
  // 先頭の空白をスキップし、値をパースし、終端を確認
  space() * value().cache() - end()
}

pub fn oni_comb_parse_json(s: &str) {
  // 文字列を文字のベクターに変換
  let input: Vec<char> = s.chars().collect();

  // キャッシュを有効にしたパーサーを使用
  let parser = json();

  // パース実行
  let _ = parser.parse(&input).success().unwrap();
}

// バイトレベルでのJSONパーサー
// 注: 現在は使用していませんが、将来的にはこちらに移行する予定
mod byte_json {
  use oni_comb_parser_rs::prelude::*;
  use std::collections::HashMap;
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

  // 空白文字をスキップ
  pub fn space<'a>() -> Parser<'a, u8, ()> {
    elm_of(&b" \t\r\n"[..]).of_many0().discard()
  }

  // 数値のパース
  pub fn number<'a>() -> Parser<'a, u8, f64> {
    // 整数部分
    let integer =
      elm_pred(|b: &u8| *b >= b'1' && *b <= b'9') - elm_pred(|b: &u8| *b >= b'0' && *b <= b'9').of_many0() | elm(b'0');

    // 小数部分
    let frac = elm(b'.') + elm_pred(|b: &u8| *b >= b'0' && *b <= b'9').of_many1();

    // 指数部分
    let exp = elm_of(&b"eE"[..]) + elm_of(&b"+-"[..]).opt() + elm_pred(|b: &u8| *b >= b'0' && *b <= b'9').of_many1();

    // 数値全体
    let number = elm(b'-').opt() + integer + frac.opt() + exp.opt();

    // バイト列を文字列に変換し、さらに浮動小数点数に変換
    number.collect().map(|bytes| {
      let s = std::str::from_utf8(bytes).unwrap();
      f64::from_str(s).unwrap()
    })
  }

  // 文字列のパース
  pub fn string<'a>() -> Parser<'a, u8, String> {
    // エスケープシーケンスの処理
    let escape_char = elm_ref(b'\\')
      * (elm(b'\\')
        | elm(b'/')
        | elm(b'"')
        | elm(b'b').map(|_| b'\x08')
        | elm(b'f').map(|_| b'\x0C')
        | elm(b'n').map(|_| b'\n')
        | elm(b'r').map(|_| b'\r')
        | elm(b't').map(|_| b'\t'));

    // 通常の文字（エスケープや引用符以外）
    let regular_char = elm_pred(|b: &u8| *b != b'\\' && *b != b'"');

    // 文字列内の任意の文字
    let string_char = regular_char | escape_char;

    // 文字列全体
    let string_parser = elm(b'"') * string_char.of_many0().collect().cache() - elm(b'"');

    // バイト列を文字列に変換
    string_parser.map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
  }

  // 真偽値のパース
  pub fn boolean<'a>() -> Parser<'a, u8, bool> {
    seq(b"true").map(|_| true) | seq(b"false").map(|_| false)
  }

  // 配列のパース
  pub fn array<'a>() -> Parser<'a, u8, Vec<JsonValue>> {
    // 空白を含むカンマ区切りのパターン
    let comma_sep = space() * elm(b',') - space();

    // 配列要素のパーサー（遅延評価）
    let elems = lazy(value).cache().of_many0_sep(comma_sep);

    // 配列全体のパーサー（角括弧で囲まれた要素）
    surround(elm(b'[') - space(), elems, space() * elm(b']'))
  }

  // オブジェクトのパース
  pub fn object<'a>() -> Parser<'a, u8, HashMap<String, JsonValue>> {
    // キーと値のペアのパーサー
    let member = string().cache() - space() - elm(b':') - space() + lazy(value).cache();

    // 空白を含むカンマ区切りのパターン
    let comma_sep = space() * elm(b',') - space();

    // オブジェクトメンバーのパーサー
    let members = member.of_many0_sep(comma_sep);

    // オブジェクト全体のパーサー（波括弧で囲まれたメンバー）
    let obj = surround(elm(b'{') - space(), members, space() * elm(b'}'));

    // メンバーをHashMapに変換
    obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
  }

  // JSON値のパース
  pub fn value<'a>() -> Parser<'a, u8, JsonValue> {
    // 各種JSONの値をパースするパーサーを組み合わせる
    // 最も頻度の高いものから順に試す（パフォーマンス向上のため）
    (
      // 単純な値（頻度が高い順）
      string().map(|text| JsonValue::Str(text)).cache()
        | number().map(|num| JsonValue::Num(num)).cache()
        | boolean().map(|b| JsonValue::Bool(b)).cache()
        | seq(b"null").map(|_| JsonValue::Null).cache()
        | array().map(|arr| JsonValue::Array(arr)).cache()
        | object().map(|obj| JsonValue::Object(obj)).cache()
    ) - space()
  }

  // JSONドキュメント全体のパース
  pub fn json<'a>() -> Parser<'a, u8, JsonValue> {
    // 先頭の空白をスキップし、値をパースし、終端を確認
    space() * value().cache() - end()
  }

  // ベンチマーク用の関数
  pub fn parse_json(s: &str) -> JsonValue {
    // 文字列をバイト列として直接処理
    let input = s.as_bytes();
    json().parse(input).success().unwrap()
  }
}
