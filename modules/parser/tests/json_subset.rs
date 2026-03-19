//! JSON subset 統合テスト (MS4 完了条件の実証)
//!
//! 再帰パーサー (MS5) がないため、値はプリミティブ(null/bool/int/string)のみ。
//! 配列・オブジェクトの値にネストした配列・オブジェクトは含まない。

use oni_comb_parser::error::ParseError;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

/// 空白をスキップするトークンラッパー
fn ws<P>(p: P) -> impl Parser<StrInputStream<'static>, Output = P::Output, Error = ParseError>
where
  P: Parser<StrInputStream<'static>, Error = ParseError>, {
  whitespace0().zip_right(p).zip_left(whitespace0())
}

/// JSON プリミティブ値
#[derive(Debug, PartialEq)]
enum JsonValue {
  Null,
  Bool(bool),
  Int(i64),
  Str(String),
  Array(Vec<JsonValue>),
  Object(Vec<(String, JsonValue)>),
}

/// プリミティブ値パーサー (null | bool | int | string)
fn json_primitive() -> impl Parser<StrInputStream<'static>, Output = JsonValue, Error = ParseError> {
  let null = tag("null").map(|_| JsonValue::Null);
  let bool_true = tag("true").map(|_| JsonValue::Bool(true));
  let bool_false = tag("false").map(|_| JsonValue::Bool(false));
  let int = integer().map(JsonValue::Int);
  let string = quoted_string().map(|s| JsonValue::Str(s.into_owned()));

  null.or(bool_true).or(bool_false).or(int).or(string)
}

/// 配列パーサー (値はプリミティブのみ)
fn json_array() -> impl Parser<StrInputStream<'static>, Output = JsonValue, Error = ParseError> {
  ws(char('['))
    .zip_right(ws(json_primitive()).sep_by0(ws(char(','))))
    .zip_left(ws(char(']')))
    .map(JsonValue::Array)
}

/// オブジェクトパーサー (値はプリミティブのみ)
fn json_object() -> impl Parser<StrInputStream<'static>, Output = JsonValue, Error = ParseError> {
  let pair = ws(quoted_string())
    .map(|s| s.into_owned())
    .zip_left(ws(char(':')))
    .zip(ws(json_primitive()));

  ws(char('{'))
    .zip_right(pair.sep_by0(ws(char(','))))
    .zip_left(ws(char('}')))
    .map(JsonValue::Object)
}

/// トップレベルの JSON 値パーサー (1 段ネスト)
fn json_value() -> impl Parser<StrInputStream<'static>, Output = JsonValue, Error = ParseError> {
  json_primitive().or(json_array()).or(json_object())
}

// ── テスト ────────────────────────────────────

#[test]
fn parse_null() {
  let mut input = StrInputStream::new("null");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Null);
}

#[test]
fn parse_true() {
  let mut input = StrInputStream::new("true");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Bool(true));
}

#[test]
fn parse_false() {
  let mut input = StrInputStream::new("false");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Bool(false));
}

#[test]
fn parse_integer() {
  let mut input = StrInputStream::new("42");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Int(42));
}

#[test]
fn parse_negative_integer() {
  let mut input = StrInputStream::new("-7");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Int(-7));
}

#[test]
fn parse_string() {
  let mut input = StrInputStream::new("\"hello\"");
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Str("hello".to_string())
  );
}

#[test]
fn parse_string_with_escapes() {
  let mut input = StrInputStream::new(r#""hello\nworld""#);
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Str("hello\nworld".to_string())
  );
}

#[test]
fn parse_empty_array() {
  let mut input = StrInputStream::new("[]");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Array(vec![]));
}

#[test]
fn parse_array_of_ints() {
  let mut input = StrInputStream::new("[1, 2, 3]");
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Array(vec![JsonValue::Int(1), JsonValue::Int(2), JsonValue::Int(3),])
  );
}

#[test]
fn parse_array_of_mixed() {
  let mut input = StrInputStream::new(r#"[1, "two", true, null]"#);
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Array(vec![
      JsonValue::Int(1),
      JsonValue::Str("two".to_string()),
      JsonValue::Bool(true),
      JsonValue::Null,
    ])
  );
}

#[test]
fn parse_empty_object() {
  let mut input = StrInputStream::new("{}");
  assert_eq!(json_value().parse_next(&mut input).unwrap(), JsonValue::Object(vec![]));
}

#[test]
fn parse_object() {
  let mut input = StrInputStream::new(r#"{"name": "oni-comb", "version": 2, "active": true}"#);
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Object(vec![
      ("name".to_string(), JsonValue::Str("oni-comb".to_string())),
      ("version".to_string(), JsonValue::Int(2)),
      ("active".to_string(), JsonValue::Bool(true)),
    ])
  );
}

#[test]
fn parse_object_with_whitespace() {
  let input_str = r#"{
        "key" : "value" ,
        "num" : 42
    }"#;
  let mut input = StrInputStream::new(input_str);
  assert_eq!(
    json_value().parse_next(&mut input).unwrap(),
    JsonValue::Object(vec![
      ("key".to_string(), JsonValue::Str("value".to_string())),
      ("num".to_string(), JsonValue::Int(42)),
    ])
  );
}

#[test]
fn parse_consumes_correct_amount() {
  let mut input = StrInputStream::new("[1, 2] rest");
  let result = json_value().parse_next(&mut input).unwrap();
  assert_eq!(result, JsonValue::Array(vec![JsonValue::Int(1), JsonValue::Int(2)]));
  assert_eq!(input.remaining(), "rest");
}
