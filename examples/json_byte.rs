use parsing_rust::core::{ParserFunctor, ParserRunner};
use parsing_rust::extension::{BasicCombinator, ConversionCombinator, RepeatCombinator};
use parsing_rust::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
  Null,
  Bool(bool),
  Str(String),
  Num(f64),
  Array(Vec<JsonValue>),
  Object(HashMap<String, JsonValue>),
}

fn space<'a>() -> Parser<'a, u8, ()> {
  one_of_set(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, f64> {
  let integer =  one_of_from_to(b'1', b'9') -  one_of_from_to(b'0', b'9').repeat(0..) | elm(b'0');
  let frac = elm(b'.') +  one_of_from_to(b'0', b'9').repeat(1..);
  let exp = one_of_set(b"eE") + one_of_set(b"+-").opt() +  one_of_from_to(b'0', b'9').repeat(1..);
  let number = elm(b'-').opt() + integer + frac.opt() + exp.opt();
  let p1 = number.collect();
  let p2 = p1.convert(std::str::from_utf8);
  let p3 = p2.convert(f64::from_str);
  p3
}

fn string<'a>() -> Parser<'a, u8, String> {
  let special_char = elm(b'\\')
    | elm(b'/')
    | elm(b'"')
    | elm(b'b').map(|_| &b'\x08')
    | elm(b'f').map(|_| &b'\x0C')
    | elm(b'n').map(|_| &b'\n')
    | elm(b'r').map(|_| &b'\r')
    | elm(b't').map(|_| &b'\t');
  let escape_sequence = elm(b'\\') * special_char;
  let char_string = (none_of_set(b"\\\"") | escape_sequence)
    .map(|e| *e)
    .repeat(1..)
    .convert(String::from_utf8);
  let utf16_char = seq(b"\\u")
    * elm_hex_digit()
      .map(|e| *e)
      .repeat(4)
      .convert(String::from_utf8)
      .convert(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.repeat(1..).map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string = elm(b'"') * (char_string | utf16_string).repeat(0..) - elm(b'"');
  string.map(|strings| strings.concat())
}

fn array<'a>() -> Parser<'a, u8, Vec<JsonValue>> {
  let elems = lazy(value).many_0_sep(elm(b',') * space());
  elm(b'[') * space() * elems - elm(b']')
}

fn object<'a>() -> Parser<'a, u8, HashMap<String, JsonValue>> {
  let member = string() - space() - elm(b':') - space() + lazy(value);
  let members = member.many_0_sep(elm(b',') * space());
  let obj = elm(b'{') * space() * members - elm(b'}');
  obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
}

fn value<'a>() -> Parser<'a, u8, JsonValue> {
  (seq(b"null").map(|_| JsonValue::Null)
    | seq(b"true").map(|_| JsonValue::Bool(true))
    | seq(b"false").map(|_| JsonValue::Bool(false))
    | number().map(|num| JsonValue::Num(num))
    | string().map(|text| JsonValue::Str(text))
    | array().map(|arr| JsonValue::Array(arr))
    | object().map(|obj| JsonValue::Object(obj)))
    - space()
}

pub fn json<'a>() -> Parser<'a, u8, JsonValue> {
  space() * value() - end()
}

#[allow(dead_code)]
fn main() {
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

  println!("{:?}", json().parse(input));
}
