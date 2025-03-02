use std::env;
use std::rc::Rc;

use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;

use oni_comb_parser_rs::prelude::*;

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

// 従来のParser実装
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

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let aster = elm_ref('*');
  let slash = elm_ref('/');
  primary().chain_left1((space() * (aster | slash) - space()).map(|e| match e {
    '*' => Expr::of_multiply,
    '/' => Expr::of_divide,
    _ => panic!("unexpected operator"),
  }))
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let plus = elm_ref('+');
  let minus = elm_ref('-');
  multitive().chain_left1((space() * (plus | minus) - space()).map(|e| match e {
    '+' => Expr::of_add,
    '-' => Expr::of_subtract,
    _ => panic!("unexpected operator"),
  }))
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
    .map_res(|s| Decimal::from_str(&s))
    .map(Expr::Value)
    .map(Rc::new)
}

// StaticParser実装
mod static_parsers {
  use super::Decimal;
  use super::Expr;
  use oni_comb_parser_rs::prelude::static_parsers::*;
  use oni_comb_parser_rs::prelude::{ConversionParser, DiscardParser, OperatorParser, RepeatParser};
  use oni_comb_parser_rs::StaticParser;
  use std::rc::Rc;
  use std::str::FromStr;

  pub fn space_static<'a>() -> StaticParser<'a, char, ()> {
    elm_of(" \t\r\n").of_many0().discard()
  }

  pub fn expr_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    additive_static()
  }

  pub fn add_static<'a>() -> StaticParser<'a, char, &'a char> {
    space_static() * elm_ref('+') - space_static()
  }

  pub fn sub_static<'a>() -> StaticParser<'a, char, &'a char> {
    space_static() * elm_ref('-') - space_static()
  }

  pub fn mul_static<'a>() -> StaticParser<'a, char, &'a char> {
    space_static() * elm_ref('*') - space_static()
  }

  pub fn div_static<'a>() -> StaticParser<'a, char, &'a char> {
    space_static() * elm_ref('/') - space_static()
  }

  pub fn multitive_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    let aster = elm_ref('*');
    let slash = elm_ref('/');
    primary_static().chain_left1((space_static() * (aster | slash) - space_static()).map(|e| match e {
      '*' => Expr::of_multiply,
      '/' => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }))
  }

  pub fn additive_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    let plus = elm_ref('+');
    let minus = elm_ref('-');
    multitive_static().chain_left1((space_static() * (plus | minus) - space_static()).map(|e| match e {
      '+' => Expr::of_add,
      '-' => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }))
  }

  pub fn primary_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    let unary_op = elm_ref('+') | elm_ref('-');
    let unary_parser = (unary_op + primary0_static())
      .map(|(c, expr): (&char, Rc<Expr>)| match c {
        '-' => Expr::Minus(Rc::clone(&expr)),
        '+' => Expr::Plus(Rc::clone(&expr)),
        _ => panic!(),
      })
      .map(Rc::new);
    unary_parser | primary0_static()
  }

  pub fn primary0_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    let paren_parser = elm_ref('(') * (space_static() * expr_static()) - (space_static() * elm_ref(')'));
    paren_parser.map(Expr::Parenthesized).map(Rc::new) | value_static()
  }

  pub fn value_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    let digit = elm_pred(|c: &char| c.is_digit(10));

    // Parse integer part
    let int_parser = digit
      .clone()
      .of_many1()
      .map(|digits: Vec<char>| digits.iter().collect::<String>());

    // Parse optional fraction part
    let frac_parser = (elm_ref('.') * digit.of_many1()).opt().map(|frac_opt| {
      if let Some(frac_digits) = frac_opt {
        let mut result = String::from(".");
        result.push_str(&frac_digits.iter().collect::<String>());
        result
      } else {
        String::new()
      }
    });

    // Combine integer and fraction parts using a different approach
    let number_parser = (int_parser + frac_parser).map(|(int_str, frac_str)| {
      let mut result = int_str;
      result.push_str(&frac_str);
      result
    });

    // Convert to Decimal and wrap in Expr::Value and Rc
    number_parser
      .map_res(|s| Decimal::from_str(&s))
      .map(Expr::Value)
      .map(Rc::new)
  }

  pub fn calculator_static<'a>() -> StaticParser<'a, char, Rc<Expr>> {
    expr_static() - end()
  }
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

// StaticParserを使用したテスト
#[test]
fn test_multitive_static() {
  init();
  let source = r"1/2";
  let input = source.chars().collect::<Vec<_>>();
  println!("start");

  let result = static_parsers::multitive_static().parse_as_result(&input).unwrap();
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
fn test_additive_static() {
  init();
  let source = r"1+2*3+1";
  let input = source.chars().collect::<Vec<_>>();
  let result = static_parsers::additive_static().parse_as_result(&input).unwrap();
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
  // 従来のParserを使用した例
  {
    let s = "1+2*3+1";
    let input = s.chars().collect::<Vec<_>>();
    let result = calculator().parse(&input).to_result().unwrap();
    println!("Parser expr = {:?}", result);
    let n = eval(result.clone());
    println!("Parser: {} = {}", s, n);
  }

  // StaticParserを使用した例
  {
    let s = "1+2*3+1";
    let input = s.chars().collect::<Vec<_>>();
    let result = static_parsers::calculator_static().parse(&input).to_result().unwrap();
    println!("StaticParser expr = {:?}", result);
    let n = eval(result.clone());
    println!("StaticParser: {} = {}", s, n);
  }
}
