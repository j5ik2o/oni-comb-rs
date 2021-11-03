use oni_comb_rs::core::{Parser, ParserFunctor, ParserRunner};
use oni_comb_rs::extension::parser::{ConversionParser, DiscardParser, OperatorParser, RepeatParser, SkipParser};
use oni_comb_rs::prelude::*;
use regex::Regex;
use std::rc::Rc;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
  If(Rc<Expr>, Rc<Expr>, Option<Rc<Expr>>),
  Block(Vec<Rc<Expr>>),
  Assignment(String, Rc<Expr>),
  ArrayLiteral(Vec<Rc<Expr>>),
  BoolLiteral(bool),
}

impl Expr {
  pub fn of_less_than(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::LessThan, lhs, rhs))
  }

  pub fn of_greater_than(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::GreaterThan, lhs, rhs))
  }

  pub fn of_less_or_equal(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::LessOrEqual, lhs, rhs))
  }

  pub fn of_greater_or_equal(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::GreaterOrEqual, lhs, rhs))
  }

  pub fn of_equal_equal(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::EqualEqual, lhs, rhs))
  }

  pub fn of_not_equal(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::NotEqual, lhs, rhs))
  }

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

#[derive(Clone, Debug)]
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

fn line<'a>() -> Parser<'a, char, Rc<Expr>> {
  println() | lazy(if_expr) | assignment() | expression_line()
}

fn if_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let condition = tag("if") * lparen() * expression() - rparen();
  (condition + line() + (tag("else") * line()).opt()).map(|((p1, p2), p3)| Rc::new(Expr::If(p1, p2, p3)))
}

fn block_expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  (lbrace() * line().of_many0() - rbrace()).map(|e| Rc::new(Expr::Block(e)))
}

fn assignment<'a>() -> Parser<'a, char, Rc<Expr>> {
  (ident() - eq() + expression() - semi_colon()).map(|(name, e)| Rc::new(Expr::Assignment(name, e)))
}

fn expression_line<'a>() -> Parser<'a, char, Rc<Expr>> {
  (expression() - semi_colon()).attempt()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  comparative()
}

fn lbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("[") - space()
}

fn rbracket<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("]") - space()
}

fn lbrace<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("{") - space()
}

fn rbrace<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("}") - space()
}

fn lparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("(") - space()
}

fn rparen<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(")") - space()
}

fn comma<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(",") - space()
}

fn semi_colon<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(";") - space()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  (tag("println") * lazy(expression).surround(lparen(), rparen()) - semi_colon())
    .map(Expr::Println)
    .map(Rc::new)
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  regex(Regex::new(r#"-?\d+"#).unwrap())
    .convert(|s| s.parse::<i64>())
    .map(Expr::IntegerLiteral)
    .map(Rc::new)
}

fn plus<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("+") - space()
}

fn minus<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("-") - space()
}

fn aster<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("*") - space()
}

fn slash<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("/") - space()
}

fn lt<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("<") - space()
}

fn lte<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("<=") - space()
}

fn gt<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(">") - space()
}

fn gte<'a>() -> Parser<'a, char, &'a str> {
  space() * tag(">=") - space()
}

fn eq<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("=") - space()
}

fn eqeq<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("==") - space()
}

fn neq<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("!=") - space()
}

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  chain_left1(
    primary(),
    (aster() | slash()).map(|e| match e {
      "*" => Expr::of_multiply,
      "/" => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  chain_left1(
    multitive(),
    (plus() | minus()).map(|e| match e {
      "+" => Expr::of_add,
      "-" => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn comparative<'a>() -> Parser<'a, char, Rc<Expr>> {
  chain_left1(
    additive(),
    (lte() | gte() | neq() | lt() | gt() | eqeq()).map(|e| match e {
      "<" => Expr::of_less_than,
      "<=" => Expr::of_less_or_equal,
      ">" => Expr::of_greater_than,
      ">=" => Expr::of_greater_or_equal,
      "==" => Expr::of_equal_equal,
      "!=" => Expr::of_not_equal,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  (lparen() * lazy(expression) - rparen())
    | function_call()
    | labelled_call()
    | array_literal()
    | bool_literal()
    | integer()
    | identifier()
}

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression).of_many1_sep(comma()).surround(lparen(), rparen());
  (ident() + p)
    .map(|(name, params)| Expr::FunctionCall(name.to_string(), params))
    .map(Rc::new)
    .attempt()
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + lazy(expression)).map(|(label, param)| LabelledParameter::new(label, param));
  (ident() + param.of_many1_sep(comma()))
    .map(|(name, params)| Expr::LabelledCall(name.to_string(), params))
    .map(Rc::new)
    .attempt()
}

fn true_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("true") - space()
}

fn false_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("false") - space()
}

fn array_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression).of_many0_sep(comma());
  surround(lbracket(), p, rbracket()).map(|e| Rc::new(Expr::ArrayLiteral(e)))
}

fn bool_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  (true_literal().map(|e| Expr::BoolLiteral(true)) | false_literal().map(|e| Expr::BoolLiteral(false))).map(Rc::new)
}

fn ident<'a>() -> Parser<'a, char, String> {
  regex(Regex::new(r#"[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap())
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::Symbol).map(Rc::new)
}

#[test]
fn test_expression() {
  let input = "if (a==1) println(10);".chars().into_iter().collect::<Vec<_>>();
  let result = line().parse(&input);
  println!("{:?}", result);
}

fn main() {}
