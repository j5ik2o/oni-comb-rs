use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput};

use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum JsonValue {
  Null,
  Bool(bool),
  Int(i64),
  Str(String),
  Array(Vec<JsonValue>),
  Object(Vec<(String, JsonValue)>),
}

fn trailing_ws<P>(p: P) -> impl Parser<StrInput<'static>, Output = P::Output, Error = ParseError>
where
  P: Parser<StrInput<'static>, Error = ParseError>, {
  p.zip_left(whitespace0())
}

fn json_primitive() -> impl Parser<StrInput<'static>, Output = JsonValue, Error = ParseError> {
  let null = tag("null").map(|_| JsonValue::Null);
  let bool_true = tag("true").map(|_| JsonValue::Bool(true));
  let bool_false = tag("false").map(|_| JsonValue::Bool(false));
  let int = integer().map(JsonValue::Int);
  let string = quoted_string().map(|s| JsonValue::Str(s.into_owned()));
  null.or(bool_true).or(bool_false).or(int).or(string)
}

fn json_array() -> impl Parser<StrInput<'static>, Output = JsonValue, Error = ParseError> {
  let comma = char(',').zip_left(whitespace0());

  trailing_ws(char('['))
    .zip_right(trailing_ws(json_primitive()).sep_by0(comma))
    .zip_left(char(']'))
    .map(JsonValue::Array)
}

fn json_object() -> impl Parser<StrInput<'static>, Output = JsonValue, Error = ParseError> {
  let colon = char(':').zip_left(whitespace0());
  let comma = char(',').zip_left(whitespace0());
  let pair = trailing_ws(quoted_string())
    .map(|s| s.into_owned())
    .zip_left(colon)
    .zip(trailing_ws(json_primitive()));
  trailing_ws(char('{'))
    .zip_right(pair.sep_by0(comma))
    .zip_left(char('}'))
    .map(JsonValue::Object)
}

fn json_value() -> impl Parser<StrInput<'static>, Output = JsonValue, Error = ParseError> {
  whitespace0()
    .zip_right(json_primitive().or(json_array()).or(json_object()))
    .zip_left(whitespace0())
}

const JSON_INPUTS: &[(&str, &str)] = &[
  ("null", "null"),
  ("integer", "42"),
  ("string", r#""hello world""#),
  ("array_3", r#"[1, 2, 3]"#),
  ("array_mixed", r#"[1, "two", true, null]"#),
  ("object", r#"{"name": "oni-comb", "version": 2, "active": true}"#),
  (
    "object_large",
    r#"{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8}"#,
  ),
];

fn parse_json(input: &'static str) -> Result<JsonValue, oni_comb_parser::fail::Fail<ParseError>> {
  let mut inp = StrInput::new(input);
  json_value().parse_next(&mut inp)
}

pub fn register(c: &mut Criterion) {
  assert_eq!(
    parse_json(r#"[1,"two",true,null]"#).unwrap(),
    parse_json(r#"[ 1 , "two" , true , null ]"#).unwrap()
  );
  assert_eq!(
    parse_json(r#"{"name":"oni-comb","version":2,"active":true}"#).unwrap(),
    parse_json(r#"{ "name" : "oni-comb" , "version" : 2 , "active" : true }"#).unwrap()
  );

  let mut group = c.benchmark_group("json");

  for (name, input) in JSON_INPUTS {
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_with_input(BenchmarkId::new("oni-comb", name), input, |b, input| {
      b.iter(|| {
        let mut inp = StrInput::new(black_box(input));
        json_value().parse_next(&mut inp)
      })
    });
  }

  group.finish();
}
