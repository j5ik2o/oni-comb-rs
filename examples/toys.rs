use oni_comb_rs::core::{Parser, ParserFunctor};
use oni_comb_rs::extension::parser::{ConversionParser, DiscardParser, OperatorParser, RepeatParser, SkipParser};
use oni_comb_rs::prelude::*;
use regex::Regex;
use std::rc::Rc;

#[derive(Debug)]
enum Operator {
  Add,            // +
  Subtract,       // -
  Multiply,       // *
  Divide,         // /
  LessThan,       // <
  LessOrEqual,    // <=
  GreaterThan,    // >
  GreaterOrEqual, // >=
  EqualEqual,     // ==
  NotEqual,       // !=
}

#[derive(Debug)]
enum Expr {
  Binary(Operator, Rc<Expr>, Rc<Expr>),
  IntegerLiteral(i64),
  Symbol(String),
  FunctionCall(String, Vec<Rc<Expr>>),
  LabelledCall(String, Vec<LabelledParameter>),
  Identifier(String),
  Plus(Rc<Expr>),
  Minus(Rc<Expr>),
  Add(Rc<Expr>, Rc<Expr>),
  Sub(Rc<Expr>, Rc<Expr>),
  Multiply(Rc<Expr>, Rc<Expr>),
  Divide(Rc<Expr>, Rc<Expr>),
  Println(Rc<Expr>),
}

impl Expr {
  pub fn of_add(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Add, lhs, rhs))
  }

  pub fn of_subtract(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Subtract, lhs, rhs))
  }

  pub fn of_multiply(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Multiply, lhs, rhs))
  }

  pub fn of_divide(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Divide, lhs, rhs))
  }
}

#[derive(Debug)]
struct LabelledParameter {
  name: String,
  parameter: Rc<Expr>,
}

impl LabelledParameter {
  pub fn new(name: String, parameter: Rc<Expr>) -> Self {
    Self { name, parameter }
  }
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  todo!()
}

fn lbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("[") - space()
}

fn rbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("]") - space()
}

fn lparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("{") - space()
}

fn rparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("}") - space()
}

fn comma<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(",") - space()
}

fn semi_colon<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(";") - space()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  (tag("println") * expression().surround(lparen(), rparen()) - semi_colon())
    .map(Expr::Println)
    .map(Rc::new)
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  regex(Regex::new(r#"-?\d+"#).unwrap())
    .convert(|s| s.parse::<i64>())
    .map(Expr::IntegerLiteral)
    .map(Rc::new)
}

fn plus<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('+') - space()
}

fn minus<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('-') - space()
}

fn aster<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('*') - space()
}

fn slash<'a>() -> Parser<'a, char, &'a char> {
  space() * elm_ref('/') - space()
}

fn multitive<'a>() {
  // let p = (aster() | slash()).map(|e| {
  //   match *e  {
  //     '+' => Expr::of_multiply,
  //     '-' => Expr::of_divide,
  //   }
  // });

  // let p3 = chain_left1(|| primary().map(|e| || e.clone()), || p);
  todo!()
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  (lparen() * expression() - rparen()) | integer() | function_call() | labelled_call() | identifier()
}

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = expression().of_many1_sep(comma()).surround(lparen(), rparen());
  (ident() + p)
    .map(|(name, params)| Expr::FunctionCall(name.to_string(), params))
    .map(Rc::new)
    .attempt()
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + expression()).map(|(label, param)| LabelledParameter::new(label, param));
  (ident() + param.of_many1_sep(comma()))
    .map(|(name, params)| Expr::LabelledCall(name.to_string(), params))
    .map(Rc::new)
    .attempt()
}

fn ident<'a>() -> Parser<'a, char, String> {
  regex(Regex::new(r#"[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap())
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::Symbol).map(Rc::new)
}

fn main() {}
