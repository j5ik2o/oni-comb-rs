use std::rc::Rc;

use regex::Regex;
use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;

use oni_comb_rs::core::{Parser, ParserFunctor, ParserMonad, ParserRunner};
use oni_comb_rs::extension::parser::{ConversionParser, DiscardParser, RepeatParser};
use oni_comb_rs::prelude::*;

#[derive(Debug, Clone)]
enum Expr {
  Value(Decimal),
  Add(Rc<Expr>, Rc<Expr>),
  Sub(Rc<Expr>, Rc<Expr>),
  Plus(Rc<Expr>),
  Minus(Rc<Expr>),
  Multiply(Rc<Expr>, Rc<Expr>),
  Divide(Rc<Expr>, Rc<Expr>),
  Parenthesized(Rc<Expr>),
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  add_sub_expr()
}

fn add_sub_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  mul_div_expr().flat_map(add_sub_rest)
}

fn add_sub_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
  let v1 = a.clone();
  let v2 = a.clone();
  let v3 = a.clone();
  let add_parser =
    space() * elm('+') * space() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Add(v1.clone(), b.clone()))));
  let sub_parser =
    space() * elm('-') * space() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Sub(v2.clone(), b.clone()))));
  add_parser | sub_parser | empty().map(move |_| v3.clone())
}

fn mul_div_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  unary().flat_map(mul_div_rest)
}

fn mul_div_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
  let v1 = a.clone();
  let v2 = a.clone();
  let v3 = a.clone();
  let mul_parser = space()
    * elm('*')
    * space()
    * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Multiply(v1.clone(), b.clone()))));
  let div_parser = space()
    * elm('/')
    * space()
    * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Divide(v2.clone(), b.clone()))));
  mul_parser | div_parser | empty().map(move |_| v3.clone())
}

fn unary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let unary_parser = ((elm('+') | elm('-')) + lazy(unary)).map(|(c, expr): (&char, Rc<Expr>)| match c {
    '-' => Rc::new(Expr::Plus(Rc::clone(&expr))),
    '+' => Rc::new(Expr::Minus(Rc::clone(&expr))),
    _ => panic!(),
  });
  unary_parser | primary()
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  surround(space() + elm('(') + space(), lazy(expr), space() + elm(')') + space())
    .map(|v| Rc::new(Expr::Parenthesized(v)))
    | value()
}

fn value<'a>() -> Parser<'a, char, Rc<Expr>> {
  regex(Regex::new(r#"([0-9])+([.]([0-9])+)?"#).unwrap())
    .convert(|s| Decimal::from_str(&s))
    .map(Expr::Value)
    .map(Rc::new)
}

fn eval(expr: Rc<Expr>) -> Decimal {
  match &*expr {
    Expr::Value(n) => *n,
    Expr::Add(l, r) => eval(l.clone()) + eval(r.clone()),
    Expr::Sub(l, r) => eval(l.clone()) - eval(r.clone()),
    Expr::Multiply(l, r) => eval(l.clone()) * eval(r.clone()),
    Expr::Divide(l, r) => eval(l.clone()) / eval(r.clone()),
    Expr::Minus(v) => eval(v.clone()) * Decimal::from(-1),
    Expr::Plus(v) => eval(v.clone()),
    Expr::Parenthesized(v) => eval(v.clone()),
  }
}

fn calculator<'a>() -> Parser<'a, char, Rc<Expr>> {
  expr() - end()
}

fn main() {
  // use std::env;
  // env::set_var("RUST_LOG", "debug");
  // let _ = env_logger::builder().is_test(true).try_init();
  let s = "(((0.1 + -1.2) * -3.3) / 4.3) + 5.9";
  let input = s.chars().into_iter().collect::<Vec<_>>();
  let result = calculator().parse(&input).unwrap();
  println!("expr = {:?}", result);
  let n = eval(result.clone());
  println!("{} = {}", s, n);
}
