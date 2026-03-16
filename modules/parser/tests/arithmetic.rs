//! 四則演算+括弧の統合テスト (MS5 完了条件の実証)
//!
//! expr = term (('+' | '-') term)*
//! term = atom (('*' | '/') atom)*
//! atom = integer | '(' expr ')'

use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

fn calc_parser(
) -> impl Parser<oni_comb_parser::str_input::StrInput<'static>, Output = i64, Error = ParseError> {
    recursive(|expr| {
        let ws_int = whitespace0().zip_right(integer()).zip_left(whitespace0());
        let atom = ws_int.or(whitespace0()
            .zip_right(char('('))
            .zip_right(expr)
            .zip_left(char(')'))
            .zip_left(whitespace0()));

        let mul_op = whitespace0()
            .zip_right(
                char('*')
                    .map(|_| (|a: i64, b: i64| a * b) as fn(i64, i64) -> i64)
                    .or(char('/').map(|_| (|a, b| a / b) as fn(i64, i64) -> i64)),
            )
            .zip_left(whitespace0());

        let add_op = whitespace0()
            .zip_right(
                char('+')
                    .map(|_| (|a: i64, b: i64| a + b) as fn(i64, i64) -> i64)
                    .or(char('-').map(|_| (|a, b| a - b) as fn(i64, i64) -> i64)),
            )
            .zip_left(whitespace0());

        let term = atom.chainl1(mul_op);
        term.chainl1(add_op)
    })
}

// ── 単一値 ────────────────────────────────────

#[test]
fn single_integer() {
    let mut input = StrInput::new("42");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 42);
}

#[test]
fn negative_integer() {
    let mut input = StrInput::new("-7");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), -7);
}

// ── 加減算 ────────────────────────────────────

#[test]
fn addition() {
    let mut input = StrInput::new("1 + 2");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 3);
}

#[test]
fn subtraction() {
    let mut input = StrInput::new("10 - 3");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 7);
}

#[test]
fn chained_addition() {
    let mut input = StrInput::new("1 + 2 + 3");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 6);
}

#[test]
fn left_associative_subtraction() {
    // (10 - 3) - 2 = 5
    let mut input = StrInput::new("10 - 3 - 2");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 5);
}

// ── 乗除算 ────────────────────────────────────

#[test]
fn multiplication() {
    let mut input = StrInput::new("3 * 4");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 12);
}

#[test]
fn division() {
    let mut input = StrInput::new("10 / 2");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 5);
}

// ── 優先順位 ──────────────────────────────────

#[test]
fn multiplication_before_addition() {
    // 1 + 2 * 3 = 1 + 6 = 7
    let mut input = StrInput::new("1 + 2 * 3");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 7);
}

#[test]
fn complex_precedence() {
    // 2 + 3 * 4 - 1 = 2 + 12 - 1 = 13
    let mut input = StrInput::new("2 + 3 * 4 - 1");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 13);
}

// ── 括弧 ─────────────────────────────────────

#[test]
fn parentheses_override_precedence() {
    // (1 + 2) * 3 = 9
    let mut input = StrInput::new("(1 + 2) * 3");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 9);
}

#[test]
fn nested_parentheses() {
    // ((2 + 3)) * 4 = 20
    let mut input = StrInput::new("((2 + 3)) * 4");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 20);
}

#[test]
fn complex_expression() {
    // 1 + 2 * (3 - 4) = 1 + 2 * (-1) = 1 + (-2) = -1
    let mut input = StrInput::new("1 + 2 * (3 - 4)");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), -1);
}

#[test]
fn deeply_nested() {
    // (((1 + 2) * 3) - 4) / 5 = ((3 * 3) - 4) / 5 = (9 - 4) / 5 = 5 / 5 = 1
    let mut input = StrInput::new("(((1 + 2) * 3) - 4) / 5");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 1);
}

// ── 空白処理 ──────────────────────────────────

#[test]
fn no_spaces() {
    let mut input = StrInput::new("1+2*3");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 7);
}

#[test]
fn extra_spaces() {
    let mut input = StrInput::new("  1  +  2  *  3  ");
    assert_eq!(calc_parser().parse_next(&mut input).unwrap(), 7);
}
