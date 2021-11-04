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
  pub fn of_global_variable_definition(name: String, value: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::GlobalVariableDefinition(name, value))
  }

  pub fn of_function_definition(name: String, parameters: Vec<String>, body: Rc<Expr>) -> Rc<Self> {
    Rc::new(Expr::FunctionDefinition(name, parameters, body))
  }

  pub fn of_function_call(name: String, args: Vec<Rc<Expr>>) -> Rc<Expr> {
    Rc::new(Expr::FunctionCall(name, args))
  }

  pub fn of_labelled_call(name: String, args: Vec<LabelledParameter>) -> Rc<Expr> {
    Rc::new(Expr::LabelledCall(name, args))
  }

  pub fn of_symbol(symbol: String) -> Rc<Expr> {
    Rc::new(Expr::Symbol(symbol))
  }

  pub fn of_bool_literal(value: bool) -> Rc<Expr> {
    Rc::new(Expr::BoolLiteral(value))
  }

  pub fn of_integer_literal(value: i64) -> Rc<Expr> {
    Rc::new(Expr::IntegerLiteral(value))
  }

  pub fn of_array_literal(values: Vec<Rc<Expr>>) -> Rc<Expr> {
    Rc::new(Expr::ArrayLiteral(values))
  }

  pub fn of_println(expr: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Println(expr))
  }

  pub fn of_block(block: Vec<Rc<Expr>>) -> Rc<Expr> {
    Rc::new(Expr::Block(block))
  }

  pub fn of_while(condition: Rc<Expr>, body: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::While(condition, body))
  }

  pub fn of_if(condition: Rc<Expr>, then: Rc<Expr>, else_: Option<Rc<Expr>>) -> Rc<Expr> {
    Rc::new(Expr::If(condition, then, else_))
  }

  pub fn of_assignment(name: String, value: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Assignment(name, value))
  }

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
  let p = (define_p + def_args_p + block_expr())
    .map(|((name, args), body)| Expr::of_function_definition(name.to_string(), args, body));
  space() * p - space()
}

fn global_variable_definition<'a>() -> Parser<'a, char, Rc<Expr>> {
  let global_p = space() * tag("global") - space();
  let global_indent_p = global_p * ident();
  let eq = space() * tag("=") - space();
  let p =
    (global_indent_p - eq + expression() - semi_colon()).map(|(name, e)| Expr::of_global_variable_definition(name, e));
  space() * p - space()
}

fn lines<'a>() -> Parser<'a, char, Vec<Rc<Expr>>> {
  line().of_many1() - space() - end()
}

fn line<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p =
    println() | lazy(while_expr) | lazy(if_expr) | lazy(for_in_expr) | assignment() | expression_line() | block_expr();
  space() * p - space()
}

fn while_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let while_p = space() * tag("while") - space();
  let condition = while_p * lazy(expression).surround(lparen(), rparen());
  let p = (condition + lazy(line)).map(|(c, body)| Expr::of_while(c, body));
  (space() * p - space()).attempt()
}

fn for_in_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let for_p = tag("for");
  let in_p = tag("in");
  let to_p = tag("to");

  let params_p = (lparen().logging("lparen") * ident().logging("ident"))
    + (in_p.logging("in") * expression().logging("in_expr ="))
    + (to_p * expression())
    - rparen();
  let p0 = for_p.logging("for_p") * params_p.logging("params") + lazy(line);
  let p = p0.logging("for_in").map(|(((name, from), to), body)| {
    Expr::of_block(vec![
      Expr::of_assignment(name.to_string(), from),
      Expr::of_while(
        Expr::of_less_than(Expr::of_symbol(name.to_string()), to),
        Expr::of_block(vec![
          body,
          Expr::of_assignment(
            name.to_string(),
            Expr::of_add(Expr::of_symbol(name.to_string()), Expr::of_integer_literal(1)),
          ),
        ]),
      ),
    ])
  });
  (space() * p - space()).attempt()
}

fn if_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let condition = (space() * tag("if") - space()) * lparen() * expression() - rparen();
  let else_p = space() * tag("else") - space();
  let p = (condition + line() + (else_p * line()).opt()).map(|((p1, p2), p3)| Expr::of_if(p1, p2, p3));
  (space() * p - space()).attempt()
}

fn block_expr<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(line).of_many0().surround(lbrace(), rbrace()).map(Expr::of_block);
  space() * p - space()
}

fn assignment<'a>() -> Parser<'a, char, Rc<Expr>> {
  let eq = space() * tag("=") - space();
  let p = (ident() - eq + expression() - semi_colon()).map(|(name, expr)| Expr::of_assignment(name, expr));
  (space() * p - space()).attempt()
}

fn expression_line<'a>() -> Parser<'a, char, Rc<Expr>> {
  (space() * expression() - semi_colon() - space()).attempt()
}

fn expression<'a>() -> Parser<'a, char, Rc<Expr>> {
  space() * comparative() - space()
}

fn println<'a>() -> Parser<'a, char, Rc<Expr>> {
  let println_p = space() * tag("println") - space();
  let p = (println_p * lazy(expression).surround(lparen(), rparen()) - semi_colon()).map(Expr::of_println);
  (space() * p - space()).attempt()
}

fn integer<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = regex(Regex::new(r#"-?\d+"#).unwrap())
    .convert(|s| s.parse::<i64>())
    .map(Expr::of_integer_literal);
  space() * p - space()
}

fn multitive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let aster = space() * tag("*") - space();
  let slash = space() * tag("/") - space();

  let p = chain_left1(
    primary(),
    (aster | slash).map(|e| match e {
      "*" => Expr::of_multiply,
      "/" => Expr::of_divide,
      _ => panic!("unexpected operator"),
    }),
  );
  space() * p - space()
}

fn additive<'a>() -> Parser<'a, char, Rc<Expr>> {
  let plus = space() * tag("+") - space();
  let minus = space() * tag("-") - space();

  let p = chain_left1(
    multitive(),
    (plus | minus).map(|e| match e {
      "+" => Expr::of_add,
      "-" => Expr::of_subtract,
      _ => panic!("unexpected operator"),
    }),
  );
  space() * p - space()
}

fn comparative<'a>() -> Parser<'a, char, Rc<Expr>> {
  let lt = space() * tag("<") - space();
  let lte = space() * tag("<=") - space();
  let gt = space() * tag(">") - space();
  let gte = space() * tag(">=") - space();
  let eqeq = space() * tag("==") - space();
  let neq = space() * tag("!=") - space();

  let p = chain_left1(
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
  );
  space() * p - space()
}

fn function_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (ident() + lazy(expression).of_many0_sep(comma()).surround(lparen(), rparen()))
    .map(|(name, params)| Expr::of_function_call(name.to_string(), params));
  (space() * p - space()).attempt()
}

fn labelled_call<'a>() -> Parser<'a, char, Rc<Expr>> {
  let param = (ident() - elm_ref('=') + lazy(expression)).map(|(label, param)| LabelledParameter::new(label, param));
  let p = (ident() + param.of_many1_sep(comma()).surround(lbracket(), rbracket()))
    .map(|(name, params)| Expr::of_labelled_call(name.to_string(), params));
  (space() * p - space()).attempt()
}

fn true_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("true") - space()
}

fn false_literal<'a>() -> Parser<'a, char, &'a str> {
  space() * tag("false") - space()
}

fn array_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = lazy(expression)
    .of_many0_sep(comma())
    .surround(lbracket(), rbracket())
    .map(Expr::of_array_literal);
  space() * p - space()
}

fn bool_literal<'a>() -> Parser<'a, char, Rc<Expr>> {
  true_literal().map(|_| Expr::of_bool_literal(true)) | false_literal().map(|_| Expr::of_bool_literal(false))
}

fn ident<'a>() -> Parser<'a, char, String> {
  space() * regex(Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap()) - space()
}

fn identifier<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = ident().map(Expr::of_symbol);
  space() * p - space()
}

fn primary<'a>() -> Parser<'a, char, Rc<Expr>> {
  let p = (lparen() * lazy(expression) - rparen())
    | integer()
    | function_call()
    | labelled_call()
    | array_literal()
    | bool_literal()
    | identifier();
  space() * p - space()
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::LabelledParameter;
  use std::env;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_example() {
    let source = r#"
    a = 1;
    b = 2;
    c = a + b;
    println(c);
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = lines().parse(&input).unwrap();
    println!("{:?}", result);
  }

  #[test]
  fn test_while() {
    let source = r#"
    while (1==2) { 1; }
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = line().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::While(cond, body) = &*result {
      if let Expr::Binary(op, a, b) = &*(*cond) {
        assert_eq!(*op, Operator::EqualEqual);
        if let &Expr::IntegerLiteral(ai) = &*(*a) {
          assert_eq!(ai, 1);
        } else {
          panic!("unexpected result");
        }
        if let &Expr::IntegerLiteral(bi) = &*(*b) {
          assert_eq!(bi, 2);
        } else {
          panic!("unexpected result");
        }
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_for() {
    init();
    let source = r#"
    for(i in 20 to 30) { 1; }
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = for_in_expr().parse(&input).unwrap();
    println!("{:?}", result);
    // if let Expr::While(cond, body) = &*result {
    //   if let Expr::Binary(op, a, b) = &*(*cond) {
    //     assert_eq!(*op, Operator::EqualEqual);
    //     if let &Expr::IntegerLiteral(ai) = &*(*a) {
    //       assert_eq!(ai, 1);
    //     } else {
    //       panic!("unexpected result");
    //     }
    //     if let &Expr::IntegerLiteral(bi) = &*(*b) {
    //       assert_eq!(bi, 2);
    //     } else {
    //       panic!("unexpected result");
    //     }
    //   } else {
    //     panic!("unexpected result");
    //   }
    // } else {
    //   panic!("unexpected result");
    // }
  }

  #[test]
  fn test_if() {
    let source = r#"
    if (1==2) { 1; }
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = line().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::If(cond, body, ..) = &*result {
      if let Expr::Binary(op, a, b) = &*(*cond) {
        assert_eq!(*op, Operator::EqualEqual);
        if let &Expr::IntegerLiteral(ai) = &*(*a) {
          assert_eq!(ai, 1);
        } else {
          panic!("unexpected result");
        }
        if let &Expr::IntegerLiteral(bi) = &*(*b) {
          assert_eq!(bi, 2);
        } else {
          panic!("unexpected result");
        }
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_if_2() {
    let source = r#"
    if (a==2) { 1; }
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = line().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::If(cond, body, ..) = &*result {
      if let Expr::Binary(op, a, b) = &*(*cond) {
        assert_eq!(*op, Operator::EqualEqual);
        if let Expr::Symbol(a) = &*(*a) {
          assert_eq!(a, "a");
        } else {
          panic!("unexpected result");
        }
        if let &Expr::IntegerLiteral(bi) = &*(*b) {
          assert_eq!(bi, 2);
        } else {
          panic!("unexpected result");
        }
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_assignment() {
    let source = r#"
    i = 1;
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = line().parse(&input).unwrap();
    println!("{:?}", result);
    if let &Expr::Assignment(ref name, ref expr) = &*result {
      assert_eq!(name, "i");
      if let &Expr::IntegerLiteral(i) = &*(*expr) {
        assert_eq!(i, 1);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_println() {
    let source = r#"
    println(10);
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = line().parse(&input).unwrap();
    println!("{:?}", result);
    if let &Expr::Println(ref expr) = &*result {
      if let &Expr::IntegerLiteral(i) = &*(*expr) {
        assert_eq!(i, 10);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_labelled_call_args_1() {
    let source = r#"
    abc[n = 5]
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::LabelledCall(func_name, args) = &*result {
      assert_eq!(func_name, "abc");
      if let LabelledParameter { name, parameter } = &args[0] {
        assert_eq!(name, "n");
        if let &Expr::IntegerLiteral(i) = &*(*parameter) {
          assert_eq!(i, 5);
        } else {
          panic!("unexpected result");
        }
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_function_call_args_0() {
    let source = r#"
    abc();
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::FunctionCall(func_name, args) = &*result {
      assert_eq!(func_name, "abc");
      assert!(args.is_empty());
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_function_call_args_1() {
    let source = r#"
    abc(1);
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::FunctionCall(func_name, args) = &*result {
      assert_eq!(func_name, "abc");
      if let &Expr::IntegerLiteral(i) = &*(args[0]) {
        assert_eq!(i, 1);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_function_call_args_2() {
    let source = r#"
    abc(1, 2);
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::FunctionCall(func_name, args) = &*result {
      assert_eq!(func_name, "abc");
      if let &Expr::IntegerLiteral(i) = &*(args[0]) {
        assert_eq!(i, 1);
      } else {
        panic!("unexpected result");
      }
      if let &Expr::IntegerLiteral(i) = &*(args[1]) {
        assert_eq!(i, 2);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_bool_true() {
    let source = r#"
    true
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let &Expr::BoolLiteral(b) = &*result {
      assert!(b);
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_bool_false() {
    let source = r#"
    false
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let &Expr::BoolLiteral(b) = &*result {
      assert!(!b);
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_bool_array_0() {
    let source = r#"
    []
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::ArrayLiteral(v) = &*result {
      assert!(v.is_empty());
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_bool_array_1() {
    let source = r#"
    [1]
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::ArrayLiteral(v) = &*result {
      assert!(!v.is_empty());
      if let &Expr::IntegerLiteral(i) = &*v[0] {
        assert_eq!(i, 1);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_bool_array_2() {
    let source = r#"
    [1, 2]
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::ArrayLiteral(v) = &*result {
      assert!(!v.is_empty());
      if let &Expr::IntegerLiteral(i) = &*v[0] {
        assert_eq!(i, 1);
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_integer() {
    let source = r#"
    10
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let &Expr::IntegerLiteral(i) = &*result {
      assert_eq!(i, 10);
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_primary_identifier() {
    let source = r#"
    abc
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = primary().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Symbol(name) = &*result {
      assert_eq!(name, "abc");
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_multitive() {
    let source = r#"1/2"#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = expression().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::Divide);
      if let Expr::IntegerLiteral(l) = &**lhs {
        assert_eq!(*l, 1);
      } else {
        panic!("unexpected result");
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_additive() {
    let source = r#"
    1 + 2
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = expression().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::Add);
      if let Expr::IntegerLiteral(l) = &**lhs {
        assert_eq!(*l, 1);
      } else {
        panic!("unexpected result");
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_comparative() {
    let source = r#"
    1>2
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = expression().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::GreaterThan);
      if let Expr::IntegerLiteral(l) = &**lhs {
        assert_eq!(*l, 1);
      } else {
        panic!("unexpected result");
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
    }
  }

  #[test]
  fn test_comparative_2() {
    let source = r#"
    a>2
    "#;
    let input = source.chars().into_iter().collect::<Vec<_>>();
    let result = expression().parse(&input).unwrap();
    println!("{:?}", result);
    if let Expr::Binary(op, lhs, rhs) = &*result {
      assert_eq!(*op, Operator::GreaterThan);
      if let Expr::Symbol(l) = &**lhs {
        assert_eq!(*l, "a");
      } else {
        panic!("unexpected result");
      }
      if let Expr::IntegerLiteral(r) = &**rhs {
        assert_eq!(*r, 2);
      } else {
        panic!("unexpected result");
      }
    } else {
      panic!("unexpected result");
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
