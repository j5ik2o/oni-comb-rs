#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::borrow::Cow;

use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

/// コンビネータ合成のみを測定するため、String 構築を行わない identifier パーサー。
/// satisfy + take_while0 の .zip() で (char, &str) を返す。
fn parse_identifier_no_alloc(s: &str) -> Option<(char, &str)> {
    let head = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut parser = head.zip(tail);
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// take_while1 は &str を返すのでアロケーション不要。
fn parse_integer_no_alloc(s: &str) -> Option<&str> {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

/// flat_map 同一型: satisfy + tag のチェーン。Box 不要なのでアロケーション 0 のはず。
fn parse_flat_map_same_type_no_alloc(s: &str) -> Option<&str> {
    let mut parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|c| match c {
        '1' => tag("one"),
        '2' => tag("two"),
        '3' => tag("three"),
        _ => tag(""),
    });
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

// ── JSON パーサー（json_full.rs と同一） ───────

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

fn json_value<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
    whitespace0().parse_next(input)?;
    match input.peek_byte() {
        Some(b'n') => tag("null").map(|_| Json::Null).parse_next(input),
        Some(b't') => tag("true").map(|_| Json::Bool(true)).parse_next(input),
        Some(b'f') => tag("false").map(|_| Json::Bool(false)).parse_next(input),
        Some(b'"') => quoted_string_cow().map(Json::Str).parse_next(input),
        Some(b'[') => json_array(input),
        Some(b'{') => json_object(input),
        Some(c) if c == b'-' || c.is_ascii_digit() => take_while1(|c: char| {
            c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+'
        })
        .map(|s: &str| Json::Num(s.parse::<f64>().unwrap()))
        .parse_next(input),
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

static JSON_STR: &str = include_str!("data/sample.json");

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    // ── Token ワークロード（ゼロアロケーション確認） ──
    let id_inputs = ["x", "foo", "foo_bar_123", "_private"];
    let int_inputs = ["0", "42", "9999999"];
    let flat_map_inputs = ["1one", "2two", "3three"];

    for input in id_inputs {
        let _ = parse_identifier_no_alloc(input);
    }
    for input in int_inputs {
        let _ = parse_integer_no_alloc(input);
    }
    for input in flat_map_inputs {
        let _ = parse_flat_map_same_type_no_alloc(input);
    }

    // ── JSON フルパース（Vec のみアロケーション） ──
    let mut input = StrInput::new(JSON_STR);
    let _ = json_value(&mut input).unwrap();
}
