use oni_comb_rs::core::{Parser, ParserFunctor};
use oni_comb_rs::extension::parser::OperatorParser;
use oni_comb_rs::prelude::*;
use crate::Expr::{AnyValueExpr, LastValueExpr, ListExpr, NoOp, PerExpr, ValueExpr};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NoOp,
    ValueExpr(u8),
    LastValueExpr,
    AnyValueExpr,
    PerExpr {
        digit: Box<Expr>,
        option: Box<Expr>,
    },
    RangeExpr {
        from: Box<Expr>,
        to: Box<Expr>,
        per_option: Box<Expr>,
    },
    ListExpr(Vec<Expr>),
    CronExpr {
        mins: Box<Expr>,
        hours: Box<Expr>,
        days: Box<Expr>,
        months: Box<Expr>,
        day_of_weeks: Box<Expr>,
    },
}
fn min_digit<'a>() -> Parser<'a, u8, Expr> {
    (elm_of(b"12345") + elm_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | (elm(b'0') * elm_of(b"0123456789")).map(|e| ValueExpr(e - 48))
        | (elm_of(b"0123456789")).map(|e| ValueExpr(e - 48))
}

fn hour_digit<'a>() -> Parser<'a, u8, Expr> {
    (elm(b'2') + elm_of(b"0123")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | (elm(b'1') + elm_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | elm(b'0') * elm_of(b"0123456789").map(|e| ValueExpr(e - 48))
        | elm_of(b"0123456789").map(|e| ValueExpr(e - 48))
}

fn day_digit<'a>() -> Parser<'a, u8, Expr> {
    (elm(b'3') + elm_of(b"01")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | (elm_of(b"12") + elm_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | (elm(b'0') * elm_of(b"123456789")).map(|e| ValueExpr(e - 48))
        | elm_of(b"123456789").map(|e| ValueExpr(e - 48))
}

fn month_digit<'a>() -> Parser<'a, u8, Expr> {
    (elm(b'1') + elm_of(b"012")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
        | (elm(b'0') * elm_of(b"123456789")).map(|e| ValueExpr(e - 48))
        | elm_of(b"123456789").map(|e| ValueExpr(e - 48))
}

fn day_of_week_digit<'a>() -> Parser<'a, u8, Expr> {
    seq(b"SUN").map(|_| ValueExpr(1))
        | seq(b"MON").map(|_| ValueExpr(2))
        | seq(b"TUE").map(|_| ValueExpr(3))
        | seq(b"WED").map(|_| ValueExpr(4))
        | seq(b"THU").map(|_| ValueExpr(5))
        | seq(b"FRI").map(|_| ValueExpr(6))
        | seq(b"SAT").map(|_| ValueExpr(7))
        | elm(b'L').map(|_| LastValueExpr)
}

fn day_of_week_text<'a>() -> Parser<'a, u8, Expr> {
    one_of(b"1234567").map(|e| ValueExpr(e as u8 - 48))
}

fn asterisk<'a>() -> Parser<'a, u8, Expr> {
    elm(b'*').map(|_| AnyValueExpr)
}

fn per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
    elm(b'/') * p
}

fn asterisk_per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
    (asterisk() + per(p)).map(|(d, op)| PerExpr {
        digit: Box::from(d.clone()),
        option: Box::from(op.clone()),
    })
}

fn range_per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
    per(p).opt().map(|e| match e {
        None => NoOp,
        Some(s) => s,
    })
}

fn list(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
    pom::parser::list(p, elm(b',')).map(|e| match e {
        e if e.len() == 1 => e.get(0).unwrap().clone(),
        e => ListExpr(e),
    })
}


fn main() {}
