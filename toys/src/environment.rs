use crate::values::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
  bindings: HashMap<String, Value>,
  pub(crate) next: Option<Rc<Environment>>,
}

impl Environment {
  pub fn as_bindings_mut(&mut self) -> &HashMap<String, Value> {
    &mut self.bindings
  }

  pub fn as_bindings(&self) -> &HashMap<String, Value> {
    &self.bindings
  }

  pub fn find_binding(&self, name: &str) -> Option<&HashMap<String, Value>> {
    match self.bindings.get(name) {
      Some(_) => Some(&self.bindings),
      None => match &self.next {
        Some(n) => {
          let r = (*n).find_binding(name);
          r
        }
        None => None,
      },
    }
  }

  pub fn new(bindings: HashMap<String, Value>, next: Option<Rc<Environment>>) -> Environment {
    Self { bindings, next }
  }
}
