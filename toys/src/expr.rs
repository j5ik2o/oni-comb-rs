use crate::labelled_parameter::LabelledParameter;
use crate::operator::Operator;

use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Binary(Operator, Rc<Expr>, Rc<Expr>),
  IntegerLiteral(i64),
  Symbol(String),
  FunctionCall(String, Vec<Rc<Expr>>),
  LabelledCall(String, Vec<LabelledParameter>),
  Identifier(String),
  Plus(Rc<Expr>),
  Minus(Rc<Expr>),
  Println(Rc<Expr>),
  While(Rc<Expr>, Rc<Expr>),
  If(Rc<Expr>, Rc<Expr>, Option<Rc<Expr>>),
  Block(Vec<Rc<Expr>>),
  Assignment(String, Rc<Expr>),
  ArrayLiteral(Vec<Rc<Expr>>),
  BoolLiteral(bool),
  Parenthesized(Rc<Expr>),
  GlobalVariableDefinition(String, Rc<Expr>),
  FunctionDefinition(String, Vec<String>, Rc<Expr>),
  Program(Vec<Rc<Expr>>),
}

impl Expr {
  pub fn of_binary(operator: Operator, lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(operator, lhs, rhs))
  }

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

  pub fn of_mod(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Mod, lhs, rhs))
  }

  pub fn of_multiply(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Multiply, lhs, rhs))
  }

  pub fn of_divide(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Binary(Operator::Divide, lhs, rhs))
  }
}
