//! JSON フルパースベンチ。
//! 107KB の sample.json を使い、他ライブラリとのランキング比較を行う。

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[path = "shared/oni_comb_json.rs"]
mod oni_comb_json;

// ── winnow JSON パーサー ─────────────────────

mod winnow_json {
  use std::str;
  use winnow::ascii::take_escaped;
  use winnow::combinator::{alt, preceded, separated, separated_pair, terminated};
  use winnow::prelude::*;
  use winnow::token::{none_of, one_of, take_while};

  #[derive(Debug, Clone)]
  #[allow(dead_code)]
  pub enum Json<'a> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'a [u8]),
    Array(Vec<Json<'a>>),
    Object(Vec<(&'a [u8], Json<'a>)>),
  }

  fn space<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<&'a [u8]> {
    take_while(0.., [b' ', b'\t', b'\r', b'\n']).parse_next(i)
  }

  fn number(i: &mut &[u8]) -> winnow::ModalResult<f64> {
    use winnow::ascii::{digit0, digit1};
    (
      winnow::combinator::opt('-'),
      alt(((one_of(b'1'..=b'9'), digit0).void(), one_of('0').void())),
      winnow::combinator::opt(('.', digit1)),
      winnow::combinator::opt((
        one_of([b'e', b'E']),
        winnow::combinator::opt(one_of([b'+', b'-'])),
        digit1,
      )),
    )
      .take()
      .map(|bytes| str::from_utf8(bytes).unwrap().parse::<f64>().unwrap())
      .parse_next(i)
  }

  fn string<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<&'a [u8]> {
    preceded(
      '"',
      terminated(
        take_escaped(
          none_of([b'\\', b'"']),
          '\\',
          one_of([b'\\', b'/', b'"', b'b', b'f', b'n', b'r', b't']),
        ),
        '"',
      ),
    )
    .parse_next(i)
  }

  fn array<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<Vec<Json<'a>>> {
    preceded(
      '[',
      terminated(separated(0.., json_value, preceded(space, ',')), preceded(space, ']')),
    )
    .parse_next(i)
  }

  fn member<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<(&'a [u8], Json<'a>)> {
    separated_pair(preceded(space, string), preceded(space, ':'), json_value).parse_next(i)
  }

  fn object<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<Vec<(&'a [u8], Json<'a>)>> {
    preceded(
      '{',
      terminated(separated(0.., member, preceded(space, ',')), preceded(space, '}')),
    )
    .parse_next(i)
  }

  fn json_value<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<Json<'a>> {
    preceded(
      space,
      alt((
        b"null".value(Json::Null),
        b"true".value(Json::Bool(true)),
        b"false".value(Json::Bool(false)),
        number.map(Json::Num),
        string.map(Json::Str),
        array.map(Json::Array),
        object.map(Json::Object),
      )),
    )
    .parse_next(i)
  }

  pub fn parse(input: &[u8]) -> winnow::ModalResult<Json<'_>> {
    terminated(json_value, space).parse_next(&mut &*input)
  }
}

// ── nom JSON パーサー ────────────────────────

mod nom_json {
  use nom::branch::alt;
  use nom::bytes::complete::{escaped, tag, take_while};
  use nom::character::complete::{char, digit0, digit1, none_of, one_of};
  use nom::combinator::{cut, map, opt, recognize, value as to};
  use nom::multi::separated_list0;
  use nom::sequence::{preceded, separated_pair, terminated};
  use nom::{IResult, Parser};
  use std::str;

  #[derive(Debug, Clone)]
  #[allow(dead_code)]
  pub enum Json<'a> {
    Null,
    Bool(bool),
    Num(f64),
    Str(&'a [u8]),
    Array(Vec<Json<'a>>),
    Object(Vec<(&'a [u8], Json<'a>)>),
  }

  fn space(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| b" \t\r\n".contains(&c))(i)
  }

  fn number(i: &[u8]) -> IResult<&[u8], f64> {
    map(
      recognize((
        opt(char('-')),
        alt((to((), (one_of("123456789"), digit0)), to((), char('0')))),
        opt((char('.'), digit1)),
        opt((one_of("eE"), opt(one_of("+-")), cut(digit1))),
      )),
      |bytes: &[u8]| str::from_utf8(bytes).unwrap().parse::<f64>().unwrap(),
    )
    .parse(i)
  }

  fn string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(
      char('"'),
      cut(terminated(
        escaped(none_of("\\\""), '\\', one_of("\\/\"bfnrt")),
        char('"'),
      )),
    )
    .parse(i)
  }

  fn array(i: &[u8]) -> IResult<&[u8], Vec<Json<'_>>> {
    preceded(
      char('['),
      cut(terminated(
        separated_list0(preceded(space, char(',')), value),
        preceded(space, char(']')),
      )),
    )
    .parse(i)
  }

  fn member(i: &[u8]) -> IResult<&[u8], (&[u8], Json<'_>)> {
    separated_pair(preceded(space, string), cut(preceded(space, char(':'))), value).parse(i)
  }

  fn object(i: &[u8]) -> IResult<&[u8], Vec<(&[u8], Json<'_>)>> {
    preceded(
      char('{'),
      cut(terminated(
        separated_list0(preceded(space, char(',')), member),
        preceded(space, char('}')),
      )),
    )
    .parse(i)
  }

  fn value(i: &[u8]) -> IResult<&[u8], Json<'_>> {
    preceded(
      space,
      alt((
        to(Json::Null, tag("null")),
        to(Json::Bool(true), tag("true")),
        to(Json::Bool(false), tag("false")),
        map(number, Json::Num),
        map(string, Json::Str),
        map(array, Json::Array),
        map(object, Json::Object),
      )),
    )
    .parse(i)
  }

  pub fn parse(i: &[u8]) -> IResult<&[u8], Json<'_>> {
    terminated(value, space).parse(i)
  }
}

// ── chumsky JSON パーサー ──────────────────────

mod chumsky_json {
  use chumsky::prelude::*;

  #[derive(Debug, Clone)]
  pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
  }

  pub fn json_parser<'a>() -> impl Parser<'a, &'a str, Json> {
    recursive(|value| {
      let number = just('-')
        .or_not()
        .then(text::int(10))
        .then(just('.').then(text::digits(10)).or_not())
        .then(
          just('e')
            .or(just('E'))
            .then(just('+').or(just('-')).or_not())
            .then(text::digits(10))
            .or_not(),
        )
        .to_slice()
        .map(|s: &str| Json::Num(s.parse().unwrap()));

      let escape = just('\\').then(choice((
        just('\\'),
        just('/'),
        just('"'),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
      )));

      let string = just('"')
        .ignore_then(
          none_of("\\\"")
            .or(escape.map(|(_, c)| c))
            .repeated()
            .collect::<String>(),
        )
        .then_ignore(just('"'));

      let array = value
        .clone()
        .separated_by(just(',').padded())
        .collect()
        .padded()
        .delimited_by(just('['), just(']'))
        .map(Json::Array);

      let member = string.clone().then_ignore(just(':').padded()).then(value.clone());

      let object = member
        .separated_by(just(',').padded())
        .collect()
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(Json::Object);

      choice((
        just("null").to(Json::Null),
        just("true").to(Json::Bool(true)),
        just("false").to(Json::Bool(false)),
        number,
        string.map(Json::Str),
        array,
        object,
      ))
      .padded()
    })
  }

  pub fn parse(input: &str) -> Option<Json> {
    json_parser().parse(input).into_output()
  }
}

// ── pom JSON パーサー ─────────────────────────

mod pom_json {
  use pom::parser::*;
  use std::str::{self, FromStr};

  #[derive(Debug)]
  #[allow(dead_code)]
  pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
  }

  fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
  }

  fn number<'a>() -> Parser<'a, u8, f64> {
    let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
    let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
    number.collect().convert(str::from_utf8).convert(f64::from_str)
  }

  fn string<'a>() -> Parser<'a, u8, String> {
    let special_char = sym(b'\\')
      | sym(b'/')
      | sym(b'"')
      | sym(b'b').map(|_| b'\x08')
      | sym(b'f').map(|_| b'\x0C')
      | sym(b'n').map(|_| b'\n')
      | sym(b'r').map(|_| b'\r')
      | sym(b't').map(|_| b'\t');
    let escape_sequence = sym(b'\\') * special_char;
    let string = sym(b'"') * (none_of(b"\\\"") | escape_sequence).repeat(0..) - sym(b'"');
    string.convert(String::from_utf8)
  }

  fn array<'a>() -> Parser<'a, u8, Vec<Json>> {
    let elems = list(call(value), sym(b',') * space());
    sym(b'[') * space() * elems - space() - sym(b']')
  }

  fn object<'a>() -> Parser<'a, u8, Vec<(String, Json)>> {
    let member = string() - space() - sym(b':') - space() + call(value);
    let members = list(member, sym(b',') * space());
    sym(b'{') * space() * members - space() - sym(b'}')
  }

  fn value<'a>() -> Parser<'a, u8, Json> {
    (seq(b"null").map(|_| Json::Null)
      | seq(b"true").map(|_| Json::Bool(true))
      | seq(b"false").map(|_| Json::Bool(false))
      | number().map(Json::Num)
      | string().map(Json::Str)
      | array().map(Json::Array)
      | object().map(Json::Object))
      - space()
  }

  pub fn parse(input: &[u8]) -> Result<Json, pom::Error> {
    (space() * value() - end()).parse(input)
  }
}

// ── ベンチマーク ─────────────────────────────

static JSON_STR: &str = include_str!("data/sample.json");
static JSON_BYTES: &[u8] = include_bytes!("data/sample.json");

fn bench_json_full(c: &mut Criterion) {
  // 正しくパースできることを確認
  assert_eq!(
    oni_comb_json::parse_complete(r#"{"a":[1,2],"b":{"c":true}}"#).unwrap(),
    oni_comb_json::parse_complete(r#" { "a" : [ 1 , 2 ] , "b" : { "c" : true } } "#).unwrap()
  );
  assert!(oni_comb_json::parse_complete(JSON_STR).is_ok());
  assert!(winnow_json::parse(JSON_BYTES).is_ok());
  assert!(nom_json::parse(JSON_BYTES).is_ok());
  assert!(chumsky_json::parse(JSON_STR).is_some());
  assert!(pom_json::parse(JSON_BYTES).is_ok());
  let mut group = c.benchmark_group("json_full");
  group.throughput(Throughput::Bytes(JSON_BYTES.len() as u64));

  group.bench_function("oni-comb", |b| {
    b.iter(|| {
      let mut input = StrInput::new(black_box(JSON_STR));
      black_box(oni_comb_json::json_parser().parse_next(&mut input).unwrap())
    })
  });

  group.bench_function("winnow", |b| {
    b.iter(|| black_box(winnow_json::parse(black_box(JSON_BYTES)).unwrap()))
  });

  group.bench_function("nom", |b| {
    b.iter(|| black_box(nom_json::parse(black_box(JSON_BYTES)).unwrap()))
  });

  group.bench_function("chumsky", |b| {
    b.iter(|| black_box(chumsky_json::parse(black_box(JSON_STR)).unwrap()))
  });

  group.bench_function("pom", |b| {
    b.iter(|| black_box(pom_json::parse(black_box(JSON_BYTES)).unwrap()))
  });

  group.finish();
}

criterion_group!(benches, bench_json_full);
criterion_main!(benches);
