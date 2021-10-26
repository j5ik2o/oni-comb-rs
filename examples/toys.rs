use oni_comb_rs::core::{Parser, ParserFunctor, ParserMonad};
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
  pub fn of_multiply(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Expr {
    Expr::Binary(Operator::Multiply, lhs, rhs)
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

fn aster<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("*") - space()
}

fn slash<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("/") - space()
}

fn plus<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("+") - space()
}

fn minus<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("-") - space()
}

fn multitive<'a>() -> Parser<'a, char, &'a str> {
  let _mul = aster().map(|_e| Expr::of_multiply);
  todo!()
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

fn add<'a>() -> Parser<'a, char, &'a char> {
  space() * elm('+') - space()
}

fn sub<'a>() -> Parser<'a, char, &'a char> {
  space() * elm('-') - space()
}

fn mul<'a>() -> Parser<'a, char, &'a char> {
  space() * elm('*') - space()
}

fn div<'a>() -> Parser<'a, char, &'a char> {
  space() * elm('/') - space()
}

fn add_sub_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  mul_div_expr().flat_map(add_sub_rest)
}

fn add_sub_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
  let v1 = a.clone();
  let v2 = a.clone();
  let v3 = a.clone();
  let add_parser = add() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Add(v1.clone(), b.clone()))));
  let sub_parser = sub() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Sub(v2.clone(), b.clone()))));
  add_parser.attempt() | sub_parser.attempt() | empty().map(move |_| v3.clone())
}

fn mul_div_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  unary().flat_map(mul_div_rest)
}

fn mul_div_rest<'a>(a: Rc<Expr>) -> Parser<'a, char, Rc<Expr>> {
  let v1 = a.clone();
  let v2 = a.clone();
  let v3 = a.clone();
  let mul_parser = mul() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Multiply(v1.clone(), b.clone()))));
  let div_parser = div() * unary().flat_map(move |b| mul_div_rest(Rc::new(Expr::Divide(v2.clone(), b.clone()))));
  mul_parser.attempt() | div_parser.attempt() | empty().map(move |_| v3.clone())
}

fn unary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let unary_parser = ((elm('+') | elm('-')) + lazy(unary))
    .map(|(c, expr): (&char, Rc<Expr>)| match c {
      '-' => Expr::Minus(Rc::clone(&expr)),
      '+' => Expr::Plus(Rc::clone(&expr)),
      _ => panic!(),
    })
    .map(Rc::new);
  unary_parser | primary()
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
  let param = (ident() - elm('=') + expression()).map(|(label, param)| LabelledParameter::new(label, param));
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
