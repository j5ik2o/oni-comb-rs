use crate::environment::Environment;
use crate::expr::Expr;
use crate::operator::Operator;
use crate::values::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
  variable_environment: Environment,
  function_environment: HashMap<String, Rc<Expr>>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      variable_environment: Environment::new(HashMap::new(), None),
      function_environment: HashMap::new(),
    }
  }

  pub fn reset(&mut self) {
    self.variable_environment = Environment::new(HashMap::new(), None);
    self.function_environment.clear();
  }

  pub fn get_value(&self, name: &str) -> &Value {
    self.variable_environment.as_bindings().get(name).unwrap()
  }

  pub fn call_main(&mut self, expr: Rc<Expr>) -> Value {
    match &*expr {
      Expr::Program(definitions) => {
        for top_level in definitions {
          match &**top_level {
            Expr::GlobalVariableDefinition(name, expr) => {
              let mut bindings = self.variable_environment.as_bindings().clone();
              bindings.insert(name.clone(), self.interpret(expr.clone()));
            }
            Expr::FunctionDefinition(name, ..) => {
              self.function_environment.insert(name.clone(), top_level.clone());
            }
            _ => panic!("unexpected top level expression"),
          }
        }
        let main_function = self.function_environment.get("main");
        match main_function {
          Some(mf) => match &**mf {
            Expr::FunctionDefinition(_, _, body) => self.interpret(body.clone()),
            _ => panic!("unexpected main function expression"),
          },
          None => panic!("No main function found"),
        }
      }
      _ => panic!("main is not a function"),
    }
  }

  pub fn interpret(&mut self, expr: Rc<Expr>) -> Value {
    match &*expr {
      Expr::Binary(op, lhs, rhs) => {
        let lhs = self.interpret(lhs.clone());
        let rhs = self.interpret(rhs.clone());
        match op {
          Operator::And => Value::Bool(lhs.as_bool() && rhs.as_bool()),
          Operator::Or => Value::Bool(lhs.as_bool() || rhs.as_bool()),
          Operator::Mod => Value::Int(lhs.as_int() % rhs.as_int()),
          Operator::Add => Value::Int(lhs.as_int() + rhs.as_int()),
          Operator::Subtract => Value::Int(lhs.as_int() - rhs.as_int()),
          Operator::Multiply => Value::Int(lhs.as_int() * rhs.as_int()),
          Operator::Divide => Value::Int(lhs.as_int() / rhs.as_int()),
          Operator::LessThan => Value::Bool(lhs.as_int() < rhs.as_int()),
          Operator::LessOrEqual => Value::Bool(lhs.as_int() <= rhs.as_int()),
          Operator::GreaterThan => Value::Bool(lhs.as_int() > rhs.as_int()),
          Operator::GreaterOrEqual => Value::Bool(lhs.as_int() >= rhs.as_int()),
          Operator::EqualEqual => Value::Bool(lhs.as_int() == rhs.as_int()),
          Operator::NotEqual => Value::Bool(lhs.as_int() != rhs.as_int()),
        }
      }
      Expr::StringLiteral(value) => Value::String(value.clone()),
      Expr::IntegerLiteral(value) => Value::Int(*value),
      Expr::Parenthesized(expr) => self.interpret(expr.clone()),
      Expr::Symbol(name) => {
        let bindings_opt = self.variable_environment.find_binding(name);
        let v = bindings_opt.unwrap().get(name).unwrap();
        v.clone()
      }
      Expr::Assignment(name, expr) => {
        let bindings_opt = self.variable_environment.find_binding(name);
        if bindings_opt.is_some() {
          let value = self.interpret(expr.clone());
          let mut bindings = self.variable_environment.as_bindings().clone();
          let r = bindings.get_mut(name).unwrap();
          *r = value.clone();
          self.variable_environment = Environment::new(bindings, self.variable_environment.next.clone());
          value
        } else {
          let value = self.interpret(expr.clone());
          let mut bindings = self.variable_environment.as_bindings().clone();
          bindings.insert(name.clone(), value.clone());
          self.variable_environment = Environment::new(bindings, self.variable_environment.next.clone());
          value
        }
      }
      Expr::Block(exprs) => {
        let mut value = None;
        for expr in exprs {
          value = Some(self.interpret(expr.clone()));
        }
        value.unwrap()
      }
      Expr::Println(args) => {
        let value = self.interpret(args.clone());
        println!("{}", value);
        value
      }
      Expr::If(condition, body, else_body) => {
        let cond = self.interpret(condition.clone());
        if cond.as_bool() {
          self.interpret(body.clone())
        } else {
          else_body
            .as_ref()
            .map(|e| self.interpret(e.clone()))
            .unwrap_or(Value::Bool(true))
        }
      }
      Expr::While(cond, body) => {
        loop {
          let condition = self.interpret(cond.clone());
          if condition.as_bool() {
            self.interpret(body.clone());
          } else {
            break;
          }
        }
        Value::Bool(true)
      }
      Expr::FunctionCall(name, actual_params) => {
        if let Expr::FunctionDefinition(_, formal_params, body) = &*self.function_environment.get(name).unwrap().clone()
        {
          let values = actual_params
            .iter()
            .map(|actual_param| self.interpret(actual_param.clone()))
            .collect::<Vec<_>>();
          let backup = self.variable_environment.clone();
          self.variable_environment = Environment::new(HashMap::new(), Some(Rc::new(backup.clone())));
          let mut i = 0;
          for formal_param_name in formal_params {
            let mut bindings = self.variable_environment.as_bindings().clone();
            bindings.insert(formal_param_name.clone(), values[i].clone());
            i += 1;
          }
          let result = self.interpret(body.clone());
          self.variable_environment = backup.clone();
          result
        } else {
          panic!("Function {} not defined", name);
        }
      }
      Expr::LabelledCall(name, actual_params) => {
        if let Expr::FunctionDefinition(_, _, body) = &*self.function_environment.get(name).unwrap().clone() {
          let name_with_values = actual_params
            .iter()
            .map(|actual_param| {
              (
                actual_param.name.clone(),
                self.interpret(actual_param.parameter.clone()),
              )
            })
            .collect::<Vec<_>>();
          let backup = self.variable_environment.clone();
          self.variable_environment = Environment::new(HashMap::new(), Some(Rc::new(backup.clone())));
          for (param_name, param_value) in name_with_values {
            let mut bindings = self.variable_environment.as_bindings().clone();
            bindings.insert(param_name, param_value);
          }
          let result = self.interpret(body.clone());
          self.variable_environment = backup.clone();
          result
        } else {
          panic!("Function {} not defined", name);
        }
      }
      expr => panic!("must not reach here: {:?}", expr),
    }
  }
}
