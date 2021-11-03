use oni_comb_rs::core::{Parser, ParserFunctor, ParserRunner};
use oni_comb_rs::extension::parser::{ConversionParser, DiscardParser, OperatorParser, RepeatParser, SkipParser};
use oni_comb_rs::prelude::*;
use regex::Regex;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
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
  While(Rc<Expr>, Rc<Expr>),
  If(Rc<Expr>, Rc<Expr>, Option<Rc<Expr>>),
  Block(Vec<Rc<Expr>>),
  Assignment(String, Rc<Expr>),
  ArrayLiteral(Vec<Rc<Expr>>),
  BoolLiteral(bool),
  GlobalVariableDefinition(String, Rc<Expr>),
  FunctionDefinition(String, Vec<String>, Rc<Expr>),
  Program(Vec<Rc<Expr>>),
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

fn program<'a>() -> Parser<'a, char, Rc<Expr>> {
  space() * top_level_definition().of_many0().map(Expr::Program).map(Rc::new)
}

fn top_level_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  global_variable_definition() | function_definition()
}

fn function_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let define_p = space() * tag("define") - space();
  let def_args_p = ident().of_many0_sep(comma()).surround(lparen(), rparen());
  (define_p + def_args_p + block_expr())
    .map(|((name, args), body)| Expr::FunctionDefinition(name.to_string(), args, body))
    .map(Rc::new)
}

fn global_variable_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let global_p = space() * tag("global") - space();
  let global_indent_p = global_p * ident();
  let eq = space() * tag("=") - space();
  let p = global_indent_p - eq + expression() - semi_colon();
  p.map(|(name, e)| Expr::GlobalVariableDefinition(name, e)).map(Rc::new)
}

fn lines<'a>() -> Parser<'a, char, Vec<Rc<Expr>>> {
  line().of_many1() - space() - end()
}

fn line<'a>() -> Parser<'a, char, Rc<Expr>> {
  println().attempt()
    | lazy(while_expr).attempt()
    | lazy(if_expr).attempt()
    | lazy(for_in_expr).attempt()
    | assignment().attempt()
    | expression_line().attempt()
    | block_expr()
}

fn while_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let while_p = space() * tag("while") - space();
  let condition = while_p * lazy(expression).surround(lparen(), rparen());
  (condition + lazy(line))
    .map(|(c, body)| Expr::While(c, body))
    .map(Rc::new)
}

fn for_in_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let for_p = space() * tag("for") - space();
  let p =
    for_p - lparen() * ident() - tag("in") + lazy(expression) - tag("to") + lazy(expression) - rparen() + lazy(line);
  p.map(|(((name, from), to), body)| {
    Rc::new(Expr::Block(vec![
      Rc::new(Expr::Assignment(name.to_string(), from)),
      Rc::new(Expr::While(
        Expr::of_less_than(Rc::new(Expr::Symbol(name.to_string())), to),
        Rc::new(Expr::Block(vec![
          body,
          Rc::new(Expr::Assignment(
            name.to_string(),
            Expr::of_add(
              Rc::new(Expr::Symbol(name.to_string())),
              Rc::new(Expr::IntegerLiteral(1)),
            ),
          )),
        ])),
      )),
    ]))
  })
}

fn if_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let condition = (space() * tag("if") - space()) * lparen() * expression() - rparen();
  (condition + line() + (tag("else") * line()).opt()).map(|((p1, p2), p3)| Rc::new(Expr::If(p1, p2, p3)))
}

fn block_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  lazy(line)
    .of_many0()
    .surround(lbrace(), rbrace())
    .map(|e| Expr::Block(e))
    .map(Rc::new)
}

fn assignment<'a>() -> Parser<'a, char, Rc<Expr>> {
  let eq = space() * tag("=") - space();
  (ident() - eq + expression() - semi_colon())
    .map(|(name, e)| Expr::Assignment(name, e))
    .map(Rc::new)
}

#[test]
fn test_assignment() {
  let source = r#"i = 1;"#;
  let input = source.chars().into_iter().collect::<Vec<_>>();
  let result = assignment().parse(&input).unwrap();
  if let &Expr::Assignment(ref name, ref expr) = &*result {
    assert_eq!(name, "i");
    if let &Expr::IntegerLiteral(i) = &*(*expr) {
      assert_eq!(i, 1);
    }
  }
}

fn expression_line<'a>() -> Parser<'a, char, Rc<Expr>> {
  (expression() - semi_colon()).attempt()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  comparative()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  let println_p = space() * tag("println") - space();
  (println_p * lazy(expression).surround(lparen(), rparen()) - semi_colon())
    .map(Expr::Println)
    .map(Rc::new)
}

#[test]
fn test_println() {
  let source = r#"println(10);"#;
  let input = source.chars().into_iter().collect::<Vec<_>>();
  let result = println().parse(&input).unwrap();
  if let &Expr::Println(ref expr) = &*result {
    if let &Expr::IntegerLiteral(i) = &*(*expr) {
      assert_eq!(i, 10);
    }
  }
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  (space() * regex(Regex::new(r#"-?\d+"#).unwrap()) - space())
    .convert(|s| s.parse::<i64>())
    .map(Expr::IntegerLiteral)
    .map(Rc::new)
}

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let aster = space() * tag("*") - space();
  let slash = space() * tag("/") - space();

  chain_left1(
    primary(),
    (aster | slash).map(|e| match e {
      "*" => Expr::of_multiply,
      "/" => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let plus = space() * tag("+") - space();
  let minus = space() * tag("-") - space();

  chain_left1(
    multitive(),
    (plus | minus).map(|e| match e {
      "+" => Expr::of_add,
      "-" => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }),
  )
}

fn comparative<'a>() -> Parser<'a, char, Rc<Expr>> {
  let lt = space() * tag("<") - space();
  let lte = space() * tag("<=") - space();
  let gt = space() * tag(">") - space();
  let gte = space() * tag(">=") - space();
  let eqeq = space() * tag("==") - space();
  let neq = space() * tag("!=") - space();

  chain_left1(
    additive(),
    (lte.attempt() | gte.attempt() | neq.attempt() | lt.attempt() | gt.attempt() | eqeq.attempt()).map(|e| match e {
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

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression).of_many1_sep(comma()).surround(lparen(), rparen());
  (ident() + p)
    .map(|(name, params)| Expr::FunctionCall(name.to_string(), params))
    .map(Rc::new)
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + lazy(expression)).map(|(label, param)| LabelledParameter::new(label, param));
  (ident() + param.of_many1_sep(comma()))
    .map(|(name, params)| Expr::LabelledCall(name.to_string(), params))
    .map(Rc::new)
}

fn true_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("true") - space()
}

fn false_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("false") - space()
}

fn array_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  lazy(expression)
    .of_many0_sep(comma())
    .surround(lbracket(), rbracket())
    .map(|e| Expr::ArrayLiteral(e))
    .map(Rc::new)
}

fn bool_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  (true_literal().map(|_| Expr::BoolLiteral(true)) | false_literal().map(|_| Expr::BoolLiteral(false))).map(Rc::new)
}

fn ident<'a>() -> Parser<'a, char, String> {
  space() * regex(Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap()) - space()
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  ident().map(Expr::Symbol).map(Rc::new)
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  (lparen() * lazy(expression) - rparen())
    | function_call().attempt()
    | labelled_call().attempt()
    | array_literal().attempt()
    | bool_literal().attempt()
    | integer()
    | identifier()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_primary_integer() {
    let source = r#"10"#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    if let &Expr::IntegerLiteral(i) = &*result {
      assert_eq!(i, 10);
    }
  }

  #[test]
  fn test_primary_identifier() {
    let source = r#"a"#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Symbol(name) = &*result {
      assert_eq!(name, "a");
    } else {
      panic!()
    }
  }

  #[test]
  fn test_multitive() {
    let source = r#"1/2"#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = multitive().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::Divide);
      if let Expr::IntegerLiteral(l) = &**lhs {
        assert_eq!(*l, 1);
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      }
    } else {
      panic!()
    }
  }

  #[test]
  fn test_additive() {
    let source = r#"1+2"#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = additive().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::Add);
      if let Expr::IntegerLiteral(l) = &**lhs {
        assert_eq!(*l, 1);
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      }
    } else {
      panic!()
    }
  }
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

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn main() {}
