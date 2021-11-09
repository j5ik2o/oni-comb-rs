use crate::environment::Environment;
use crate::expr::Expr;
use crate::operator::Operator;
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

  pub fn get_value(&self, name: &str) -> &i64 {
    self.variable_environment.as_bindings().get(name).unwrap()
  }

  pub fn call_main(&mut self, expr: Rc<Expr>) -> i64 {
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

  pub fn interpret(&mut self, expr: Rc<Expr>) -> i64 {
    match &*expr {
      Expr::Binary(op, lhs, rhs) => {
        let lhs = self.interpret(lhs.clone());
        let rhs = self.interpret(rhs.clone());
        match op {
          Operator::Mod => lhs % rhs,
          Operator::Add => lhs + rhs,
          Operator::Subtract => lhs - rhs,
          Operator::Multiply => lhs * rhs,
          Operator::Divide => lhs / rhs,
          Operator::LessThan => {
            if lhs < rhs {
              1
            } else {
              0
            }
          }
          Operator::LessOrEqual => {
            if lhs <= rhs {
              1
            } else {
              0
            }
          }
          Operator::GreaterThan => {
            if lhs > rhs {
              1
            } else {
              0
            }
          }
          Operator::GreaterOrEqual => {
            if lhs >= rhs {
              1
            } else {
              0
            }
          }
          Operator::EqualEqual => {
            if lhs == rhs {
              1
            } else {
              0
            }
          }
          Operator::NotEqual => {
            if lhs != rhs {
              1
            } else {
              0
            }
          }
        }
      }
      Expr::IntegerLiteral(value) => *value,
      Expr::Parenthesized(expr) => self.interpret(expr.clone()),
      Expr::Symbol(name) => {
        let bindings_opt = self.variable_environment.find_binding(name);
        let v = bindings_opt.unwrap().get(name).unwrap();
        *v
      }
      Expr::FunctionCall(name, actual_params) => {
        if let Expr::FunctionDefinition(_def_name, formal_parmas, body) =
          &*self.function_environment.get(name).unwrap().clone()
        {
          let values = actual_params
            .iter()
            .map(|actual_param| self.interpret(actual_param.clone()))
            .collect::<Vec<_>>();
          let backup = self.variable_environment.clone();
          self.variable_environment = Environment::new(HashMap::new(), Some(Rc::new(backup.clone())));
          let mut i = 0;
          for formal_param_name in formal_parmas {
            let mut bindings = self.variable_environment.as_bindings().clone();
            bindings.insert(formal_param_name.clone(), values[i]);
            i += 1;
          }
          let result = self.interpret(body.clone());
          self.variable_environment = backup.clone();
          result
        } else {
          panic!("Function {} not defined", name);
        }
      }
      Expr::Assignment(name, expr) => {
        let bindings_opt = self.variable_environment.find_binding(name);
        if bindings_opt.is_some() {
          let value = self.interpret(expr.clone());
          let mut bindings = self.variable_environment.as_bindings().clone();
          let r = bindings.get_mut(name).unwrap();
          *r = value;
          self.variable_environment = Environment::new(bindings, self.variable_environment.next.clone());
          value
        } else {
          let value = self.interpret(expr.clone());
          let mut bindings = self.variable_environment.as_bindings().clone();
          bindings.insert(name.clone(), value);
          self.variable_environment = Environment::new(bindings, self.variable_environment.next.clone());
          value
        }
      }
      Expr::Block(exprs) => {
        let mut value = 0;
        for expr in exprs {
          value = self.interpret(expr.clone());
        }
        value
      }
      Expr::Println(args) => {
        let value = self.interpret(args.clone());
        println!("{}", value);
        value
      }
      Expr::If(condition, body, else_body) => {
        let cond = self.interpret(condition.clone());
        if cond != 0 {
          self.interpret(body.clone())
        } else {
          else_body.as_ref().map(|e| self.interpret(e.clone())).unwrap_or(1)
        }
      }
      Expr::While(cond, body) => {
        loop {
          let condition = self.interpret(cond.clone());
          if condition != 0 {
            self.interpret(body.clone());
          } else {
            break;
          }
        }
        1
      }
      expr => panic!("must not reach here: {:?}", expr),
    }
  }
}
