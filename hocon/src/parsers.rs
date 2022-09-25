use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;

use crate::model::config_array_value::ConfigArrayValue;
use crate::model::config_duration_value::ConfigDurationValue;
use crate::model::config_include_value::ConfigIncludeValue;
use crate::model::config_number_value::ConfigNumberValue;
use crate::model::config_object_value::ConfigObjectValue;
use crate::model::config_value::ConfigValue;
use crate::model::time_unit::TimeUnit;
use crate::model::ConfigMergeable;
use oni_comb_parser_rs::prelude::*;

fn space_or_comment<'a>() -> Parser<'a, u8, ()> {
  let sp_tab_cr_lf = elm_of(b" \t\r\n").of_many0();

  let head = elm_ref(b'#').collect() | seq(b"//");
  let tail = take_till0(|c| matches!(*c, b'\r' | b'\n'));
  let comment = (sp_tab_cr_lf.clone().opt() * head + tail).collect();

  (comment.opt() + sp_tab_cr_lf).discard()
}

fn include_method<'a>() -> Parser<'a, u8, String> {
  (seq(b"file") | seq(b"url"))
    .collect()
    .map_res(std::str::from_utf8)
    .map(String::from)
}

fn include_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (seq(b"include") * elm(b' ') * include_method() + path().surround(seq(b"(\""), seq(b"\")")))
    .map(|(method, path)| ConfigValue::Include(ConfigIncludeValue::new(method, path)))
}

fn number<'a>() -> Parser<'a, u8, (Option<&'a u8>, String, Option<String>, Option<String>)> {
  let integer = (elm_digit_1_9_ref() + elm_digit_ref().of_many0()).collect() | elm_ref(b'0').collect();
  let frac = elm_ref(b'.') + elm_digit_ref().of_many1();
  let exp = elm_of(b"eE") + elm_of(b"+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref(b'-').opt()
    + integer.map_res(std::str::from_utf8).map(String::from)
    + frac.collect().map_res(std::str::from_utf8).map(String::from).opt()
    + exp.collect().map_res(std::str::from_utf8).map(String::from).opt();
  number
    .surround(space_or_comment(), space_or_comment())
    .map(|(((s, i), f), e)| (s, i, f, e))
}

fn duration<'a>() -> Parser<'a, u8, (ConfigNumberValue, TimeUnit)> {
  let ns = (seq(b"ns").attempt()
    | seq(b"nano").attempt()
    | seq(b"nanos").attempt()
    | seq(b"nanosecond").attempt()
    | seq(b"nanoseconds"))
  .map(|_| TimeUnit::Nanoseconds);
  let us = (seq(b"us").attempt()
    | seq(b"micro").attempt()
    | seq(b"micros").attempt()
    | seq(b"microsecond").attempt()
    | seq(b"microseconds"))
  .map(|_| TimeUnit::Microseconds);
  let ms = (seq(b"ms").attempt()
    | seq(b"milli").attempt()
    | seq(b"millis").attempt()
    | seq(b"millisecond").attempt()
    | seq(b"milliseconds"))
  .map(|_| TimeUnit::Milliseconds);
  let s = (seq(b"s").attempt() | seq(b"second").attempt() | seq(b"seconds")).map(|_| TimeUnit::Seconds);
  let m = (seq(b"m").attempt() | seq(b"minute").attempt() | seq(b"minutes")).map(|_| TimeUnit::Minutes);
  let h = (seq(b"h").attempt() | seq(b"hour").attempt() | seq(b"hours")).map(|_| TimeUnit::Hours);
  let d = (seq(b"d").attempt() | seq(b"day").attempt() | seq(b"days")).map(|_| TimeUnit::Days);

  number_value() + (ns.attempt() | us.attempt() | ms.attempt() | s.attempt() | m.attempt() | h.attempt() | d)
}

fn duration_value<'a>() -> Parser<'a, u8, ConfigValue> {
  duration().map(|(nv, u)| ConfigValue::Duration(ConfigDurationValue::new(nv, u)))
}

fn number_value<'a>() -> Parser<'a, u8, ConfigNumberValue> {
  number().map(|(s, i, f, e)| match (s, i, f, e) {
    (None, i, None, None) => ConfigNumberValue::from(i),
    (Some(_), i, None, None) => ConfigNumberValue::from(i),
    (_, i, Some(f), None) => {
      let mut s = i;
      s.push_str(&f);
      ConfigNumberValue::from(s)
    }
    (_, i, Some(f), Some(e)) => {
      let mut s = i;
      s.push_str(&f);
      s.push_str(&e);
      ConfigNumberValue::from(s)
    }
    _ => panic!("no match !!!"),
  })
}

fn string_double_quote_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'"').surround(space_or_comment(), space_or_comment())
}

fn string_single_quote_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'\'').surround(space_or_comment(), space_or_comment())
}

fn string<'a>() -> Parser<'a, u8, String> {
  let special_char = elm_ref(b'\\')
    | elm_ref(b'/')
    | elm_ref(b'"')
    | elm_ref(b'b').map(|_| &b'\x08')
    | elm_ref(b'f').map(|_| &b'\x0C')
    | elm_ref(b'n').map(|_| &b'\n')
    | elm_ref(b'r').map(|_| &b'\r')
    | elm_ref(b't').map(|_| &b'\t');
  let escape_sequence = elm_ref(b'\\') * special_char;
  let char_string = (none_ref_of(b"\\\"'") | escape_sequence)
    .map(Clone::clone)
    .of_many1()
    .map_res(String::from_utf8);
  let utf16_char = seq(b"\\u")
    * elm_hex_digit()
      .of_count(4)
      .map_res(String::from_utf8)
      .map_res(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string_surround1 = (char_string.clone().attempt() | utf16_string.clone())
    .of_many0()
    .surround(string_double_quote_bracket(), string_double_quote_bracket());
  let string_surround2 = (char_string.clone().attempt() | utf16_string.clone())
    .of_many0()
    .surround(string_single_quote_bracket(), string_single_quote_bracket());
  (string_surround2.attempt() | string_surround1).map(|strings| strings.concat())
}

fn string_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  string().map(ConfigValue::String)
}

fn array_left_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'[').surround(space_or_comment(), space_or_comment())
}

fn array_right_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b']').surround(space_or_comment(), space_or_comment())
}

fn array<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  let elems = lazy(config_value).of_many0_sep(comma());
  elems.surround(array_left_bracket(), array_right_bracket())
}

fn kv<'a>() -> Parser<'a, u8, ()> {
  elm_ref_of(b"=:").discard()
}

fn key<'a>() -> Parser<'a, u8, String> {
  (path() | string()).surround(space_or_comment(), space_or_comment())
}

fn property<'a>() -> Parser<'a, u8, (String, ConfigValue)> {
  key() + ((kv() * lazy(config_value)).attempt() | object_config_value().attempt() | array_config_value())
}

fn property_config_value<'a>() -> Parser<'a, u8, (String, ConfigValue)> {
  property().map(|(k, v)| (k, v))
}

fn object_left_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'{').surround(space_or_comment(), space_or_comment())
}

fn object_right_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'}').surround(space_or_comment(), space_or_comment())
}

fn comma<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b',').surround(space_or_comment(), space_or_comment())
}

fn object<'a>() -> Parser<'a, u8, HashMap<String, ConfigValue>> {
  let properties: Parser<'a, u8, Vec<(String, ConfigValue)>> = lazy(property).of_many0_sep(comma().opt());
  let obj: Parser<'a, u8, Vec<(String, ConfigValue)>> =
    properties.surround(object_left_bracket(), object_right_bracket());
  obj.map(|properties| {
    let m: HashMap<String, ConfigValue> = HashMap::new();
    properties.into_iter().fold(m, |mut r, e| {
      match r.get_mut(&e.0) {
        Some(v) => v.push(e.1),
        None => {
          r.insert(e.0, e.1);
        }
      };
      r
    })
  })
}

fn path_element<'a>() -> Parser<'a, u8, &'a [u8]> {
  (elm_alpha() | elm_of(b"-_")).of_many1().collect()
}

fn path<'a>() -> Parser<'a, u8, String> {
  path_element()
    .of_many1_sep(elm(b'.'))
    .collect()
    .map_res(std::str::from_utf8)
    .map(String::from)
}

fn reference_left_bracket<'a>() -> Parser<'a, u8, &'a [u8]> {
  seq(b"${").collect().surround(space_or_comment(), space_or_comment())
}

fn reference_right_bracket<'a>() -> Parser<'a, u8, &'a [u8]> {
  seq(b"}").surround(space_or_comment(), space_or_comment())
}

fn reference<'a>() -> Parser<'a, u8, (bool, String)> {
  (elm_ref(b'?').opt().map(|v| v.is_some())
    + path_element()
      .of_many1_sep(elm(b'.'))
      .collect()
      .map_res(std::str::from_utf8)
      .map(String::from))
  .surround(reference_left_bracket(), reference_right_bracket())
}

fn simple_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (seq(b"null").map(|_| ConfigValue::Null)
    | seq(b"true").map(|_| ConfigValue::Bool(true))
    | seq(b"false").map(|_| ConfigValue::Bool(false))
    | duration_value().attempt()
    | number_value().map(ConfigValue::Number).attempt()
    | string_config_value())
  .surround(space_or_comment(), space_or_comment())
}

fn object_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  object().map(|v| ConfigValue::Object(ConfigObjectValue::new(v)))
}

fn array_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  array().map(|v| ConfigValue::Array(ConfigArrayValue::new(v)))
}

fn reference_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  reference().map(|(missing, ref_name)| ConfigValue::Reference {
    prev: None,
    path: ref_name,
    missing,
  })
}

fn config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (simple_config_value() | object_config_value() | array_config_value() | reference_config_value())
    .surround(space_or_comment(), space_or_comment())
}

fn config<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  property_config_value()
    .of_many1()
    .map(|values: Vec<(String, ConfigValue)>| {
      let map = values.into_iter().fold(HashMap::new(), |mut key_values, (k, v)| {
        match key_values.get_mut(&k) {
          None => {
            key_values.insert(k, v);
          }
          Some(cv) => {
            cv.merge_with(v);
          }
        }
        key_values
      });
      vec![ConfigValue::Object(ConfigObjectValue::new(map))]
    })
    .attempt()
    | include_config_value().of_many1().attempt()
    | (object_config_value().attempt() | array_config_value()).of_many0()
}

pub fn hocon<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  space_or_comment() * config() - end()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::try_init();
  }

  #[test]
  fn include_test() {
    let input = br#"include file("abc.conf")"#;
    let binding = hocon().parse(input).to_result().unwrap();
    let result = binding[0].clone();
    assert_eq!(
      result,
      ConfigValue::Include(ConfigIncludeValue::new("file".to_string(), "abc.conf".to_string()))
    )
  }

  #[test]
  fn string_single_quote() {
    let result = string_config_value().parse(br#"'abc'"#);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(ast, ConfigValue::String("abc".to_string()));
  }

  #[test]
  fn string_double_quote() {
    let result = string_config_value().parse(br#""abc""#);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(ast, ConfigValue::String("abc".to_string()));
  }

  #[test]
  fn a_1() {
    let input = br#"
        a=1
        "#;
    let result = hocon().parse(input);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(
      ast[0],
      ConfigValue::Object(ConfigObjectValue::from((
        "a".to_string(),
        ConfigValue::Number(ConfigNumberValue::from("1".to_owned()))
      )))
    );
  }

  #[test]
  fn b_1() {
    let input = br#"
        b=1
        "#;
    let result = hocon().parse(input);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(
      ast[0],
      ConfigValue::Object(ConfigObjectValue::from((
        "b".to_string(),
        ConfigValue::Number(ConfigNumberValue::from("1".to_owned()))
      )))
    );
  }

  #[test]
  fn bom() {
    let input = br#"
        # test abc
        foo = "bar"
        "#;
    let result = hocon().parse(input);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(
      ast.first().unwrap().clone(),
      ConfigValue::Object(ConfigObjectValue::from((
        "foo".to_string(),
        ConfigValue::String("bar".to_string())
      )))
    );
  }

  #[test]
  fn path_as_key() {
    let input = br#"
        a.b.c=1
        "#;
    let result = hocon().parse(input);
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(
      ast[0],
      ConfigValue::Object(ConfigObjectValue::from((
        "a.b.c".to_string(),
        ConfigValue::Number(ConfigNumberValue::from("1".to_owned()))
      )))
    );
  }

  #[test]
  fn test_array() {
    let result = hocon().parse(
      br#"
    foo: [ 1s, 2.1, 3, 4, 5 ]
    "#,
    );
    assert!(result.is_success());
    let ast = result.to_result().ok().unwrap();
    assert_eq!(
      ast.first().unwrap().clone(),
      ConfigValue::Object(ConfigObjectValue::from((
        "foo".to_string(),
        ConfigValue::Array(ConfigArrayValue::new(vec![
          ConfigValue::Duration(ConfigDurationValue::new(
            ConfigNumberValue::from("1".to_owned()),
            TimeUnit::Seconds
          )),
          ConfigValue::Number(ConfigNumberValue::from("2.1".to_string())),
          ConfigValue::Number(ConfigNumberValue::from(3)),
          ConfigValue::Number(ConfigNumberValue::from(4)),
          ConfigValue::Number(ConfigNumberValue::from(5))
        ]))
      )))
    );
  }

  #[test]
  fn test_object() {
    let result = hocon().parse(
      br#"
    foo {
      bar = "baz",
      test {
        a = "b"
      }
    }
    "#,
    );
    println!("{:?}", result.clone().to_result().ok().unwrap());
    assert!(result.is_success());
  }

  #[test]
  fn test_json() {
    let input = br#"
	{
        "Image": {
            "Width":  800,
            "Height": 600,
            "Title":  "View from 15th Floor",
            "Thumbnail": {
                "Url":    "http://www.example.com/image/481989943",
                "Height": 125,
                "Width":  100
            },
            "Animated" : false,
            "IDs": [116, 943, 234, 38793]
        },
        "escaped characters": "\u2192\uD83D\uDE00\"\t\uD834\uDD1E"
    }"#;
    let result = hocon().parse(input);
    println!("{:?}", result.clone().to_result().ok().unwrap());
    assert!(result.is_success());
  }
}
