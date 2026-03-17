//! chumsky ベンチマーク互換の JSON パースベンチ。
//! 107KB の sample.json を使い、他ライブラリとのランキング比較を行う。

use std::borrow::Cow;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── oni-comb JSON パーサー ────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Json<'a> {
  Null,
  Bool(bool),
  Num(f64),
  Str(Cow<'a, str>),
  Array(Vec<Json<'a>>),
  Object(Vec<(Cow<'a, str>, Json<'a>)>),
}

/// fn 再帰 + 先頭バイト分岐で JSON 値をパースする。
/// `recursive()` の `Box<dyn Parser>` + vtable を回避し、
/// `or()` チェーンの線形スキャンも排除する。
fn json_value<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  whitespace0().parse_next(input)?;

  match input.peek_byte() {
    Some(b'n') => tag("null").map(|_| Json::Null).parse_next(input),
    Some(b't') => tag("true").map(|_| Json::Bool(true)).parse_next(input),
    Some(b'f') => tag("false").map(|_| Json::Bool(false)).parse_next(input),
    Some(b'"') => quoted_string_cow().map(Json::Str).parse_next(input),
    Some(b'[') => json_array(input),
    Some(b'{') => json_object(input),
    Some(c) if c == b'-' || c.is_ascii_digit() => {
      take_while1(|c: char| c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+')
        .map(|s: &str| Json::Num(s.parse::<f64>().unwrap()))
        .parse_next(input)
    }
    _ => Err(Fail::Backtrack(ParseError::expected_description(
      input.offset(),
      "JSON value",
    ))),
  }
}

fn json_array<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  char('[').parse_next(input)?;
  let mut items = Vec::new();
  whitespace0().parse_next(input)?;
  if input.peek_byte() == Some(b']') {
    char(']').parse_next(input)?;
    return Ok(Json::Array(items));
  }
  items.push(json_value(input)?);
  loop {
    whitespace0().parse_next(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        items.push(json_value(input)?);
      }
      _ => break,
    }
  }
  whitespace0().parse_next(input)?;
  char(']').parse_next(input)?;
  Ok(Json::Array(items))
}

fn json_object<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  char('{').parse_next(input)?;
  let mut pairs = Vec::new();
  whitespace0().parse_next(input)?;
  if input.peek_byte() == Some(b'}') {
    char('}').parse_next(input)?;
    return Ok(Json::Object(pairs));
  }
  pairs.push(json_member(input)?);
  loop {
    whitespace0().parse_next(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        pairs.push(json_member(input)?);
      }
      _ => break,
    }
  }
  whitespace0().parse_next(input)?;
  char('}').parse_next(input)?;
  Ok(Json::Object(pairs))
}

fn json_member<'a>(input: &mut StrInput<'a>) -> PResult<(Cow<'a, str>, Json<'a>), ParseError> {
  whitespace0().parse_next(input)?;
  let key = quoted_string_cow().parse_next(input)?;
  whitespace0().parse_next(input)?;
  char(':').parse_next(input)?;
  let val = json_value(input)?;
  Ok((key, val))
}

fn json_parser<'a>() -> impl Parser<StrInput<'a>, Output = Json<'a>, Error = ParseError> {
  fn_parser(json_value)
}

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

// pom は 107KB JSON のフルパースに対応する JSON パーサーの実装が困難なため除外。
// token レベルのベンチ（comparison ベンチ）で pom の数値は計測済み。

#[cfg(never)]
mod pom_json {
  use pom::parser::*;

  #[derive(Debug)]
  pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
  }

  fn space() -> pom::parser::Parser<char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
  }

  fn number() -> pom::parser::Parser<char, f64> {
    let integer = one_of("123456789") - one_of("0123456789").repeat(0..) | sym('0');
    let frac = sym('.') + one_of("0123456789").repeat(1..);
    let exp = one_of("eE") + one_of("+-").opt() + one_of("0123456789").repeat(1..);
    let number = sym('-').opt() + integer + frac.opt() + exp.opt();
    number
      .collect()
      .map(|s: Vec<&char>| s.into_iter().collect::<String>().parse::<f64>().unwrap())
  }

  fn string() -> pom::parser::Parser<char, String> {
    let escape = sym('\\')
      * (sym('"') | sym('\\') | sym('/') | sym('n').map(|_| '\n') | sym('r').map(|_| '\r') | sym('t').map(|_| '\t'));
    let char_string = none_of("\"\\") | escape;
    sym('"') * char_string.repeat(0..).map(|cs| cs.into_iter().collect()) - sym('"')
  }

  fn array() -> pom::parser::Parser<char, Vec<Json>> {
    sym('[') * space() * call(value).sep_by(sym(',') * space()) - space() - sym(']')
  }

  fn object() -> pom::parser::Parser<char, Vec<(String, Json)>> {
    let member = string() - space() - sym(':') - space() + call(value);
    sym('{') * space() * member.sep_by(sym(',') * space()) - space() - sym('}')
  }

  fn value() -> pom::parser::Parser<char, Json> {
    (tag("null").map(|_| Json::Null)
      | tag("true").map(|_| Json::Bool(true))
      | tag("false").map(|_| Json::Bool(false))
      | number().map(Json::Num)
      | string().map(Json::Str)
      | array().map(Json::Array)
      | object().map(Json::Object))
      - space()
  }

  pub fn parse(input: &str) -> Result<Json, pom::Error> {
    let chars: Vec<char> = input.chars().collect();
    (space() * value()).parse(&chars)
  }
}

// ── ベンチマーク ─────────────────────────────

static JSON_STR: &str = include_str!("data/sample.json");
static JSON_BYTES: &[u8] = include_bytes!("data/sample.json");

fn bench_json_full(c: &mut Criterion) {
  // 正しくパースできることを確認
  {
    let mut input = StrInput::new(JSON_STR);
    assert!(json_parser().parse_next(&mut input).is_ok());
  }
  assert!(winnow_json::parse(JSON_BYTES).is_ok());
  assert!(nom_json::parse(JSON_BYTES).is_ok());
  assert!(chumsky_json::parse(JSON_STR).is_some());
  let mut group = c.benchmark_group("json_full");
  group.throughput(Throughput::Bytes(JSON_BYTES.len() as u64));

  group.bench_function("oni-comb", |b| {
    b.iter(|| {
      let mut input = StrInput::new(black_box(JSON_STR));
      black_box(json_parser().parse_next(&mut input).unwrap())
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

  group.finish();
}

criterion_group!(benches, bench_json_full);
criterion_main!(benches);
