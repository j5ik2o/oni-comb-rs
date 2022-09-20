use oni_comb_parser_rs::prelude::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
  Null,
  Bool(bool),
  String(String),
  Number(ConfigNumberValue),
  Array(Vec<ConfigValue>),
  Object(HashMap<String, ConfigValue>),
  Reference(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigNumberValue {
  SignedLong(i64),
  UnsignedLong(u64),
  Float(f64)
}

#[derive(Debug, Clone)]
pub enum Config {
  Object(HashMap<String, ConfigValue>),
  Array(Vec<ConfigValue>),
  KeyValue(String, ConfigValue),
}

fn space_or_comment<'a>() -> Parser<'a, u8, ()> {
  let sp_tab_cr_lf = elm_of(b" \t\r\n").of_many0();

  let head = elm_ref(b'#').collect() | seq(b"//");
  let tail = take_till0(|c| matches!(*c, b'\r' | b'\n'));
  let comment = (sp_tab_cr_lf.clone().opt() * head + tail).collect();

  (comment.opt() + sp_tab_cr_lf).discard()
}

fn number<'a>() -> Parser<'a, u8, (Option<&'a u8>, String, Option<String>, Option<String>)> {
  let integer = (elm_digit_1_9_ref() + elm_digit_ref().of_many0()).collect() | elm_ref(b'0').collect();
  let frac = elm_ref(b'.') + elm_digit_ref().of_many1();
  let exp = elm_of(b"eE") + elm_of(b"+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref(b'-').opt()
      + integer.map_res(std::str::from_utf8).map(String::from)
      + frac.collect().map_res(std::str::from_utf8).map(String::from).opt()
      + exp.collect().map_res(std::str::from_utf8).map(String::from).opt();
  number.map(|(((s, i), f), e)| {
    (s, i, f, e)
  })
}

fn number_value<'a>() -> Parser<'a, u8, ConfigNumberValue> {
  number().map(|(s, i, f, e)| {
    match (s, i, f, e) {
      (None, i, None, None) =>  {
        let n = u64::from_str(&i).unwrap();
        ConfigNumberValue::UnsignedLong(n)
      }
      (Some(_), i, None, None) =>  {
        let n = i64::from_str(&i).unwrap();
        ConfigNumberValue::SignedLong(n)
      }
      (_,  i, Some(f), _) =>  {
        let mut s = i;
        s.push_str(&f);
        println!("s = {}", s);
        let n = f64::from_str(&s).unwrap();
        ConfigNumberValue::Float(n)
      }
      _ => panic!("no match !!!")
    }
  })
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
  let char_string = (none_ref_of(b"\\\"") | escape_sequence)
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
  let string = surround(elm_ref(b'"'), (char_string | utf16_string).of_many0(), elm_ref(b'"'));
  string.map(|strings| strings.concat())
}

fn array<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  let elems = lazy(value).of_many0_sep(space_or_comment() + elm_ref(b',') + space_or_comment());
  surround(
    elm_ref(b'[') + space_or_comment(),
    elems,
    space_or_comment() + elm_ref(b']'),
  )
}

fn kv<'a>() -> Parser<'a, u8, ()> {
  elm_ref_of(b"=:").discard()
}

fn key<'a>() -> Parser<'a, u8, String> {
  path().map_res(std::str::from_utf8).map(String::from) | string()
}

fn property<'a>() -> Parser<'a, u8, (String, ConfigValue)> {
  let k = key().surround(space_or_comment(), space_or_comment());
  let v = lazy(value).surround(space_or_comment(), space_or_comment());
  k + ((kv() * v) | object_value() | array().map(ConfigValue::Array))
}

fn left_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'{').surround(space_or_comment(), space_or_comment())
}

fn right_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'}').surround(space_or_comment(), space_or_comment())
}

fn comma<'a>()-> Parser<'a, u8, &'a u8> {
  elm_ref(b',').surround(space_or_comment(), space_or_comment())
}

fn object<'a>() -> Parser<'a, u8, HashMap<String, ConfigValue>> {
  let members = lazy(property).of_many0_sep(comma());
  let obj = members.surround(left_bracket(), right_bracket());
  obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
}

fn path_element<'a>() -> Parser<'a, u8, &'a [u8]> {
  (elm_alpha() | elm_of(b"-_")).of_many1().collect()
}

fn path<'a>() -> Parser<'a, u8, &'a [u8]> {
  path_element().of_many1_sep(elm(b'.')).collect()
}

fn reference<'a>() -> Parser<'a, u8, String> {
  surround(seq(b"${"), path_element().of_many1_sep(elm(b'.')).collect(), elm(b'}'))
    .map_res(std::str::from_utf8)
    .map(String::from)
}

fn simple_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (seq(b"null").map(|_| ConfigValue::Null)
    | seq(b"true").map(|_| ConfigValue::Bool(true))
    | seq(b"false").map(|_| ConfigValue::Bool(false))
    | number_value().map(ConfigValue::Number)
    | string().map(ConfigValue::String))
    - space_or_comment()
}

fn object_value<'a>() -> Parser<'a, u8, ConfigValue> {
  object().map(ConfigValue::Object)
}

fn array_value<'a>() -> Parser<'a, u8, ConfigValue> {
  array().map(ConfigValue::Array)
}

fn reference_value<'a>() -> Parser<'a, u8, ConfigValue> {
  reference().map(ConfigValue::Reference)
}

fn value<'a>() -> Parser<'a, u8, ConfigValue> {
  (simple_value()
    | object_value()
    | array_value()
    | reference_value())
    - space_or_comment()
}

fn config<'a>() -> Parser<'a, u8, Config> {
  property().map(|(k, v)| Config::KeyValue(k, v)) | object().map(Config::Object) | array().map(Config::Array)
}

pub fn hocon<'a>() -> Parser<'a, u8, Vec<Config>> {
  space_or_comment() * config().of_many0() - end()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_array() {
    let result = hocon().parse(br#"
    foo: [ 1, 2.1, 3, 4, 5 ]
    "#);
    println!("{:?}", result.clone().to_result().ok().unwrap());
    assert!(result.is_success());
  }

  #[test]
  fn test_object() {
    let result = hocon().parse(br#"
    foo {
      bar : "baz",
      test : {
        a: "b"
      }
    }
    "#);
    println!("{:?}", result.clone().to_result().ok().unwrap());
    assert!(result.is_success());
  }
}
