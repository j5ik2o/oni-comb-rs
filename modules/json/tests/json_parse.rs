use std::borrow::Cow;
use std::collections::BTreeMap;

use oni_comb_json::{parse, JsonValue};

// ── Primitives ──────────────────────────────────

#[test]
fn parse_null() {
  assert_eq!(parse("null").unwrap(), JsonValue::Null);
}

#[test]
fn parse_true() {
  assert_eq!(parse("true").unwrap(), JsonValue::Bool(true));
}

#[test]
fn parse_false() {
  assert_eq!(parse("false").unwrap(), JsonValue::Bool(false));
}

#[test]
fn parse_integer() {
  assert_eq!(parse("42").unwrap(), JsonValue::Number(42.0));
}

#[test]
fn parse_negative_integer() {
  assert_eq!(parse("-7").unwrap(), JsonValue::Number(-7.0));
}

#[test]
fn parse_float() {
  assert_eq!(parse("3.14").unwrap(), JsonValue::Number(3.14));
}

#[test]
fn parse_exponent() {
  assert_eq!(parse("1.5e10").unwrap(), JsonValue::Number(1.5e10));
}

#[test]
fn parse_zero() {
  assert_eq!(parse("0").unwrap(), JsonValue::Number(0.0));
}

#[test]
fn parse_string() {
  assert_eq!(parse(r#""hello""#).unwrap(), JsonValue::String(Cow::Borrowed("hello")));
}

#[test]
fn parse_string_with_escapes() {
  assert_eq!(
    parse(r#""hello\nworld""#).unwrap(),
    JsonValue::String(Cow::Owned("hello\nworld".to_string()))
  );
}

#[test]
fn parse_string_unicode_escape() {
  assert_eq!(
    parse(r#""\u2192""#).unwrap(),
    JsonValue::String(Cow::Owned("→".to_string()))
  );
}

#[test]
fn parse_string_surrogate_pair() {
  assert_eq!(
    parse(r#""\uD83D\uDE00""#).unwrap(),
    JsonValue::String(Cow::Owned("😀".to_string()))
  );
}

// ── Arrays ──────────────────────────────────────

#[test]
fn parse_empty_array() {
  assert_eq!(parse("[]").unwrap(), JsonValue::Array(vec![]));
}

#[test]
fn parse_array_of_ints() {
  assert_eq!(
    parse("[1, 2, 3]").unwrap(),
    JsonValue::Array(vec![
      JsonValue::Number(1.0),
      JsonValue::Number(2.0),
      JsonValue::Number(3.0),
    ])
  );
}

#[test]
fn parse_array_mixed() {
  let result = parse(r#"[1, "two", true, null]"#).unwrap();
  assert_eq!(
    result,
    JsonValue::Array(vec![
      JsonValue::Number(1.0),
      JsonValue::String(Cow::Borrowed("two")),
      JsonValue::Bool(true),
      JsonValue::Null,
    ])
  );
}

#[test]
fn parse_nested_arrays() {
  let result = parse("[[[[1]]]]").unwrap();
  assert_eq!(
    result,
    JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Array(
      vec![JsonValue::Number(1.0)]
    )])])])
  );
}

// ── Objects ─────────────────────────────────────

#[test]
fn parse_empty_object() {
  assert_eq!(parse("{}").unwrap(), JsonValue::Object(BTreeMap::new()));
}

#[test]
fn parse_object() {
  let result = parse(r#"{"name": "oni-comb", "version": 2}"#).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert(Cow::Borrowed("name"), JsonValue::String(Cow::Borrowed("oni-comb")));
  expected.insert(Cow::Borrowed("version"), JsonValue::Number(2.0));
  assert_eq!(result, JsonValue::Object(expected));
}

#[test]
fn parse_nested_object() {
  let result = parse(r#"{"a": {"b": {"c": 1}}}"#).unwrap();
  let mut c_map = BTreeMap::new();
  c_map.insert(Cow::Borrowed("c"), JsonValue::Number(1.0));
  let mut b_map = BTreeMap::new();
  b_map.insert(Cow::Borrowed("b"), JsonValue::Object(c_map));
  let mut a_map = BTreeMap::new();
  a_map.insert(Cow::Borrowed("a"), JsonValue::Object(b_map));
  assert_eq!(result, JsonValue::Object(a_map));
}

// ── Whitespace ──────────────────────────────────

#[test]
fn parse_with_surrounding_whitespace() {
  assert_eq!(parse("  { \"a\" : 1 }  ").unwrap(), {
    let mut m = BTreeMap::new();
    m.insert(Cow::Borrowed("a"), JsonValue::Number(1.0));
    JsonValue::Object(m)
  });
}

#[test]
fn parse_trailing_text_is_error() {
  assert!(parse("{} trailing").is_err());
}

// ── Errors ──────────────────────────────────────

#[test]
fn parse_empty_input_is_error() {
  assert!(parse("").is_err());
}

#[test]
fn parse_invalid_is_error() {
  assert!(parse("xyz").is_err());
}

#[test]
fn parse_lone_high_surrogate_is_error() {
  assert!(parse(r#""\uD83D""#).is_err());
}

// ── Error location ──────────────────────────────

#[test]
fn error_has_line_info() {
  let input = "{\n  \"key\": }";
  let err = parse(input).unwrap_err();
  // Error should be somewhere around line 2
  // The exact position depends on where the parser fails
  assert!(err.position > 0);
}

// ── Complex ─────────────────────────────────────

#[test]
fn parse_complex_json() {
  let input = r#"{
    "Image": {
      "Width": 800,
      "Height": 600,
      "Title": "View from 15th Floor",
      "IDs": [116, 943, 234, 38793],
      "Animated": false,
      "Thumbnail": {
        "Url": "http://www.example.com/image/481989943",
        "Height": 125,
        "Width": 100
      }
    }
  }"#;
  let result = parse(input);
  assert!(result.is_ok());
  if let JsonValue::Object(ref top) = result.unwrap() {
    assert!(top.contains_key("Image"));
  } else {
    panic!("Expected object");
  }
}
