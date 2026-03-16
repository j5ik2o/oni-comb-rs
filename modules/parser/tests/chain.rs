use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

fn integer_parser(
) -> impl Parser<oni_comb_parser::str_input::StrInput<'static>, Output = i64, Error = ParseError> {
    take_while1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<i64>().unwrap())
}

// ── chainl1 ───────────────────────────────────

#[test]
fn chainl1_single_operand() {
    let mut parser = integer_parser()
        .chainl1(char('+').map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("42");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 42);
}

#[test]
fn chainl1_two_operands() {
    let mut parser = integer_parser()
        .chainl1(char('+').map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("1+2");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 3);
}

#[test]
fn chainl1_left_associative() {
    // 10 - 3 - 2 = (10 - 3) - 2 = 5  (左結合)
    let mut parser = integer_parser()
        .chainl1(char('-').map(|_| (|a: i64, b: i64| a - b) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("10-3-2");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 5);
}

#[test]
fn chainl1_multiple_operators() {
    let add = char('+').map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64);
    let sub = char('-').map(|_| (|a: i64, b: i64| a - b) as fn(i64, i64) -> i64);
    let mut parser = integer_parser().chainl1(add.or(sub));
    let mut input = StrInput::new("10+3-2");

    // (10 + 3) - 2 = 11
    assert_eq!(parser.parse_next(&mut input).unwrap(), 11);
}

#[test]
fn chainl1_fails_on_no_operand() {
    let mut parser = integer_parser()
        .chainl1(char('+').map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("abc");

    assert!(matches!(
        parser.parse_next(&mut input),
        Err(Fail::Backtrack(_))
    ));
}

#[test]
fn chainl1_stops_at_non_operator() {
    let mut parser = integer_parser()
        .chainl1(char('+').map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("1+2*3");

    // '*' は operator ではないので 1+2 で停止
    assert_eq!(parser.parse_next(&mut input).unwrap(), 3);
    assert_eq!(input.offset(), 3);
}

// ── chainr1 ───────────────────────────────────

#[test]
fn chainr1_single_operand() {
    let mut parser = integer_parser()
        .chainr1(char('^').map(|_| (|a: i64, b: i64| a.pow(b as u32)) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("2");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 2);
}

#[test]
fn chainr1_right_associative() {
    // 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 2 ^ 9 = 512  (右結合)
    let mut parser = integer_parser()
        .chainr1(char('^').map(|_| (|a: i64, b: i64| a.pow(b as u32)) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("2^3^2");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 512);
}

#[test]
fn chainr1_two_operands() {
    let mut parser = integer_parser()
        .chainr1(char('^').map(|_| (|a: i64, b: i64| a.pow(b as u32)) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("2^10");

    assert_eq!(parser.parse_next(&mut input).unwrap(), 1024);
}

#[test]
fn chainr1_fails_on_no_operand() {
    let mut parser = integer_parser()
        .chainr1(char('^').map(|_| (|a: i64, b: i64| a.pow(b as u32)) as fn(i64, i64) -> i64));
    let mut input = StrInput::new("abc");

    assert!(matches!(
        parser.parse_next(&mut input),
        Err(Fail::Backtrack(_))
    ));
}
