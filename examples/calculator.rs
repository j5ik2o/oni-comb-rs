use std::env;
use std::rc::Rc;

use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;

use oni_comb_rs::core::{Parser, ParserFunctor, ParserRunner};
use oni_comb_rs::extension::parser::{ConversionParser, DiscardParser, RepeatParser};
use oni_comb_rs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
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

impl Expr {
  pub fn of_add(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Add(lhs, rhs))
  }

  pub fn of_subtract(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Sub(lhs, rhs))
  }

  pub fn of_multiply(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Multiply(lhs, rhs))
  }

  pub fn of_divide(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Divide(lhs, rhs))
  }
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  additive()
}

fn add<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('+') - space()
}

fn sub<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('-') - space()
}

fn mul<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('*') - space()
}

fn div<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('/') - space()
}

// fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
//   multitive().flat_map(additive_rest)
// }
//
// fn rest0<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
//   additive_rest(a).flat_map(|e| {
//     (add() | sub() | mul() | div())
//       .exists()
//       .flat_map(move |b| if b { rest0(e.clone()) } else { successful(e.clone()) })
//   })
// }
//
// fn additive_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
//   let v1 = a.clone();
//   let v2 = a.clone();
//   let v3 = a.clone();
//   let add_parser = add() * unary().flat_map(move |b| additive_rest(Rc::new(Expr::Add(v1.clone(), b.clone()))));
//   let sub_parser = sub() * unary().flat_map(move |b| additive_rest(Rc::new(Expr::Sub(v2.clone(), b.clone()))));
//   add_parser.attempt() | sub_parser.attempt() | empty().map(move |_| v3.clone())
// }
//
// fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
//   unary().flat_map(multitive_rest)
// }
//
// fn multitive_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
//   let v1 = a.clone();
//   let v2 = a.clone();
//   let v3 = a.clone();
//   let mul_parser = mul() * unary().flat_map(move |b| multitive_rest(Rc::new(Expr::Multiply(v1.clone(), b.clone()))));
//   let div_parser = div() * unary().flat_map(move |b| multitive_rest(Rc::new(Expr::Divide(v2.clone(), b.clone()))));
//   mul_parser.attempt() | div_parser.attempt() | empty().map(move |_| v3.clone())
// }

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let aster = elm_ref('*');
  let slash = elm_ref('/');

  let p = chain_left1(
    primary(),
    (space() * (aster | slash) - space()).map(|e| match e {
      '*' => Expr::of_multiply,
      '/' => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }),
  );
  p
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let plus = elm_ref('+');
  let minus = elm_ref('-');

  let p = chain_left1(
    multitive(),
    (space() * (plus | minus) - space()).map(|e| match e {
      '+' => Expr::of_add,
      '-' => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }),
  );
  p
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let unary_parser = ((elm_ref('+') | elm_ref('-')) + lazy(primary))
    .map(|(c, expr): (&char, Rc<Expr>)| match c {
      '-' => Expr::Minus(Rc::clone(&expr)),
      '+' => Expr::Plus(Rc::clone(&expr)),
      _ => panic!(),
    })
    .map(Rc::new);
  unary_parser | primary0()
}

fn primary0<'a>() -> Parser<'a, char, Rc<Expr>> {
  surround(
    space() + elm_ref('(') + space(),
    lazy(expr),
    space() + elm_ref(')') + space(),
  )
  .map(Expr::Parenthesized)
  .map(Rc::new)
    | value()
}

fn value<'a>() -> Parser<'a, char, Rc<Expr>> {
  regex(r"^\d+([.]\d+)?")
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
fn init() {
  env::set_var("RUST_LOG", "debug");
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_multitive() {
  init();
  let source = r"1/2";
  let input = source.chars().collect::<Vec<_>>();
  println!("start");

  let result = multitive().parse_as_result(&input).unwrap();
  println!("{:?}", result);
  assert_eq!(
    Expr::Divide(
      Rc::new(Expr::Value(Decimal::from(1))),
      Rc::new(Expr::Value(Decimal::from(2)))
    ),
    *result
  );
}

#[test]
fn test_additive() {
  init();
  let source = r"1+2*3+1";
  let input = source.chars().collect::<Vec<_>>();
  let result = additive().parse_as_result(&input).unwrap();
  println!("{:?}", result);
  assert_eq!(
    Expr::Add(
      Rc::new(Expr::Value(Decimal::from(1))),
      Rc::new(Expr::Value(Decimal::from(2)))
    ),
    *result
  );
}

fn main() {
  init();
  // use std::env;
  // env::set_var("RUST_LOG", "debug");
  // let _ = env_logger::builder().is_test(true).try_init();
  // let s = "(((0.1 + -1.2) * -3.3) / 4.3) + 5.9";
  let s = "1+2*3+1";
  let input = s.chars().collect::<Vec<_>>();
  let result = calculator().parse(&input).to_result().unwrap();
  println!("expr = {:?}", result);
  let n = eval(result.clone());
  println!("{} = {}", s, n);
}
