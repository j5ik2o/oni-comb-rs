//! chumsky ベンチマーク互換の JSON パースベンチ。
//! 107KB の sample.json を使い、他ライブラリとのランキング比較を行う。

use std::string::String;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── oni-comb JSON パーサー ────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

fn ws() -> oni_comb_parser::text::take_while::TakeWhile0<fn(char) -> bool> {
    whitespace0()
}

fn json_parser() -> impl Parser<StrInput<'static>, Output = Json, Error = ParseError> {
    recursive(|value| {
        let number = satisfy(|c: char| c == '-' || c.is_ascii_digit())
            .zip(take_while0(|c: char| {
                c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-'
            }))
            .map(|(first, rest): (char, &str)| {
                let mut s = String::with_capacity(1 + rest.len());
                s.push(first);
                s.push_str(rest);
                Json::Num(s.parse::<f64>().unwrap())
            });

        let array = ws()
            .zip_right(char('['))
            .zip_right(
                ws()
                    .zip_right(value.clone())
                    .zip_left(ws())
                    .sep_by0(ws().zip_right(char(',')).zip_left(ws())),
            )
            .zip_left(ws().zip_right(char(']')))
            .map(Json::Array);

        let pair = ws()
            .zip_right(quoted_string())
            .zip_left(ws())
            .zip_left(char(':'))
            .zip(ws().zip_right(value).zip_left(ws()));
        let object = ws()
            .zip_right(char('{'))
            .zip_right(pair.sep_by0(ws().zip_right(char(',')).zip_left(ws())))
            .zip_left(ws().zip_right(char('}')))
            .map(Json::Object);

        let null = tag("null").map(|_| Json::Null);
        let bool_true = tag("true").map(|_| Json::Bool(true));
        let bool_false = tag("false").map(|_| Json::Bool(false));
        let str_val = quoted_string().map(Json::Str);

        ws()
            .zip_right(
                null.or(bool_true)
                    .or(bool_false)
                    .or(number)
                    .or(str_val)
                    .or(array)
                    .or(object),
            )
            .zip_left(ws())
    })
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
            terminated(
                separated(0.., json_value, preceded(space, ',')),
                preceded(space, ']'),
            ),
        )
        .parse_next(i)
    }

    fn member<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<(&'a [u8], Json<'a>)> {
        separated_pair(preceded(space, string), preceded(space, ':'), json_value).parse_next(i)
    }

    fn object<'a>(i: &mut &'a [u8]) -> winnow::ModalResult<Vec<(&'a [u8], Json<'a>)>> {
        preceded(
            '{',
            terminated(
                separated(0.., member, preceded(space, ',')),
                preceded(space, '}'),
            ),
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
        separated_pair(
            preceded(space, string),
            cut(preceded(space, char(':'))),
            value,
        )
        .parse(i)
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

    group.finish();
}

criterion_group!(benches, bench_json_full);
criterion_main!(benches);
